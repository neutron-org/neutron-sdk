mod msg;
mod types;

pub use msg::NeutronMsg;
pub use types::ProtobufAny;

// This is a signal, such that any contract that imports these helpers will only run on the
// neutron blockchain
#[no_mangle]
extern "C" fn requires_neutron() {}