use cosmwasm_std::{
    entry_point, from_binary, from_slice, DepsMut, Env, IbcBasicResponse, IbcChannelCloseMsg,
    IbcChannelConnectMsg, IbcChannelOpenMsg, IbcOrder, IbcPacketAckMsg, IbcPacketReceiveMsg,
    IbcPacketTimeoutMsg, IbcReceiveResponse, StdError, StdResult,
};
use obi::OBIDecode;

use crate::ibc_msg::{
    AcknowledgementMsg, OracleRequest, OracleRequestPacketAcknowledgement, OracleResponsePacketData,
};
use crate::obi_calldata;
use crate::state::{CHANNEL, PRICES};

pub const IBC_APP_VERSION: &str = "band-feed_v1";

#[entry_point]
/// enforces ordering and versioning constraints
pub fn ibc_channel_open(_deps: DepsMut, _env: Env, msg: IbcChannelOpenMsg) -> StdResult<()> {
    let channel = msg.channel();

    if channel.order != IbcOrder::Unordered {
        return Err(StdError::generic_err("Only supports unordered channels"));
    }

    if channel.version.as_str() != IBC_APP_VERSION {
        return Err(StdError::generic_err(format!(
            "Must set version to `{}`",
            IBC_APP_VERSION
        )));
    }

    if let Some(counter_version) = msg.counterparty_version() {
        if counter_version != IBC_APP_VERSION {
            return Err(StdError::generic_err(format!(
                "Counterparty version must be `{}`",
                IBC_APP_VERSION
            )));
        }
    }

    Ok(())
}

#[entry_point]
/// once it's established, save channel name
pub fn ibc_channel_connect(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelConnectMsg,
) -> StdResult<IbcBasicResponse> {
    let channel = msg.channel();

    let channel_id = &channel.endpoint.channel_id;
    CHANNEL.save(deps.storage, channel_id)?;

    Ok(IbcBasicResponse::new()
        .add_attribute("action", "ibc_connect")
        .add_attribute("channel_id", channel_id))
}

#[entry_point]
/// On closed channel, simply delete the account from our local store
pub fn ibc_channel_close(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelCloseMsg,
) -> StdResult<IbcBasicResponse> {
    let channel = msg.channel();

    // remove the channel
    CHANNEL.remove(deps.storage);

    Ok(IbcBasicResponse::new()
        .add_attribute("action", "ibc_close")
        .add_attribute("channel_id", channel.endpoint.channel_id.clone()))
}

#[entry_point]
/// never should be called as the other side never sends packets
// TODO: what will happen if return error in these callbacks?
pub fn ibc_packet_receive(
    deps: DepsMut,
    _env: Env,
    packet: IbcPacketReceiveMsg,
) -> StdResult<IbcReceiveResponse> {
    // TODO: receive OracleResponsePacketData
    // TODO: check result.status first!

    let data: OracleResponsePacketData = from_binary(&packet.packet.data)?;
    // TODO: error handling
    let result_data: obi_calldata::Output = OBIDecode::try_from_slice(data.result.as_slice())
        .ok()
        .expect("correct output datatype");
    PRICES.save(
        deps.storage,
        &(data.resolve_status.to_string(), Some(result_data.rates)),
    )?;

    Ok(IbcReceiveResponse::new()
        .set_ack(b"{}")
        .add_attribute("action", "ibc_packet_ack"))
}

#[entry_point]
pub fn ibc_packet_ack(
    _deps: DepsMut,
    _env: Env,
    msg: IbcPacketAckMsg,
) -> StdResult<IbcBasicResponse> {
    // which local channel was this packet send from
    // let caller = msg.original_packet.src.channel_id;

    // we need to parse the ack based on our request
    // TODO: use from_binary?
    let packet: OracleRequest = from_slice(&msg.original_packet.data)?;
    match packet {
        OracleRequest::OracleRequestPacketData { .. } => {
            let res: AcknowledgementMsg<OracleRequestPacketAcknowledgement> =
                from_slice(&msg.acknowledgement.data)?;
            let request_id: String = res
                .into_result()
                .map(|r| r.request_id.to_string())
                .unwrap_or("".to_string());
            Ok(IbcBasicResponse::new()
                .add_attribute("action", "acknowledge_oracle_request_packet_data")
                .add_attribute("request_id", request_id))
        }
    }
}

#[entry_point]
/// we just ignore these now. shall we store some info?
pub fn ibc_packet_timeout(
    _deps: DepsMut,
    _env: Env,
    _msg: IbcPacketTimeoutMsg,
) -> StdResult<IbcBasicResponse> {
    Ok(IbcBasicResponse::new().add_attribute("action", "ibc_packet_timeout"))
}
