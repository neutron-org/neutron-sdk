pub mod bindings;
mod errors;
pub mod interchain_queries;
pub mod interchain_txs;
pub mod proto_types;
pub mod query;
pub mod sudo;

pub use errors::error::{NeutronError, NeutronResult};

// This is a signal, such that any contract that imports these helpers will only run on the
// neutron blockchain
#[no_mangle]
extern "C" fn requires_neutron() {}
