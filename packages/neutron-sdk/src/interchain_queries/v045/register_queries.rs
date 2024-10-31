use crate::interchain_queries::helpers::{register_interchain_query, update_interchain_query};
use crate::interchain_queries::types::{
    QueryPayload, TransactionFilterItem, TransactionFilterOp, TransactionFilterValue,
};
use crate::interchain_queries::v045::types::{
    BANK_STORE_KEY, DISTRIBUTION_STORE_KEY, HEIGHT_FIELD, KEY_BOND_DENOM, PARAMS_STORE_KEY,
    RECIPIENT_FIELD, SLASHING_STORE_KEY, STAKING_STORE_KEY, WASM_STORE_KEY,
};
use crate::{
    errors::error::NeutronResult,
    interchain_queries::helpers::decode_and_convert,
    interchain_queries::v045::helpers::{
        create_balances_query_keys, create_delegation_key, create_fee_pool_key,
        create_gov_proposal_keys, create_gov_proposals_voters_votes_keys, create_params_store_key,
        create_total_denom_key, create_unbonding_delegation_key, create_validator_key,
        create_validator_signing_info_key, create_wasm_contract_store_key,
    },
};
use cosmwasm_std::{Addr, CosmosMsg};
use neutron_std::types::neutron::interchainqueries::KvKey;

/// Creates a message to register an Interchain Query to get balance of account on remote chain for list of denoms
///
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **addr** address of an account on remote chain for which you want to get balances;
/// * **denoms** denominations of the coins for which you want to get balance;
/// * **update_period** is used to say how often the query must be updated.
pub fn new_register_balances_query_msg(
    contract: Addr,
    connection_id: String,
    addr: String,
    denoms: Vec<String>,
    update_period: u64,
) -> NeutronResult<CosmosMsg> {
    let kv_keys = create_balances_query_keys(addr, denoms)?;
    register_interchain_query(
        contract,
        QueryPayload::KV(kv_keys),
        connection_id,
        update_period,
    )
}

/// Creates a message to register an Interchain Query to get balance of account on remote chain for a particular denom
///
/// * **contract** is an address of the contract that registers the query. Must be contract that sends this message.
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **addr** address of an account on remote chain for which you want to get balances;
/// * **denom** denomination of the coin for which you want to get balance;
/// * **update_period** is used to say how often the query must be updated.
#[deprecated(note = "Please use new_register_balances_query_msg instead")]
pub fn new_register_balance_query_msg(
    contract: Addr,
    connection_id: String,
    addr: String,
    denom: String,
    update_period: u64,
) -> NeutronResult<CosmosMsg> {
    new_register_balances_query_msg(contract, connection_id, addr, vec![denom], update_period)
}

/// Creates a message to register an Interchain Query to get total supply on remote chain for particular denom
///
/// * **contract** is an address of the contract that registers the query. Must be contract that sends this message.
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **denom** denomination of the coin for which you want to get total supply;
/// * **update_period** is used to say how often the query must be updated.
pub fn new_register_bank_total_supply_query_msg(
    contract: Addr,
    connection_id: String,
    denoms: Vec<String>,
    update_period: u64,
) -> NeutronResult<CosmosMsg> {
    let mut kv_keys: Vec<KvKey> = Vec::with_capacity(denoms.len());

    for denom in denoms {
        let supply_key = create_total_denom_key(denom)?;

        let kv_key = KvKey {
            path: BANK_STORE_KEY.to_string(),
            key: supply_key,
        };

        kv_keys.push(kv_key)
    }

    register_interchain_query(
        contract,
        QueryPayload::KV(kv_keys),
        connection_id,
        update_period,
    )
}

/// Creates a message to register an Interchain Query to get fee pool on remote chain from distribution module
///
/// * **contract** is an address of the contract that registers the query. Must be contract that sends this message.
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **update_period** is used to say how often the query must be updated.
pub fn new_register_distribution_fee_pool_query_msg(
    contract: Addr,
    connection_id: String,
    update_period: u64,
) -> NeutronResult<CosmosMsg> {
    let kv_key = KvKey {
        path: DISTRIBUTION_STORE_KEY.to_string(),
        key: create_fee_pool_key()?,
    };

    register_interchain_query(
        contract,
        QueryPayload::KV(vec![kv_key]),
        connection_id,
        update_period,
    )
}

/// Creates a message to register an Interchain Query to get governance proposals on remote chain
///
/// * **contract** is an address of the contract that registers the query. Must be contract that sends this message.
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **proposals_ids** is a list of proposals ids from remote chain.
/// * **update_period** is used to say how often the query must be updated.
pub fn new_register_gov_proposals_query_msg(
    contract: Addr,
    connection_id: String,
    proposals_ids: Vec<u64>,
    update_period: u64,
) -> NeutronResult<CosmosMsg> {
    let kv_keys = create_gov_proposal_keys(proposals_ids)?;

    register_interchain_query(
        contract,
        QueryPayload::KV(kv_keys),
        connection_id,
        update_period,
    )
}

/// Creates a message to update an Interchain Query to get governance proposals on remote chain
///
/// * **contract** is an address of the contract that registers the query. Must be contract that sends this message.
/// * **query_id** is an IBC connection identifier between Neutron and remote chain;
/// * **proposals_ids** is a list of proposals ids from remote chain.
/// * **new_update_period** is used to update period of how often the query must be updated.
pub fn update_gov_proposals_query_msg(
    contract: Addr,
    query_id: u64,
    proposals_ids: Vec<u64>,
    new_update_period: u64,
) -> NeutronResult<CosmosMsg> {
    let kv_keys = create_gov_proposal_keys(proposals_ids)?;

    update_interchain_query(contract, query_id, kv_keys, new_update_period, None)
}

/// Creates a message to register an Interchain Query to get governance proposals votes on the remote chain
///
/// * **contract** is an address of the contract that registers the query. Must be contract that sends this message.
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **proposals_ids** is a list of proposals ids from remote chain.
/// * **voters** is a list of voter to get voting info from remote chain.
/// * **update_period** is used to say how often the query must be updated.
pub fn new_register_gov_proposals_voters_votes_query_msg(
    contract: Addr,
    connection_id: String,
    proposals_ids: Vec<u64>,
    voters: Vec<String>,
    update_period: u64,
) -> NeutronResult<CosmosMsg> {
    let kv_keys = create_gov_proposals_voters_votes_keys(proposals_ids, voters)?;

    register_interchain_query(
        contract,
        QueryPayload::KV(kv_keys),
        connection_id,
        update_period,
    )
}

/// Creates a message to update an Interchain Query to get governance proposals votes on the remote chain
///
/// * **contract** is an address of the contract that registers the query. Must be contract that sends this message.
/// * **query_id** is an IBC connection identifier between Neutron and remote chain;
/// * **proposals_ids** is a list of proposals ids from remote chain.
/// * **voters** is a list of voter to get voting info from remote chain.
/// * **new_update_period** is used to update period of how often the query must be updated.
pub fn update_gov_proposals_votes_query_msg(
    contract: Addr,
    query_id: u64,
    proposals_ids: Vec<u64>,
    voters: Vec<String>,
    new_update_period: u64,
) -> NeutronResult<CosmosMsg> {
    let kv_keys = create_gov_proposals_voters_votes_keys(proposals_ids, voters)?;

    update_interchain_query(contract, query_id, kv_keys, new_update_period, None)
}

/// Creates a message to register an Interchain Query to get validator info on remote chain
///
/// * **contract** is an address of the contract that registers the query. Must be contract that sends this message.
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **validator** is an validator operator address of an account on remote chain for which you want to get rewards ;
/// * **update_period** is used to say how often the query must be updated.
pub fn new_register_staking_validators_query_msg(
    contract: Addr,
    connection_id: String,
    validators: Vec<String>,
    update_period: u64,
) -> NeutronResult<CosmosMsg> {
    let mut kv_keys: Vec<KvKey> = Vec::with_capacity(validators.len());

    for validator in validators {
        let val_addr = decode_and_convert(&validator)?;

        let kv_key = KvKey {
            path: STAKING_STORE_KEY.to_string(),
            key: create_validator_key(&val_addr)?,
        };

        kv_keys.push(kv_key)
    }

    register_interchain_query(
        contract,
        QueryPayload::KV(kv_keys),
        connection_id,
        update_period,
    )
}

/// Creates a message to register an Interchain Query to get validators signing infos on remote chain
///
/// * **contract** is an address of the contract that registers the query. Must be contract that sends this message.
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **validators** is an list of validators valcons addresses of an account on remote chain for which you want to get rewards ;
/// * **update_period** is used to say how often the query must be updated.
pub fn new_register_validators_signing_infos_query_msg(
    contract: Addr,
    connection_id: String,
    validators: Vec<String>,
    update_period: u64,
) -> NeutronResult<CosmosMsg> {
    let mut kv_keys: Vec<KvKey> = Vec::with_capacity(validators.len());

    for validator in validators {
        let valcons_addr = decode_and_convert(&validator)?;

        let kv_key = KvKey {
            path: SLASHING_STORE_KEY.to_string(),
            key: create_validator_signing_info_key(&valcons_addr)?,
        };

        kv_keys.push(kv_key)
    }

    register_interchain_query(
        contract,
        QueryPayload::KV(kv_keys),
        connection_id,
        update_period,
    )
}

/// Creates a message to register an Interchain Query to get delegations of particular delegator on remote chain.
///
/// * **contract** is an address of the contract that registers the query. Must be contract that sends this message.
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **delegator** is an address of an account on remote chain for which you want to get list of delegations;
/// * **validators** is a list of validators addresses for which you want to get delegations from particular **delegator**;
/// * **update_period** is used to say how often the query must be updated.
pub fn new_register_delegator_delegations_query_msg(
    contract: Addr,
    connection_id: String,
    delegator: String,
    validators: Vec<String>,
    update_period: u64,
) -> NeutronResult<CosmosMsg> {
    let delegator_addr = decode_and_convert(&delegator)?;

    // Allocate memory for such KV keys as:
    // * staking module params to get staking denomination
    // * validators structures to calculate amount of delegated tokens
    // * delegations structures to get info about delegations itself
    let mut keys: Vec<KvKey> = Vec::with_capacity(validators.len() * 2 + 1);

    // create KV key to get BondDenom from staking module params
    keys.push(KvKey {
        path: PARAMS_STORE_KEY.to_string(),
        key: create_params_store_key(STAKING_STORE_KEY, KEY_BOND_DENOM),
    });

    for v in validators {
        let val_addr = decode_and_convert(&v)?;

        // create delegation key to get delegation structure
        keys.push(KvKey {
            path: STAKING_STORE_KEY.to_string(),
            key: create_delegation_key(&delegator_addr, &val_addr)?,
        });

        // create validator key to get validator structure
        keys.push(KvKey {
            path: STAKING_STORE_KEY.to_string(),
            key: create_validator_key(&val_addr)?,
        })
    }

    register_interchain_query(
        contract,
        QueryPayload::KV(keys),
        connection_id,
        update_period,
    )
}

/// Creates a message to register an Interchain Query to get unbonding delegations of particular delegator on remote chain.
///
/// * **contract** is an address of the contract that registers the query. Must be contract that sends this message.
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **delegator** is an address of an account on remote chain for which you want to get list of unbonding delegations;
/// * **validators** is a list of validators addresses for which you want to get unbonding delegations from particular **delegator**;
/// * **update_period** is used to say how often the query must be updated.
pub fn new_register_delegator_unbonding_delegations_query_msg(
    contract: Addr,
    connection_id: String,
    delegator: String,
    validators: Vec<String>,
    update_period: u64,
) -> NeutronResult<CosmosMsg> {
    let delegator_addr = decode_and_convert(&delegator)?;

    // Allocate memory, one KV key per validator
    let mut keys: Vec<KvKey> = Vec::with_capacity(validators.len());

    for v in validators {
        let val_addr = decode_and_convert(&v)?;

        // create unbonding delegation key to get unbonding delegation structure
        keys.push(KvKey {
            path: STAKING_STORE_KEY.to_string(),
            key: create_unbonding_delegation_key(&delegator_addr, &val_addr)?,
        })
    }

    register_interchain_query(
        contract,
        QueryPayload::KV(keys),
        connection_id,
        update_period,
    )
}

/// Creates a message to register an Interchain Query to get wasm contract store on remote chain
/// from **wasm** module
///
/// * **contract** is an address of the contract that registers the query. Must be contract that sends this message.
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **contract_address** is an address of a contract on a remote chain;
/// * **key** is a wasm contract store key;
/// * **update_period** is used to say how often the query must be updated.
///
/// Obtaining a **key** might not be a trivial task. One could list all contract's storage keys
/// using `$CHAIN_BIN query wasm contract-state all $CONTRACT_ADDRESS --output json | jq`.
/// The listed keys will be in format of plain hexadecimal string which is hard to understand
/// just by looking it. One could pipe this string into `| xxd -r -p | hexdump -C` and examine
/// its contents.
pub fn new_register_wasm_contract_store_query_msg(
    contract: Addr,
    connection_id: String,
    contract_address: String,
    key: impl AsRef<[u8]>,
    update_period: u64,
) -> NeutronResult<CosmosMsg> {
    let converted_addr_bytes = decode_and_convert(contract_address.as_str())?;
    let wasm_key = create_wasm_contract_store_key(converted_addr_bytes, key.as_ref())?;

    let kv_key = KvKey {
        path: WASM_STORE_KEY.to_string(),
        key: wasm_key,
    };

    register_interchain_query(
        contract,
        QueryPayload::KV(vec![kv_key]),
        connection_id,
        update_period,
    )
}

/// Creates a message to register an Interchain Query to get transfer events to a recipient on a remote chain.
///
/// * **contract** is an address of the contract that registers the query. Must be contract that sends this message.
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **recipient** is an address of an account on remote chain for which you want to get list of transfer transactions;
/// * **update_period** is used to say how often the query must be updated.
/// * **min_height** is used to set min height for query (by default = 0).
pub fn new_register_transfers_query_msg(
    contract: Addr,
    connection_id: String,
    recipient: String,
    update_period: u64,
    min_height: Option<u64>,
) -> NeutronResult<CosmosMsg> {
    let mut query_data = vec![TransactionFilterItem {
        field: RECIPIENT_FIELD.to_string(),
        op: TransactionFilterOp::Eq,
        value: TransactionFilterValue::String(recipient),
    }];
    if let Some(min_height) = min_height {
        query_data.push(TransactionFilterItem {
            field: HEIGHT_FIELD.to_string(),
            op: TransactionFilterOp::Gte,
            value: TransactionFilterValue::Int(min_height),
        })
    }

    register_interchain_query(
        contract,
        QueryPayload::TX(query_data),
        connection_id,
        update_period,
    )
}
