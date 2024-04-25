// @generated
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// Defines amount of blocks required before query becomes available for
    /// removal by anybody
    #[prost(uint64, tag = "1")]
    pub query_submit_timeout: u64,
    /// Amount of coins deposited for the query.
    #[prost(message, repeated, tag = "2")]
    pub query_deposit: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// Amount of tx hashes to be removed during a single EndBlock. Can vary to
    /// balance between network cleaning speed and EndBlock duration. A zero value
    /// means no limit.
    #[prost(uint64, tag = "3")]
    pub tx_query_removal_limit: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisteredQuery {
    /// The unique id of the registered query.
    #[prost(uint64, tag = "1")]
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
    pub connection_id: ::prost::alloc::string::String,
    /// Parameter that defines how often the query must be updated.
    #[prost(uint64, tag = "7")]
    pub update_period: u64,
    /// The local chain last block height when the query result was updated.
    #[prost(uint64, tag = "8")]
    pub last_submitted_result_local_height: u64,
    /// The remote chain last block height when the query result was updated.
    #[prost(message, optional, tag = "9")]
    pub last_submitted_result_remote_height:
        ::core::option::Option<cosmos_sdk_proto::ibc::core::client::v1::Height>,
    /// Amount of coins deposited for the query.
    #[prost(message, repeated, tag = "10")]
    pub deposit: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// Timeout before query becomes available for everybody to remove.
    #[prost(uint64, tag = "11")]
    pub submit_timeout: u64,
    /// The local chain height when the query was registered.
    #[prost(uint64, tag = "12")]
    pub registered_at_height: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KvKey {
    /// Path (storage prefix) to the storage where you want to read value by key
    /// (usually name of cosmos-sdk module: 'staking', 'bank', etc.)
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// Key you want to read from the storage
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// GenesisState defines the interchainqueries module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub registered_queries: ::prost::alloc::vec::Vec<RegisteredQuery>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
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
    pub connection_id: ::prost::alloc::string::String,
    /// is used to specify how often (in neutron blocks) the query must be updated
    #[prost(uint64, tag = "5")]
    pub update_period: u64,
    /// is the signer of the message
    #[prost(string, tag = "6")]
    pub sender: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterInterchainQueryResponse {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitQueryResult {
    #[prost(uint64, tag = "1")]
    pub query_id: u64,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    /// is the IBC client ID for an IBC connection between Neutron chain and target
    /// chain (where the result was obtained from)
    #[prost(string, tag = "3")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub result: ::core::option::Option<QueryResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResult {
    #[prost(message, repeated, tag = "1")]
    pub kv_results: ::prost::alloc::vec::Vec<StorageValue>,
    #[prost(message, optional, tag = "2")]
    pub block: ::core::option::Option<Block>,
    #[prost(uint64, tag = "3")]
    pub height: u64,
    #[prost(uint64, tag = "4")]
    pub revision: u64,
    #[prost(bool, tag = "5")]
    pub allow_kv_callbacks: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageValue {
    /// is the substore name (acc, staking, etc.)
    #[prost(string, tag = "1")]
    pub storage_prefix: ::prost::alloc::string::String,
    /// is the key in IAVL store
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// is the value in IAVL store
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// is the Merkle Proof which proves existence of key-value pair in IAVL
    /// storage
    #[prost(message, optional, tag = "4")]
    pub proof: ::core::option::Option<tendermint_proto::v0_38::crypto::ProofOps>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    /// We need to know block X+1 to verify response of transaction for block X
    /// since LastResultsHash is root hash of all results from the txs from the
    /// previous block
    #[prost(message, optional, tag = "1")]
    pub next_block_header: ::core::option::Option<::prost_types::Any>,
    /// We need to know block X to verify inclusion of transaction for block X
    #[prost(message, optional, tag = "2")]
    pub header: ::core::option::Option<::prost_types::Any>,
    #[prost(message, optional, tag = "3")]
    pub tx: ::core::option::Option<TxValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxValue {
    #[prost(message, optional, tag = "1")]
    pub response: ::core::option::Option<tendermint_proto::v0_38::abci::ExecTxResult>,
    /// is the Merkle Proof which proves existence of response in block with height
    /// next_block_header.Height
    #[prost(message, optional, tag = "2")]
    pub delivery_proof: ::core::option::Option<tendermint_proto::v0_38::crypto::Proof>,
    /// is the Merkle Proof which proves existence of data in block with height
    /// header.Height
    #[prost(message, optional, tag = "3")]
    pub inclusion_proof: ::core::option::Option<tendermint_proto::v0_38::crypto::Proof>,
    /// is body of the transaction
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitQueryResultResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveInterchainQueryRequest {
    #[prost(uint64, tag = "1")]
    pub query_id: u64,
    /// is the signer of the message
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveInterchainQueryResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateInterchainQueryRequest {
    #[prost(uint64, tag = "1")]
    pub query_id: u64,
    #[prost(message, repeated, tag = "2")]
    pub new_keys: ::prost::alloc::vec::Vec<KvKey>,
    #[prost(uint64, tag = "3")]
    pub new_update_period: u64,
    #[prost(string, tag = "4")]
    pub new_transactions_filter: ::prost::alloc::string::String,
    /// is the signer of the message
    #[prost(string, tag = "5")]
    pub sender: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateInterchainQueryResponse {}
/// MsgUpdateParams is the MsgUpdateParams request type.
///
/// Since: 0.47
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRegisteredQueriesRequest {
    #[prost(string, repeated, tag = "1")]
    pub owners: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRegisteredQueriesResponse {
    #[prost(message, repeated, tag = "1")]
    pub registered_queries: ::prost::alloc::vec::Vec<RegisteredQuery>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRegisteredQueryRequest {
    #[prost(uint64, tag = "1")]
    pub query_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRegisteredQueryResponse {
    #[prost(message, optional, tag = "1")]
    pub registered_query: ::core::option::Option<RegisteredQuery>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRegisteredQueryResultRequest {
    #[prost(uint64, tag = "1")]
    pub query_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRegisteredQueryResultResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<QueryResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub height: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastRemoteHeight {
    #[prost(string, tag = "1")]
    pub connection_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastRemoteHeightResponse {
    #[prost(uint64, tag = "1")]
    pub height: u64,
}
// @@protoc_insertion_point(module)
