// import all types from v045 package
// to it available from v047 package (kinda proxy) since they work with Cosmos SDK 0.47 as usual
pub use crate::interchain_queries::v045::types::*;

use crate::interchain_queries::types::KVReconstruct;
use crate::{bindings::types::StorageValue, errors::error::NeutronResult};

use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// But at the same time we replace v045::Balances with the Balances below to create a structure
// compatible with Cosmos SDK 0.47
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
/// A structure that can be reconstructed from **StorageValues**'s for the **Balance Interchain Query**.
/// Contains amounts of coins that are held by some account on remote chain.
pub struct Balances {
    pub amounts: Vec<Uint128>,
}

impl KVReconstruct for Balances {
    fn reconstruct(storage_values: &[StorageValue]) -> NeutronResult<Balances> {
        let mut amounts: Vec<Uint128> = Vec::with_capacity(storage_values.len());

        for kv in storage_values {
            let amount = Uint128::from_str(&String::from_utf8(kv.value.to_vec())?)?;
            amounts.push(amount);
        }

        Ok(Balances { amounts })
    }
}
