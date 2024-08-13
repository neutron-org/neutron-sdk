#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
// #![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const NEUTROND_VERSION: &str = include_str!("proto_types/NEUTRON_COMMIT");

pub mod bindings;
mod errors;
pub mod interchain_queries;
pub mod interchain_txs;
#[allow(deprecated, clippy::module_inception)]
pub mod proto_types;
pub mod query;
mod serde;
pub mod shim;
pub mod sudo;

pub use errors::error::{NeutronError, NeutronResult};
pub use shim::{cosmwasm_to_proto_coins, try_proto_to_cosmwasm_coins};

// This is a signal, such that any contract that imports these helpers will only run on the
// neutron blockchain
#[no_mangle]
extern "C" fn requires_neutron() {}
