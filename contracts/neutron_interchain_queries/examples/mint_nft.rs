use crate::config::INTERCHAIN_QUERY_ID;

use cw_orch::daemon::networks::PION_1;
use cw_orch::daemon::DaemonBuilder;

use neutron_interchain_queries::contract::NeutronInterchainQueries;
use neutron_interchain_queries::msg::ExecuteMsgFns;

use tokio::runtime::Runtime;
pub const TOKEN_ID: &str = "80";

mod config;
pub fn main() -> cw_orch::anyhow::Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    let rt = Runtime::new()?;
    let chain = DaemonBuilder::default()
        .chain(PION_1)
        .handle(rt.handle())
        .build()?;

    let bad_kids = NeutronInterchainQueries::new(INTERCHAIN_QUERY_ID, chain);

    // Actually mint the bad kid on the local chain
    bad_kids.mint_nft(TOKEN_ID.to_string())?;

    Ok(())
}
