use crate::{
    bindings::types::{KVKey, ProtobufAny},
    interchain_queries::types::{QueryPayload, QueryType, TransactionFilterItem, MAX_TX_FILTERS},
    sudo::msg::RequestPacketTimeoutHeight,
    NeutronError, NeutronResult,
};

use crate::bindings::dex::msg::DexMsg;
use cosmwasm_std::{Binary, Coin, CosmosMsg, CustomMsg, DenomUnit, StdError, Uint128};
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

        /// **register_fee** is a fees required to be payed to register interchain account
        register_fee: Option<Vec<Coin>>,
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

    /// TokenFactory message.
    /// Contracts can set before send hooks for denoms, namespaced under the contract's address.
    SetBeforeSendHook {
        denom: String,
        contract_addr: String,
    },

    /// TokenFactoryMessage
    /// Contracts can force specified `amount` of an existing factory denom
    /// that they are admin of to a `transfer_to_address` from a `transfer_from_address`.
    ForceTransfer {
        denom: String,
        amount: Uint128,
        transfer_from_address: String,
        transfer_to_address: String,
    },

    /// TokenFactoryMessage
    /// Contracts can set a metadata for of an existing factory denom
    /// that they are admin of.
    SetDenomMetadata {
        /// **description** description of a token
        description: String,
        /// **denom_units** represents the list of DenomUnit's for a given coin
        denom_units: Vec<DenomUnit>,
        /// **base** represents the base denom (should be the DenomUnit with exponent = 0).
        base: String,
        /// **display** indicates the suggested denom that should be
        /// displayed in clients.
        display: String,
        /// **name** defines the name of the token (eg: Cosmos Atom)
        name: String,
        /// **symbol** is the token symbol usually shown on exchanges (eg: ATOM). This can
        /// be the same as the display.
        symbol: String,
        /// **uri** to a document (on or off-chain) that contains additional information. Optional.
        uri: String,
        /// **uri_hash** is a sha256 hash of a document pointed by URI. It's used to verify that
        /// the document didn't change. Optional.
        uri_hash: String,
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

    /// Contractmanager message
    /// Resubmits failed acknowledgement.
    /// Acknowledgement failure is created when contract returns error or acknowledgement is out of gas.
    /// [Permissioned - only from contract that is initial caller of IBC transaction]
    ResubmitFailure { failure_id: u64 },

    /// Dex messages
    Dex(DexMsg),
}

impl NeutronMsg {
    /// Basic helper to define a register interchain account message:
    /// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
    /// * **interchain_account_id** is an identifier of your new interchain account. Can be any string.
    pub fn register_interchain_account(
        connection_id: String,
        interchain_account_id: String,
        register_fee: Option<Vec<Coin>>,
    ) -> Self {
        NeutronMsg::RegisterInterchainAccount {
            connection_id,
            interchain_account_id,
            register_fee,
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
    /// * **update_period** is used to say how often (in neutron blocks) the query must be updated.
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
    /// * **new_update_period** is used to say how often (in neutron blocks) the query must be updated.
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

    #[deprecated(
        since = "0.11.0",
        note = "Used only for querying old proposals. Will fail if executed in a new proposal. Use submit_proposal_execute_message instead"
    )]
    /// Basic helper to define an  ibc upgrade proposal passed to AdminModule:
    /// * **proposal** is struct which contains proposal that upgrades network.
    pub fn submit_upgrade_proposal(proposal: UpgradeProposal) -> Self {
        NeutronMsg::SubmitAdminProposal {
            admin_proposal: AdminProposal::UpgradeProposal(proposal),
        }
    }

    #[deprecated(
        since = "0.11.0",
        note = "Used only for querying old proposals. Will fail if executed in a new proposal. Use submit_proposal_execute_message instead"
    )]
    /// Basic helper to define an ibc update client change proposal passed to AdminModule:
    /// * **proposal** is struct which contains proposal updates cliient.
    pub fn submit_client_update_proposal(proposal: ClientUpdateProposal) -> Self {
        NeutronMsg::SubmitAdminProposal {
            admin_proposal: AdminProposal::ClientUpdateProposal(proposal),
        }
    }

    /// Basic helper to define sdk47 compatible proposal passed to AdminModule:
    /// * **proposal** is struct which contains JSON encoded sdk message.
    pub fn submit_proposal_execute_message(proposal: ProposalExecuteMessage) -> Self {
        NeutronMsg::SubmitAdminProposal {
            admin_proposal: AdminProposal::ProposalExecuteMessage(proposal),
        }
    }

    /// Basic helper to build create denom message passed to TokenFactory module:
    /// * **subdenom** is a subdenom name for denom to be created.
    pub fn submit_create_denom(subdenom: impl Into<String>) -> Self {
        NeutronMsg::CreateDenom {
            subdenom: subdenom.into(),
        }
    }

    /// Basic helper to define change of admin for a token passed to TokenFactory module:
    /// * **denom** is a name of the denom to change an admin for;
    /// * **new_admin_address** is a new admin address for a denom.
    pub fn submit_change_admin(
        denom: impl Into<String>,
        new_admin_address: impl Into<String>,
    ) -> Self {
        NeutronMsg::ChangeAdmin {
            denom: denom.into(),
            new_admin_address: new_admin_address.into(),
        }
    }

    /// Basic helper to define mint tokens passed to TokenFactory module:
    /// * **denom** is a name of the denom;
    /// * **amount** is an amount of tokens to mint;
    /// * **mint_to_address** is an address that will receive minted tokens.
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

    /// Basic helper to define burn tokens passed to TokenFactory module:
    /// * **denom** is a name of the denom;
    /// * **amount** is an amount of tokens to burn.
    pub fn submit_burn_tokens(denom: impl Into<String>, amount: Uint128) -> Self {
        NeutronMsg::BurnTokens {
            denom: denom.into(),
            amount,
            burn_from_address: String::new(),
        }
    }

    /// Basic helper to create set before send hook message passed to TokenFactory module:
    /// * **denom** is a name for denom for hook to be created.
    pub fn submit_set_before_send_hook(
        denom: impl Into<String>,
        contract_addr: impl Into<String>,
    ) -> Self {
        NeutronMsg::SetBeforeSendHook {
            denom: denom.into(),
            contract_addr: contract_addr.into(),
        }
    }

    /// Basic helper to create force transfer message passed to TokenFactory module:
    /// * **denom** is a name for a denom to transfer;
    /// * **amount** is an amount of **denom** tokens to transfer;
    /// * **from_address** is from which address to transfer tokens;
    /// * **to_address** is where to transfer tokens.
    pub fn submit_force_transfer(
        denom: impl Into<String>,
        amount: Uint128,
        from_address: impl Into<String>,
        to_address: impl Into<String>,
    ) -> Self {
        NeutronMsg::ForceTransfer {
            denom: denom.into(),
            amount,
            transfer_from_address: from_address.into(),
            transfer_to_address: to_address.into(),
        }
    }
    /// Basic helper to create a set denom metadata message passed to TokenFactory module:
    /// * **description** description of a token;
    /// * **denom_units** represents the list of DenomUnit's for a given coin;
    /// * **base** represents the base denom (should be the DenomUnit with exponent = 0);
    /// * **display** indicates the suggested denom that should be
    /// displayed in clients;
    /// * **name** defines the name of the token (eg: Cosmos Atom);
    /// * **symbol** is the token symbol usually shown on exchanges (eg: ATOM). This can
    /// be the same as the display;
    /// * **uri** to a document (on or off-chain) that contains additional information. Optional;
    /// * **uri_hash** is a sha256 hash of a document pointed by URI. It's used to verify that
    /// the document didn't change. Optional.
    #[allow(clippy::too_many_arguments)]
    pub fn submit_set_denom_metadata(
        description: String,
        denom_units: Vec<DenomUnit>,
        base: String,
        display: String,
        name: String,
        symbol: String,
        uri: String,
        uri_hash: String,
    ) -> Self {
        NeutronMsg::SetDenomMetadata {
            description,
            denom_units,
            base,
            display,
            name,
            symbol,
            uri,
            uri_hash,
        }
    }

    /// Basic helper to define add schedule passed to Cron module:
    /// * **name** is a name of the schedule;
    /// * **period** is a period of schedule execution in blocks;
    /// * **msgs** is the messages that will be executed.
    pub fn submit_add_schedule(name: String, period: u64, msgs: Vec<MsgExecuteContract>) -> Self {
        NeutronMsg::AddSchedule { name, period, msgs }
    }

    /// Basic helper to define remove schedule passed to Cron module:
    /// * **name** is a name of the schedule to be removed.
    pub fn submit_remove_schedule(name: String) -> Self {
        NeutronMsg::RemoveSchedule { name }
    }

    /// Basic helper to define resubmit failure passed to Contractmanager module:
    /// * **failure_id** is an id of the failure to be resubmitted.
    pub fn submit_resubmit_failure(failure_id: u64) -> Self {
        NeutronMsg::ResubmitFailure { failure_id }
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
/// MsgRegisterInterchainAccountResponse defines the Msg/RegisterInterchainAccount response type.
pub struct MsgRegisterInterchainAccountResponse {
    /// **channel_id** is a ...
    pub channel_id: String,
    /// **port_id** is a ...
    pub port_id: String,
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
/// MsgIbcTransferResponse defines the response for Msg/IbcTransfer.
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
    pub upgraded_client_state: ProtobufAny,
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
