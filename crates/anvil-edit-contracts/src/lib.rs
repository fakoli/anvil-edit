//! Semantic domain types shared by Anvil Edit components.
//!
//! These Rust types are not a frozen wire format. Concrete serialization and
//! transport remain an explicit product decision.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod configuration;

pub use configuration::{
    ConfigurationError, ConfigurationIdentity, ConfigurationMode, ContractVersion,
    FOUNDATION_CONTRACT_VERSION, Sha256Digest,
};
