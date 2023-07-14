use neutron_interchain_queries::msg::InstantiateMsg;
use neutron_interchain_queries::msg::ExecuteMsg;
use neutron_interchain_queries::msg::QueryMsg;
use cosmwasm_schema::write_api;


fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}