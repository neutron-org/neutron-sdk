use neutron_std_derive::CosmwasmExt;
/// Alert defines the basic meta-data necessary for the alerts module to resolve
/// a claim that the price of a CurrencyPair on-chain is deviating from the price
/// off-chain.
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
#[proto_message(type_url = "/slinky.alerts.v1.Alert")]
pub struct Alert {
    /// height represents the height for which the alert is filed.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
    /// signer is the signer of this alert, this is the address that will receive
    /// the reward in the case of a positive conclusion, or whose bond will get
    /// slashed in the event of a negative conclusion.
    #[prost(string, tag = "2")]
    pub signer: ::prost::alloc::string::String,
    /// currency_pair is the currency-pair that this claim asserts is deviating
    /// from the price off-chain.
    #[prost(message, optional, tag = "3")]
    pub currency_pair: ::core::option::Option<super::super::types::v1::CurrencyPair>,
}
/// AlertStatus contains the module specific state for an alert: Has the alert
/// been concluded? What height was the alert submitted, what height should the
/// alert be purged?
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
#[proto_message(type_url = "/slinky.alerts.v1.AlertStatus")]
pub struct AlertStatus {
    /// ConclusionStatus determines whether the alert has been concluded.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub conclusion_status: u64,
    /// SubmissionHeight is the height that the alert was submitted in.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub submission_height: u64,
    /// SubmissionTimestamp is the block-timestamp of the block that the alert was
    /// submitted in (as a UTC value in Unix time).
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub submission_timestamp: u64,
    /// PurgeHeight is the height at which the alert should be purged.
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub purge_height: u64,
}
/// AlertWithStatus represents a wrapper around the Alert and AlertStatus
/// objects, this is so that the module specific information about Alerts can be
/// packaged together.
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
#[proto_message(type_url = "/slinky.alerts.v1.AlertWithStatus")]
pub struct AlertWithStatus {
    /// alert is the alert that this status corresponds to.
    #[prost(message, optional, tag = "1")]
    pub alert: ::core::option::Option<Alert>,
    /// status is the status of the alert.
    #[prost(message, optional, tag = "2")]
    pub status: ::core::option::Option<AlertStatus>,
}
/// Signature is a container for a signer address mapped to a signature.
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
#[proto_message(type_url = "/slinky.alerts.v1.Signature")]
pub struct Signature {
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// MultiSigConcluson defines a conclusion that is accompanied by a set of
/// signatures. The signature is defined over the alert UID, status, OracleData,
/// and PriceBound. The signatures are used to verify that the conclusion is
/// valid.
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
#[proto_message(type_url = "/slinky.alerts.v1.MultiSigConclusion")]
pub struct MultiSigConclusion {
    /// alert is the alert that this conclusion corresponds to.
    #[prost(message, optional, tag = "1")]
    pub alert: ::core::option::Option<Alert>,
    /// oracle_data is the oracle data that this conclusion references.
    #[prost(message, optional, tag = "2")]
    pub extended_commit_info:
        ::core::option::Option<super::super::super::tendermint::abci::ExtendedCommitInfo>,
    /// signatures is a map of signer -> signature. Where the signature is over
    /// Alert.UID, PriceBound, the marshalled ExtendedCommitInfo, and status.
    #[prost(message, repeated, tag = "3")]
    pub signatures: ::prost::alloc::vec::Vec<Signature>,
    /// price-bound is the price bound of the currency-pair off-chain for the
    /// designated time-range.
    #[prost(message, optional, tag = "4")]
    pub price_bound: ::core::option::Option<PriceBound>,
    /// status is the status of the conclusion.
    #[prost(bool, tag = "5")]
    pub status: bool,
    /// CurrencyPairID is the ID of the currency-pair that this conclusion
    /// corresponds to.
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub currency_pair_i_d: u64,
}
/// MultiSigConclusionVerificationParams defines the parameters necessary to
/// verify a MultiSigConclusion. It contains a map between signer and public key.
/// Notice, the public-key (value) are the base-64 encoded bytes of the public
/// key. And the signer (key) is the bech32 encoded address of the signer.
/// Notice, all public keys must be secp256 keys.
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
#[proto_message(type_url = "/slinky.alerts.v1.MultiSigConclusionVerificationParams")]
pub struct MultiSigConclusionVerificationParams {
    /// signers is a map of signer -> public key.
    #[prost(message, repeated, tag = "1")]
    pub signers: ::prost::alloc::vec::Vec<crate::shim::Any>,
}
/// PriceBound represents the bounds of the price of a currency-pair off chain
/// for a designated time-range
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
#[proto_message(type_url = "/slinky.alerts.v1.PriceBound")]
pub struct PriceBound {
    #[prost(string, tag = "1")]
    pub high: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub low: ::prost::alloc::string::String,
}
/// AlertParams is the set of parameters for the x/Alerts module's Alerting. It
/// defines whether or not Alerts can be submitted, and if so, the minimum
/// bond amount required to submit an Alert.
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
#[proto_message(type_url = "/slinky.alerts.v1.AlertParams")]
pub struct AlertParams {
    /// Enabled is a boolean defining whether or not Alerts can be submitted
    /// to the module
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    /// BondAmount is the minimum amount of bond required to submit an
    /// Alert
    #[prost(message, optional, tag = "2")]
    pub bond_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// MaxBlockAge defines the maximum age of an Alert before it is pruned, notice
    /// this is defined wrt. the height that the Alert references, i.e Alerts are
    /// only relevant until Alert.Height + MaxBlockAge is reached.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_block_age: u64,
}
/// PruningParams defines the criterion for pruning Alerts from the state.
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
#[proto_message(type_url = "/slinky.alerts.v1.PruningParams")]
pub struct PruningParams {
    /// Enabled defines whether Alerts are to be pruned
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    /// BlocksToPrune defines the number of blocks until an Alert will be pruned
    /// from state, notice this is defined wrt. the current block height, i.e
    /// Alerts will be stored in state until current_height + BlocksToPrune is
    /// reached.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub blocks_to_prune: u64,
}
/// Params is the set of parameters for the x/Alerts module.
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
#[proto_message(type_url = "/slinky.alerts.v1.Params")]
pub struct Params {
    /// AlertParams is the set of parameters for the x/Alerts module's Alerting.
    #[prost(message, optional, tag = "1")]
    pub alert_params: ::core::option::Option<AlertParams>,
    /// ConclusionVerificationParams is the set of parameters for the x/Alerts
    /// module's conclusion verification.
    #[prost(message, optional, tag = "2")]
    pub conclusion_verification_params: ::core::option::Option<crate::shim::Any>,
    /// PruningParams is the set of parameters for the x/Alerts module's pruning.
    #[prost(message, optional, tag = "3")]
    pub pruning_params: ::core::option::Option<PruningParams>,
}
/// GenesisState is the state that must be provided at genesis. It contains
/// params for the module, and the set initial Alerts.
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
#[proto_message(type_url = "/slinky.alerts.v1.GenesisState")]
pub struct GenesisState {
    /// Params is the set of x/Alerts parameters
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// Alerts is the set of Alerts that have been submitted to the module
    #[prost(message, repeated, tag = "2")]
    pub alerts: ::prost::alloc::vec::Vec<AlertWithStatus>,
}
/// AlertsRequest is the request type for the Query.Alerts RPC method, the status
/// field indicates whether the request should return only Unconcluded /
/// Concluded Alerts, or all Alerts.
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
#[proto_message(type_url = "/slinky.alerts.v1.AlertsRequest")]
#[proto_query(path = "/slinky.alerts.v1.Query/Alerts", response_type = AlertsResponse)]
pub struct AlertsRequest {
    #[prost(enumeration = "AlertStatusId", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub status: i32,
}
/// AlertsResponse is the response type for the Query.Alerts RPC method, it
/// contains the list of Alerts that are being tracked by the alerts module.
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
#[proto_message(type_url = "/slinky.alerts.v1.AlertsResponse")]
pub struct AlertsResponse {
    #[prost(message, repeated, tag = "1")]
    pub alerts: ::prost::alloc::vec::Vec<Alert>,
}
/// ParamsRequest is the request type for the Query.Params RPC method.
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
#[proto_message(type_url = "/slinky.alerts.v1.ParamsRequest")]
#[proto_query(path = "/slinky.alerts.v1.Query/Params", response_type = ParamsResponse)]
pub struct ParamsRequest {}
/// ParamsResponse is the response type for the Query.Params RPC method, it
/// contains the Params of the module.
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
#[proto_message(type_url = "/slinky.alerts.v1.ParamsResponse")]
pub struct ParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// AlertStatus is the type for the status of an Alert, it can be Unconcluded or
/// Concluded.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum AlertStatusId {
    ConclusionStatusUnspecified = 0,
    ConclusionStatusUnconcluded = 1,
    ConclusionStatusConcluded = 2,
}
impl AlertStatusId {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AlertStatusId::ConclusionStatusUnspecified => "CONCLUSION_STATUS_UNSPECIFIED",
            AlertStatusId::ConclusionStatusUnconcluded => "CONCLUSION_STATUS_UNCONCLUDED",
            AlertStatusId::ConclusionStatusConcluded => "CONCLUSION_STATUS_CONCLUDED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONCLUSION_STATUS_UNSPECIFIED" => Some(Self::ConclusionStatusUnspecified),
            "CONCLUSION_STATUS_UNCONCLUDED" => Some(Self::ConclusionStatusUnconcluded),
            "CONCLUSION_STATUS_CONCLUDED" => Some(Self::ConclusionStatusConcluded),
            _ => None,
        }
    }
}
/// ValidatorAlertIncentive defines the incentive strategy to be executed for a
/// validator that has been confirmed to have at fault for an x/alerts alert.
/// This strategy is expected to slash half of the validator's stake.
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
#[proto_message(type_url = "/slinky.alerts.v1.ValidatorAlertIncentive")]
pub struct ValidatorAlertIncentive {
    /// The validator that has been confirmed to have been at fault for an alert.
    #[prost(message, optional, tag = "1")]
    pub validator: ::core::option::Option<super::super::super::tendermint::abci::Validator>,
    /// AlertSigner is the signer of the alert referenced by the conclusion that
    /// created this incentive.
    #[prost(string, tag = "2")]
    pub alert_signer: ::prost::alloc::string::String,
    /// AlertHeight is the height at which the infraction occurred
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub alert_height: u64,
}
/// MsgAlert defines a message to create an alert.
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
#[proto_message(type_url = "/slinky.alerts.v1.MsgAlert")]
pub struct MsgAlert {
    /// alert is the alert to be filed
    #[prost(message, optional, tag = "1")]
    pub alert: ::core::option::Option<Alert>,
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
#[proto_message(type_url = "/slinky.alerts.v1.MsgAlertResponse")]
pub struct MsgAlertResponse {}
/// MsgConclusion defines a message carrying a Conclusion made by the SecondTier,
/// which will be used to close an alert. And trigger any ramifications of the
/// conclusion.
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
#[proto_message(type_url = "/slinky.alerts.v1.MsgConclusion")]
pub struct MsgConclusion {
    /// signer is the signer of this transaction (notice, this may not always be a
    /// node from the SecondTier)
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// conclusion is the conclusion to be filed
    #[prost(message, optional, tag = "2")]
    pub conclusion: ::core::option::Option<crate::shim::Any>,
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
#[proto_message(type_url = "/slinky.alerts.v1.MsgConclusionResponse")]
pub struct MsgConclusionResponse {}
/// MsgUpdateParams defines the message type expected by the UpdateParams rpc. It
/// contains an authority address, and the new Params for the x/alerts module.
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
#[proto_message(type_url = "/slinky.alerts.v1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// authority is the address of the authority that is submitting the update
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params is the new set of parameters for the x/alerts module
    #[prost(message, optional, tag = "2")]
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
#[proto_message(type_url = "/slinky.alerts.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct AlertsQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> AlertsQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn alerts(&self, status: i32) -> Result<AlertsResponse, cosmwasm_std::StdError> {
        AlertsRequest { status }.query(self.querier)
    }
    pub fn params(&self) -> Result<ParamsResponse, cosmwasm_std::StdError> {
        ParamsRequest {}.query(self.querier)
    }
}
