use crate::bindings::types::StorageValue;
use crate::interchain_queries::helpers::decode_and_convert;
use crate::interchain_queries::types::KVReconstruct;
use crate::interchain_queries::v045::helpers::create_params_store_key;
use crate::interchain_queries::v045::types::KEY_BOND_DENOM;
use crate::interchain_queries::v047::helpers::{
    create_account_denom_balance_key, create_delegation_key, create_fee_pool_key,
    create_gov_proposal_key, create_total_denom_key, create_validator_key,
    create_validator_signing_info_key,
};
use crate::interchain_queries::v047::types::{
    Balances, Delegations, FeePool, GovernmentProposal, Proposal, SigningInfo, StakingValidator,
    StdDelegation, TallyResult, TotalSupply, UnbondingDelegations, UnbondingEntry,
    UnbondingResponse, Validator as ContractValidator, ValidatorSigningInfo, DECIMAL_PLACES,
    STAKING_PARAMS_KEY, STAKING_STORE_KEY,
};
use crate::{NeutronError, NeutronResult};
use base64::prelude::*;
use base64::Engine;
use cosmos_sdk_proto::cosmos::base::v1beta1::{Coin, DecCoin};
use cosmos_sdk_proto::cosmos::distribution::v1beta1::FeePool as CosmosFeePool;
use cosmos_sdk_proto::cosmos::gov::v1beta1::{
    Proposal as CosmosProposal, TallyResult as CosmosTallyResult,
};
use cosmos_sdk_proto::cosmos::slashing::v1beta1::ValidatorSigningInfo as CosmosValidatorSigningInfo;
use cosmos_sdk_proto::cosmos::staking::v1beta1::{
    Commission, CommissionRates, Delegation, Description, Params, Validator,
};
use cosmos_sdk_proto::traits::Message;
use cosmwasm_std::{Addr, Binary, Coin as StdCoin, Decimal, Timestamp, Uint128};
use hex;
use std::ops::Mul;
use std::str::FromStr;

// raw hex data from KV storage created using https://github.com/neutron-org/icq-compliance-officer.
pub const BALANCES_HEX_RESPONSE: &str = "343934323133353631";
pub const TOTAL_SUPPLY_HEX_RESPONSE: &str = "31363434313731393838393035373639";
pub const FEE_POOL_HEX_RESPONSE: &str =
    "0a630a446962632f31324441343233303445453143453936303731463731324141344435383138364144313143333136354330444344413731453031374135344633393335453636121b3434343235323231373030303030303030303030303030303030300a630a446962632f31344639424333453434423841394331424531464230383938304641423837303334433939303545463137434632463530303846433038353231383831314343121b3138393735333433323030303030303030303030303030303030300a620a446962632f31464244443538443433384234443034443236434246423245373232433138393834413046314135323436384334463432463337443130324633443346333939121a32353435353334383030303030303030303030303030303030300a620a446962632f32313831414142303231384541433234424339463836424431333634464242464133453645334643433235453838453345363843313544433645373532443836121a38393736343437323030303030303030303030303030303030300a5e0a446962632f323731373130394139353535394633413137454643304338393742373639314532324132323742333046433343354345374137434538383438313632393730341216393030303030303030303030303030303030303030300a640a446962632f34324534374135424137303845424536453043323237303036323534463237383445323039463444424433433642423737454443344232394546383735453845121c323332353636383932303030303030303030303030303030303030300a620a446962632f38314430384243333946423532304542443934384346303137393130444436393730324433344246354143313630463736443342354346433434344542434530121a33333633393935353030303030303030303030303030303030300a1c0a0574686574611213313531373130393430373239393334333933380a290a057561746f6d12203133353338343330393338303535303237343635383338343139363137323031";
pub const GOV_PROPOSAL_HEX_RESPONSE: &str = "08011291030a232f636f736d6f732e676f762e76312e4d7367457865634c6567616379436f6e74656e7412e9020ab7020a202f636f736d6f732e676f762e763162657461312e5465787450726f706f73616c1292020a4441646a7573746d656e74206f6620626c6f636b735f7065725f7965617220746f20636f6d6520616c69676e656420776974682061637475616c20626c6f636b2074696d6512c9015468697320676f7665726e616e63652070726f706f73616c20697320666f722061646a7573746d656e74206f6620626c6f636b735f7065725f7965617220706172616d6574657220746f206e6f726d616c697a652074686520696e666c6174696f6e207261746520616e642072657761726420726174652e5c6e2069706673206c696e6b3a2068747470733a2f2f697066732e696f2f697066732f516d587145427235367865557a4670676a736d444b4d5369743369716e4b6144454c347461627850586f7a397863122d636f736d6f73313064303779323635676d6d757674347a30773961773838306a6e73723730306a367a6e396b6e1803222f0a0e3937313138393033353236373939120c3430323338303537373233341a0c3332303534353430303030302201302a0b0897c1c7e40510e4838e13320b0897ab91e50510e4838e133a120a057561746f6d1209353132313030303030420c088fcccae405109399d2ac024a0c088fb694e505109399d2ac025a4441646a7573746d656e74206f6620626c6f636b735f7065725f7965617220746f20636f6d6520616c69676e656420776974682061637475616c20626c6f636b2074696d6562c9015468697320676f7665726e616e63652070726f706f73616c20697320666f722061646a7573746d656e74206f6620626c6f636b735f7065725f7965617220706172616d6574657220746f206e6f726d616c697a652074686520696e666c6174696f6e207261746520616e642072657761726420726174652e5c6e2069706673206c696e6b3a2068747470733a2f2f697066732e696f2f697066732f516d587145427235367865557a4670676a736d444b4d5369743369716e4b6144454c347461627850586f7a397863";
pub const STAKING_PARAMS_HEX_RESPONSE: &str = "0a040880c60a109601180720904e2a057561746f6d321135303030303030303030303030303030303a1532353030303030303030303030303030303030303042123235303030303030303030303030303030304a12353030303030303030303030303030303030";
pub const STAKING_VALIDATOR_HEX_RESPONSE: &str = "0a34636f736d6f7376616c6f70657231307636777664656e65653872396c36776c73706863677572326c746c387a746b6672766a396112430a1d2f636f736d6f732e63727970746f2e656432353531392e5075624b657912220a20da3f8d90a407031bb7eaf76ecb5b031c96487998e2ee7c67995222cefd3b329120032a0f32353030323832373430353233363432213235303032383237343035323336343030303030303030303030303030303030303a3f0a0a656172746820f09f8c8e1210436f696e6261736520437573746f64791a0a68797068612e636f6f702a134120746573746e65742076616c696461746f7240f4b5704a0c0893efd1f605109bfa83af02524d0a3e0a123230303030303030303030303030303030301213313030303030303030303030303030303030301a1331303030303030303030303030303030303030120b08ff98e8f10510b7c886275a0731303030303030721b3130323030303030333030303030303030303030303030303030307a1c38363935303034363237303030303030303030303030303030303030";
pub const DELEGATOR_DELEGATIONS_HEX_RESPONSE: &str = "0a2d636f736d6f7331706d6a776d306138707673326d3870617579673272756c6479386e657934716e387a676439361234636f736d6f7376616c6f70657231307636777664656e65653872396c36776c73706863677572326c746c387a746b6672766a39611a1635303030303030303030303030303030303030303030";
pub const DELEGATOR_UNBONDING_DELEGATIONS_HEX_RESPONSE: &str = "0a2d636f736d6f73316d396c33353878756e6868776473303536387a6134396d7a68767578783975787265357475641234636f736d6f7376616c6f7065723138686c356339786e35647a6532673530756177306c326d723032657735377a6b3061756b746e1a2108ed02120c08ba97f9ac0610f6abf18f021a0531303030302205313030303028011a2008f902120b08c797f9ac0610e59a89011a053230303030220532303030302802";
pub const VALIDATOR_SIGNING_INFO_HEX_RESPONSE: &str = "0a34636f736d6f7376616c636f6e73313966353366717132387636706d7a383737646e653735643464376c307236356432373530707718102200";

#[test]
fn test_balance_reconstruct() {
    struct TestCase {
        addr: String,
        coins: Vec<(String, String)>,
    }
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            addr: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
            coins: vec![("uosmo".to_string(), "100".to_string())],
        },
        TestCase {
            addr: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
            coins: vec![
                ("uosmo".to_string(), "100".to_string()),
                ("uatom".to_string(), "500".to_string()),
                ("uluna".to_string(), "80".to_string()),
            ],
        },
        TestCase {
            addr: "osmo1yz54ncxj9csp7un3xled03q6thrrhy9cztkfzs".to_string(),
            coins: vec![("uluna".to_string(), "".to_string())],
        },
    ];

    for ts in test_cases {
        let mut st_values: Vec<StorageValue> = vec![];

        let converted_addr_bytes = decode_and_convert(ts.addr.as_str()).unwrap();
        for coin in &ts.coins {
            let balance_key =
                create_account_denom_balance_key(converted_addr_bytes.clone(), &coin.0).unwrap();

            let s = StorageValue {
                storage_prefix: "".to_string(),
                key: Binary::new(balance_key),
                value: Binary::new(coin.1.clone().into_bytes()),
            };
            st_values.push(s);
        }

        let balances = Balances::reconstruct(&st_values).unwrap();
        assert_eq!(balances.coins.len(), ts.coins.len());
        for (i, coin) in balances.coins.iter().enumerate() {
            assert_eq!(coin.denom, ts.coins[i].0);
            // special testcase where value is an empty string
            if ts.coins[i].1.is_empty() {
                assert_eq!(coin.amount, Uint128::zero());
                continue;
            }
            assert_eq!(coin.amount, Uint128::from_str(&ts.coins[i].1).unwrap())
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
                        abstain: Uint128::from(1u128),
                        no: Uint128::from(2u128),
                        no_with_veto: Uint128::from(3u128),
                        yes: Uint128::from(4u128),
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
        staking_params: Params,
        delegations: Vec<Delegation>,
        validators: Vec<Validator>,
        expected_result: NeutronResult<Delegations>,
    }
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            staking_params: Params {
                unbonding_time: None,
                max_validators: 0,
                max_entries: 0,
                historical_entries: 0,
                bond_denom: "stake".to_string(),
                min_commission_rate: "".to_string(),
            },
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
            staking_params: Params {
                unbonding_time: None,
                max_validators: 0,
                max_entries: 0,
                historical_entries: 0,
                bond_denom: "stake".to_string(),
                min_commission_rate: "".to_string(),
            },
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
            staking_params: Params {
                unbonding_time: None,
                max_validators: 0,
                max_entries: 0,
                historical_entries: 0,
                bond_denom: "stake".to_string(),
                min_commission_rate: "".to_string(),
            },
            delegations: vec![],
            validators: vec![],
            expected_result: Ok(Delegations {
                delegations: vec![],
            }),
        },
        TestCase {
            staking_params: Default::default(),
            delegations: vec![],
            validators: vec![],
            expected_result: Err(NeutronError::InvalidQueryResultFormat(
                "params is empty".into(),
            )),
        },
        TestCase {
            staking_params: Params {
                unbonding_time: None,
                max_validators: 0,
                max_entries: 0,
                historical_entries: 0,
                bond_denom: "stake".to_string(),
                min_commission_rate: "".to_string(),
            },
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
            key: Binary::new(vec![STAKING_PARAMS_KEY]),
            value: {
                if ts.staking_params.bond_denom.is_empty() {
                    return Default::default();
                }
                Binary::from(ts.staking_params.encode_to_vec())
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

    let s = StorageValue {
        storage_prefix: String::default(), // not used in reconstruct
        key: Binary::new(create_account_denom_balance_key("addr", "uatom").unwrap()),
        value: Binary::from_base64(base64_input.as_str()).unwrap(),
    };
    let bank_balances = Balances::reconstruct(&[s]).unwrap();
    assert_eq!(
        bank_balances,
        Balances {
            coins: vec![StdCoin::new(494213561u128, "uatom")]
        }
    );
}

#[test]
fn test_balance_reconstruct_from_empty_value() {
    let s = StorageValue {
        storage_prefix: String::default(), // not used in reconstruct
        key: Binary::new(create_account_denom_balance_key("addr", "uatom").unwrap()),
        value: Binary::from(vec![]),
    };
    let bank_balances = Balances::reconstruct(&[s]).unwrap();
    assert_eq!(
        bank_balances,
        Balances {
            coins: vec![StdCoin::new(0u128, "uatom")]
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
                amount: Uint128::from(1644171988905769u64), // mutating
            }]
        }
    );
}

#[test]
fn test_delegations_reconstruct_overflow() {
    struct TestCase {
        staking_params: Params,
        delegations: Vec<Delegation>,
        validators: Vec<Validator>,
        expected_result: NeutronResult<Delegations>,
    }
    let test_cases: Vec<TestCase> = vec![TestCase {
        staking_params: Params {
            unbonding_time: None,
            max_validators: 0,
            max_entries: 0,
            historical_entries: 0,
            bond_denom: "stake".to_string(),
            min_commission_rate: "".to_string(),
        },
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
                if ts.staking_params.bond_denom.is_empty() {
                    return Default::default();
                }
                Binary::from(ts.staking_params.encode_to_vec())
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
                    "cosmosvaloper10v6wvdenee8r9l6wlsphcgur2ltl8ztkfrvj9a" // mutating
                ),
                consensus_pubkey: Some(vec![
                    10, 32, 218, 63, 141, 144, 164, 7, 3, 27, 183, 234, 247, 110, 203, 91, 3, 28,
                    150, 72, 121, 152, 226, 238, 124, 103, 153, 82, 34, 206, 253, 59, 50, 145,
                ]),
                jailed: false,
                status: 3,
                tokens: String::from("250028274052364"),
                delegator_shares: String::from("250028274052364000000000000000000"),
                moniker: Some(String::from("earth 🌎")),
                identity: Some(String::from("Coinbase Custody")),
                website: Some(String::from("hypha.coop")),
                security_contact: Some(String::from("")),
                details: Some(String::from("A testnet validator")),
                unbonding_height: 1841908u64,
                unbonding_time: Some(1590982547u64),
                rate: Some(Decimal::from_str("0.200000000000000000").unwrap()),
                max_rate: Some(Decimal::from_str("1.00000000000000000").unwrap()),
                max_change_rate: Some(Decimal::from_str("1.000000000000000000").unwrap()),
                update_time: Some(1580862591u64), // mutating
                min_self_delegation: Decimal::from_str("1000000").unwrap(),
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
                proposal_type: Some(String::from("/cosmos.gov.v1.MsgExecLegacyContent")),
                total_deposit: vec![StdCoin {
                    denom: String::from("uatom"),
                    amount: Uint128::from(512100000u64),
                }],
                status: 3i32,
                submit_time: Some(1553064087u64),       // mutating
                deposit_end_time: Some(1554273687u64),  // mutating
                voting_start_time: Some(1553114639u64), // 0001-01-01T00:00:00Z
                voting_end_time: Some(1554324239u64),   // 0001-01-01T00:00:00Z
                final_tally_result: Some(TallyResult {
                    yes: Uint128::from(97118903526799u128),
                    no: Uint128::from(320545400000u128),
                    abstain: Uint128::from(402380577234u128),
                    no_with_veto: Uint128::zero(),
                }),
            }]
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
            coins: vec![
                StdCoin {
                    denom: String::from(
                        "ibc/12DA42304EE1CE96071F712AA4D58186AD11C3165C0DCDA71E017A54F3935E66"
                    ),
                    amount: Uint128::from(444252217u64), // mutating
                },
                StdCoin {
                    denom: String::from(
                        "ibc/14F9BC3E44B8A9C1BE1FB08980FAB87034C9905EF17CF2F5008FC085218811CC"
                    ),
                    amount: Uint128::from(189753432u64), // mutating
                },
                StdCoin {
                    denom: String::from(
                        "ibc/1FBDD58D438B4D04D26CBFB2E722C18984A0F1A52468C4F42F37D102F3D3F399"
                    ),
                    amount: Uint128::from(25455348u64), // mutating
                },
                StdCoin {
                    denom: String::from(
                        "ibc/2181AAB0218EAC24BC9F86BD1364FBBFA3E6E3FCC25E88E3E68C15DC6E752D86"
                    ),
                    amount: Uint128::from(89764472u64), // mutating
                },
                StdCoin {
                    denom: String::from(
                        "ibc/2717109A95559F3A17EFC0C897B7691E22A227B30FC3C5CE7A7CE88481629704"
                    ),
                    amount: Uint128::from(9000u64), // mutating
                },
                StdCoin {
                    denom: String::from(
                        "ibc/42E47A5BA708EBE6E0C227006254F2784E209F4DBD3C6BB77EDC4B29EF875E8E"
                    ),
                    amount: Uint128::from(2325668920u64), // mutating
                },
                StdCoin {
                    denom: String::from(
                        "ibc/81D08BC39FB520EBD948CF017910DD69702D34BF5AC160F76D3B5CFC444EBCE0"
                    ),
                    amount: Uint128::from(33639955u64), // mutating
                },
                StdCoin {
                    denom: String::from("theta"),
                    amount: Uint128::from(1u64), // mutating
                },
                StdCoin {
                    denom: String::from("uatom"),
                    amount: Uint128::from(13538430938055u64), // mutating
                },
            ]
        }
    );
}

#[test]
fn test_delegations_reconstruct_from_hex() {
    let staking_params_bytes = hex::decode(STAKING_PARAMS_HEX_RESPONSE).unwrap(); // decode hex string to bytes
    let staking_params_base64_input = BASE64_STANDARD.encode(staking_params_bytes); // encode bytes to base64 string
    let staking_validator_bytes = hex::decode(STAKING_VALIDATOR_HEX_RESPONSE).unwrap(); // decode hex string to bytes
    let staking_validator_base64_input = BASE64_STANDARD.encode(staking_validator_bytes); // encode bytes to base64 string
    let delegation_bytes = hex::decode(DELEGATOR_DELEGATIONS_HEX_RESPONSE).unwrap(); // decode hex string to bytes
    let delegation_base64_input = BASE64_STANDARD.encode(delegation_bytes); // encode bytes to base64 string

    let mut st_values: Vec<StorageValue> = vec![StorageValue {
        storage_prefix: String::default(), // not used in reconstruct
        key: Binary::default(),            // not used in reconstruct
        value: Binary::from_base64(staking_params_base64_input.as_str()).unwrap(),
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
                delegator: Addr::unchecked("cosmos1pmjwm0a8pvs2m8pauyg2ruldy8ney4qn8zgd96"), // mutating
                validator: String::from("cosmosvaloper10v6wvdenee8r9l6wlsphcgur2ltl8ztkfrvj9a"), // mutating
                amount: StdCoin {
                    denom: String::from("uatom"),
                    amount: Uint128::from(5000u64),
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
