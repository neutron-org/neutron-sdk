use cosmwasm_std::{Binary, CosmosMsg, CustomMsg};
use neutron_std::shim::Any;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[deprecated(
    note = "Please use neutron-std grpc messages instead of wasmbindings",
    since = "0.12.0"
)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// A number of Custom messages that can call into the Neutron bindings.
pub enum NeutronMsg {
    /// SubmitAdminProposal sends a proposal to neutron's Admin module.
    /// This type of messages can be only executed by Neutron DAO.
    SubmitAdminProposal { admin_proposal: AdminProposal },
}

impl From<NeutronMsg> for CosmosMsg<NeutronMsg> {
    fn from(msg: NeutronMsg) -> CosmosMsg<NeutronMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for NeutronMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// AdminProposal defines the struct for various proposals which Neutron's Admin Module may accept.
pub enum AdminProposal {
    /// Proposal to change params. Note that this works for old params.
    /// New params has their own `MsgUpdateParams` msgs that can be supplied to `ProposalExecuteMessage`
    ParamChangeProposal(ParamChangeProposal),

    #[deprecated(
        since = "0.11.0",
        note = "Used only for querying old proposals. Will fail if executed in a new proposal. Use ProposalExecuteMessage instead"
    )]
    /// Deprecated. Proposal to upgrade IBC client
    UpgradeProposal(UpgradeProposal),

    #[deprecated(
        since = "0.11.0",
        note = "Used only for querying old proposals. Will fail if executed in a new proposal. Use ProposalExecuteMessage instead"
    )]
    /// Deprecated. Proposal to update IBC client
    ClientUpdateProposal(ClientUpdateProposal),

    /// Proposal to execute CosmosMsg.
    ProposalExecuteMessage(ProposalExecuteMessage),

    #[deprecated(
        since = "0.7.0",
        note = "Used only for querying old proposals. Will fail if executed in a new proposal. Use ProposalExecuteMessage instead"
    )]
    /// Deprecated. Proposal to upgrade network
    SoftwareUpgradeProposal(SoftwareUpgradeProposal),

    #[deprecated(
        since = "0.7.0",
        note = "Used only for querying old proposals. Will fail if executed in a new proposal. Use ProposalExecuteMessage instead"
    )]
    /// Deprecated. Proposal to cancel existing software upgrade
    CancelSoftwareUpgradeProposal(CancelSoftwareUpgradeProposal),

    /// Deprecated. Will fail to execute if you use it.
    #[deprecated(
        since = "0.7.0",
        note = "Used only for querying old proposals. Will fail if executed in a new proposal. Use ProposalExecuteMessage instead"
    )]
    /// Deprecated. Proposal to pin wasm contract codes
    PinCodesProposal(PinCodesProposal),

    #[deprecated(
        since = "0.7.0",
        note = "Used only for querying old proposals. Will fail if executed in a new proposal. Use ProposalExecuteMessage instead"
    )]
    /// Deprecated. Deprecated. Proposal to unpin wasm contract codes.
    UnpinCodesProposal(UnpinCodesProposal),

    #[deprecated(
        since = "0.7.0",
        note = "Used only for querying old proposals. Will fail if executed in a new proposal. Use ProposalExecuteMessage instead"
    )]
    /// Deprecated. Proposal to call sudo on contract.
    SudoContractProposal(SudoContractProposal),

    #[deprecated(
        since = "0.7.0",
        note = "Used only for querying old proposals. Will fail if executed in a new proposal. Use ProposalExecuteMessage instead"
    )]
    /// Deprecated. Proposal to update contract admin.
    UpdateAdminProposal(UpdateAdminProposal),

    #[deprecated(
        since = "0.7.0",
        note = "Used only for querying old proposals. Will fail if executed in a new proposal. Use ProposalExecuteMessage instead"
    )]
    /// Deprecated. Proposal to clear contract admin.
    ClearAdminProposal(ClearAdminProposal),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// ParamChangeProposal defines the struct for single parameter change proposal.
pub struct ParamChangeProposal {
    /// **title** is a text title of proposal. Non unique.
    pub title: String,
    /// **description** is a text description of proposal. Non unique.
    pub description: String,
    /// **param_changes** is a vector of params to be changed. Non unique.
    pub param_changes: Vec<ParamChange>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// ParamChange defines the struct for parameter change request.
pub struct ParamChange {
    /// **subspace** is a key of module to which the parameter to change belongs. Unique for each module.
    pub subspace: String,
    /// **key** is a name of parameter. Unique for subspace.
    pub key: String,
    /// **value** is a new value for given parameter. Non unique.
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// Plan defines the struct for planned upgrade.
pub struct Plan {
    /// **name** is a name for the upgrade
    pub name: String,
    /// **height** is a height at which the upgrade must be performed
    pub height: i64,
    /// **info** is any application specific upgrade info to be included on-chain
    pub info: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[deprecated(
    since = "0.11.0",
    note = "Used only for querying old proposals. Will fail if executed in a new proposal. Use ProposalExecuteMessage instead"
)]
/// UpgradeProposal defines the struct for IBC upgrade proposal.
pub struct UpgradeProposal {
    /// **title** is a text title of proposal.
    pub title: String,
    /// **description** is a text description of proposal.
    pub description: String,
    /// **plan** is a plan of upgrade.
    pub plan: Plan,
    /// **upgraded_client_state** is an upgraded client state.
    pub upgraded_client_state: Any,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[deprecated(
    since = "0.11.0",
    note = "Used only for querying old proposals. Will fail if executed in a new proposal. Use ProposalExecuteMessage instead"
)]
/// ClientUpdateProposal defines the struct for client update proposal.
pub struct ClientUpdateProposal {
    /// **title** is a text title of proposal.
    pub title: String,
    /// **description** is a text description of proposal. Non unique.
    pub description: String,
    /// **subject_client_id** is a subject client id.
    pub subject_client_id: String,
    /// **substitute_client_id** is a substitute client id.
    pub substitute_client_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// ProposalExecuteMessage defines the struct for sdk47 compatible admin proposal.
pub struct ProposalExecuteMessage {
    /// **message** is a json representing an sdk message passed to admin module to execute.
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// MsgExecuteContract defines a call to the contract execution
pub struct MsgExecuteContract {
    /// **contract** is a contract address that will be called
    pub contract: String,
    /// **msg** is a contract call message
    pub msg: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[deprecated(
    since = "0.7.0",
    note = "Used only for querying old proposals. Will fail if executed in a new proposal. Use ProposalExecuteMessage instead"
)]
/// Deprecated. SoftwareUpgradeProposal defines the struct for software upgrade proposal.
pub struct SoftwareUpgradeProposal {
    /// **title** is a text title of proposal. Non unique.
    pub title: String,
    /// **description** is a text description of proposal. Non unique.
    pub description: String,
    /// **plan** is a plan of upgrade.
    pub plan: Plan,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[deprecated(
    since = "0.7.0",
    note = "Used only for querying old proposals. Will fail if executed in a new proposal. Use ProposalExecuteMessage instead"
)]
/// Deprecated. CancelSoftwareUpgradeProposal defines the struct for cancel software upgrade proposal.
pub struct CancelSoftwareUpgradeProposal {
    /// **title** is a text title of proposal. Non unique.
    pub title: String,
    /// **description** is a text description of proposal. Non unique.
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[deprecated(
    since = "0.7.0",
    note = "Used only for querying old proposals. Will fail if executed in a new proposal. Use ProposalExecuteMessage instead"
)]
/// Deprecated. SudoContractProposal defines the struct for sudo execution proposal.
pub struct SudoContractProposal {
    /// **title** is a text title of proposal.
    pub title: String,
    /// **description** is a text description of proposal.
    pub description: String,
    /// **contract** is an address of contract to be executed.
    pub contract: String,
    /// ***msg*** is a sudo message.
    pub msg: Binary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[deprecated(
    since = "0.7.0",
    note = "Used only for querying old proposals. Will fail if executed in a new proposal. Use ProposalExecuteMessage instead"
)]
/// Deprecated. PinCodesProposal defines the struct for pin contract codes proposal.
pub struct PinCodesProposal {
    /// **title** is a text title of proposal.
    pub title: String,
    /// **description** is a text description of proposal.
    pub description: String,
    /// **code_ids** is an array of codes to be pined.
    pub code_ids: Vec<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[deprecated(
    since = "0.7.0",
    note = "Used only for querying old proposals. Will fail if executed in a new proposal. Use ProposalExecuteMessage instead"
)]
/// Deprecated. UnpinCodesProposal defines the struct for unpin contract codes proposal.
pub struct UnpinCodesProposal {
    /// **title** is a text title of proposal.
    pub title: String,
    /// **description** is a text description of proposal.
    pub description: String,
    /// **code_ids** is an array of codes to be unpined.
    pub code_ids: Vec<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[deprecated(
    since = "0.7.0",
    note = "Used only for querying old proposals. Will fail if executed in a new proposal. Use ProposalExecuteMessage instead"
)]
/// Deprecated. UpdateAdminProposal defines the struct for update admin proposal.
pub struct UpdateAdminProposal {
    /// **title** is a text title of proposal.
    pub title: String,
    /// **description** is a text description of proposal.
    pub description: String,
    /// ***new_admin*** is an address of new admin
    pub new_admin: String,
    /// **contract** is an address of contract to update admin.
    pub contract: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[deprecated(
    since = "0.7.0",
    note = "Used only for querying old proposals. Will fail if executed in a new proposal. Use ProposalExecuteMessage instead"
)]
/// Deprecated. SudoContractProposal defines the struct for clear admin proposal.
pub struct ClearAdminProposal {
    /// **title** is a text title of proposal.
    pub title: String,
    /// **description** is a text description of proposal.
    pub description: String,
    /// **contract** is an address of contract admin will be removed.
    pub contract: String,
}
