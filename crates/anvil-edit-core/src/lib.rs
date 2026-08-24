//! Latency-sensitive Anvil Edit coordination primitives.
//!
//! The foundation crate currently implements immutable configuration pinning
//! and a single-writer exact-revision generation primitive. It does not yet
//! run a session actor, dispatch inference, persist traces, or apply edits.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod configuration;
mod coordination;

pub use configuration::{
    ActiveConfigurationIdentity, ConfigurationIdentityProvider, PinnedConfigurationIdentity,
};
pub use coordination::{
    LatestRevision, RevisionFence, RevisionGeneration, RevisionObservation, RevisionStateError,
};

/// A truthful summary of the currently implemented Core surface.
pub const FOUNDATION_STATUS: &str = "configuration-pinning-and-revision-fence-primitives";
