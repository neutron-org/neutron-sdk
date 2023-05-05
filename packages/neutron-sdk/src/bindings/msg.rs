use crate::{
    bindings::types::{KVKey, ProtobufAny},
    interchain_queries::types::{QueryPayload, QueryType, TransactionFilterItem, MAX_TX_FILTERS},
    sudo::msg::RequestPacketTimeoutHeight,
    NeutronError, NeutronResult,
};

use cosmwasm_std::{Binary, Coin, CosmosMsg, CustomMsg, StdError, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json_wasm::to_string;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
/// IbcFee defines struct for fees that refund the relayer for `SudoMsg` messages submission.
/// Unused fee kind will be returned back to message sender.
/// Please refer to these links for more information:
/// IBC transaction structure - <https://docs.neutron.org/neutron/interchain-txs/messages/#msgsubmittx>
/// General mechanics of fee payments - <https://docs.neutron.org/neutron/feerefunder/overview/#general-mechanics>
pub struct IbcFee {
    /// **recv_fee** currently is used for compatibility with ICS-29 interface only and must be set to zero (i.e. 0untrn),
    /// because Neutron's fee module can't refund relayer for submission of Recv IBC packets due to compatibility with target chains.
    pub recv_fee: Vec<Coin>,
    /// **ack_fee** is an amount of coins to refund relayer for submitting ack message for a particular IBC packet.
    pub ack_fee: Vec<Coin>,
    /// **timeout_fee** amount of coins to refund relayer for submitting timeout message for a particular IBC packet.
    pub timeout_fee: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// A number of Custom messages that can call into the Neutron bindings.
pub enum NeutronMsg {
    /// RegisterInterchainAccount registers an interchain account on remote chain.
    RegisterInterchainAccount {
        /// **connection_id** is an IBC connection identifier between Neutron and remote chain.
        connection_id: String,

        /// **interchain_account_id** is an identifier of your new interchain account. Can be any string.
        /// This identifier allows contracts to have multiple interchain accounts on remote chains.
        interchain_account_id: String,
    },

    /// SubmitTx starts the process of executing any Cosmos-SDK *msgs* on remote chain.
    SubmitTx {
        /// **connection_id** is an IBC connection identifier between Neutron and remote chain.
        connection_id: String,

        /// **interchain_account_id** is an identifier of your interchain account from which you want to execute msgs.
        interchain_account_id: String,

        /// **msgs** is a list of protobuf encoded Cosmos-SDK messages you want to execute on remote chain.
        msgs: Vec<ProtobufAny>,

        /// **memo** is a memo you want to attach to your interchain transaction.It behaves like a memo in usual Cosmos transaction.
        memo: String,

        /// **timeout** is a timeout in seconds after which the packet times out.
        timeout: u64,

        /// ***fee** is an ibc fee for the transaction.
        fee: IbcFee,
    },

    /// RegisterInterchainQuery registers an interchain query.
    RegisterInterchainQuery {
        /// **query_type** is a query type identifier ('tx' or 'kv' for now).
        query_type: String,

        /// **keys** is the KV-storage keys for which we want to get values from remote chain.
        keys: Vec<KVKey>,

        /// **transactions_filter** is the filter for transaction search ICQ.
        transactions_filter: String,

        /// **connection_id** is an IBC connection identifier between Neutron and remote chain.
        connection_id: String,

        /// **update_period** is used to say how often the query must be updated.
        update_period: u64,
    },

    /// RegisterInterchainQuery updates an interchain query.
    UpdateInterchainQuery {
        /// **query_id** is the ID of the query we want to update.
        query_id: u64,

        /// **new_keys** is the new query keys to retrive.
        new_keys: Option<Vec<KVKey>>,

        /// **new_update_period** is a new update period of the query.
        new_update_period: Option<u64>,

        /// **new_transactions_filter** is a new transactions filter of the query.
        new_transactions_filter: Option<String>,
    },

    /// RemoveInterchainQuery removes as interchain query.
    RemoveInterchainQuery {
        /// **query_id** is ID of the query we want to remove.
        query_id: u64,
    },
    /// IbcTransfer sends a fungible token packet over IBC.
    IbcTransfer {
        // the port on which the packet will be sent
        source_port: String,
        // the channel by which the packet will be sent
        source_channel: String,
        // the tokens to be transferred
        token: Coin,
        // the sender address
        sender: String,
        // the recipient address on the destination chain
        receiver: String,
        // Timeout height relative to the current block height.
        // The timeout is disabled when set to 0.
        timeout_height: RequestPacketTimeoutHeight,
        // Timeout timestamp in absolute nanoseconds since unix epoch.
        // The timeout is disabled when set to 0.
        timeout_timestamp: u64,
        // Memo to be sent along with transaction.
        memo: String,
        // Fees to refund relayer for different kinds of `SudoMsg` transmission
        // Unused fee types will be returned to msg sender.
        fee: IbcFee,
    },
    /// SubmitAdminProposal sends a proposal to neutron's Admin module.
    /// This type of messages can be only executed by Neutron DAO.
    SubmitAdminProposal { admin_proposal: AdminProposal },

    /// TokenFactory message.
    /// Contracts can create denoms, namespaced under the contract's address.
    /// A contract may create any number of independent sub-denoms.
    CreateDenom { subdenom: String },
    /// TokenFactory message.
    /// Contracts can change the admin of a denom that they are the admin of.
    ChangeAdmin {
        denom: String,
        new_admin_address: String,
    },
    /// TokenFactory message.
    /// Contracts can mint native tokens for an existing factory denom
    /// that they are the admin of.
    MintTokens {
        denom: String,
        amount: Uint128,
        mint_to_address: String,
    },
    /// TokenFactory message.
    /// Contracts can burn native tokens for an existing factory denom
    /// that they are the admin of.
    /// Currently, the burn from address must be the admin contract.
    BurnTokens {
        denom: String,
        amount: Uint128,
        /// Must be set to `""` for now
        burn_from_address: String,
    },

    /// AddSchedule adds new schedule with a given `name`.
    /// Until schedule is removed it will execute all `msgs` every `period` blocks.
    /// First execution is at least on `current_block + period` block.
    /// [Permissioned - DAO Only]
    AddSchedule {
        /// Name of a new schedule.
        /// Needed to be able to `RemoveSchedule` and to log information about it
        name: String,
        /// period in blocks with which `msgs` will be executed
        period: u64,
        /// list of cosmwasm messages to be executed
        msgs: Vec<MsgExecuteContract>,
    },
    /// RemoveSchedule removes the schedule with a given `name`.
    /// [Permissioned - DAO or Security DAO only]
    RemoveSchedule { name: String },
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
    /// * **memo** is a memo you want to attach to your interchain transaction. It behaves like a memo in usual Cosmos transaction;
    /// * **timeout** is a timeout in seconds after which the packet times out.
    /// * **fee** is a fee that is used for different kinds of callbacks. Unused fee types will be returned to msg sender.
    pub fn submit_tx(
        connection_id: String,
        interchain_account_id: String,
        msgs: Vec<ProtobufAny>,
        memo: String,
        timeout: u64,
        fee: IbcFee,
    ) -> Self {
        NeutronMsg::SubmitTx {
            connection_id,
            interchain_account_id,
            msgs,
            memo,
            timeout,
            fee,
        }
    }

    /// Basic helper to define a register interchain query message:
    /// * **query** is a query type identifier ('tx' or 'kv' for now) with a payload:
    ///   - when the query enum is 'kv' then payload is the KV-storage keys for which we want to get
    ///     values from remote chain;
    ///   - when the query enum is 'tx' then payload is the filters for transaction search ICQ,
    ///     maximum allowed number of filters is 32.
    /// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
    /// * **update_period** is used to say how often the query must be updated.
    pub fn register_interchain_query(
        query: QueryPayload,
        connection_id: String,
        update_period: u64,
    ) -> NeutronResult<Self> {
        Ok(match query {
            QueryPayload::KV(keys) => NeutronMsg::RegisterInterchainQuery {
                query_type: QueryType::KV.into(),
                keys,
                transactions_filter: String::new(),
                connection_id,
                update_period,
            },
            QueryPayload::TX(transactions_filters) => {
                if transactions_filters.len() > MAX_TX_FILTERS {
                    return Err(NeutronError::TooManyTransactionFilters {
                        max: MAX_TX_FILTERS,
                    });
                } else {
                    NeutronMsg::RegisterInterchainQuery {
                        query_type: QueryType::TX.into(),
                        keys: vec![],
                        transactions_filter: to_string(&transactions_filters)
                            .map_err(|e| StdError::generic_err(e.to_string()))?,
                        connection_id,
                        update_period,
                    }
                }
            }
        })
    }

    /// Basic helper to define a update interchain query message:
    /// * **query_id** is ID of the query we want to update;
    /// * **new_keys** is encoded keys to query;
    /// * **new_update_period** is used to say how often the query must be updated.
    pub fn update_interchain_query(
        query_id: u64,
        new_keys: Option<Vec<KVKey>>,
        new_update_period: Option<u64>,
        new_transactions_filter: Option<Vec<TransactionFilterItem>>,
    ) -> NeutronResult<Self> {
        Ok(NeutronMsg::UpdateInterchainQuery {
            query_id,
            new_keys,
            new_update_period,
            new_transactions_filter: match new_transactions_filter {
                Some(filters) => {
                    if filters.len() > MAX_TX_FILTERS {
                        return Err(NeutronError::TooManyTransactionFilters {
                            max: MAX_TX_FILTERS,
                        });
                    } else {
                        Some(
                            to_string(&filters)
                                .map_err(|e| StdError::generic_err(e.to_string()))?,
                        )
                    }
                }
                None => None,
            },
        })
    }

    /// Basic helper to define a remove interchain query message:
    /// * **query_id** is ID of the query we want to remove.
    pub fn remove_interchain_query(query_id: u64) -> Self {
        NeutronMsg::RemoveInterchainQuery { query_id }
    }

    /// Basic helper to define a parameter change proposal passed to AdminModule:
    /// * **proposal** is struct which contains proposal that should change network parameter.
    pub fn submit_param_change_proposal(proposal: ParamChangeProposal) -> Self {
        NeutronMsg::SubmitAdminProposal {
            admin_proposal: AdminProposal::ParamChangeProposal(proposal),
        }
    }

    /// Basic helper to define a parameter change proposal passed to AdminModule:
    /// * **proposal** is struct which contains proposal that sets upgrade block.
    pub fn submit_software_upgrade_proposal(proposal: SoftwareUpgradeProposal) -> Self {
        NeutronMsg::SubmitAdminProposal {
            admin_proposal: AdminProposal::SoftwareUpgradeProposal(proposal),
        }
    }

    /// Basic helper to define a parameter change proposal passed to AdminModule:
    /// * **proposal** is struct which contains proposal that cancels software upgrade.
    pub fn submit_cancel_software_upgrade_proposal(
        proposal: CancelSoftwareUpgradeProposal,
    ) -> Self {
        NeutronMsg::SubmitAdminProposal {
            admin_proposal: AdminProposal::CancelSoftwareUpgradeProposal(proposal),
        }
    }

    /// Basic helper to define a parameter change proposal passed to AdminModule:
    /// * **proposal** is struct which contains proposal that upgrades network.
    pub fn submit_upgrade_proposal(proposal: UpgradeProposal) -> Self {
        NeutronMsg::SubmitAdminProposal {
            admin_proposal: AdminProposal::UpgradeProposal(proposal),
        }
    }

    /// Basic helper to define a parameter change proposal passed to AdminModule:
    /// * **proposal** is struct which contains proposal that pins code ids.
    pub fn submit_pin_codes_proposal(proposal: PinCodesProposal) -> Self {
        NeutronMsg::SubmitAdminProposal {
            admin_proposal: AdminProposal::PinCodesProposal(proposal),
        }
    }

    /// Basic helper to define a parameter change proposal passed to AdminModule:
    /// * **proposal** is struct which contains proposal that unpins codes ids.
    pub fn submit_unpin_codes_proposal(proposal: UnpinCodesProposal) -> Self {
        NeutronMsg::SubmitAdminProposal {
            admin_proposal: AdminProposal::UnpinCodesProposal(proposal),
        }
    }

    /// Basic helper to define a parameter change proposal passed to AdminModule:
    /// * **proposal** is struct which contains proposal updates cliient.
    pub fn submit_client_update_proposal(proposal: ClientUpdateProposal) -> Self {
        NeutronMsg::SubmitAdminProposal {
            admin_proposal: AdminProposal::ClientUpdateProposal(proposal),
        }
    }

    /// Basic helper to define a parameter change proposal passed to AdminModule:
    /// * **proposal** is struct which contains proposal updates admin of contract.
    pub fn submit_update_admin_proposal(proposal: UpdateAdminProposal) -> Self {
        NeutronMsg::SubmitAdminProposal {
            admin_proposal: AdminProposal::UpdateAdminProposal(proposal),
        }
    }

    /// Basic helper to define a parameter change proposal passed to AdminModule:
    /// * **proposal** is struct which contains proposal that clears admin of contract.
    pub fn submit_clear_admin_proposal(proposal: ClearAdminProposal) -> Self {
        NeutronMsg::SubmitAdminProposal {
            admin_proposal: AdminProposal::ClearAdminProposal(proposal),
        }
    }

    pub fn submit_create_denom(subdenom: impl Into<String>) -> Self {
        NeutronMsg::CreateDenom {
            subdenom: subdenom.into(),
        }
    }

    pub fn submit_change_admin(
        denom: impl Into<String>,
        new_admin_address: impl Into<String>,
    ) -> Self {
        NeutronMsg::ChangeAdmin {
            denom: denom.into(),
            new_admin_address: new_admin_address.into(),
        }
    }

    pub fn submit_mint_tokens(
        denom: impl Into<String>,
        amount: Uint128,
        mint_to_address: impl Into<String>,
    ) -> Self {
        NeutronMsg::MintTokens {
            denom: denom.into(),
            amount,
            mint_to_address: mint_to_address.into(),
        }
    }

    pub fn submit_burn_tokens(denom: impl Into<String>, amount: Uint128) -> Self {
        NeutronMsg::BurnTokens {
            denom: denom.into(),
            amount,
            burn_from_address: String::new(),
        }
    }

    pub fn submit_add_schedule(name: String, period: u64, msgs: Vec<MsgExecuteContract>) -> Self {
        NeutronMsg::AddSchedule { name, period, msgs }
    }

    pub fn submit_remove_schedule(name: String) -> Self {
        NeutronMsg::RemoveSchedule { name }
    }
}

impl From<NeutronMsg> for CosmosMsg<NeutronMsg> {
    fn from(msg: NeutronMsg) -> CosmosMsg<NeutronMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for NeutronMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// Describes response structure for **RegisterInterchainQuery** msg.
pub struct MsgRegisterInterchainQueryResponse {
    /// **id** is an identifier of newly registered interchain query.
    pub id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// MsgSubmitTxResponse defines the response for Msg/SubmitTx.
pub struct MsgSubmitTxResponse {
    /// **sequence_id** is a channel's sequence_id for outgoing ibc packet. Unique per a channel.
    pub sequence_id: u64,
    /// **channel** is a src channel on neutron side trasaction was submitted from.
    pub channel: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// MsgSubmitTxResponse defines the response for Msg/IbcTransfer.
pub struct MsgIbcTransferResponse {
    /// **sequence_id** is a channel's sequence_id for outgoing ibc packet. Unique per a channel.
    pub sequence_id: u64,
    /// **channel** is a src channel on neutron side trasaction was submitted from.
    pub channel: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// AdminProposal defines the struct for various proposals which Neutron's Admin Module may accept.
pub enum AdminProposal {
    ParamChangeProposal(ParamChangeProposal),
    SoftwareUpgradeProposal(SoftwareUpgradeProposal),
    CancelSoftwareUpgradeProposal(CancelSoftwareUpgradeProposal),
    UpgradeProposal(UpgradeProposal),
    ClientUpdateProposal(ClientUpdateProposal),
    PinCodesProposal(PinCodesProposal),
    UnpinCodesProposal(UnpinCodesProposal),
    SudoContractProposal(SudoContractProposal),
    UpdateAdminProposal(UpdateAdminProposal),
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
/// SoftwareUpgradeProposal defines the struct for software upgrade proposal.
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
/// CancelSoftwareUpgradeProposal defines the struct for cancel software upgrade proposal.
pub struct CancelSoftwareUpgradeProposal {
    /// **title** is a text title of proposal. Non unique.
    pub title: String,
    /// **description** is a text description of proposal. Non unique.
    pub description: String,
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
/// UpgradeProposal defines the struct for  upgrade proposal.
pub struct UpgradeProposal {
    /// **title** is a text title of proposal.
    pub title: String,
    /// **description** is a text description of proposal.
    pub description: String,
    /// **plan** is a plan of upgrade.
    pub plan: Plan,
    /// **upgraded_client_state** is an upgraded client state.
    pub upgraded_client_state: ProtobufAny,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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
/// PinCodesProposal defines the struct for pin contract codes proposal.
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
/// UnpinCodesProposal defines the struct for unpin contract codes proposal.
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
/// SudoContractProposal defines the struct for sudo execution proposal.
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
/// UpdateAdminProposal defines the struct for  update admin proposal.
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
/// SudoContractProposal defines the struct for clear admin proposal.
pub struct ClearAdminProposal {
    /// **title** is a text title of proposal.
    pub title: String,
    /// **description** is a text description of proposal.
    pub description: String,
    /// **contract** is an address of contract admin will be removed.
    pub contract: String,
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
