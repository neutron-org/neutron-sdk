use crate::config::INTERCHAIN_QUERY_ID;
use crate::config::IBC_CONNECTION_ID;
use crate::config::STARTGAZE_NFT_ADDRESS;
use cw_orch::daemon::networks::PION_1;
use cw_orch::daemon::DaemonBuilder;
use cw_orch::prelude::ContractInstance;
use cw_orch::prelude::CwOrchInstantiate;
use cw_orch::prelude::CwOrchUpload;
use cw_orch::prelude::TxHandler;
use neutron_interchain_queries::contract::NeutronInterchainQueries;
use neutron_interchain_queries::contract::INTERCHAIN_ACCOUNT_ID;
use neutron_interchain_queries::msg::ExecuteMsgFns;
use neutron_interchain_queries::msg::InstantiateMsg;

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

    let bad_kids = NeutronInterchainQueries::new(INTERCHAIN_QUERY_ID, chain.clone());

    // Uploading
    bad_kids.upload()?;

    // Instantiating the contract
    bad_kids.instantiate(&InstantiateMsg{
    	connection_id: IBC_CONNECTION_ID.to_string(),
    	contract_addr: STARTGAZE_NFT_ADDRESS.to_string()
    }, Some(&chain.sender()), None)?;

    // // Registering the ica account
    bad_kids.register_ica()?;

    println!("{:?}", format!("hermes create channel --a-chain pion-1 --a-connection connection-92 --a-port icacontroller-{}.{} --b-port icahost --order ordered",bad_kids.address()?, INTERCHAIN_ACCOUNT_ID ));

    Ok(())
}
