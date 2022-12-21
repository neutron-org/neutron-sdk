use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use neutron_interchain_txs::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use neutron_sdk::bindings::query::QueryInterchainAccountAddressResponse;
use neutron_sdk::sudo::msg::SudoMsg;
use std::env::current_dir;
use std::fs::create_dir_all;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(MigrateMsg), &out_dir);
    export_schema(&schema_for!(SudoMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(
        &schema_for!(QueryInterchainAccountAddressResponse),
        &out_dir,
    );
}
