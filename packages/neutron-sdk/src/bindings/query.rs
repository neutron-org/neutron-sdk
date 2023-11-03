use crate::bindings::msg::{QueryCondition, Timestamp};
use crate::bindings::types::{Failure, InterchainQueryResult, RegisteredQuery};
use cosmwasm_std::{Binary, Coin, CustomQuery};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// The queries to interact with neutron specific blockchain modules.
pub enum NeutronQuery {
    /// Query a result of registered interchain query on remote chain
    InterchainQueryResult {
        /// **query_id** is an ID registered interchain query
        query_id: u64,
    },

    /// Query a registered interchain account address for a specific connection_id
    /// Every contract may have as many interchain accounts as necessary.
    InterchainAccountAddress {
        /// **owner_address** is an address of contract which registered interchain account
        owner_address: String,

        /// **interchain_account_id** is an identifier of your interchain account. Can be any string
        /// This identifier allows contracts to have multiple interchain accounts on remote chains
        interchain_account_id: String,

        /// **connection_id** is an IBC connection identifier between Neutron and remote chain
        connection_id: String,
    },

    /// Query all registered interchain queries on all remote chains
    RegisteredInterchainQueries {
        owners: Vec<String>,
        connection_id: String,
        pagination: PageRequest,
    },

    /// Query registered interchain query with a specific query_id
    RegisteredInterchainQuery {
        /// **query_id** is an ID registered interchain query
        query_id: u64,
    },

    /// Query total amount of burned neutron fees
    TotalBurnedNeutronsAmount {},

    /// Query minimum IBC fee
    MinIbcFee {},

    /// TokenFactory query. Given a subdenom minted by a contract via
    /// [`NeutronMsg::MintTokens`](crate::bindings::msg::NeutronMsg::MintTokens),
    /// returns the full denom as used by [`BankMsg::Send`](cosmwasm_std::BankMsg::Send).
    FullDenom {
        creator_addr: String,
        subdenom: String,
    },

    /// TokenFactory query. Returns the admin of a denom, if the denom is a TokenFactory denom.
    DenomAdmin { subdenom: String },

    /// TokenFactory query. Returns the before send hook address of a denom, if the denom is a TokenFactory denom.
    BeforeSendHook { denom: String },

    /// Contractmanager query. Returns the failures for a particular contract address.
    Failures {
        address: String,
        pagination: PageRequest,
    },

    /// Incentives module queries
    Incentives(IncentivesQuery),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageRequest {
    /// **key** is a value returned in PageResponse.next_key to begin
    /// querying the next page most efficiently. Only one of offset or key
    /// should be set.
    pub key: Binary,
    /// **offset** is a numeric offset that can be used when key is unavailable.
    /// It is less efficient than using key. Only one of offset or key should
    /// be set.
    pub offset: u64,
    /// **limit** is the total number of results to be returned in the result page.
    /// If left empty it will default to a value to be set by each app.
    pub limit: u64,
    /// **count_total** is set to true  to indicate that the result set should include
    /// a count of the total number of items available for pagination in UIs.
    /// count_total is only respected when offset is used. It is ignored when key
    /// is set.
    pub count_total: bool,
    /// reverse is set to true if results are to be returned in the descending order.
    pub reverse: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryRegisteredQueriesResponse {
    /// **registered_queries** is a list of registered queries
    pub registered_queries: Vec<RegisteredQuery>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryRegisteredQueryResponse {
    /// **registered_query** is a registered query
    pub registered_query: RegisteredQuery,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryRegisteredQueryResultResponse {
    pub result: InterchainQueryResult,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryInterchainAccountAddressResponse {
    /// **interchain_account_address** is a interchain account address on the remote chain
    pub interchain_account_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryFailuresResponse {
    /// **failures** is a list of failures of sudo handler calls
    pub failures: Vec<Failure>,
}

impl CustomQuery for NeutronQuery {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IncentivesQuery {
    ModuleStatus {},
    GaugeByID {
        id: u64,
    },
    Gauges {
        status: String,
        denom: String,
    },
    StakeByID {
        stake_id: u64,
    },
    Stakes {
        owner: String,
    },
    FutureRewardEstimate {
        owner: String,
        stake_ids: Vec<u64>,
        num_epochs: i64,
    },
    AccountHistory {
        account: String,
    },
    GaugeQualifyingValue {
        id: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GaugeByIDResponse {
    /// **gauge** is the found gauge
    pub gauge: Gauge,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GaugesResponse {
    /// **gauges** is the list of gauges
    pub gauges: Vec<Gauge>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct StakeByIDResponse {
    /// **stake** is the found stake
    pub stake: Stake,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct StakesResponse {
    /// **stakes** is the list of stakes
    pub stakes: Vec<Stake>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FutureRewardEstimateResponse {
    /// **coins** is the future estimate of rewards
    pub coins: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AccountHistoryResponse {
    /// **coins** is the account history // TODO: normal comment
    pub coins: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GaugeQualifyingValueResponse {
    /// **qualifying_value** // TODO: comment
    pub qualifying_value: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Gauge {
    /// **id** is the gauge id
    pub id: u64,
    /// **is_perpetual**  shows if it's a perpetual or non-perpetual gauge
    ///  Non-perpetual gauges distribute their tokens equally per epoch while the
    ///  gauge is in the active period. Perpetual gauges distribute all their tokens
    ///  at a single time and only distribute their tokens again once the gauge is
    ///  refilled
    pub is_perpetual: bool,
    /// **distribute_to** shows which lock the gauge should distribute to by time
    /// duration or by timestamp
    pub distribute_to: QueryCondition,
    /// **coins** are coins to be distributed by the gauge
    pub coins: Vec<Coin>,
    /// **start_time** is the distribution start time
    pub start_time: Timestamp,
    /// **num_epochs_paid_over** is the number of epochs distribution
    /// will be completed over
    pub num_epochs_paid_over: u64,
    /// **filled_epochs** describes the number of epochs distribution
    /// have been completed already
    pub filled_epochs: u64,
    /// **distributed_coins** describes coins that have been distributed already from
    /// this gauge.
    pub distributed_coins: Vec<Coin>,
    /// **pricing_tick** is the price that liquidity within the gauge range will be priced at
    pub pricing_tick: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Stake {
    /// **id** is the stake id
    pub id: u64,
    /// **owner** is the account originating the stake. Only the owner can withdraw
    /// coins from the stake.
    pub owner: String,
    /// **start_time** is the time at which the coins in the lock were staked
    pub start_time: Timestamp,
    /// **coins** are the tokens staked, and managed by the module account
    pub coins: Vec<Coin>,
    /// **start_dist_epoch** is the dist epoch (defaulting to the day) at which the
    /// coins in the lock were staked. This is used by distribution logic to filter
    /// on stakes that have existed for longer than the distribution period (you
    /// can only qualify for today's rewards if you staked your LP tokens
    /// yesterday). We use int64 instead of uint64 to make testing easier.
    pub start_dist_epoch: i64,
}
