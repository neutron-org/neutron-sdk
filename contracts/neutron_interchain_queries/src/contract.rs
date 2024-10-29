use cosmos_sdk_proto::cosmos::bank::v1beta1::MsgSend;
use cosmos_sdk_proto::cosmos::tx::v1beta1::{TxBody, TxRaw};
use cosmos_sdk_proto::traits::Message;
use cosmwasm_std::{
    entry_point, to_json_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Response, StdError, StdResult, Uint128,
};
use cw2::set_contract_version;
use neutron_sdk::interchain_queries::v047::register_queries::new_register_validators_signing_infos_query_msg;
use neutron_std::types::neutron::interchainqueries::{KvKey, RegisteredQuery};

use crate::msg::{
    Cw20BalanceResponse, ExecuteMsg, GetRecipientTxsResponse, InstantiateMsg, MigrateMsg, QueryMsg,
};
use crate::state::{Transfer, RECIPIENT_TXS, TRANSFERS};
use neutron_sdk::interchain_queries::helpers::{
    remove_interchain_query as helpers_remove_interchain_query,
    update_interchain_query as helpers_update_interchain_query,
};
use neutron_sdk::interchain_queries::v047::queries::{
    query_balance, query_bank_total, query_delegations, query_distribution_fee_pool,
    query_government_proposals, query_staking_validators, query_unbonding_delegations,
    query_validators_signing_infos,
};
use neutron_sdk::interchain_queries::{
    check_query_type, get_registered_query, query_kv_result,
    v047::{
        register_queries::{
            new_register_balances_query_msg, new_register_bank_total_supply_query_msg,
            new_register_delegator_delegations_query_msg,
            new_register_delegator_unbonding_delegations_query_msg,
            new_register_distribution_fee_pool_query_msg, new_register_gov_proposals_query_msg,
            new_register_staking_validators_query_msg, new_register_transfers_query_msg,
            new_register_wasm_contract_store_query_msg,
        },
        types::{COSMOS_SDK_TRANSFER_MSG_URL, RECIPIENT_FIELD},
    },
};
use neutron_sdk::sudo::msg::{Height, SudoMsg};
use neutron_sdk::{NeutronError, NeutronResult};

use neutron_sdk::interchain_queries::types::{
    QueryType, TransactionFilterItem, TransactionFilterOp, TransactionFilterValue,
};
use serde_json_wasm;

/// defines the incoming transfers limit to make a case of failed callback possible.
const MAX_ALLOWED_TRANSFER: u64 = 20000;
const MAX_ALLOWED_MESSAGES: usize = 20;

const CONTRACT_NAME: &str = concat!("crates.io:neutron-sdk__", env!("CARGO_PKG_NAME"));
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> NeutronResult<Response> {
    deps.api.debug("WASMDEBUG: instantiate");
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    env: Env,
    _: MessageInfo,
    msg: ExecuteMsg,
) -> NeutronResult<Response> {
    match msg {
        ExecuteMsg::RegisterBalancesQuery {
            connection_id,
            addr,
            denoms,
            update_period,
        } => register_balances_query(
            env.contract.address,
            connection_id,
            addr,
            denoms,
            update_period,
        ),
        ExecuteMsg::RegisterBankTotalSupplyQuery {
            connection_id,
            denoms,
            update_period,
        } => register_bank_total_supply_query(
            env.contract.address,
            connection_id,
            denoms,
            update_period,
        ),
        ExecuteMsg::RegisterDistributionFeePoolQuery {
            connection_id,
            update_period,
        } => {
            register_distribution_fee_pool_query(env.contract.address, connection_id, update_period)
        }
        ExecuteMsg::RegisterGovernmentProposalsQuery {
            connection_id,
            proposals_ids,
            update_period,
        } => register_gov_proposal_query(
            env.contract.address,
            connection_id,
            proposals_ids,
            update_period,
        ),
        ExecuteMsg::RegisterStakingValidatorsQuery {
            connection_id,
            validators,
            update_period,
        } => register_staking_validators_query(
            env.contract.address,
            connection_id,
            validators,
            update_period,
        ),
        ExecuteMsg::RegisterValidatorsSigningInfosQuery {
            connection_id,
            validators,
            update_period,
        } => register_validators_signing_infos_query(
            env.contract.address,
            connection_id,
            validators,
            update_period,
        ),
        ExecuteMsg::RegisterDelegatorDelegationsQuery {
            connection_id,
            delegator,
            validators,
            update_period,
        } => register_delegations_query(
            env.contract.address,
            connection_id,
            delegator,
            validators,
            update_period,
        ),
        ExecuteMsg::RegisterDelegatorUnbondingDelegationsQuery {
            connection_id,
            delegator,
            validators,
            update_period,
        } => register_unbonding_delegations_query(
            env.contract.address,
            connection_id,
            delegator,
            validators,
            update_period,
        ),
        ExecuteMsg::RegisterTransfersQuery {
            connection_id,
            recipient,
            update_period,
            min_height,
        } => register_transfers_query(
            env.contract.address,
            connection_id,
            recipient,
            update_period,
            min_height,
        ),
        ExecuteMsg::RegisterCw20BalanceQuery {
            connection_id,
            update_period,
            cw20_contract_address,
            account_address,
        } => register_cw20_balance_query(
            env.contract.address,
            connection_id,
            update_period,
            cw20_contract_address,
            account_address,
        ),
        ExecuteMsg::UpdateInterchainQuery {
            query_id,
            new_keys,
            new_update_period,
            new_recipient,
        } => update_interchain_query(
            env.contract.address,
            query_id,
            new_keys,
            new_update_period,
            new_recipient,
        ),
        ExecuteMsg::RemoveInterchainQuery { query_id } => {
            remove_interchain_query(env.contract.address, query_id)
        }
    }
}

pub fn register_balances_query(
    contract: Addr,
    connection_id: String,
    addr: String,
    denoms: Vec<String>,
    update_period: u64,
) -> NeutronResult<Response> {
    let msg: CosmosMsg =
        new_register_balances_query_msg(contract, connection_id, addr, denoms, update_period)?;

    Ok(Response::new().add_message(msg))
}

pub fn register_bank_total_supply_query(
    contract: Addr,
    connection_id: String,
    denoms: Vec<String>,
    update_period: u64,
) -> NeutronResult<Response> {
    let msg =
        new_register_bank_total_supply_query_msg(contract, connection_id, denoms, update_period)?;

    Ok(Response::new().add_message(msg))
}

pub fn register_distribution_fee_pool_query(
    contract: Addr,
    connection_id: String,
    update_period: u64,
) -> NeutronResult<Response> {
    let msg = new_register_distribution_fee_pool_query_msg(contract, connection_id, update_period)?;

    Ok(Response::new().add_message(msg))
}

pub fn register_gov_proposal_query(
    contract: Addr,
    connection_id: String,
    proposals_ids: Vec<u64>,
    update_period: u64,
) -> NeutronResult<Response> {
    let msg = new_register_gov_proposals_query_msg(
        contract,
        connection_id,
        proposals_ids,
        update_period,
    )?;

    Ok(Response::new().add_message(msg))
}

pub fn register_staking_validators_query(
    contract: Addr,
    connection_id: String,
    validators: Vec<String>,
    update_period: u64,
) -> NeutronResult<Response> {
    let msg = new_register_staking_validators_query_msg(
        contract,
        connection_id,
        validators,
        update_period,
    )?;

    Ok(Response::new().add_message(msg))
}

pub fn register_validators_signing_infos_query(
    contract: Addr,
    connection_id: String,
    validators: Vec<String>,
    update_period: u64,
) -> NeutronResult<Response> {
    let msg = new_register_validators_signing_infos_query_msg(
        contract,
        connection_id,
        validators,
        update_period,
    )?;

    Ok(Response::new().add_message(msg))
}

pub fn register_delegations_query(
    contract: Addr,
    connection_id: String,
    delegator: String,
    validators: Vec<String>,
    update_period: u64,
) -> NeutronResult<Response> {
    let msg = new_register_delegator_delegations_query_msg(
        contract,
        connection_id,
        delegator,
        validators,
        update_period,
    )?;

    Ok(Response::new().add_message(msg))
}

pub fn register_unbonding_delegations_query(
    contract: Addr,
    connection_id: String,
    delegator: String,
    validators: Vec<String>,
    update_period: u64,
) -> NeutronResult<Response> {
    let msg = new_register_delegator_unbonding_delegations_query_msg(
        contract,
        connection_id,
        delegator,
        validators,
        update_period,
    )?;

    Ok(Response::new().add_message(msg))
}

pub fn register_transfers_query(
    contract: Addr,
    connection_id: String,
    recipient: String,
    update_period: u64,
    min_height: Option<u64>,
) -> NeutronResult<Response> {
    let msg = new_register_transfers_query_msg(
        contract,
        connection_id,
        recipient,
        update_period,
        min_height,
    )?;

    Ok(Response::new().add_message(msg))
}

pub fn register_cw20_balance_query(
    contract: Addr,
    connection_id: String,
    update_period: u64,
    cw20_contract_address: String,
    account_address: String,
) -> NeutronResult<Response> {
    // cw_storage_plus uses this prefix for maps
    let mut storage_key = vec![0u8, 7u8];

    storage_key.extend_from_slice("balance".as_bytes());
    storage_key.extend_from_slice(account_address.as_bytes());

    let msg = new_register_wasm_contract_store_query_msg(
        contract,
        connection_id,
        cw20_contract_address,
        &storage_key,
        update_period,
    )?;

    Ok(Response::new().add_message(msg))
}

pub fn update_interchain_query(
    contract: Addr,
    query_id: u64,
    new_keys: Vec<KvKey>,
    new_update_period: u64,
    new_recipient: Option<String>,
) -> NeutronResult<Response> {
    let new_filter = new_recipient.map(|recipient| {
        vec![TransactionFilterItem {
            field: RECIPIENT_FIELD.to_string(),
            op: TransactionFilterOp::Eq,
            value: TransactionFilterValue::String(recipient),
        }]
    });

    let update_msg = helpers_update_interchain_query(
        contract,
        query_id,
        new_keys,
        new_update_period,
        new_filter,
    )?;
    Ok(Response::new().add_message(update_msg))
}

pub fn remove_interchain_query(contract: Addr, query_id: u64) -> NeutronResult<Response> {
    let remove_msg = helpers_remove_interchain_query(contract, query_id)?;
    Ok(Response::new().add_message(remove_msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> NeutronResult<Binary> {
    match msg {
        //TODO: check if query.result.height is too old (for all interchain queries)
        QueryMsg::Balance { query_id } => Ok(to_json_binary(&query_balance(deps, env, query_id)?)?),
        QueryMsg::BankTotalSupply { query_id } => {
            Ok(to_json_binary(&query_bank_total(deps, env, query_id)?)?)
        }
        QueryMsg::DistributionFeePool { query_id } => Ok(to_json_binary(
            &query_distribution_fee_pool(deps, env, query_id)?,
        )?),
        QueryMsg::StakingValidators { query_id } => Ok(to_json_binary(&query_staking_validators(
            deps, env, query_id,
        )?)?),
        QueryMsg::ValidatorsSigningInfos { query_id } => Ok(to_json_binary(
            &query_validators_signing_infos(deps, env, query_id)?,
        )?),
        QueryMsg::GovernmentProposals { query_id } => Ok(to_json_binary(
            &query_government_proposals(deps, env, query_id)?,
        )?),
        QueryMsg::GetDelegations { query_id } => {
            Ok(to_json_binary(&query_delegations(deps, env, query_id)?)?)
        }
        QueryMsg::GetUnbondingDelegations { query_id } => Ok(to_json_binary(
            &query_unbonding_delegations(deps, env, query_id)?,
        )?),
        QueryMsg::Cw20Balance { query_id } => {
            Ok(to_json_binary(&query_cw20_balance(deps, env, query_id)?)?)
        }
        QueryMsg::GetRegisteredQuery { query_id } => {
            Ok(to_json_binary(&get_registered_query(deps, query_id)?)?)
        }
        QueryMsg::GetRecipientTxs { recipient } => query_recipient_txs(deps, recipient),
    }
}

fn query_recipient_txs(deps: Deps, recipient: String) -> NeutronResult<Binary> {
    let txs = RECIPIENT_TXS
        .load(deps.storage, &recipient)
        .unwrap_or_default();
    Ok(to_json_binary(&GetRecipientTxsResponse { transfers: txs })?)
}

pub fn query_cw20_balance(
    deps: Deps,
    _env: Env,
    registered_query_id: u64,
) -> NeutronResult<Cw20BalanceResponse> {
    let registered_query = get_registered_query(deps, registered_query_id)?;

    check_query_type(registered_query.query_type, QueryType::KV)?;

    let balance: Uint128 = query_kv_result(deps, registered_query_id)?;
    Ok(Cw20BalanceResponse { balance })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    deps.api.debug("WASMDEBUG: migrate");
    Ok(Response::default())
}

#[entry_point]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> NeutronResult<Response> {
    match msg {
        // For handling tx query result
        SudoMsg::TxQueryResult {
            query_id,
            height,
            data,
        } => sudo_tx_query_result(deps, env, query_id, height, data),

        // For handling kv query result
        SudoMsg::KVQueryResult { query_id } => sudo_kv_query_result(deps, env, query_id),
        _ => Ok(Response::default()),
    }
}

/// sudo_check_tx_query_result is an example callback for transaction query results that stores the
/// deposits received as a result on the registered query in the contract's state.
pub fn sudo_tx_query_result(
    deps: DepsMut,
    _env: Env,
    query_id: u64,
    _height: Height,
    data: Binary,
) -> NeutronResult<Response> {
    // Decode the transaction data
    let tx: TxRaw = TxRaw::decode(data.as_slice())?;
    let body: TxBody = TxBody::decode(tx.body_bytes.as_slice())?;

    // Get the registered query by ID and retrieve the raw query string
    let registered_query: RegisteredQuery = get_registered_query(deps.as_ref(), query_id)?;
    let transactions_filter = registered_query.transactions_filter;

    #[allow(clippy::match_single_binding)]
    // Depending of the query type, check the transaction data to see whether is satisfies
    // the original query. If you don't write specific checks for a transaction query type,
    // all submitted results will be treated as valid.
    //
    // TODO: come up with solution to determine transactions filter type
    match registered_query.query_type {
        _ => {
            // For transfer queries, query data looks like `[{"field:"transfer.recipient", "op":"eq", "value":"some_address"}]`
            let query_data: Vec<TransactionFilterItem> =
                serde_json_wasm::from_str(transactions_filter.as_str())?;

            let recipient = query_data
                .iter()
                .find(|x| x.field == RECIPIENT_FIELD && x.op == TransactionFilterOp::Eq)
                .map(|x| match &x.value {
                    TransactionFilterValue::String(v) => v.as_str(),
                    _ => "",
                })
                .unwrap_or("");

            let deposits = recipient_deposits_from_tx_body(body, recipient)?;
            // If we didn't find a Send message with the correct recipient, return an error, and
            // this query result will be rejected by Neutron: no data will be saved to state.
            if deposits.is_empty() {
                return Err(NeutronError::Std(StdError::generic_err(
                    "failed to find a matching transaction message",
                )));
            }

            let mut stored_transfers: u64 = TRANSFERS.load(deps.storage).unwrap_or_default();
            stored_transfers += deposits.len() as u64;
            TRANSFERS.save(deps.storage, &stored_transfers)?;
            check_deposits_size(&deposits)?;

            let mut stored_deposits: Vec<Transfer> = RECIPIENT_TXS
                .load(deps.storage, recipient)
                .unwrap_or_default();
            stored_deposits.extend(deposits);
            RECIPIENT_TXS.save(deps.storage, recipient, &stored_deposits)?;
            Ok(Response::new())
        }
    }
}

/// parses tx body and retrieves transactions to the given recipient.
fn recipient_deposits_from_tx_body(
    tx_body: TxBody,
    recipient: &str,
) -> NeutronResult<Vec<Transfer>> {
    let mut deposits: Vec<Transfer> = vec![];
    // Only handle up to MAX_ALLOWED_MESSAGES messages, everything else
    // will be ignored to prevent 'out of gas' conditions.
    // Note: in real contracts you will have to somehow save ignored
    // data in order to handle it later.
    for msg in tx_body.messages.iter().take(MAX_ALLOWED_MESSAGES) {
        // Skip all messages in this transaction that are not Send messages.
        if msg.type_url != *COSMOS_SDK_TRANSFER_MSG_URL.to_string() {
            continue;
        }

        // Parse a Send message and check that it has the required recipient.
        let transfer_msg: MsgSend = MsgSend::decode(msg.value.as_slice())?;
        if transfer_msg.to_address == recipient {
            for coin in transfer_msg.amount {
                deposits.push(Transfer {
                    sender: transfer_msg.from_address.clone(),
                    amount: coin.amount.clone(),
                    denom: coin.denom,
                    recipient: recipient.to_string(),
                });
            }
        }
    }
    Ok(deposits)
}

// checks whether there are deposits that are greater then MAX_ALLOWED_TRANSFER.
fn check_deposits_size(deposits: &Vec<Transfer>) -> StdResult<()> {
    for deposit in deposits {
        match deposit.amount.parse::<u64>() {
            Ok(amount) => {
                if amount > MAX_ALLOWED_TRANSFER {
                    return Err(StdError::generic_err(format!(
                        "maximum allowed transfer is {}",
                        MAX_ALLOWED_TRANSFER
                    )));
                };
            }
            Err(error) => {
                return Err(StdError::generic_err(format!(
                    "failed to cast transfer amount to u64: {}",
                    error
                )));
            }
        };
    }
    Ok(())
}

/// sudo_kv_query_result is the contract's callback for KV query results. Note that only the query
/// id is provided, so you need to read the query result from the state.
pub fn sudo_kv_query_result(deps: DepsMut, _env: Env, query_id: u64) -> NeutronResult<Response> {
    deps.api.debug(
        format!(
            "WASMDEBUG: sudo_kv_query_result received; query_id: {:?}",
            query_id,
        )
        .as_str(),
    );

    // TODO: provide an actual example. Currently to many things are going to change
    // after @pro0n00gler's PRs to implement this.

    Ok(Response::default())
}
