#![warn(clippy::unwrap_used, clippy::expect_used)]

pub mod contract;
pub mod msg;
pub mod state;

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod testing;
