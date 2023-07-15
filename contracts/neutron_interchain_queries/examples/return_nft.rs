use cosmwasm_std::coin;
use cosmwasm_std::coins;
use crate::config::INTERCHAIN_QUERY_ID;


use cw_orch::daemon::networks::PION_1;
use cw_orch::daemon::DaemonBuilder;

use neutron_interchain_queries::contract::NeutronInterchainQueries;

use neutron_interchain_queries::msg::ExecuteMsgFns;


use neutron_interchain_queries::msg::QueryMsgFns;
use tokio::runtime::Runtime;
mod config;

pub const RECEIVER: &str = "stars18yj2mc7hjk2zqtwr9exfyj625kffwmjg3dr7tv";
pub const TOKEN_ID: &str = "80";
pub const FUNDS_AMOUNT: u128 = 100;

pub fn main() -> cw_orch::anyhow::Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    let rt = Runtime::new()?;
    let chain = DaemonBuilder::default()
        .chain(PION_1)
        .handle(rt.handle())
        .build()?;

    let bad_kids = NeutronInterchainQueries::new(INTERCHAIN_QUERY_ID, chain);

    let denom = bad_kids.token_denom(TOKEN_ID.to_string())?;

    // // Registering the ica account
    // // They pay 2000untrn for submitting a message on stargaze remotely
    bad_kids.unlock_nft(RECEIVER.to_string(),TOKEN_ID.to_string(), &[coin(FUNDS_AMOUNT,denom), coin(2000,"untrn")])?;

    Ok(())
}
