use std::sync::Arc;

use anvil_edit_contracts::ConfigurationIdentity;
use arc_swap::ArcSwap;

/// Supplies one immutable configuration identity to a request.
pub trait ConfigurationIdentityProvider {
    /// Pins and returns the provider's current foundation identity.
    fn pin(&self) -> PinnedConfigurationIdentity;
}

/// The atomically replaceable, process-local configuration identity pointer.
pub struct ActiveConfigurationIdentity {
    current: ArcSwap<ConfigurationIdentity>,
}

impl ActiveConfigurationIdentity {
    /// Starts with a structurally valid standalone configuration identity.
    #[must_use]
    pub fn new(initial: ConfigurationIdentity) -> Self {
        Self {
            current: ArcSwap::from_pointee(initial),
        }
    }

    /// Atomically replaces the identity after validation outside this slice.
    ///
    /// This foundation seam deliberately does not implement configuration
    /// provenance, policy intersection, or an Events reconciler. A future
    /// complete snapshot provider must perform those checks before replacement.
    #[must_use]
    pub fn replace_after_external_validation(
        &self,
        next: ConfigurationIdentity,
    ) -> Arc<ConfigurationIdentity> {
        self.current.swap(Arc::new(next))
    }
}

impl ConfigurationIdentityProvider for ActiveConfigurationIdentity {
    fn pin(&self) -> PinnedConfigurationIdentity {
        PinnedConfigurationIdentity {
            identity: self.current.load_full(),
        }
    }
}

/// A request-local identity reference stable across later replacements.
#[derive(Clone)]
pub struct PinnedConfigurationIdentity {
    identity: Arc<ConfigurationIdentity>,
}

impl PinnedConfigurationIdentity {
    /// Returns the configuration identity retained by this request-local pin.
    #[must_use]
    pub fn identity(&self) -> &ConfigurationIdentity {
        &self.identity
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;

    use anvil_edit_contracts::Sha256Digest;

    use super::*;

    fn identity(revision: &str, digest_byte: char) -> ConfigurationIdentity {
        ConfigurationIdentity::standalone(
            "standalone/default",
            revision,
            Sha256Digest::new(digest_byte.to_string().repeat(64)).expect("fixture digest is valid"),
        )
        .expect("fixture snapshot is valid")
    }

    #[test]
    fn request_pin_survives_concurrent_replacement() {
        let active = Arc::new(ActiveConfigurationIdentity::new(identity("r1", 'a')));
        let request_pin = active.pin();
        let updater = Arc::clone(&active);

        thread::spawn(move || updater.replace_after_external_validation(identity("r2", 'b')))
            .join()
            .expect("replacement thread succeeds");

        assert_eq!(request_pin.identity().revision(), "r1");
        assert_eq!(active.pin().identity().revision(), "r2");
    }
}
