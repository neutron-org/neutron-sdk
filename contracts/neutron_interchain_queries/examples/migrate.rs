
use crate::config::INTERCHAIN_QUERY_ID;
use cw_orch::daemon::networks::PION_1;
use cw_orch::daemon::DaemonBuilder;
use cw_orch::prelude::ContractInstance;

use cw_orch::prelude::CwOrchMigrate;
use cw_orch::prelude::CwOrchUpload;
use neutron_interchain_queries::contract::NeutronInterchainQueries;



use neutron_interchain_queries::msg::MigrateMsg;
use tokio::runtime::Runtime;
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

    // bad_kids.upload()?;

    bad_kids.migrate(&MigrateMsg {}, 1144)?;
    Ok(())
}
