#![allow(
    rustdoc::bare_urls,
    rustdoc::broken_intra_doc_links,
    clippy::derive_partial_eq_without_eq
)]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

pub use prost;
pub use prost_types::Any;

/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const COSMOS_SDK_VERSION: &str = include_str!("neutron/NEUTRON_COMMIT");

pub mod neutron {
    pub mod dex {
        include!("neutron/neutron.dex.rs");
    }
}

/// Cosmos protobuf definitions.
pub mod cosmos {
    pub mod dex {
        include!("neutron/neutron.dex.rs");
    }

    pub mod base {
        pub mod v1beta1 {
            include!("neutron/cosmos.base.v1beta1.rs");
        }

        /// Query support.
        pub mod query {
            pub mod v1beta1 {
                include!("neutron/cosmos.base.query.v1beta1.rs");
            }
        }

        /// Services for the upgrade module.
        pub mod upgrade {
            pub mod v1beta1 {
                include!("neutron/cosmos.upgrade.v1beta1.rs");
            }
        }
    }
}
