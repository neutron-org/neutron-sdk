use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
#[allow(deprecated)]
use neutron_sdk::bindings::msg::NeutronMsg;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();
    #[allow(deprecated)]
    export_schema(&schema_for!(NeutronMsg), &out_dir);
}
