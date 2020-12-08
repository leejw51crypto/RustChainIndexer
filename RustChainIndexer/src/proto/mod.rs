pub mod cosmos {
    pub mod auth {
        pub mod v1beta1 {
            include!("cosmos.auth.v1beta1.rs");
        }
    }
    pub mod staking {
        pub mod v1beta1 {
            include!("cosmos.staking.v1beta1.rs");
        }
    }
    pub mod base {
        pub mod abci {
            pub mod v1beta1 {
                include!("cosmos.base.abci.v1beta1.rs");
            }
        }

        pub mod query {
            pub mod v1beta1 {
                include!("cosmos.base.query.v1beta1.rs");
            }
        }

        pub mod v1beta1 {
            include!("cosmos.base.v1beta1.rs");
        }
    }
    pub mod crypto {
        pub mod multisig {
            pub mod v1beta1 {
                include!("cosmos.crypto.multisig.v1beta1.rs");
            }
        }
    }
    pub mod tx {
        pub mod signing {
            pub mod v1beta1 {
                include!("cosmos.tx.signing.v1beta1.rs");
            }
        }
        pub mod v1beta1 {
            include!("cosmos.tx.v1beta1.rs");
        }
    }
}

pub mod tendermint {
    pub mod abci {
        include!("tendermint.abci.rs");
    }
    pub mod blockchain {
        include!("tendermint.blockchain.rs");
    }
    pub mod crypto {
        include!("tendermint.crypto.rs");
    }
    pub mod types {
        include!("tendermint.types.rs");
    }
    pub mod version {
        include!("tendermint.version.rs");
    }
}
