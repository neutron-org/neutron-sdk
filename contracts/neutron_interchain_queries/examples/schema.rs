use cosmwasm_schema::write_api;
use neutron_interchain_queries::msg::ExecuteMsg;
use neutron_interchain_queries::msg::InstantiateMsg;
use neutron_interchain_queries::msg::QueryMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
