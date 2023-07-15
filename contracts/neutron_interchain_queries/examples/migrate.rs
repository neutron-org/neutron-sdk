
use cw_orch::prelude::CwOrchMigrate;
use cosmwasm_std::Empty;
use neutron_interchain_queries::contract::INTERCHAIN_ACCOUNT_ID;
use cw_orch::prelude::ContractInstance;
use cw_orch::prelude::TxHandler;
use cw_orch::prelude::CwOrchInstantiate;
use cw_orch::prelude::CwOrchUpload;
use cw_orch::daemon::networks::PION_1;
use neutron_interchain_queries::msg::InstantiateMsg;
use neutron_interchain_queries::msg::MigrateMsg;
use tokio::runtime::Runtime;
use cw_orch::daemon::DaemonBuilder;	
use neutron_interchain_queries::contract::NeutronInterchainQueries;
use neutron_interchain_queries::msg::ExecuteMsgFns;


pub const STARTGAZE_NFT_ADDRESS: &str  = "";


pub const INTERCHAIN_QUERY_ID: &str = "bad-kids:queries";
pub fn main()-> cw_orch::anyhow::Result<()>{

	env_logger::init();
	dotenv::dotenv().ok();


	let rt = Runtime::new()?;
	let chain = DaemonBuilder::default()
		.chain(PION_1)
		.handle(rt.handle())
		.build()?;

	let bad_kids = NeutronInterchainQueries::new(INTERCHAIN_QUERY_ID, chain);

	bad_kids.upload()?;

	bad_kids.migrate(&MigrateMsg{}, bad_kids.code_id()?)?;
	Ok(())
}


