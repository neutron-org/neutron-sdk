use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// this query goes to neutron and get stored ICA with a specific query
    InterchainAccountAddress {
        interchain_account_id: String,
        connection_id: String,
    },
    // this query returns ICA from contract store, which saved from acknowledgement
    InterchainAccountAddressFromContract {
        interchain_account_id: String,
    },
    // this query returns acknowledgement result after interchain transaction
    AcknowledgementResult {
        interchain_account_id: String,
        sequence_id: u64,
    },
    // this query returns the sequence ID of the channel the last ack came from
    LastAckSeqId {},
    Balance {
        query_id: u64,
    },
    GetRecipientTxs {
        recipient: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Register {
        connection_id: String,
        interchain_account_id: String,
    },
    Delegate {
        interchain_account_id: String,
        validator: String,
        amount: u128,
        denom: String,
        timeout: Option<u64>,
    },
    Undelegate {
        interchain_account_id: String,
        validator: String,
        amount: u128,
        denom: String,
        timeout: Option<u64>,
    },
    CleanAckResults {},
    CleanRecipientTxs {},
    SetFees {
        denom: String,
        recv_fee: u128,
        ack_fee: u128,
        timeout_fee: u128,
    },
    RegisterBalanceQuery {
        connection_id: String,
        update_period: u64,
        addr: String,
        denom: String,
    },
    RegisterTransfersQuery {
        connection_id: String,
        update_period: u64,
        recipient: String,
        min_height: Option<u64>,
    },
    RemoveInterchainQuery {
        query_id: u64,
    },
}
