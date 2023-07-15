
use cw_orch::prelude::TxHandler;
use cw_orch::prelude::CwOrchInstantiate;
use cw_orch::prelude::CwOrchUpload;
use cw_orch::daemon::networks::PION_1;
use neutron_interchain_queries::msg::InstantiateMsg;
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

	let bad_kids = NeutronInterchainQueries::new(INTERCHAIN_QUERY_ID, chain.clone());

	// Uploading
	bad_kids.upload()?;

	// Instantiating the contract
	bad_kids.instantiate(&InstantiateMsg{
		connection_id: "connection-82".to_string(),
		contract_addr: "stars1mrtt39mc5d6zhawje9a24uh2wjf9jv0g0vtgqj5etyljmt29q07s6te037".to_string()
	}, Some(&chain.sender()), None)?;

	// Registering the ica account
	bad_kids.register_ica()?;


	// Interchain account creation via hermes
	/*

	Command::new("export")
        .arg("PATH=~/.hermes:$PATH")
        .spawn()
        .expect("export command failed to start");

	Command::new("PATH=~/.hermes:$PATH")
        .arg("hermes")
        .spawn()
        .expect("ls command failed to start");



	*/


	Ok(())
}


