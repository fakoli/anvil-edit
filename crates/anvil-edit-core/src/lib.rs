//! Latency-sensitive Anvil Edit coordination primitives.
//!
//! The foundation crate currently implements only immutable configuration
//! pinning. It does not dispatch inference, persist traces, or apply edits.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod configuration;

pub use configuration::{
    ActiveConfigurationIdentity, ConfigurationIdentityProvider, PinnedConfigurationIdentity,
};

/// A truthful summary of the currently implemented Core surface.
pub const FOUNDATION_STATUS: &str = "configuration-identity-pinning-only";
