use std::collections::BTreeSet;

use crate::{
    CheckResult, ContractError, ContractVersion, FOUNDATION_CONTRACT_VERSION, Identifier,
    MonotonicTick, ReasonCode, RecordEnvelope, Sha256Digest, WallClockMicros,
};

/// Backwards-compatible name for configuration construction failures.
pub type ConfigurationError = ContractError;

/// The source of an immutable configuration snapshot.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ConfigurationMode {
    /// Configuration was resolved locally without a fleet controller.
    Standalone,
    /// Configuration originated from a managed desired revision and was narrowed locally.
    Managed,
}

/// Stable identity pinned into each request that uses a configuration snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigurationIdentity {
    contract_version: ContractVersion,
    snapshot_id: Identifier,
    revision: Identifier,
    digest: Sha256Digest,
    mode: ConfigurationMode,
}

impl ConfigurationIdentity {
    /// Creates a structurally valid standalone configuration identity.
    pub fn standalone(
        snapshot_id: impl Into<String>,
        revision: impl Into<String>,
        digest: Sha256Digest,
    ) -> Result<Self, ConfigurationError> {
        Self::new(snapshot_id, revision, digest, ConfigurationMode::Standalone)
    }

    /// Creates a structurally valid managed configuration identity.
    pub fn managed(
        snapshot_id: impl Into<String>,
        revision: impl Into<String>,
        digest: Sha256Digest,
    ) -> Result<Self, ConfigurationError> {
        Self::new(snapshot_id, revision, digest, ConfigurationMode::Managed)
    }

    fn new(
        snapshot_id: impl Into<String>,
        revision: impl Into<String>,
        digest: Sha256Digest,
        mode: ConfigurationMode,
    ) -> Result<Self, ConfigurationError> {
        Ok(Self {
            contract_version: FOUNDATION_CONTRACT_VERSION,
            snapshot_id: Identifier::new("snapshot_id", snapshot_id)?,
            revision: Identifier::new("revision", revision)?,
            digest,
            mode,
        })
    }

    /// Returns the semantic contract version.
    #[must_use]
    pub const fn contract_version(&self) -> ContractVersion {
        self.contract_version
    }

    /// Returns the immutable snapshot identifier.
    #[must_use]
    pub fn snapshot_id(&self) -> &str {
        self.snapshot_id.as_str()
    }

    /// Returns the immutable configuration revision.
    #[must_use]
    pub fn revision(&self) -> &str {
        self.revision.as_str()
    }

    /// Returns the digest binding the effective configuration.
    #[must_use]
    pub const fn digest(&self) -> &Sha256Digest {
        &self.digest
    }

    /// Returns how this snapshot was resolved.
    #[must_use]
    pub const fn mode(&self) -> ConfigurationMode {
        self.mode
    }
}

/// Required component classes within one complete configuration snapshot.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ConfigurationComponentKind {
    /// Opportunity and prediction policy.
    PredictionPolicy,
    /// Context selection and freshness policy.
    ContextPolicy,
    /// Candidate display and abstention policy.
    DisplayPolicy,
    /// Explicit capability-selection policy.
    RoutingPolicy,
    /// Finite authorization policy.
    AuthorizationPolicy,
    /// Native prompt protocol selection.
    PromptProtocol,
    /// Allowed explicit capability aliases.
    CapabilityPack,
    /// Candidate normalization and validation policy.
    NormalizationPolicy,
}

impl ConfigurationComponentKind {
    const REQUIRED: [Self; 8] = [
        Self::PredictionPolicy,
        Self::ContextPolicy,
        Self::DisplayPolicy,
        Self::RoutingPolicy,
        Self::AuthorizationPolicy,
        Self::PromptProtocol,
        Self::CapabilityPack,
        Self::NormalizationPolicy,
    ];
}

/// Immutable identity for one configuration component.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComponentIdentity {
    /// Logical component identifier.
    pub id: Identifier,
    /// Immutable component revision.
    pub revision: Identifier,
    /// Digest of the canonical component artifact.
    pub digest: Sha256Digest,
}

/// One typed component included in a complete snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigurationComponent {
    /// Component role within the snapshot.
    pub kind: ConfigurationComponentKind,
    /// Immutable component identity.
    pub identity: ComponentIdentity,
}

/// Exact managed desired-state provenance retained separately from active identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DesiredConfigurationProvenance {
    /// Desired event identifier.
    pub desired_event_id: Identifier,
    /// Locally bound authority identifier.
    pub authority: Identifier,
    /// Logical managed resource key.
    pub resource: Identifier,
    /// Monotonic generation within the authority/resource binding.
    pub generation: u64,
    /// Immutable desired revision.
    pub revision: Identifier,
    /// Locally registered adapter identifier.
    pub adapter: Identifier,
    /// Immutable locally registered adapter revision.
    pub adapter_revision: Identifier,
    /// Digest of the referenced configuration bundle artifact.
    pub artifact_digest: Sha256Digest,
    /// Independent bundle-contract version.
    pub bundle_contract_version: ContractVersion,
}

/// Provider and activation information for a complete snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigurationSource {
    /// Local provider implementation identity.
    pub provider: Identifier,
    /// Immutable provider revision.
    pub provider_revision: Identifier,
    /// Local activation attempt that made the snapshot eligible.
    pub activation_attempt_id: Identifier,
    /// Previous active snapshot, when replacement occurred.
    pub previous_snapshot_id: Option<Identifier>,
    /// Managed desired provenance, absent in standalone mode.
    pub desired: Option<DesiredConfigurationProvenance>,
}

/// The immutable, locally validated configuration used by prediction work.
///
/// Construction proves only structural completeness and mode/provenance
/// consistency. It is not deployment, executor health, qualification, or
/// policy-promotion evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigurationSnapshot {
    envelope: RecordEnvelope,
    identity: ConfigurationIdentity,
    source: ConfigurationSource,
    components: Vec<ConfigurationComponent>,
    effective_local_policy_digest: Sha256Digest,
    externally_narrowed: bool,
    valid_until_wall: Option<WallClockMicros>,
}

impl ConfigurationSnapshot {
    /// Creates a structurally complete immutable snapshot.
    pub fn new(
        envelope: RecordEnvelope,
        identity: ConfigurationIdentity,
        source: ConfigurationSource,
        components: Vec<ConfigurationComponent>,
        effective_local_policy_digest: Sha256Digest,
        externally_narrowed: bool,
        valid_until_wall: Option<WallClockMicros>,
    ) -> Result<Self, ConfigurationError> {
        if envelope.id().as_str() != identity.snapshot_id() {
            return Err(ContractError::InvalidState(
                "configuration envelope and snapshot identity must agree",
            ));
        }
        let managed = identity.mode() == ConfigurationMode::Managed;
        if managed != source.desired.is_some() {
            return Err(ContractError::InvalidState(
                "managed mode and desired provenance must agree",
            ));
        }

        if source
            .desired
            .as_ref()
            .is_some_and(|desired| desired.generation == 0)
        {
            return Err(ContractError::InvalidState(
                "managed generation must start at one",
            ));
        }

        let kinds = components
            .iter()
            .map(|component| component.kind)
            .collect::<BTreeSet<_>>();
        if kinds.len() != components.len() {
            return Err(ContractError::DuplicateReference(
                "configuration component kind",
            ));
        }
        if kinds != ConfigurationComponentKind::REQUIRED.into_iter().collect() {
            return Err(ContractError::InvalidState(
                "configuration snapshot must contain every required component exactly once",
            ));
        }

        Ok(Self {
            envelope,
            identity,
            source,
            components,
            effective_local_policy_digest,
            externally_narrowed,
            valid_until_wall,
        })
    }

    /// Returns the common causal envelope.
    #[must_use]
    pub const fn envelope(&self) -> &RecordEnvelope {
        &self.envelope
    }

    /// Returns the identity pinned by requests.
    #[must_use]
    pub const fn identity(&self) -> &ConfigurationIdentity {
        &self.identity
    }

    /// Returns provider and activation provenance.
    #[must_use]
    pub const fn source(&self) -> &ConfigurationSource {
        &self.source
    }

    /// Returns the complete ordered component set.
    #[must_use]
    pub fn components(&self) -> &[ConfigurationComponent] {
        &self.components
    }

    /// Returns the effective locally compiled policy digest.
    #[must_use]
    pub const fn effective_local_policy_digest(&self) -> &Sha256Digest {
        &self.effective_local_policy_digest
    }

    /// Returns whether an external proposal was narrowed by local policy.
    #[must_use]
    pub const fn externally_narrowed(&self) -> bool {
        self.externally_narrowed
    }

    /// Returns the optional wall-clock validity limit.
    #[must_use]
    pub const fn valid_until_wall(&self) -> Option<WallClockMicros> {
        self.valid_until_wall
    }
}

/// Lifecycle state observed for an immutable configuration snapshot.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ConfigurationLifecycleState {
    /// Validated and staged but not active.
    Staged,
    /// Atomically selected for new prediction work.
    Active,
    /// Replaced by another active snapshot.
    Superseded,
    /// Rejected before activation.
    Rejected,
    /// Replaced by a verified prior snapshot after failed activation.
    RolledBack,
    /// Activation state cannot yet be determined after interruption.
    Indeterminate,
}

/// Append-only lifecycle evidence for an immutable configuration snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigurationLifecycleObservation {
    /// Common causal envelope.
    pub envelope: RecordEnvelope,
    /// Configuration snapshot being observed.
    pub snapshot_id: Identifier,
    /// Observed lifecycle state.
    pub state: ConfigurationLifecycleState,
    /// Bounded source-free reasons for the observation.
    pub reason_codes: Vec<ReasonCode>,
}

/// Check results produced while reconciling managed desired configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReconciliationChecks {
    /// Desired envelope and bundle schema validation.
    pub schema: CheckResult,
    /// Core, adapter, and product compatibility validation.
    pub compatibility: CheckResult,
    /// Artifact digest, size, and provenance validation.
    pub artifact: CheckResult,
    /// Local policy intersection and narrow-only validation.
    pub local_policy: CheckResult,
    /// Atomic activation result.
    pub activation: CheckResult,
    /// Read-back verification against Core's exact active identity.
    pub verification: CheckResult,
}

/// Terminal managed-configuration reconciliation outcome.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ReconciliationOutcome {
    /// The exact desired revision became active without further narrowing.
    Applied,
    /// The bundle became active after local policy narrowed its effect.
    AppliedWithLocalNarrowing,
    /// Validation, preparation, activation, or verification failed.
    Failed,
    /// Local approval is required before activation.
    AwaitingApproval,
    /// Delivery was an idempotent older or duplicate generation.
    NoChange,
    /// Recovery must observe Core before choosing an action.
    Indeterminate,
}

/// Source-free evidence joining external desired state to local activation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigurationReconciliationObservation {
    /// Common causal envelope.
    pub envelope: RecordEnvelope,
    /// Exact desired state being reconciled.
    pub desired: DesiredConfigurationProvenance,
    /// Reconciliation operation identifier.
    pub operation_id: Identifier,
    /// Local activation attempt identifier.
    pub activation_attempt_id: Identifier,
    /// Prior active snapshot, when one existed.
    pub prior_snapshot_id: Option<Identifier>,
    /// Proposed effective snapshot.
    pub proposed_snapshot_id: Option<Identifier>,
    /// Receive tick on the reconciler clock.
    pub received_at: MonotonicTick,
    /// Optional staging tick on the same declared clock.
    pub staged_at: Option<MonotonicTick>,
    /// Optional activation tick on the same declared clock.
    pub activated_at: Option<MonotonicTick>,
    /// Optional verification tick on the same declared clock.
    pub verified_at: Option<MonotonicTick>,
    /// Optional rollback tick on the same declared clock.
    pub rolled_back_at: Option<MonotonicTick>,
    /// Results for every reconciliation gate.
    pub checks: ReconciliationChecks,
    /// Terminal reconciliation outcome.
    pub outcome: ReconciliationOutcome,
    /// Exact effective active snapshot after reconciliation, when known.
    pub effective_snapshot: Option<ConfigurationIdentity>,
    /// Bounded source-free reason codes.
    pub reason_codes: Vec<ReasonCode>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    fn identifier(field: &'static str, value: &str) -> Identifier {
        Identifier::new(field, value).expect("fixture identifier")
    }

    fn digest(character: char) -> Sha256Digest {
        Sha256Digest::new(character.to_string().repeat(64)).expect("fixture digest")
    }

    fn components() -> Vec<ConfigurationComponent> {
        let digest_characters = ['b', 'c', 'd', 'e', 'f', '0', '1', '2'];
        ConfigurationComponentKind::REQUIRED
            .into_iter()
            .zip(digest_characters)
            .enumerate()
            .map(|(index, (kind, digest_character))| ConfigurationComponent {
                kind,
                identity: ComponentIdentity {
                    id: identifier("component_id", &format!("component-{index}")),
                    revision: identifier("component_revision", "r1"),
                    digest: digest(digest_character),
                },
            })
            .collect()
    }

    #[test]
    fn accepts_complete_standalone_snapshot() {
        let snapshot = ConfigurationSnapshot::new(
            crate::test_support::envelope("standalone/default", 1),
            ConfigurationIdentity::standalone("standalone/default", "r1", digest('a'))
                .expect("identity"),
            ConfigurationSource {
                provider: identifier("provider", "local-file"),
                provider_revision: identifier("provider_revision", "r1"),
                activation_attempt_id: identifier("activation_attempt", "activate-1"),
                previous_snapshot_id: None,
                desired: None,
            },
            components(),
            digest('f'),
            false,
            None,
        )
        .expect("complete standalone snapshot");

        assert_eq!(
            snapshot.identity().contract_version(),
            ContractVersion::new(0, 2)
        );
        assert_eq!(snapshot.components().len(), 8);
        assert_eq!(snapshot.identity().digest().as_str(), DIGEST);
    }

    #[test]
    fn rejects_missing_component_or_managed_provenance_mismatch() {
        let mut incomplete = components();
        incomplete.pop();
        let source = ConfigurationSource {
            provider: identifier("provider", "local-file"),
            provider_revision: identifier("provider_revision", "r1"),
            activation_attempt_id: identifier("activation_attempt", "activate-1"),
            previous_snapshot_id: None,
            desired: None,
        };

        let missing = ConfigurationSnapshot::new(
            crate::test_support::envelope("standalone/default", 1),
            ConfigurationIdentity::standalone("standalone/default", "r1", digest('a'))
                .expect("identity"),
            source.clone(),
            incomplete,
            digest('f'),
            false,
            None,
        );
        assert_eq!(
            missing,
            Err(ContractError::InvalidState(
                "configuration snapshot must contain every required component exactly once"
            ))
        );

        let mismatch = ConfigurationSnapshot::new(
            crate::test_support::envelope("managed/default", 1),
            ConfigurationIdentity::managed("managed/default", "r1", digest('a')).expect("identity"),
            source,
            components(),
            digest('f'),
            false,
            None,
        );
        assert_eq!(
            mismatch,
            Err(ContractError::InvalidState(
                "managed mode and desired provenance must agree"
            ))
        );
    }
}
