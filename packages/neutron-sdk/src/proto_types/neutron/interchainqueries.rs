use neutron_std_derive::CosmwasmExt;
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.Params")]
pub struct Params {
    /// Defines amount of blocks required before query becomes available for
    /// removal by anybody
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub query_submit_timeout: u64,
    /// Amount of coins deposited for the query.
    #[prost(message, repeated, tag = "2")]
    pub query_deposit: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    /// Amount of tx hashes to be removed during a single EndBlock. Can vary to
    /// balance between network cleaning speed and EndBlock duration. A zero value
    /// means no limit.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub tx_query_removal_limit: u64,
    /// Maximum amount of keys in a registered key value query
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_kv_query_keys_count: u64,
    /// max_transactions_filters defines maximum allowed amount of tx filters in msgRegisterInterchainQuery
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_transactions_filters: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.RegisteredQuery")]
pub struct RegisteredQuery {
    /// The unique id of the registered query.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    /// The address that registered the query.
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// The query type identifier: `kv` or `tx` now
    #[prost(string, tag = "3")]
    pub query_type: ::prost::alloc::string::String,
    /// The KV-storage keys for which we want to get values from remote chain
    #[prost(message, repeated, tag = "4")]
    pub keys: ::prost::alloc::vec::Vec<KvKey>,
    /// The filter for transaction search ICQ
    #[prost(string, tag = "5")]
    pub transactions_filter: ::prost::alloc::string::String,
    /// The IBC connection ID for getting ConsensusState to verify proofs
    #[prost(string, tag = "6")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    /// Parameter that defines how often the query must be updated.
    #[prost(uint64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub update_period: u64,
    /// The local chain last block height when the query result was updated.
    #[prost(uint64, tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_submitted_result_local_height: u64,
    /// The remote chain last block height when the query result was updated.
    #[prost(message, optional, tag = "9")]
    pub last_submitted_result_remote_height:
        ::core::option::Option<super::super::ibc::core::client::v1::Height>,
    /// Amount of coins deposited for the query.
    #[prost(message, repeated, tag = "10")]
    pub deposit: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    /// Timeout before query becomes available for everybody to remove.
    #[prost(uint64, tag = "11")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub submit_timeout: u64,
    /// The local chain height when the query was registered.
    #[prost(uint64, tag = "12")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub registered_at_height: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.KVKey")]
pub struct KvKey {
    /// Path (storage prefix) to the storage where you want to read value by key
    /// (usually name of cosmos-sdk module: 'staking', 'bank', etc.)
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// Key you want to read from the storage
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// GenesisState defines the interchainqueries module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.GenesisState")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub registered_queries: ::prost::alloc::vec::Vec<RegisteredQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.MsgRegisterInterchainQuery")]
pub struct MsgRegisterInterchainQuery {
    /// defines a query type: `kv` or `tx` now
    #[prost(string, tag = "1")]
    pub query_type: ::prost::alloc::string::String,
    /// is used to define KV-storage keys for which we want to get values from
    /// remote chain
    #[prost(message, repeated, tag = "2")]
    pub keys: ::prost::alloc::vec::Vec<KvKey>,
    /// is used to define a filter for transaction search ICQ
    #[prost(string, tag = "3")]
    pub transactions_filter: ::prost::alloc::string::String,
    /// is IBC connection ID for getting ConsensusState to verify proofs
    #[prost(string, tag = "4")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    /// is used to specify how often (in neutron blocks) the query must be updated
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub update_period: u64,
    /// is the signer of the message
    #[prost(string, tag = "6")]
    pub sender: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.MsgRegisterInterchainQueryResponse")]
pub struct MsgRegisterInterchainQueryResponse {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.MsgSubmitQueryResult")]
pub struct MsgSubmitQueryResult {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "queryID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub query_id: u64,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    /// is the IBC client ID for an IBC connection between Neutron chain and target
    /// chain (where the result was obtained from)
    /// Deprecated: populating this field does not make any affect
    #[deprecated]
    #[prost(string, tag = "3")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub result: ::core::option::Option<QueryResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.QueryResult")]
pub struct QueryResult {
    #[prost(message, repeated, tag = "1")]
    pub kv_results: ::prost::alloc::vec::Vec<StorageValue>,
    #[prost(message, optional, tag = "2")]
    pub block: ::core::option::Option<Block>,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub revision: u64,
    #[prost(bool, tag = "5")]
    pub allow_kv_callbacks: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.StorageValue")]
pub struct StorageValue {
    /// is the substore name (acc, staking, etc.)
    #[prost(string, tag = "1")]
    pub storage_prefix: ::prost::alloc::string::String,
    /// is the key in IAVL store
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// is the value in IAVL store
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// is the Merkle Proof which proves existence of key-value pair in IAVL
    /// storage
    #[prost(message, optional, tag = "4")]
    pub proof: ::core::option::Option<super::super::tendermint::crypto::ProofOps>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.Block")]
pub struct Block {
    /// We need to know block X+1 to verify response of transaction for block X
    /// since LastResultsHash is root hash of all results from the txs from the
    /// previous block
    #[prost(message, optional, tag = "1")]
    pub next_block_header: ::core::option::Option<crate::shim::Any>,
    /// We need to know block X to verify inclusion of transaction for block X
    #[prost(message, optional, tag = "2")]
    pub header: ::core::option::Option<crate::shim::Any>,
    #[prost(message, optional, tag = "3")]
    pub tx: ::core::option::Option<TxValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.TxValue")]
pub struct TxValue {
    #[prost(message, optional, tag = "1")]
    pub response: ::core::option::Option<super::super::tendermint::abci::ExecTxResult>,
    /// is the Merkle Proof which proves existence of response in block with height
    /// next_block_header.Height
    #[prost(message, optional, tag = "2")]
    pub delivery_proof: ::core::option::Option<super::super::tendermint::crypto::Proof>,
    /// is the Merkle Proof which proves existence of data in block with height
    /// header.Height
    #[prost(message, optional, tag = "3")]
    pub inclusion_proof: ::core::option::Option<super::super::tendermint::crypto::Proof>,
    /// is body of the transaction
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.MsgSubmitQueryResultResponse")]
pub struct MsgSubmitQueryResultResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.MsgRemoveInterchainQueryRequest")]
pub struct MsgRemoveInterchainQueryRequest {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "queryID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub query_id: u64,
    /// is the signer of the message
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.MsgRemoveInterchainQueryResponse")]
pub struct MsgRemoveInterchainQueryResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.MsgUpdateInterchainQueryRequest")]
pub struct MsgUpdateInterchainQueryRequest {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "queryID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub query_id: u64,
    #[prost(message, repeated, tag = "2")]
    pub new_keys: ::prost::alloc::vec::Vec<KvKey>,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub new_update_period: u64,
    #[prost(string, tag = "4")]
    pub new_transactions_filter: ::prost::alloc::string::String,
    /// is the signer of the message
    #[prost(string, tag = "5")]
    pub sender: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.MsgUpdateInterchainQueryResponse")]
pub struct MsgUpdateInterchainQueryResponse {}
/// MsgUpdateParams is the MsgUpdateParams request type.
///
/// Since: 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// Authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/interchainqueries parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
///
/// Since: 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.QueryParamsRequest")]
#[proto_query(
    path = "/neutron.interchainqueries.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.QueryRegisteredQueriesRequest")]
#[proto_query(
    path = "/neutron.interchainqueries.Query/RegisteredQueries",
    response_type = QueryRegisteredQueriesResponse
)]
pub struct QueryRegisteredQueriesRequest {
    #[prost(string, repeated, tag = "1")]
    pub owners: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "2")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.QueryRegisteredQueriesResponse")]
pub struct QueryRegisteredQueriesResponse {
    #[prost(message, repeated, tag = "1")]
    pub registered_queries: ::prost::alloc::vec::Vec<RegisteredQuery>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.QueryRegisteredQueryRequest")]
#[proto_query(
    path = "/neutron.interchainqueries.Query/RegisteredQuery",
    response_type = QueryRegisteredQueryResponse
)]
pub struct QueryRegisteredQueryRequest {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "queryID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub query_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.QueryRegisteredQueryResponse")]
pub struct QueryRegisteredQueryResponse {
    #[prost(message, optional, tag = "1")]
    pub registered_query: ::core::option::Option<RegisteredQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.QueryRegisteredQueryResultRequest")]
#[proto_query(
    path = "/neutron.interchainqueries.Query/QueryResult",
    response_type = QueryRegisteredQueryResultResponse
)]
pub struct QueryRegisteredQueryResultRequest {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "queryID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub query_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.QueryRegisteredQueryResultResponse")]
pub struct QueryRegisteredQueryResultResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<QueryResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.Transaction")]
pub struct Transaction {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.QueryLastRemoteHeight")]
#[proto_query(
    path = "/neutron.interchainqueries.Query/LastRemoteHeight",
    response_type = QueryLastRemoteHeightResponse
)]
pub struct QueryLastRemoteHeight {
    #[prost(string, tag = "1")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/neutron.interchainqueries.QueryLastRemoteHeightResponse")]
pub struct QueryLastRemoteHeightResponse {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
}
pub struct InterchainqueriesQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> InterchainqueriesQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn registered_queries(
        &self,
        owners: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        connection_id: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryRegisteredQueriesResponse, cosmwasm_std::StdError> {
        QueryRegisteredQueriesRequest {
            owners,
            connection_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn registered_query(
        &self,
        query_id: u64,
    ) -> Result<QueryRegisteredQueryResponse, cosmwasm_std::StdError> {
        QueryRegisteredQueryRequest { query_id }.query(self.querier)
    }
    pub fn query_result(
        &self,
        query_id: u64,
    ) -> Result<QueryRegisteredQueryResultResponse, cosmwasm_std::StdError> {
        QueryRegisteredQueryResultRequest { query_id }.query(self.querier)
    }
    pub fn last_remote_height(
        &self,
        connection_id: ::prost::alloc::string::String,
    ) -> Result<QueryLastRemoteHeightResponse, cosmwasm_std::StdError> {
        QueryLastRemoteHeight { connection_id }.query(self.querier)
    }
}
