pub mod neutron {
    pub mod dex {
        include!("neutron.dex.rs");
    }

    pub mod interchaintxs {
        pub mod v1 {
            include!("neutron.interchaintxs.v1.rs");
        }
    }

    pub mod feeburner {
        include!("neutron.feeburner.rs");
    }

    pub mod interchainqueries {
        include!("neutron.interchainqueries.rs");
    }

    pub mod cron {
        include!("neutron.cron.rs");
    }
    pub mod transfer {
        include!("neutron.transfer.rs");
    }

    pub mod feerefunder {
        include!("neutron.feerefunder.rs");
    }

    pub mod contractmanager {
        include!("neutron.contractmanager.rs");
        pub mod v1 {
            include!("neutron.contractmanager.v1.rs");
        }
    }
}

pub mod osmosis {
    pub mod tokenfactory {
        pub mod v1beta1 {
            include!("osmosis.tokenfactory.v1beta1.rs");
        }
    }
}
