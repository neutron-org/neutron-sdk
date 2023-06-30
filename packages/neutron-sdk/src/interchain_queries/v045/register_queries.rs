use crate::interchain_queries::types::{
    QueryPayload, TransactionFilterItem, TransactionFilterOp, TransactionFilterValue,
};
use crate::interchain_queries::v045::types::{
    BANK_STORE_KEY, DISTRIBUTION_STORE_KEY, GOV_STORE_KEY, HEIGHT_FIELD, KEY_BOND_DENOM,
    PARAMS_STORE_KEY, RECIPIENT_FIELD, STAKING_STORE_KEY, WASM_STORE_KEY,
};
use crate::{
    bindings::{msg::NeutronMsg, types::KVKey},
    errors::error::NeutronResult,
    interchain_queries::helpers::decode_and_convert,
    interchain_queries::v045::helpers::{
        create_account_denom_balance_key, create_delegation_key, create_fee_pool_key,
        create_gov_proposal_key, create_params_store_key, create_total_denom_key,
        create_validator_key, create_wasm_contract_store_key,
    },
};
use cosmwasm_std::Binary;

/// Creates a message to register an Interchain Query to get balance of account on remote chain for particular denom
///
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **addr** address of an account on remote chain for which you want to get balances;
/// * **denom** denomination of the coin for which you want to get balance;
/// * **update_period** is used to say how often the query must be updated.
pub fn new_register_balance_query_msg(
    connection_id: String,
    addr: String,
    denom: String,
    update_period: u64,
) -> NeutronResult<NeutronMsg> {
    let converted_addr_bytes = decode_and_convert(addr.as_str())?;

    let balance_key = create_account_denom_balance_key(converted_addr_bytes, denom)?;

    let kv_key = KVKey {
        path: BANK_STORE_KEY.to_string(),
        key: Binary(balance_key),
    };

    NeutronMsg::register_interchain_query(
        QueryPayload::KV(vec![kv_key]),
        connection_id,
        update_period,
    )
}

/// Creates a message to register an Interchain Query to get total supply on remote chain for particular denom
///
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **denom** denomination of the coin for which you want to get total supply;
/// * **update_period** is used to say how often the query must be updated.
pub fn new_register_bank_total_supply_query_msg(
    connection_id: String,
    denoms: Vec<String>,
    update_period: u64,
) -> NeutronResult<NeutronMsg> {
    let mut kv_keys: Vec<KVKey> = Vec::with_capacity(denoms.len());

    for denom in denoms {
        let supply_key = create_total_denom_key(denom)?;

        let kv_key = KVKey {
            path: BANK_STORE_KEY.to_string(),
            key: Binary(supply_key),
        };

        kv_keys.push(kv_key)
    }

    NeutronMsg::register_interchain_query(QueryPayload::KV(kv_keys), connection_id, update_period)
}

/// Creates a message to register an Interchain Query to get fee pool on remote chain from distribution module
///
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **update_period** is used to say how often the query must be updated.
pub fn new_register_distribution_fee_pool_query_msg(
    connection_id: String,
    update_period: u64,
) -> NeutronResult<NeutronMsg> {
    let kv_key = KVKey {
        path: DISTRIBUTION_STORE_KEY.to_string(),
        key: Binary(create_fee_pool_key()?),
    };

    NeutronMsg::register_interchain_query(
        QueryPayload::KV(vec![kv_key]),
        connection_id,
        update_period,
    )
}

/// Creates a message to register an Interchain Query to get governance proposal on remote chain
///
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **proposal_id** is a proposal id from remote chain.
/// * **update_period** is used to say how often the query must be updated.
pub fn new_register_gov_proposal_query_msg(
    connection_id: String,
    proposals_ids: Vec<u64>,
    update_period: u64,
) -> NeutronResult<NeutronMsg> {
    let mut kv_keys: Vec<KVKey> = Vec::with_capacity(proposals_ids.len());

    for id in proposals_ids {
        let kv_key = KVKey {
            path: GOV_STORE_KEY.to_string(),
            key: Binary(create_gov_proposal_key(id)?),
        };

        kv_keys.push(kv_key)
    }

    NeutronMsg::register_interchain_query(QueryPayload::KV(kv_keys), connection_id, update_period)
}

/// Creates a message to register an Interchain Query to get validator info on remote chain
///
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **validator** is an validator operator address of an account on remote chain for which you want to get rewards ;
/// * **update_period** is used to say how often the query must be updated.
pub fn new_register_staking_validators_query_msg(
    connection_id: String,
    validators: Vec<String>,
    update_period: u64,
) -> NeutronResult<NeutronMsg> {
    let mut kv_keys: Vec<KVKey> = Vec::with_capacity(validators.len());

    for validator in validators {
        let val_addr = decode_and_convert(&validator)?;

        let kv_key = KVKey {
            path: STAKING_STORE_KEY.to_string(),
            key: Binary(create_validator_key(&val_addr)?),
        };

        kv_keys.push(kv_key)
    }

    NeutronMsg::register_interchain_query(QueryPayload::KV(kv_keys), connection_id, update_period)
}

/// Creates a message to register an Interchain Query to get delegations of particular delegator on remote chain.
///
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **delegator** is an address of an account on remote chain for which you want to get list of delegations;
/// * **validators** is a list of validators addresses for which you want to get delegations from particular **delegator**;
/// * **update_period** is used to say how often the query must be updated.
pub fn new_register_delegator_delegations_query_msg(
    connection_id: String,
    delegator: String,
    validators: Vec<String>,
    update_period: u64,
) -> NeutronResult<NeutronMsg> {
    let delegator_addr = decode_and_convert(delegator.as_str())?;

    // Allocate memory for such KV keys as:
    // * staking module params to get staking denomination
    // * validators structures to calculate amount of delegated tokens
    // * delegations structures to get info about delegations itself
    let mut keys: Vec<KVKey> = Vec::with_capacity(validators.len() * 2 + 1);

    // create KV key to get BondDenom from staking module params
    keys.push(KVKey {
        path: PARAMS_STORE_KEY.to_string(),
        key: Binary(create_params_store_key(STAKING_STORE_KEY, KEY_BOND_DENOM)),
    });

    for v in &validators {
        // create delegation key to get delegation structure
        let val_addr = decode_and_convert(v.as_str())?;
        keys.push(KVKey {
            path: STAKING_STORE_KEY.to_string(),
            key: Binary(create_delegation_key(&delegator_addr, &val_addr)?),
        });

        // create validator key to get validator structure
        keys.push(KVKey {
            path: STAKING_STORE_KEY.to_string(),
            key: Binary(create_validator_key(&val_addr)?),
        })
    }

    NeutronMsg::register_interchain_query(QueryPayload::KV(keys), connection_id, update_period)
}

/// Creates a message to register an Interchain Query to get wasm contract store on remote chain
/// from **wasm** module
///
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
    connection_id: String,
    contract_address: String,
    key: impl AsRef<[u8]>,
    update_period: u64,
) -> NeutronResult<NeutronMsg> {
    let converted_addr_bytes = decode_and_convert(contract_address.as_str())?;
    let wasm_key = create_wasm_contract_store_key(converted_addr_bytes, key.as_ref())?;

    let kv_key = KVKey {
        path: WASM_STORE_KEY.to_string(),
        key: Binary(wasm_key),
    };

    NeutronMsg::register_interchain_query(
        QueryPayload::KV(vec![kv_key]),
        connection_id,
        update_period,
    )
}

/// Creates a message to register an Interchain Query to get transfer events to a recipient on a remote chain.
///
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **recipient** is an address of an account on remote chain for which you want to get list of transfer transactions;
/// * **update_period** is used to say how often the query must be updated.
/// * **min_height** is used to set min height for query (by default = 0).
pub fn new_register_transfers_query_msg(
    connection_id: String,
    recipient: String,
    update_period: u64,
    min_height: Option<u64>,
) -> NeutronResult<NeutronMsg> {
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

    NeutronMsg::register_interchain_query(
        QueryPayload::TX(query_data),
        connection_id,
        update_period,
    )
}
