// import all unchanged helpers and methods from v045 package
// to make it available from v047 package (kinda proxy) since they work with Cosmos SDK 0.47 as usual
pub use crate::interchain_queries::v045::helpers;
pub use crate::interchain_queries::v045::register_queries;

pub mod queries;
pub mod types;

#[cfg(test)]
mod testing;
