use crate::bindings::types::StorageValue;
use crate::interchain_queries::helpers::decode_and_convert;
use crate::interchain_queries::types::{AddressBytes, KVReconstruct};
use crate::interchain_queries::v045::helpers::{
    create_account_denom_balance_key, create_delegation_key, create_fee_pool_key,
    create_gov_proposal_key, create_gov_proposal_votes_key, create_params_store_key,
    create_total_denom_key, create_validator_key, create_validator_signing_info_key,
    deconstruct_account_denom_balance_key,
};
use crate::interchain_queries::v045::types::BALANCES_PREFIX;
use crate::interchain_queries::v045::types::{
    Balances, Delegations, FeePool, GovernmentProposal, GovernmentProposalVotes, Proposal,
    ProposalVote, SigningInfo, StakingValidator, StdDelegation, TallyResult, TotalSupply,
    UnbondingDelegations, UnbondingEntry, UnbondingResponse, Validator as ContractValidator,
    ValidatorSigningInfo, WeightedVoteOption, DECIMAL_PLACES, KEY_BOND_DENOM, STAKING_STORE_KEY,
};
use crate::{NeutronError, NeutronResult};
use base64::prelude::*;
use base64::Engine;
use cosmos_sdk_proto::cosmos::base::v1beta1::{Coin, DecCoin};
use cosmos_sdk_proto::cosmos::distribution::v1beta1::FeePool as CosmosFeePool;
use cosmos_sdk_proto::cosmos::gov::v1beta1::{
    Proposal as CosmosProposal, TallyResult as CosmosTallyResult, Vote,
    WeightedVoteOption as CosmosWeightedVoteOption,
};
use cosmos_sdk_proto::cosmos::slashing::v1beta1::ValidatorSigningInfo as CosmosValidatorSigningInfo;
use cosmos_sdk_proto::cosmos::staking::v1beta1::{
    Commission, CommissionRates, Delegation, Description, Validator,
};
use cosmos_sdk_proto::traits::Message;
use cosmwasm_std::{to_json_binary, Addr, Binary, Coin as StdCoin, Decimal, Timestamp, Uint128};
use hex;
use std::ops::Mul;
use std::str::FromStr;

// raw hex data from KV storage created using https://github.com/neutron-org/icq-compliance-officer.
pub const BALANCES_HEX_RESPONSE: &str = "0a057374616b6512083939393939303030";
pub const TOTAL_SUPPLY_HEX_RESPONSE: &str = "333030303031303938";
pub const FEE_POOL_HEX_RESPONSE: &str =
    "0a1d0a057374616b6512143231393630303030303030303030303030303030";
pub const GOV_PROPOSAL_HEX_RESPONSE: &str = "0801129f010a202f636f736d6f732e676f762e763162657461312e5465787450726f706f73616c127b0a11416464204e65772056616c696461746f721266546869732070726f706f73616c20726571756573747320616464696e672061206e65772076616c696461746f7220746f20746865206e6574776f726b20746f20696d70726f766520646563656e7472616c697a6174696f6e20616e642073656375726974792e1801220c0a01301201301a01302201302a0c08c9fdd3a20610988990d103320c08c9c3dea20610988990d1033a0d0a057374616b65120431303030420b088092b8c398feffffff014a0b088092b8c398feffffff01";
pub const GOV_PROPOSAL_VOTES_HEX_RESPONSE: &str = "0804122d636f736d6f73313068397374633576366e7467657967663578663934356e6a717135683332723533757175767722170801121331303030303030303030303030303030303030";
pub const STAKING_DENOM_HEX_RESPONSE: &str = "227374616b6522";
pub const STAKING_VALIDATOR_HEX_RESPONSE: &str = "0a34636f736d6f7376616c6f706572313566716a706a39307275686a353771336c366135686461307274373767366d63656b326d747112430a1d2f636f736d6f732e63727970746f2e656432353531392e5075624b657912220a20b20c07b3eb900df72b48c24e9a2e06ff4fe73bbd255e433af8eae3b1988e698820032a09313030303030303030321b3130303030303030303030303030303030303030303030303030303a080a066d796e6f64654a00524a0a3b0a1231303030303030303030303030303030303012123230303030303030303030303030303030301a113130303030303030303030303030303030120b089cfcd3a20610e0dc890b5a0131";
pub const DELEGATOR_DELEGATIONS_HEX_RESPONSE: &str = "0a2d636f736d6f73313566716a706a39307275686a353771336c366135686461307274373767366d63757a3777386e1234636f736d6f7376616c6f706572313566716a706a39307275686a353771336c366135686461307274373767366d63656b326d74711a1b313030303030303030303030303030303030303030303030303030";
pub const DELEGATOR_UNBONDING_DELEGATIONS_HEX_RESPONSE: &str = "0a2d636f736d6f73316d396c33353878756e6868776473303536387a6134396d7a68767578783975787265357475641234636f736d6f7376616c6f7065723138686c356339786e35647a6532673530756177306c326d723032657735377a6b3061756b746e1a2108ed02120c08ba97f9ac0610f6abf18f021a0531303030302205313030303028011a2008f902120b08c797f9ac0610e59a89011a053230303030220532303030302802";
pub const VALIDATOR_SIGNING_INFO_HEX_RESPONSE: &str = "0a34636f736d6f7376616c636f6e73313966353366717132387636706d7a383737646e653735643464376c307236356432373530707718102200";

#[test]
fn test_balance_reconstruct() {
    struct TestCase {
        addr: String,
        coins: Vec<(String, Uint128)>,
    }
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            addr: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
            coins: vec![("uosmo".to_string(), Uint128::from(100u128))],
        },
        TestCase {
            addr: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
            coins: vec![
                ("uosmo".to_string(), Uint128::from(100u128)),
                ("uatom".to_string(), Uint128::from(500u128)),
                ("uluna".to_string(), Uint128::from(80u128)),
            ],
        },
        TestCase {
            addr: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
            coins: vec![],
        },
    ];

    for ts in test_cases {
        let mut st_values: Vec<StorageValue> = vec![];

        let converted_addr_bytes = decode_and_convert(ts.addr.as_str()).unwrap();
        for coin in &ts.coins {
            let balance_key =
                create_account_denom_balance_key(converted_addr_bytes.clone(), &coin.0).unwrap();

            let balance_amount = Coin {
                denom: coin.0.clone(),
                amount: coin.1.to_string(),
            };
            let s = StorageValue {
                storage_prefix: "".to_string(),
                key: Binary::new(balance_key),
                value: Binary::new(balance_amount.encode_to_vec()),
            };
            st_values.push(s);
        }

        let balances = Balances::reconstruct(&st_values).unwrap();
        assert_eq!(balances.coins.len(), ts.coins.len());
        for (i, coin) in balances.coins.iter().enumerate() {
            assert_eq!(coin.denom, ts.coins[i].0);
            assert_eq!(coin.amount, ts.coins[i].1)
        }
    }
}

#[test]
fn test_bank_total_supply_reconstruct() {
    struct TestValue {
        denom: String,
        amount: String,
    }
    struct TestCase {
        values: Vec<TestValue>,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            values: vec![TestValue {
                denom: "uatom".to_string(),
                amount: "100".to_string(),
            }],
        },
        TestCase {
            values: vec![
                TestValue {
                    denom: "uatom".to_string(),
                    amount: "100".to_string(),
                },
                TestValue {
                    denom: "uosmo".to_string(),
                    amount: "200".to_string(),
                },
            ],
        },
        TestCase { values: vec![] },
    ];

    for ts in test_cases {
        let mut st_values: Vec<StorageValue> = vec![];

        for case in &ts.values {
            let denom_key = create_total_denom_key(case.denom.as_str()).unwrap();
            let s = StorageValue {
                storage_prefix: "".to_string(),
                key: Binary::new(denom_key),
                value: Binary::new(case.amount.as_bytes().to_vec()),
            };
            st_values.push(s);
        }

        let total_supply = TotalSupply::reconstruct(&st_values).unwrap();
        assert_eq!(total_supply.coins.len(), ts.values.len());
        for (i, coin) in total_supply.coins.iter().enumerate() {
            assert_eq!(coin.denom, ts.values[i].denom);
            assert_eq!(
                coin.amount,
                Uint128::from_str(ts.values[i].amount.as_str()).unwrap()
            )
        }
    }
}

#[test]
fn test_staking_validators_reconstruct() {
    struct TestCase {
        validators: Vec<Validator>,
        expected_result: NeutronResult<StakingValidator>,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            validators: vec![Validator {
                operator_address: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3".to_string(),
                consensus_pubkey: None,
                jailed: false,
                status: 0,
                tokens: "1000000000000000000".to_string(),
                delegator_shares: "1000000000000000000".to_string(),
                description: None,
                unbonding_height: 0,
                unbonding_time: None,
                commission: None,
                min_self_delegation: "".to_string(),
            }],
            expected_result: Ok(StakingValidator {
                validators: vec![ContractValidator {
                    operator_address: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3"
                        .to_string(),
                    status: 0,
                    consensus_pubkey: None,
                    tokens: "1000000000000000000".to_string(),
                    delegator_shares: "1000000000000000000".to_string(),
                    moniker: None,
                    identity: None,
                    website: None,
                    security_contact: None,
                    details: None,
                    unbonding_height: 0,
                    unbonding_time: None,
                    rate: None,
                    max_rate: None,
                    max_change_rate: None,
                    update_time: None,
                    min_self_delegation: Decimal::from_str("0").unwrap(),
                    jailed: false,
                }],
            }),
        },
        TestCase {
            validators: vec![Validator {
                operator_address: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3".to_string(),
                consensus_pubkey: Some(prost_types::Any {
                    type_url: "consensus_pubkey".to_string(),
                    value: vec![],
                }),
                jailed: false,
                status: 0,
                tokens: "1000000000000000000".to_string(),
                delegator_shares: "1000000000000000000".to_string(),
                description: Some(Description {
                    moniker: "Test validator".to_string(),
                    identity: "JHFDHHFHF".to_string(),
                    website: "https://neutron.org".to_string(),
                    security_contact: "".to_string(),
                    details: "Validator details".to_string(),
                }),
                unbonding_height: 0,
                unbonding_time: Some(prost_types::Timestamp {
                    seconds: 1203981203,
                    nanos: 123123,
                }),
                commission: Some(Commission {
                    commission_rates: Some(CommissionRates {
                        rate: "5000000000000000000".to_string(), // Dec(5) is 5+18 zeros
                        max_rate: "20000000000000000000".to_string(), // Dec(20) is 20+18 zeros
                        max_change_rate: "1000000000000000000".to_string(), // Dec(1) is 1+18 zeros
                    }),
                    update_time: Some(prost_types::Timestamp {
                        seconds: 56324234,
                        nanos: 1343,
                    }),
                }),
                min_self_delegation: "".to_string(),
            }],
            expected_result: Ok(StakingValidator {
                validators: vec![ContractValidator {
                    operator_address: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3"
                        .to_string(),
                    status: 0,
                    consensus_pubkey: Some(vec![]),
                    tokens: "1000000000000000000".to_string(),
                    delegator_shares: "1000000000000000000".to_string(),
                    moniker: Some("Test validator".to_string()),
                    identity: Some("JHFDHHFHF".to_string()),
                    website: Some("https://neutron.org".to_string()),
                    security_contact: Some("".to_string()),
                    details: Some("Validator details".to_string()),
                    unbonding_height: 0,
                    unbonding_time: Some(1203981203),
                    rate: Some(Decimal::from_str("5").unwrap()),
                    max_rate: Some(Decimal::from_str("20").unwrap()),
                    max_change_rate: Some(Decimal::from_str("1").unwrap()),
                    update_time: Some(56324234),
                    min_self_delegation: Decimal::from_str("0").unwrap(),
                    jailed: false,
                }],
            }),
        },
        TestCase {
            validators: vec![
                Validator {
                    operator_address: "cosmosvaloper132juzk0gdmwuxvx4phug7m3ymyatxlh9734g4w"
                        .to_string(),
                    consensus_pubkey: Some(prost_types::Any {
                        type_url: "consensus_pubkey".to_string(),
                        value: vec![1u8, 2u8, 3u8, 4u8],
                    }),
                    jailed: false,
                    status: 0,
                    tokens: "1000000000000000000".to_string(),
                    delegator_shares: "1000000000000000000".to_string(),
                    description: None,
                    unbonding_height: 0,
                    unbonding_time: None,
                    commission: None,
                    min_self_delegation: "".to_string(),
                },
                Validator {
                    operator_address: "cosmosvaloper1sjllsnramtg3ewxqwwrwjxfgc4n4ef9u2lcnj0"
                        .to_string(),
                    consensus_pubkey: None,
                    jailed: false,
                    status: 0,
                    tokens: "2000000000000000000".to_string(),
                    delegator_shares: "3000000000000000000".to_string(),
                    description: None,
                    unbonding_height: 0,
                    unbonding_time: None,
                    commission: None,
                    min_self_delegation: "".to_string(),
                },
            ],
            expected_result: Ok(StakingValidator {
                validators: vec![
                    ContractValidator {
                        operator_address: "cosmosvaloper132juzk0gdmwuxvx4phug7m3ymyatxlh9734g4w"
                            .to_string(),
                        status: 0,
                        consensus_pubkey: Some(vec![1u8, 2u8, 3u8, 4u8]),
                        tokens: "1000000000000000000".to_string(),
                        delegator_shares: "1000000000000000000".to_string(),
                        moniker: None,
                        identity: None,
                        website: None,
                        security_contact: None,
                        details: None,
                        unbonding_height: 0,
                        unbonding_time: None,
                        rate: None,
                        max_rate: None,
                        max_change_rate: None,
                        update_time: None,
                        min_self_delegation: Decimal::from_str("0").unwrap(),
                        jailed: false,
                    },
                    ContractValidator {
                        operator_address: "cosmosvaloper1sjllsnramtg3ewxqwwrwjxfgc4n4ef9u2lcnj0"
                            .to_string(),
                        status: 0,
                        consensus_pubkey: None,
                        tokens: "2000000000000000000".to_string(),
                        delegator_shares: "3000000000000000000".to_string(),
                        moniker: None,
                        identity: None,
                        website: None,
                        security_contact: None,
                        details: None,
                        unbonding_height: 0,
                        unbonding_time: None,
                        rate: None,
                        max_rate: None,
                        max_change_rate: None,
                        update_time: None,
                        min_self_delegation: Decimal::from_str("0").unwrap(),
                        jailed: false,
                    },
                ],
            }),
        },
        TestCase {
            validators: vec![],
            expected_result: Ok(StakingValidator { validators: vec![] }),
        },
    ];

    for ts in test_cases {
        let mut st_values: Vec<StorageValue> = vec![];

        for validator in &ts.validators {
            let val_addr = decode_and_convert(validator.operator_address.as_str()).unwrap();

            let validator_key = create_validator_key(&val_addr).unwrap();
            let s = StorageValue {
                storage_prefix: "".to_string(),
                key: Binary::new(validator_key),
                value: Binary::new(validator.encode_to_vec()),
            };
            st_values.push(s);
        }

        let stakin_validator = StakingValidator::reconstruct(&st_values);

        assert_eq!(stakin_validator, ts.expected_result)
    }
}

#[test]
fn test_validators_signing_infos_reconstruct() {
    struct TestCase {
        signing_infos: Vec<CosmosValidatorSigningInfo>,
        expected_result: NeutronResult<SigningInfo>,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            signing_infos: vec![CosmosValidatorSigningInfo {
                address: "cosmosvalcons1yjf46k064988jdjje068zmrqg8xh4fqqe2wwnl".to_string(),
                start_height: 1,
                index_offset: 1,
                jailed_until: None,
                tombstoned: false,
                missed_blocks_counter: 987675,
            }],
            expected_result: Ok(SigningInfo {
                signing_infos: vec![ValidatorSigningInfo {
                    address: "cosmosvalcons1yjf46k064988jdjje068zmrqg8xh4fqqe2wwnl".to_string(),
                    start_height: 1,
                    index_offset: 1,
                    jailed_until: None,
                    tombstoned: false,
                    missed_blocks_counter: 987675,
                }],
            }),
        },
        TestCase {
            signing_infos: vec![CosmosValidatorSigningInfo {
                address: "cosmosvalcons1yjf46k064988jdjje068zmrqg8xh4fqqe2wwnl".to_string(),
                start_height: 1,
                index_offset: 1,
                jailed_until: Some(prost_types::Timestamp {
                    seconds: 321654,
                    nanos: 123123,
                }),
                tombstoned: false,
                missed_blocks_counter: 987675,
            }],
            expected_result: Ok(SigningInfo {
                signing_infos: vec![ValidatorSigningInfo {
                    address: "cosmosvalcons1yjf46k064988jdjje068zmrqg8xh4fqqe2wwnl".to_string(),
                    start_height: 1,
                    index_offset: 1,
                    jailed_until: Some(321654),
                    tombstoned: false,
                    missed_blocks_counter: 987675,
                }],
            }),
        },
        TestCase {
            signing_infos: vec![
                CosmosValidatorSigningInfo {
                    address: "cosmosvalcons1yjf46k064988jdjje068zmrqg8xh4fqqe2wwnl".to_string(),
                    start_height: 1,
                    index_offset: 1,
                    jailed_until: None,
                    tombstoned: true,
                    missed_blocks_counter: 987675,
                },
                CosmosValidatorSigningInfo {
                    address: "cosmosvalcons16tnak7apushwznnd3wtku8gm0rt3xytz6ut006".to_string(),
                    start_height: 1,
                    index_offset: 1,
                    jailed_until: Some(prost_types::Timestamp {
                        seconds: 321654,
                        nanos: 123123,
                    }),
                    tombstoned: false,
                    missed_blocks_counter: 345012,
                },
            ],
            expected_result: Ok(SigningInfo {
                signing_infos: vec![
                    ValidatorSigningInfo {
                        address: "cosmosvalcons1yjf46k064988jdjje068zmrqg8xh4fqqe2wwnl".to_string(),
                        start_height: 1,
                        index_offset: 1,
                        jailed_until: None,
                        tombstoned: true,
                        missed_blocks_counter: 987675,
                    },
                    ValidatorSigningInfo {
                        address: "cosmosvalcons16tnak7apushwznnd3wtku8gm0rt3xytz6ut006".to_string(),
                        start_height: 1,
                        index_offset: 1,
                        jailed_until: Some(321654),
                        tombstoned: false,
                        missed_blocks_counter: 345012,
                    },
                ],
            }),
        },
        TestCase {
            signing_infos: vec![],
            expected_result: Ok(SigningInfo {
                signing_infos: vec![],
            }),
        },
    ];

    for ts in test_cases {
        let mut st_values: Vec<StorageValue> = vec![];

        for info in &ts.signing_infos {
            let val_addr = decode_and_convert(info.address.as_str()).unwrap();

            let signing_info_key = create_validator_signing_info_key(&val_addr).unwrap();
            let s = StorageValue {
                storage_prefix: "".to_string(),
                key: Binary::new(signing_info_key),
                value: Binary::new(info.encode_to_vec()),
            };
            st_values.push(s);
        }

        let signing_infos = SigningInfo::reconstruct(&st_values);

        assert_eq!(signing_infos, ts.expected_result)
    }
}

#[test]
fn test_government_proposals_reconstruct() {
    struct TestCase {
        proposals: Vec<CosmosProposal>,
        expected_result: NeutronResult<GovernmentProposal>,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            proposals: vec![CosmosProposal {
                proposal_id: 1,
                content: Some(prost_types::Any {
                    type_url: "proposal_type".to_string(),
                    value: vec![],
                }),
                status: 1,
                final_tally_result: None,
                submit_time: None,
                deposit_end_time: None,
                total_deposit: vec![Coin {
                    amount: "100000".to_string(),
                    denom: "stake".to_string(),
                }],
                voting_start_time: None,
                voting_end_time: None,
            }],
            expected_result: Ok(GovernmentProposal {
                proposals: vec![Proposal {
                    proposal_id: 1,
                    proposal_type: Some("proposal_type".to_string()),
                    total_deposit: vec![StdCoin::new(100000u128, "stake")],
                    status: 1,
                    submit_time: None,
                    deposit_end_time: None,
                    voting_start_time: None,
                    voting_end_time: None,
                    final_tally_result: None,
                }],
            }),
        },
        TestCase {
            proposals: vec![CosmosProposal {
                proposal_id: 1,
                content: Some(prost_types::Any {
                    type_url: "proposal_type".to_string(),
                    value: vec![],
                }),
                status: 1,
                final_tally_result: Some(CosmosTallyResult {
                    abstain: "1".to_string(),
                    no: "2".to_string(),
                    no_with_veto: "3".to_string(),
                    yes: "4".to_string(),
                }),
                submit_time: Some(prost_types::Timestamp {
                    seconds: 2222222,
                    nanos: 123123,
                }),
                deposit_end_time: Some(prost_types::Timestamp {
                    seconds: 3333333,
                    nanos: 123123,
                }),
                total_deposit: vec![Coin {
                    amount: "100000".to_string(),
                    denom: "stake".to_string(),
                }],
                voting_start_time: Some(prost_types::Timestamp {
                    seconds: 4444444,
                    nanos: 123123,
                }),
                voting_end_time: Some(prost_types::Timestamp {
                    seconds: 555555555,
                    nanos: 123123,
                }),
            }],
            expected_result: Ok(GovernmentProposal {
                proposals: vec![Proposal {
                    proposal_id: 1,
                    proposal_type: Some("proposal_type".to_string()),
                    total_deposit: vec![StdCoin::new(100000u128, "stake")],
                    status: 1,
                    submit_time: Some(2222222),
                    deposit_end_time: Some(3333333),
                    voting_start_time: Some(4444444),
                    voting_end_time: Some(555555555),
                    final_tally_result: Some(TallyResult {
                        abstain: Uint128::from(1u64),
                        no: Uint128::from(2u64),
                        no_with_veto: Uint128::from(3u64),
                        yes: Uint128::from(4u64),
                    }),
                }],
            }),
        },
        TestCase {
            proposals: vec![
                CosmosProposal {
                    proposal_id: 1,
                    content: Some(prost_types::Any {
                        type_url: "proposal_type1".to_string(),
                        value: vec![],
                    }),
                    status: 1,
                    final_tally_result: None,
                    submit_time: None,
                    deposit_end_time: None,
                    total_deposit: vec![Coin {
                        amount: "100000".to_string(),
                        denom: "stake".to_string(),
                    }],
                    voting_start_time: None,
                    voting_end_time: None,
                },
                CosmosProposal {
                    proposal_id: 2,
                    content: Some(prost_types::Any {
                        type_url: "proposal_type2".to_string(),
                        value: vec![],
                    }),
                    status: 1,
                    final_tally_result: None,
                    submit_time: None,
                    deposit_end_time: None,
                    total_deposit: vec![Coin {
                        amount: "200000".to_string(),
                        denom: "osmo".to_string(),
                    }],
                    voting_start_time: None,
                    voting_end_time: None,
                },
            ],
            expected_result: Ok(GovernmentProposal {
                proposals: vec![
                    Proposal {
                        proposal_id: 1,
                        proposal_type: Some("proposal_type1".to_string()),
                        total_deposit: vec![StdCoin::new(100000u128, "stake")],
                        status: 1,
                        submit_time: None,
                        deposit_end_time: None,
                        voting_start_time: None,
                        voting_end_time: None,
                        final_tally_result: None,
                    },
                    Proposal {
                        proposal_id: 2,
                        proposal_type: Some("proposal_type2".to_string()),
                        total_deposit: vec![StdCoin::new(200000u128, "osmo")],
                        status: 1,
                        submit_time: None,
                        deposit_end_time: None,
                        voting_start_time: None,
                        voting_end_time: None,
                        final_tally_result: None,
                    },
                ],
            }),
        },
        TestCase {
            proposals: vec![],
            expected_result: Ok(GovernmentProposal { proposals: vec![] }),
        },
    ];

    for ts in test_cases {
        let mut st_values: Vec<StorageValue> = vec![];

        for proposal in &ts.proposals {
            let proposal_key = create_gov_proposal_key(proposal.proposal_id).unwrap();
            let s = StorageValue {
                storage_prefix: "".to_string(),
                key: Binary::new(proposal_key),
                value: Binary::new(proposal.encode_to_vec()),
            };
            st_values.push(s);
        }

        let gov_proposal = GovernmentProposal::reconstruct(&st_values);

        assert_eq!(gov_proposal, ts.expected_result)
    }
}

#[allow(deprecated)]
#[test]
fn test_proposal_votes_reconstruct() {
    struct TestCase {
        proposal_votes: Vec<Vote>,
        expected_result: NeutronResult<GovernmentProposalVotes>,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            proposal_votes: vec![Vote {
                proposal_id: 1,
                voter: "cosmos1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
                option: 0,
                options: vec![CosmosWeightedVoteOption {
                    weight: "1000000000000000000".to_string(),
                    option: 1,
                }],
            }],
            expected_result: Ok(GovernmentProposalVotes {
                proposal_votes: vec![ProposalVote {
                    proposal_id: 1,
                    voter: "cosmos1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
                    options: vec![WeightedVoteOption {
                        weight: "1000000000000000000".to_string(),
                        option: 1,
                    }],
                }],
            }),
        },
        TestCase {
            proposal_votes: vec![
                Vote {
                    proposal_id: 1,
                    voter: "cosmos1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
                    option: 0,
                    options: vec![CosmosWeightedVoteOption {
                        weight: "1000000000000000000".to_string(),
                        option: 1,
                    }],
                },
                Vote {
                    proposal_id: 2,
                    voter: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
                    option: 0,
                    options: vec![CosmosWeightedVoteOption {
                        weight: "567890".to_string(),
                        option: 2,
                    }],
                },
            ],
            expected_result: Ok(GovernmentProposalVotes {
                proposal_votes: vec![
                    ProposalVote {
                        proposal_id: 1,
                        voter: "cosmos1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
                        options: vec![WeightedVoteOption {
                            weight: "1000000000000000000".to_string(),
                            option: 1,
                        }],
                    },
                    ProposalVote {
                        proposal_id: 2,
                        voter: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
                        options: vec![WeightedVoteOption {
                            weight: "567890".to_string(),
                            option: 2,
                        }],
                    },
                ],
            }),
        },
        TestCase {
            proposal_votes: vec![],
            expected_result: Ok(GovernmentProposalVotes {
                proposal_votes: vec![],
            }),
        },
    ];

    for ts in test_cases {
        let mut st_values: Vec<StorageValue> = vec![];

        for votes in &ts.proposal_votes {
            let proposal_key = create_gov_proposal_votes_key(votes.proposal_id).unwrap();
            let s = StorageValue {
                storage_prefix: "".to_string(),
                key: Binary::new(proposal_key),
                value: Binary::new(votes.encode_to_vec()),
            };
            st_values.push(s);
        }

        let gov_proposals_votes = GovernmentProposalVotes::reconstruct(&st_values);

        assert_eq!(gov_proposals_votes, ts.expected_result)
    }
}

#[test]
fn test_fee_pool_reconstruct() {
    struct TestCase {
        coins: Vec<(String, Uint128)>,
    }
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            coins: vec![("uosmo".to_string(), Uint128::from(100u128))],
        },
        TestCase {
            coins: vec![
                ("uosmo".to_string(), Uint128::from(100u128)),
                ("uatom".to_string(), Uint128::from(500u128)),
                ("uluna".to_string(), Uint128::from(80u128)),
            ],
        },
        TestCase { coins: vec![] },
    ];

    for ts in test_cases {
        let mut coins: Vec<DecCoin> = vec![];

        for coin in &ts.coins {
            let balance_amount = DecCoin {
                denom: coin.0.clone(),
                amount: coin
                    .1
                    .mul(Uint128::one().mul(Uint128::from(10u64).pow(DECIMAL_PLACES))) // adjust to Dec gogo proto format
                    .to_string(),
            };

            coins.push(balance_amount);
        }

        let fee_pool = CosmosFeePool {
            community_pool: coins,
        };

        let fee_pool_key = create_fee_pool_key().unwrap();

        let st_value = StorageValue {
            storage_prefix: "".to_string(),
            key: Binary::new(fee_pool_key),
            value: Binary::new(fee_pool.encode_to_vec()),
        };

        let fee_pool_coins = FeePool::reconstruct(&[st_value]).unwrap();
        assert_eq!(fee_pool_coins.coins.len(), ts.coins.len());
        for (i, coin) in fee_pool_coins.coins.iter().enumerate() {
            assert_eq!(coin.denom, ts.coins[i].0);
            assert_eq!(coin.amount, ts.coins[i].1)
        }
    }
}

#[test]
fn test_delegations_reconstruct() {
    struct TestCase {
        stake_denom: String,
        delegations: Vec<Delegation>,
        validators: Vec<Validator>,
        expected_result: NeutronResult<Delegations>,
    }
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            stake_denom: "stake".to_string(),
            delegations: vec![Delegation {
                delegator_address: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
                validator_address: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3".to_string(),
                shares: "1000000000000000000".to_string(),
            }],
            validators: vec![Validator {
                operator_address: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3".to_string(),
                consensus_pubkey: None,
                jailed: false,
                status: 0,
                tokens: "1000000000000000000".to_string(),
                delegator_shares: "1000000000000000000".to_string(),
                description: None,
                unbonding_height: 0,
                unbonding_time: None,
                commission: None,
                min_self_delegation: "".to_string(),
            }],
            expected_result: Ok(Delegations {
                delegations: vec![StdDelegation {
                    delegator: Addr::unchecked("osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs"),
                    validator: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3".to_string(),
                    amount: StdCoin::new(1000000000000000000u128, "stake"),
                }],
            }),
        },
        TestCase {
            stake_denom: "stake".to_string(),
            delegations: vec![
                Delegation {
                    delegator_address: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
                    validator_address: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3"
                        .to_string(),
                    shares: "1000000000000000000".to_string(),
                },
                Delegation {
                    delegator_address: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
                    validator_address: "osmovaloper1lzhlnpahvznwfv4jmay2tgaha5kmz5qxwmj9we"
                        .to_string(),
                    shares: "1000000000000000000".to_string(),
                },
            ],
            validators: vec![
                Validator {
                    operator_address: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3"
                        .to_string(),
                    consensus_pubkey: None,
                    jailed: false,
                    status: 0,
                    tokens: "1000000000000000000".to_string(),
                    delegator_shares: "1000000000000000000".to_string(),
                    description: None,
                    unbonding_height: 0,
                    unbonding_time: None,
                    commission: None,
                    min_self_delegation: "".to_string(),
                },
                Validator {
                    operator_address: "osmovaloper1lzhlnpahvznwfv4jmay2tgaha5kmz5qxwmj9we"
                        .to_string(),
                    consensus_pubkey: None,
                    jailed: false,
                    status: 0,
                    tokens: "1000000000000000000".to_string(),
                    delegator_shares: "1000000000000000000".to_string(),
                    description: None,
                    unbonding_height: 0,
                    unbonding_time: None,
                    commission: None,
                    min_self_delegation: "".to_string(),
                },
            ],
            expected_result: Ok(Delegations {
                delegations: vec![
                    StdDelegation {
                        delegator: Addr::unchecked("osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs"),
                        validator: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3".to_string(),
                        amount: StdCoin::new(1000000000000000000u128, "stake"),
                    },
                    StdDelegation {
                        delegator: Addr::unchecked("osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs"),
                        validator: "osmovaloper1lzhlnpahvznwfv4jmay2tgaha5kmz5qxwmj9we".to_string(),
                        amount: StdCoin::new(1000000000000000000u128, "stake"),
                    },
                ],
            }),
        },
        TestCase {
            stake_denom: "stake".to_string(),
            delegations: vec![],
            validators: vec![],
            expected_result: Ok(Delegations {
                delegations: vec![],
            }),
        },
        TestCase {
            stake_denom: Default::default(),
            delegations: vec![],
            validators: vec![],
            expected_result: Err(NeutronError::InvalidQueryResultFormat(
                "denom is empty".into(),
            )),
        },
        TestCase {
            stake_denom: "stake".to_string(),
            delegations: vec![Delegation {
                delegator_address: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
                validator_address: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3".to_string(),
                shares: "1000000000000000000".to_string(),
            }],
            validators: vec![],
            expected_result: Err(NeutronError::InvalidQueryResultFormat(
                "validator is empty".into(),
            )),
        },
    ];

    for ts in &test_cases {
        // prepare storage values
        let mut st_values: Vec<StorageValue> = vec![StorageValue {
            storage_prefix: STAKING_STORE_KEY.to_string(),
            key: Binary::new(create_params_store_key(STAKING_STORE_KEY, KEY_BOND_DENOM)),
            value: {
                if ts.stake_denom.is_empty() {
                    return Default::default();
                }
                to_json_binary(&ts.stake_denom).unwrap()
            },
        }];

        for (i, d) in ts.delegations.iter().enumerate() {
            let delegator_addr = decode_and_convert(&d.delegator_address).unwrap();
            let val_addr = decode_and_convert(&d.validator_address).unwrap();

            st_values.push(StorageValue {
                storage_prefix: STAKING_STORE_KEY.to_string(),
                key: Binary::new(create_delegation_key(&delegator_addr, &val_addr).unwrap()),
                value: Binary::from(d.encode_to_vec()),
            });

            if let Some(v) = ts.validators.get(i) {
                st_values.push(StorageValue {
                    storage_prefix: STAKING_STORE_KEY.to_string(),
                    key: Binary::new(create_validator_key(&val_addr).unwrap()),
                    value: Binary::from(v.encode_to_vec()),
                });
            }
        }

        // test reconstruction
        let delegations = Delegations::reconstruct(&st_values);

        assert_eq!(delegations, ts.expected_result)
    }
}

#[test]
fn test_balance_reconstruct_from_hex() {
    let bytes = hex::decode(BALANCES_HEX_RESPONSE).unwrap(); // decode hex string to bytes
    let base64_input = BASE64_STANDARD.encode(bytes); // encode bytes to base64 string
    let balance_key = create_account_denom_balance_key(
        decode_and_convert("osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs").unwrap(),
        "uosmo",
    )
    .unwrap();

    let s = StorageValue {
        storage_prefix: String::default(), // not used in reconstruct
        key: Binary::from(balance_key),
        value: Binary::from_base64(base64_input.as_str()).unwrap(),
    };
    let bank_balances = Balances::reconstruct(&[s]).unwrap();
    assert_eq!(
        bank_balances,
        Balances {
            coins: vec![StdCoin {
                denom: String::from("uosmo"),
                amount: Uint128::from(99999000u64),
            }]
        }
    );
}

#[test]
fn test_balance_reconstruct_from_empty_value() {
    let balance_key = create_account_denom_balance_key(
        decode_and_convert("osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs").unwrap(),
        "uosmo",
    )
    .unwrap();
    let s = StorageValue {
        storage_prefix: String::default(), // not used in reconstruct
        key: Binary::from(balance_key),
        value: Binary::from(vec![]),
    };
    let bank_balances = Balances::reconstruct(&[s]).unwrap();
    assert_eq!(
        bank_balances,
        Balances {
            coins: vec![StdCoin::new(0u128, "uosmo")]
        }
    );
}

#[test]
fn test_bank_total_supply_reconstruct_from_hex() {
    let bytes = hex::decode(TOTAL_SUPPLY_HEX_RESPONSE).unwrap(); // decode hex string to bytes
    let base64_input = BASE64_STANDARD.encode(bytes); // encode bytes to base64 string

    let s = StorageValue {
        storage_prefix: String::default(), // not used in reconstruct
        key: Binary::new(create_total_denom_key("stake").unwrap()),
        value: Binary::from_base64(base64_input.as_str()).unwrap(),
    };
    let total_supply = TotalSupply::reconstruct(&[s]).unwrap();
    assert_eq!(
        total_supply,
        TotalSupply {
            coins: vec![StdCoin {
                denom: String::from("stake"),
                amount: Uint128::from(300001098u64), // mutating
            }]
        }
    );
}

#[test]
fn test_staking_validators_reconstruct_from_hex() {
    let bytes = hex::decode(STAKING_VALIDATOR_HEX_RESPONSE).unwrap(); // decode hex string to bytes
    let base64_input = BASE64_STANDARD.encode(bytes); // encode bytes to base64 string

    let s = StorageValue {
        storage_prefix: String::default(), // not used in reconstruct
        key: Binary::default(),            // not used in reconstruct
        value: Binary::from_base64(base64_input.as_str()).unwrap(),
    };
    let staking_validator = StakingValidator::reconstruct(&[s]).unwrap();
    assert_eq!(
        staking_validator,
        StakingValidator {
            validators: vec![ContractValidator {
                operator_address: String::from(
                    "cosmosvaloper15fqjpj90ruhj57q3l6a5hda0rt77g6mcek2mtq" // mutating
                ),
                consensus_pubkey: Some(vec![
                    10, 32, 178, 12, 7, 179, 235, 144, 13, 247, 43, 72, 194, 78, 154, 46, 6, 255,
                    79, 231, 59, 189, 37, 94, 67, 58, 248, 234, 227, 177, 152, 142, 105, 136,
                ]),
                jailed: false,
                status: 3,
                tokens: String::from("100000000"),
                delegator_shares: String::from("100000000000000000000000000"),
                moniker: Some(String::from("mynode")),
                identity: Some(String::from("")),
                website: Some(String::from("")),
                security_contact: Some(String::from("")),
                details: Some(String::from("")),
                unbonding_height: 0u64,
                unbonding_time: Some(0u64),
                rate: Some(Decimal::from_str("0.100000000000000000").unwrap()),
                max_rate: Some(Decimal::from_str("0.200000000000000000").unwrap()),
                max_change_rate: Some(Decimal::from_str("0.010000000000000000").unwrap()),
                update_time: Some(1683291676u64), // mutating
                min_self_delegation: Decimal::one(),
            }]
        }
    );
}

#[test]
fn test_validators_signing_infos_reconstruct_from_hex() {
    let bytes = hex::decode(VALIDATOR_SIGNING_INFO_HEX_RESPONSE).unwrap(); // decode hex string to bytes
    let base64_input = BASE64_STANDARD.encode(bytes); // encode bytes to base64 string

    let s = StorageValue {
        storage_prefix: String::default(), // not used in reconstruct
        key: Binary::default(),            // not used in reconstruct
        value: Binary::from_base64(base64_input.as_str()).unwrap(),
    };
    let signing_info = SigningInfo::reconstruct(&[s]).unwrap();
    assert_eq!(
        signing_info,
        SigningInfo {
            signing_infos: vec![ValidatorSigningInfo {
                address: "cosmosvalcons19f53fqq28v6pmz877dne75d4d7l0r65d2750pw".to_string(),
                start_height: 0,
                index_offset: 16,
                jailed_until: Some(0),
                tombstoned: false,
                missed_blocks_counter: 0,
            }]
        }
    );
}

#[test]
fn test_government_proposals_reconstruct_from_hex() {
    let bytes = hex::decode(GOV_PROPOSAL_HEX_RESPONSE).unwrap(); // decode hex string to bytes
    let base64_input = BASE64_STANDARD.encode(bytes); // encode bytes to base64 string

    let s = StorageValue {
        storage_prefix: String::default(), // not used in reconstruct
        key: Binary::default(),            // not used in reconstruct
        value: Binary::from_base64(base64_input.as_str()).unwrap(),
    };
    let proposals = GovernmentProposal::reconstruct(&[s]).unwrap();
    assert_eq!(
        proposals,
        GovernmentProposal {
            proposals: vec![Proposal {
                proposal_id: 1u64,
                proposal_type: Some(String::from("/cosmos.gov.v1beta1.TextProposal")),
                total_deposit: vec![StdCoin {
                    denom: String::from("stake"),
                    amount: Uint128::from(1000u64),
                }],
                status: 1i32,
                submit_time: Some(1683291849u64),      // mutating
                deposit_end_time: Some(1683464649u64), // mutating
                voting_start_time: Some(18446744011573954816u64), // 0001-01-01T00:00:00Z
                voting_end_time: Some(18446744011573954816u64), // 0001-01-01T00:00:00Z
                final_tally_result: Some(TallyResult {
                    yes: Uint128::zero(),
                    no: Uint128::zero(),
                    abstain: Uint128::zero(),
                    no_with_veto: Uint128::zero(),
                }),
            }]
        }
    );
}

#[test]
fn test_proposal_votes_reconstruct_from_hex() {
    let bytes = hex::decode(GOV_PROPOSAL_VOTES_HEX_RESPONSE).unwrap(); // decode hex string to bytes
    let base64_input = BASE64_STANDARD.encode(bytes); // encode bytes to base64 string

    let s = StorageValue {
        storage_prefix: String::default(), // not used in reconstruct
        key: Binary::default(),            // not used in reconstruct
        value: Binary::from_base64(base64_input.as_str()).unwrap(),
    };
    let votes = GovernmentProposalVotes::reconstruct(&[s]).unwrap();
    assert_eq!(
        votes,
        GovernmentProposalVotes {
            proposal_votes: vec![ProposalVote {
                proposal_id: 4,
                voter: "cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw".to_string(),
                options: vec![WeightedVoteOption {
                    weight: "1000000000000000000".to_string(),
                    option: 1,
                }],
            }],
        }
    );
}

#[test]
fn test_fee_pool_reconstruct_from_hex() {
    let bytes = hex::decode(FEE_POOL_HEX_RESPONSE).unwrap(); // decode hex string to bytes
    let base64_input = BASE64_STANDARD.encode(bytes); // encode bytes to base64 string

    let s = StorageValue {
        storage_prefix: String::default(), // not used in reconstruct
        key: Binary::default(),            // not used in reconstruct
        value: Binary::from_base64(base64_input.as_str()).unwrap(),
    };
    let fee_pool = FeePool::reconstruct(&[s]).unwrap();
    assert_eq!(
        fee_pool,
        FeePool {
            coins: vec![StdCoin {
                denom: String::from("stake"),
                amount: Uint128::from(21u64), // mutating
            }]
        }
    );
}

#[test]
fn test_delegations_reconstruct_from_hex() {
    let staking_denom_bytes = hex::decode(STAKING_DENOM_HEX_RESPONSE).unwrap(); // decode hex string to bytes
    let staking_denom_base64_input = BASE64_STANDARD.encode(staking_denom_bytes); // encode bytes to base64 string
    let staking_validator_bytes = hex::decode(STAKING_VALIDATOR_HEX_RESPONSE).unwrap(); // decode hex string to bytes
    let staking_validator_base64_input = BASE64_STANDARD.encode(staking_validator_bytes); // encode bytes to base64 string
    let delegation_bytes = hex::decode(DELEGATOR_DELEGATIONS_HEX_RESPONSE).unwrap(); // decode hex string to bytes
    let delegation_base64_input = BASE64_STANDARD.encode(delegation_bytes); // encode bytes to base64 string

    let mut st_values: Vec<StorageValue> = vec![StorageValue {
        storage_prefix: String::default(), // not used in reconstruct
        key: Binary::default(),            // not used in reconstruct
        value: Binary::from_base64(staking_denom_base64_input.as_str()).unwrap(),
    }];
    st_values.push(StorageValue {
        storage_prefix: String::default(), // not used in reconstruct
        key: Binary::default(),            // not used in reconstruct
        value: Binary::from_base64(delegation_base64_input.as_str()).unwrap(),
    });
    st_values.push(StorageValue {
        storage_prefix: String::default(), // not used in reconstruct
        key: Binary::default(),            // not used in reconstruct
        value: Binary::from_base64(staking_validator_base64_input.as_str()).unwrap(),
    });

    let delegations = Delegations::reconstruct(&st_values).unwrap();
    assert_eq!(
        delegations,
        Delegations {
            delegations: vec![StdDelegation {
                delegator: Addr::unchecked("cosmos15fqjpj90ruhj57q3l6a5hda0rt77g6mcuz7w8n"), // mutating
                validator: String::from("cosmosvaloper15fqjpj90ruhj57q3l6a5hda0rt77g6mcek2mtq"), // mutating
                amount: StdCoin {
                    denom: String::from("stake"),
                    amount: Uint128::from(100000000u64),
                },
            }],
        }
    );
}

#[test]
fn test_unbonding_delegations_reconstruct_from_hex() {
    let unbonding_delegations_bytes =
        hex::decode(DELEGATOR_UNBONDING_DELEGATIONS_HEX_RESPONSE).unwrap(); // decode hex string to bytes
    let unbonding_delegations_base64_input = BASE64_STANDARD.encode(unbonding_delegations_bytes); // encode bytes to base64 string

    let st_values: Vec<StorageValue> = vec![StorageValue {
        storage_prefix: String::default(), // not used in reconstruct
        key: Binary::default(),            // not used in reconstruct
        value: Binary::from_base64(unbonding_delegations_base64_input.as_str()).unwrap(),
    }];

    let unbonding_delegations = UnbondingDelegations::reconstruct(&st_values).unwrap();
    assert_eq!(
        unbonding_delegations,
        UnbondingDelegations {
            unbonding_responses: vec![UnbondingResponse {
                delegator_address: Addr::unchecked("cosmos1m9l358xunhhwds0568za49mzhvuxx9uxre5tud"),
                validator_address: String::from(
                    "cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn"
                ),
                entries: vec![
                    UnbondingEntry {
                        balance: Uint128::new(10_000),
                        completion_time: Some(Timestamp::from_nanos(1704872890570185206)),
                        creation_height: 365,
                        initial_balance: Uint128::new(10_000),
                    },
                    UnbondingEntry {
                        balance: Uint128::new(20_000),
                        completion_time: Some(Timestamp::from_nanos(1704872903002248037)),
                        creation_height: 377,
                        initial_balance: Uint128::new(20_000),
                    },
                ],
            }]
        }
    );
}

#[test]
fn test_deconstruct_account_denom_balance_key() {
    struct TestCase {
        key: Vec<u8>,
        expected_result: NeutronResult<(AddressBytes, String)>,
    }

    let testcases = vec![
        TestCase {
            // valid key
            key: create_account_denom_balance_key("addr1", "uatom").unwrap(),
            expected_result: Ok((
                "addr1".bytes().collect::<AddressBytes>(),
                "uatom".to_string(),
            )),
        },
        TestCase {
            // empty key
            key: vec![],
            expected_result: Err(NeutronError::AccountDenomBalanceKeyDeconstructionError(
                "invalid key length".to_string(),
            )),
        },
        TestCase {
            // first element in the key is not BALANCES_PREFIX
            key: vec![81],
            expected_result: Err(NeutronError::AccountDenomBalanceKeyDeconstructionError(
                "first element in key does not equal to BALANCES_PREFIX: 81 != 2".to_string(),
            )),
        },
        TestCase {
            // first element in the key is BALANCES_PREFIX but key length is invalid
            key: vec![BALANCES_PREFIX],
            expected_result: Err(NeutronError::AccountDenomBalanceKeyDeconstructionError(
                "invalid key length".to_string(),
            )),
        },
        TestCase {
            // invalid address length in key
            // second element must define addr length, but here we say that length is 10,
            // but actually it's 1 which is wrong
            key: vec![BALANCES_PREFIX, 10, 1],
            expected_result: Err(NeutronError::AccountDenomBalanceKeyDeconstructionError(
                "address length in key is invalid".to_string(),
            )),
        },
        TestCase {
            // first element is correct, addr length is good, but there is no denom in the key
            key: vec![BALANCES_PREFIX, 2, 1, 2],
            expected_result: Err(NeutronError::AccountDenomBalanceKeyDeconstructionError(
                "denom in key can't be empty".to_string(),
            )),
        },
        TestCase {
            // must fail since [0, 159, 146, 150] are invalid UTF-8 bytes
            key: vec![BALANCES_PREFIX, 2, 1, 2, 0, 159, 146, 150],
            expected_result: Err(NeutronError::FromUTF8Error(
                String::from_utf8(vec![0, 159, 146, 150]).unwrap_err(),
            )),
        },
    ];

    for tc in testcases {
        assert_eq!(
            deconstruct_account_denom_balance_key(tc.key),
            tc.expected_result
        );
    }
}

#[test]
fn test_delegations_reconstruct_overflow() {
    struct TestCase {
        stake_denom: String,
        delegations: Vec<Delegation>,
        validators: Vec<Validator>,
        expected_result: NeutronResult<Delegations>,
    }
    let test_cases: Vec<TestCase> = vec![TestCase {
        stake_denom: "stake".to_string(),
        delegations: vec![Delegation {
            delegator_address: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
            validator_address: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3".to_string(),
            shares: "340282366920938463463".to_string(),
        }],
        validators: vec![Validator {
            operator_address: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3".to_string(),
            consensus_pubkey: None,
            jailed: false,
            status: 0,
            tokens: "340282366920938463463".to_string(),
            delegator_shares: "340282366920938463463".to_string(),
            description: None,
            unbonding_height: 0,
            unbonding_time: None,
            commission: None,
            min_self_delegation: "".to_string(),
        }],
        expected_result: Ok(Delegations {
            delegations: vec![StdDelegation {
                delegator: Addr::unchecked("osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs"),
                validator: "osmovaloper1r2u5q6t6w0wssrk6l66n3t2q3dw2uqny4gj2e3".to_string(),
                amount: StdCoin::new(340282366920938463463u128, "stake"),
            }],
        }),
    }];

    for ts in &test_cases {
        // prepare storage values
        let mut st_values: Vec<StorageValue> = vec![StorageValue {
            storage_prefix: STAKING_STORE_KEY.to_string(),
            key: Binary::new(create_params_store_key(STAKING_STORE_KEY, KEY_BOND_DENOM)),
            value: {
                if ts.stake_denom.is_empty() {
                    return Default::default();
                }
                to_json_binary(&ts.stake_denom).unwrap()
            },
        }];

        for (i, d) in ts.delegations.iter().enumerate() {
            let delegator_addr = decode_and_convert(&d.delegator_address).unwrap();
            let val_addr = decode_and_convert(&d.validator_address).unwrap();

            st_values.push(StorageValue {
                storage_prefix: STAKING_STORE_KEY.to_string(),
                key: Binary::new(create_delegation_key(&delegator_addr, &val_addr).unwrap()),
                value: Binary::from(d.encode_to_vec()),
            });

            if let Some(v) = ts.validators.get(i) {
                st_values.push(StorageValue {
                    storage_prefix: STAKING_STORE_KEY.to_string(),
                    key: Binary::new(create_validator_key(&val_addr).unwrap()),
                    value: Binary::from(v.encode_to_vec()),
                });
            }
        }

        // test reconstruction
        let delegations = Delegations::reconstruct(&st_values);

        assert_eq!(delegations, ts.expected_result)
    }
}
