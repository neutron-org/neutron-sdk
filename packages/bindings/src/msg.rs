use crate::types::{KVKey, ProtobufAny};
use cosmwasm_std::{CosmosMsg, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// A number of Custom messages that can call into the Neutron bindings
pub enum NeutronMsg {
    /// RegisterInterchainAccount registers an interchain account on remote chain
    RegisterInterchainAccount {
        /// **connection_id** is an IBC connection identifier between Neutron and remote chain
        connection_id: String,

        /// **interchain_account_id** is an identifier of your new interchain account. Can be any string
        /// This identifier allows contracts to have multiple interchain accounts on remote chains
        interchain_account_id: String,
    },

    /// SubmitTx starts the process of executing any Cosmos-SDK *msgs* on remote chain
    SubmitTX {
        /// **connection_id** is an IBC connection identifier between Neutron and remote chain
        connection_id: String,

        /// **interchain_account_id** is an identifier of your interchain account from which you want to execute msgs
        interchain_account_id: String,

        /// **msgs** is a list of protobuf encoded Cosmos-SDK messages you want to execute on remote chain
        msgs: Vec<ProtobufAny>,

        /// **memo** is a memo you want to attach to your interchain transaction.It behaves like a memo in usual Cosmos transaction
        memo: String,
    },

    /// RegisterInterchainQuery registers an interchain query
    RegisterInterchainQuery {
        /// **query_type** is a query type identifier ('tx' or 'kv' for now)
        query_type: String,

        /// **keys** is the KV-storage keys for which we want to get values from remote chain
        keys: Vec<KVKey>,

        /// **transactions_filter** is the filter for transaction search ICQ
        transactions_filter: String,

        /// **zone_id** is used to identify the chain of interest
        zone_id: String,

        /// **connection_id** is an IBC connection identifier between Neutron and remote chain
        connection_id: String,

        /// **update_period** is used to say how often the query must be updated.
        update_period: u64,
    },

    /// RegisterInterchainQuery updates an interchain query
    UpdateInterchainQuery {
        /// **query_id** is ID of the query we want to update
        query_id: u64,

        /// **new_query_data** is new data(basicly query itself) we want to be performed
        new_query_data: Option<String>,

        /// **new_update_period** is a new update period of the query
        new_update_period: Option<u64>,
    },

    /// RemoveInterchainQuery removes as interchain query
    RemoveInterchainQuery {
        /// **query_id** is ID of the query we want to remove
        query_id: u64,
    },
}

impl NeutronMsg {
    /// Basic helper to define a register interchain account message:
    /// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
    /// * **interchain_account_id** is an identifier of your new interchain account. Can be any string.
    pub fn register_interchain_account(
        connection_id: String,
        interchain_account_id: String,
    ) -> Self {
        NeutronMsg::RegisterInterchainAccount {
            connection_id,
            interchain_account_id,
        }
    }

    /// Basic helper to define a submit tx message:
    /// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
    /// * **interchain_account_id** is an identifier of your interchain account from which you want to execute msgs;
    /// * **msgs** is a list of protobuf encoded Cosmos-SDK messages you want to execute on remote chain;
    /// * **memo** is a memo you want to attach to your interchain transaction.It behaves like a memo in usual Cosmos transaction.
    pub fn submit_tx(
        connection_id: String,
        interchain_account_id: String,
        msgs: Vec<ProtobufAny>,
        memo: String,
    ) -> Self {
        NeutronMsg::SubmitTX {
            connection_id,
            interchain_account_id,
            msgs,
            memo,
        }
    }

    /// Basic helper to define a register interchain query message:
    /// * **query_type** is a query type identifier ('tx' or 'kv' for now)
    /// * **keys** is the KV-storage keys for which we want to get values from remote chain
    /// * **transactions_filter** is the filter for transaction search ICQ
    /// * **zone_id** is used to identify the chain of interest
    /// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
    /// * **update_period** is used to say how often the query must be updated.
    pub fn register_interchain_query(
        query_type: String,
        keys: Vec<KVKey>,
        transactions_filter: String,
        zone_id: String,
        connection_id: String,
        update_period: u64,
    ) -> Self {
        NeutronMsg::RegisterInterchainQuery {
            query_type,
            keys,
            transactions_filter,
            zone_id,
            connection_id,
            update_period,
        }
    }

    /// Basic helper to define a update interchain query message:
    /// * **query_id** is ID of the query we want to update
    /// * **new_query_data** is a JSON encoded data of query;
    /// * **new_update_period** is used to say how often the query must be updated.
    pub fn update_interchain_query(
        query_id: u64,
        new_query_data: Option<String>,
        new_update_period: Option<u64>,
    ) -> Self {
        NeutronMsg::UpdateInterchainQuery {
            query_id,
            new_query_data,
            new_update_period,
        }
    }

    /// Basic helper to define a remove interchain query message:
    /// * **query_id** is ID of the query we want to remove
    pub fn remove_interchain_query(query_id: u64) -> Self {
        NeutronMsg::RemoveInterchainQuery { query_id }
    }
}

impl From<NeutronMsg> for CosmosMsg<NeutronMsg> {
    fn from(msg: NeutronMsg) -> CosmosMsg<NeutronMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for NeutronMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// Describes response structure for **RegisterInterchainQuery** msg
pub struct MsgRegisterInterchainQueryResponse {
    /// **id** is an identifier of newly registered interchain query
    pub id: u64,
}
