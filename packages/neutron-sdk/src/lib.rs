#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
// #![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

pub mod bindings;
mod errors;
pub mod interchain_queries;
pub mod interchain_txs;
pub mod sudo;

pub use errors::error::{NeutronError, NeutronResult};

// This is a signal, such that any contract that imports these helpers will only run on the
// neutron blockchain
#[no_mangle]
extern "C" fn requires_neutron() {}
