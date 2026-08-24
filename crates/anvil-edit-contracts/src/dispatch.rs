use std::collections::BTreeSet;

use crate::{
    ComponentIdentity, ConfigurationIdentity, ContentReference, DataClass, DurationMicros,
    Identifier, MonotonicTick, ReasonCode, RecordEnvelope, Sha256Digest,
};

/// Whether an inference attempt can influence the editor UI.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum VisibilityMode {
    /// Candidate may proceed to display policy.
    Visible,
    /// Candidate is retained only as permitted shadow evidence.
    Shadow,
}

/// Finite purpose for a prediction request.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PredictionPurpose {
    /// Interactive next-edit prediction.
    NextEdit,
    /// Explicitly requested hidden-prediction reveal.
    ExplicitReveal,
    /// Deterministic offline replay.
    Replay,
    /// Live non-visible evaluation.
    ShadowEvaluation,
}

/// Policy action chosen for one prediction opportunity.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum DispatchAction {
    /// Terminate without inference.
    Abstain,
    /// Dispatch a visible-eligible inference attempt.
    Dispatch,
    /// Dispatch a non-visible inference attempt.
    ShadowDispatch,
    /// Delay a decision without silently dispatching.
    Defer,
}

/// Explicit executor target selected by Edit policy.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ExecutorSelection {
    /// Logical capability alias executed without silent substitution.
    CapabilityAlias(Identifier),
    /// Explicit standalone executor identity.
    StandaloneEndpoint(Identifier),
}

/// Relative latency budgets transmitted without comparing unrelated clocks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RelativeBudgets {
    /// Maximum queue duration.
    pub queue: DurationMicros,
    /// Maximum inference duration.
    pub inference: DurationMicros,
    /// Remaining opportunity-to-render duration.
    pub render: DurationMicros,
}

/// Relation between one attempt and earlier work in the same group.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AttemptRelation {
    /// First attempt in a group.
    Initial,
    /// Same policy-selected capability retried after an observable failure.
    Retry,
    /// Concurrent explicitly recorded competitor.
    Race,
    /// Different capability attempted after failure or unavailability.
    Fallback,
    /// Policy-selected higher-cost capability for a harder opportunity.
    Escalation,
}

/// Causal identity for retries, races, fallbacks, and escalations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttemptIdentity {
    /// Common attempt-group identifier.
    pub group_id: Identifier,
    /// Zero-based ordinal within the attempt group.
    pub ordinal: u32,
    /// Relation to earlier work.
    pub relation: AttemptRelation,
    /// Prior attempt identifier when the relation requires one.
    pub prior_attempt_id: Option<Identifier>,
    /// Bounded reason for non-initial attempts.
    pub reason: Option<ReasonCode>,
}

impl AttemptIdentity {
    /// Validates that initial and related attempt fields agree.
    pub fn validate(&self) -> Result<(), crate::ContractError> {
        let initial = self.relation == AttemptRelation::Initial;
        if initial
            && (self.ordinal != 0 || self.prior_attempt_id.is_some() || self.reason.is_some())
        {
            return Err(crate::ContractError::InvalidState(
                "initial attempt must have ordinal zero and no prior attempt or relation reason",
            ));
        }
        if !initial
            && (self.ordinal == 0 || self.prior_attempt_id.is_none() || self.reason.is_none())
        {
            return Err(crate::ContractError::InvalidState(
                "related attempt must name a prior attempt, nonzero ordinal, and reason",
            ));
        }
        Ok(())
    }
}

/// Immutable experiment assignment attached before policy-specific gating.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExperimentAssignment {
    /// Experiment identity.
    pub experiment_id: Identifier,
    /// Immutable experiment revision.
    pub experiment_revision: Identifier,
    /// Assigned cohort or arm.
    pub arm: Identifier,
    /// Assignment-unit identity scoped to this experiment.
    pub assignment_unit: Identifier,
}

/// Policy decision to attempt, suppress, shadow, or defer inference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DispatchDecision {
    /// Common causal envelope.
    pub envelope: RecordEnvelope,
    /// Source prediction opportunity.
    pub opportunity_id: Identifier,
    /// Exact active configuration pinned for the decision.
    pub configuration: ConfigurationIdentity,
    /// Immutable prediction-policy identity.
    pub prediction_policy: ComponentIdentity,
    /// Immutable context-policy identity.
    pub context_policy: ComponentIdentity,
    /// Immutable native protocol identity.
    pub protocol: ComponentIdentity,
    /// Immutable routing-policy identity.
    pub routing_policy: ComponentIdentity,
    /// Policy action.
    pub action: DispatchAction,
    /// Explicit target, absent only for abstain or defer.
    pub executor: Option<ExecutorSelection>,
    /// Finite request purpose.
    pub purpose: PredictionPurpose,
    /// Visible or shadow mode.
    pub visibility: VisibilityMode,
    /// Relative queue, inference, and render budgets.
    pub budgets: RelativeBudgets,
    /// Bounded source-free decision reasons.
    pub reason_codes: Vec<ReasonCode>,
    /// Attempt-group identity and relation.
    pub attempt: AttemptIdentity,
    /// Optional pre-gating experiment assignment.
    pub experiment: Option<ExperimentAssignment>,
}

impl DispatchDecision {
    /// Validates action, target, purpose, visibility, and attempt shape.
    pub fn validate(&self) -> Result<(), crate::ContractError> {
        self.attempt.validate()?;
        match self.action {
            DispatchAction::Abstain | DispatchAction::Defer => {
                if self.executor.is_some() {
                    return Err(crate::ContractError::InvalidState(
                        "abstain or defer decision cannot select an executor",
                    ));
                }
            }
            DispatchAction::Dispatch => {
                if self.executor.is_none() || self.visibility != VisibilityMode::Visible {
                    return Err(crate::ContractError::InvalidState(
                        "visible dispatch must select an executor and visible mode",
                    ));
                }
            }
            DispatchAction::ShadowDispatch => {
                if self.executor.is_none() || self.visibility != VisibilityMode::Shadow {
                    return Err(crate::ContractError::InvalidState(
                        "shadow dispatch must select an executor and shadow mode",
                    ));
                }
            }
        }
        if self.purpose == PredictionPurpose::ShadowEvaluation
            && self.visibility != VisibilityMode::Shadow
        {
            return Err(crate::ContractError::InvalidState(
                "shadow-evaluation purpose requires shadow visibility",
            ));
        }
        Ok(())
    }
}

/// Allow or deny result for one independent authorization dimension.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PermissionDecision {
    /// Dimension is explicitly allowed within the grant bounds.
    Allow,
    /// Dimension is denied.
    Deny,
}

/// Independent authorization dimensions; no grant implies an adjacent grant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PermissionSet {
    /// Read selected runtime source into local memory.
    pub runtime_read: PermissionDecision,
    /// Serialize and dispatch content to the named executor destination.
    pub executor_dispatch: PermissionDecision,
    /// Retain content in a named governed store.
    pub persistence: PermissionDecision,
    /// Reconstruct a permitted corpus in Lab.
    pub replay: PermissionDecision,
    /// Send a previewed package to an external destination.
    pub export: PermissionDecision,
    /// Use content to change model weights or adapters.
    pub training: PermissionDecision,
    /// Send context to a non-visible model attempt.
    pub shadow: PermissionDecision,
    /// Include bounded task fields.
    pub task_context: PermissionDecision,
    /// Join outcomes across declared checkpoints.
    pub outcome_correlation: PermissionDecision,
}

impl PermissionSet {
    /// Returns a fail-closed set with every independent dimension denied.
    #[must_use]
    pub const fn deny_all() -> Self {
        Self {
            runtime_read: PermissionDecision::Deny,
            executor_dispatch: PermissionDecision::Deny,
            persistence: PermissionDecision::Deny,
            replay: PermissionDecision::Deny,
            export: PermissionDecision::Deny,
            training: PermissionDecision::Deny,
            shadow: PermissionDecision::Deny,
            task_context: PermissionDecision::Deny,
            outcome_correlation: PermissionDecision::Deny,
        }
    }
}

/// Result of protected-content filtering before dispatch.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ProtectedContentResult {
    /// No protected content was detected under the configured checks.
    Clear,
    /// A smaller context was compiled after permitted removal.
    RemovedByRecordedPolicy,
    /// Dispatch was denied without echoing protected bytes.
    Denied,
}

/// Destination and operator trust identity known before serialization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DestinationIdentity {
    /// Logical destination identity.
    pub destination: Identifier,
    /// Operator trust domain.
    pub operator_trust_domain: Identifier,
    /// Authenticated peer or principal identity at the permitted disclosure level.
    pub peer_identity: Identifier,
}

/// Lifetime of a finite authorization grant.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GrantLifetime {
    /// Grant can be consumed exactly once.
    OneShot,
    /// Grant expires after a relative duration on the issuer's declared clock.
    Relative(DurationMicros),
}

/// Terminal authorization decision.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GrantDecision {
    /// The named dispatch is authorized within all recorded limits.
    Granted,
    /// Dispatch is denied before serialization.
    Denied,
}

/// Finite authorization to read named snapshot content in the local runtime.
///
/// This record exists before context compilation and is deliberately separate
/// from the later destination-bound `ExecutionGrant`. A dispatch grant cannot
/// retroactively authorize the reads used to create a context pack.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeReadGrant {
    /// Common causal envelope.
    pub envelope: RecordEnvelope,
    /// Snapshot whose governed content may be read.
    pub snapshot_id: Identifier,
    /// Exact source-free content handles covered by the decision.
    pub authorized_content: Vec<ContentReference>,
    /// Finite operation purpose.
    pub purpose: PredictionPurpose,
    /// Independent authorization dimensions.
    pub permissions: PermissionSet,
    /// Protected-content filtering result.
    pub protected_content: ProtectedContentResult,
    /// Effective compiled policy digest.
    pub effective_policy_digest: Sha256Digest,
    /// Grant issuer identity.
    pub issuer: Identifier,
    /// Issue tick on the issuer's monotonic clock.
    pub issued_at: MonotonicTick,
    /// One-shot or relative grant lifetime.
    pub lifetime: GrantLifetime,
    /// Terminal grant decision.
    pub decision: GrantDecision,
    /// Bounded denial or narrowing reasons.
    pub reason_codes: Vec<ReasonCode>,
}

impl RuntimeReadGrant {
    /// Enforces a read-only, non-adjacent authorization shape.
    pub fn validate(&self) -> Result<(), crate::ContractError> {
        let read_allowed = self.permissions.runtime_read == PermissionDecision::Allow;
        if (self.decision == GrantDecision::Granted) != read_allowed {
            return Err(crate::ContractError::InvalidState(
                "runtime-read decision and runtime-read permission must agree",
            ));
        }
        if self.permissions.executor_dispatch == PermissionDecision::Allow
            || self.permissions.persistence == PermissionDecision::Allow
            || self.permissions.replay == PermissionDecision::Allow
            || self.permissions.export == PermissionDecision::Allow
            || self.permissions.training == PermissionDecision::Allow
            || self.permissions.shadow == PermissionDecision::Allow
            || self.permissions.task_context == PermissionDecision::Allow
            || self.permissions.outcome_correlation == PermissionDecision::Allow
        {
            return Err(crate::ContractError::InvalidState(
                "runtime-read grant cannot authorize an adjacent purpose",
            ));
        }
        if self.decision == GrantDecision::Granted && self.authorized_content.is_empty() {
            return Err(crate::ContractError::InvalidState(
                "granted runtime read must bind at least one content handle",
            ));
        }
        validate_content_manifest(&self.authorized_content)
    }
}

/// Finite authorization compiled before source-bearing serialization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionGrant {
    /// Common causal envelope.
    pub envelope: RecordEnvelope,
    /// Source dispatch decision.
    pub dispatch_decision_id: Identifier,
    /// Exact context pack whose content classes and digests were evaluated.
    pub context_pack_id: Identifier,
    /// Named destination and trust domain.
    pub destination: DestinationIdentity,
    /// Explicit capability or standalone endpoint identity.
    pub executor: ExecutorSelection,
    /// Immutable native protocol identity.
    pub protocol: ComponentIdentity,
    /// Finite request purpose.
    pub purpose: PredictionPurpose,
    /// Visible or shadow mode.
    pub visibility: VisibilityMode,
    /// Content classes permitted for this exact decision.
    pub allowed_content_classes: Vec<DataClass>,
    /// Exact source-free content handles bound before serialization.
    pub authorized_content: Vec<ContentReference>,
    /// Protected-content filtering result.
    pub protected_content: ProtectedContentResult,
    /// Independent authorization dimensions.
    pub permissions: PermissionSet,
    /// Effective compiled policy digest.
    pub effective_policy_digest: Sha256Digest,
    /// Grant issuer identity.
    pub issuer: Identifier,
    /// Issue tick on the issuer's monotonic clock.
    pub issued_at: MonotonicTick,
    /// One-shot or relative grant lifetime.
    pub lifetime: GrantLifetime,
    /// Terminal grant decision.
    pub decision: GrantDecision,
    /// Bounded denial or narrowing reasons.
    pub reason_codes: Vec<ReasonCode>,
}

impl ExecutionGrant {
    /// Validates the grant decision against dispatch and shadow permissions.
    pub fn validate(&self) -> Result<(), crate::ContractError> {
        let dispatch_allowed = self.permissions.executor_dispatch == PermissionDecision::Allow;
        if (self.decision == GrantDecision::Granted) != dispatch_allowed {
            return Err(crate::ContractError::InvalidState(
                "grant decision and executor-dispatch permission must agree",
            ));
        }
        if self.visibility == VisibilityMode::Shadow
            && self.decision == GrantDecision::Granted
            && self.permissions.shadow != PermissionDecision::Allow
        {
            return Err(crate::ContractError::InvalidState(
                "shadow dispatch requires an independent shadow grant",
            ));
        }
        if self.visibility == VisibilityMode::Visible
            && self.permissions.shadow == PermissionDecision::Allow
        {
            return Err(crate::ContractError::InvalidState(
                "visible dispatch cannot consume shadow permission",
            ));
        }
        if self.permissions.runtime_read == PermissionDecision::Allow {
            return Err(crate::ContractError::InvalidState(
                "dispatch grant cannot retroactively authorize runtime reads",
            ));
        }
        if self.permissions.persistence == PermissionDecision::Allow
            || self.permissions.replay == PermissionDecision::Allow
            || self.permissions.export == PermissionDecision::Allow
            || self.permissions.training == PermissionDecision::Allow
            || self.permissions.task_context == PermissionDecision::Allow
            || self.permissions.outcome_correlation == PermissionDecision::Allow
        {
            return Err(crate::ContractError::InvalidState(
                "dispatch grant cannot authorize an adjacent purpose",
            ));
        }
        if self.decision == GrantDecision::Granted && self.authorized_content.is_empty() {
            return Err(crate::ContractError::InvalidState(
                "granted dispatch must bind at least one content handle",
            ));
        }
        validate_content_manifest(&self.authorized_content)?;
        let actual_classes = self
            .authorized_content
            .iter()
            .map(ContentReference::data_class)
            .collect::<BTreeSet<_>>();
        let declared_classes = self
            .allowed_content_classes
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        if declared_classes.len() != self.allowed_content_classes.len()
            || declared_classes != actual_classes
        {
            return Err(crate::ContractError::AggregateMismatch(
                "allowed_content_classes",
            ));
        }
        if self.decision == GrantDecision::Granted
            && self
                .authorized_content
                .iter()
                .any(|content| content.data_class() == DataClass::P4HighlySensitive)
        {
            return Err(crate::ContractError::InvalidState(
                "protected content cannot be granted for executor dispatch",
            ));
        }
        if self.protected_content == ProtectedContentResult::Denied
            && self.decision == GrantDecision::Granted
        {
            return Err(crate::ContractError::InvalidState(
                "protected-content denial cannot produce a granted dispatch",
            ));
        }
        Ok(())
    }
}

fn validate_content_manifest(content: &[ContentReference]) -> Result<(), crate::ContractError> {
    let unique = content
        .iter()
        .map(|item| (item.purpose_scope(), item.id()))
        .collect::<BTreeSet<_>>();
    if unique.len() != content.len() {
        Err(crate::ContractError::DuplicateReference(
            "authorized_content",
        ))
    } else {
        Ok(())
    }
}

/// Bounded output limits for a single inference request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OutputBudget {
    /// Maximum generated tokens.
    pub max_tokens: u32,
    /// Maximum normalized edits.
    pub max_edits: u16,
    /// Maximum total replacement bytes.
    pub max_replacement_bytes: u64,
}

/// Determinism-relevant generation identity without free-form settings.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenerationSettings {
    /// Whether the executor is requested to use deterministic generation.
    pub deterministic: bool,
    /// Seed when supported and declared.
    pub seed: Option<u64>,
    /// Digest of the complete canonical executor settings.
    pub settings_digest: Sha256Digest,
}

/// A single explicit inference attempt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PredictionRequest {
    /// Common causal envelope.
    pub envelope: RecordEnvelope,
    /// Canonical identifier used to join executor evidence.
    pub request_correlation_id: Identifier,
    /// Source opportunity.
    pub opportunity_id: Identifier,
    /// Exact context pack.
    pub context_pack_id: Identifier,
    /// Configuration identity inherited from the dispatch.
    pub configuration: ConfigurationIdentity,
    /// Immutable prediction-policy identity.
    pub prediction_policy: ComponentIdentity,
    /// Immutable native protocol identity.
    pub protocol: ComponentIdentity,
    /// Explicit executor target.
    pub executor: ExecutorSelection,
    /// Output limits.
    pub output_budget: OutputBudget,
    /// Determinism-relevant settings.
    pub generation: GenerationSettings,
    /// Source dispatch decision.
    pub dispatch_decision_id: Identifier,
    /// Consumed authorization grant.
    pub execution_grant_id: Identifier,
    /// Relative queue, inference, and render budgets.
    pub budgets: RelativeBudgets,
    /// Dispatch tick on the producer's monotonic clock.
    pub dispatched_at: MonotonicTick,
    /// Cancellation identity propagated to context and executor work.
    pub cancellation_id: Identifier,
    /// Attempt-group identity and relation.
    pub attempt: AttemptIdentity,
    /// Optional experiment assignment.
    pub experiment: Option<ExperimentAssignment>,
}

impl PredictionRequest {
    /// Validates bounded output and attempt structure.
    pub fn validate(&self) -> Result<(), crate::ContractError> {
        self.attempt.validate()?;
        if self.output_budget.max_tokens == 0
            || self.output_budget.max_edits == 0
            || self.output_budget.max_replacement_bytes == 0
        {
            return Err(crate::ContractError::InvalidState(
                "prediction request output budgets must be nonzero",
            ));
        }
        Ok(())
    }
}

/// Executor cache state reported for one request.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum CacheState {
    /// No reusable prefix or runtime cache was used.
    Cold,
    /// Identical prefix or declared cache state was reused.
    WarmExact,
    /// A mostly matching prefix or partial cache was reused.
    WarmPartial,
    /// Executor did not expose cache state.
    Unknown,
}

/// Terminal executor status for a request.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ServingStatus {
    /// Generation completed within executor bounds.
    Completed,
    /// Cancellation terminated work.
    Cancelled,
    /// Relative deadline was exhausted.
    DeadlineExceeded,
    /// Selected explicit route or executor was unavailable.
    Unavailable,
    /// Executor failed for another bounded reason.
    Failed,
}

/// Executor-owned timing observations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ServingDurations {
    /// Queue duration when known.
    pub queue: Option<DurationMicros>,
    /// Dispatch or receipt to first token when known.
    pub time_to_first_token: Option<DurationMicros>,
    /// First token to termination when known.
    pub decode: Option<DurationMicros>,
    /// Total executor duration when known.
    pub total: Option<DurationMicros>,
}

/// Executor-owned identity and timing evidence joined to a request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServingObservation {
    /// Common causal envelope.
    pub envelope: RecordEnvelope,
    /// Canonical request correlation identifier.
    pub request_correlation_id: Identifier,
    /// Resolved model repository when reported.
    pub model_repository: Option<Identifier>,
    /// Immutable resolved model revision when reported.
    pub model_revision: Option<Identifier>,
    /// Immutable tokenizer identity when reported.
    pub tokenizer: Option<ComponentIdentity>,
    /// Immutable prompt or template identity when reported.
    pub prompt_template: Option<ComponentIdentity>,
    /// Quantization or conversion artifact digest when reported.
    pub quantization_digest: Option<Sha256Digest>,
    /// Runtime image or implementation revision when reported.
    pub runtime_revision: Option<Identifier>,
    /// Digest of material runtime flags when reported.
    pub runtime_flags_digest: Option<Sha256Digest>,
    /// Hardware class at the permitted disclosure level.
    pub hardware_class: Option<Identifier>,
    /// Executor identity at the permitted disclosure level.
    pub executor_identity: Option<Identifier>,
    /// Queue, TTFT, decode, and total observations.
    pub durations: ServingDurations,
    /// Generated token count when reported.
    pub generated_tokens: Option<u64>,
    /// Cache state.
    pub cache_state: CacheState,
    /// Terminal executor status.
    pub status: ServingStatus,
    /// Bounded executor reason codes.
    pub reason_codes: Vec<ReasonCode>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(field: &'static str, value: &str) -> Identifier {
        Identifier::new(field, value).expect("fixture identifier")
    }

    fn digest() -> Sha256Digest {
        Sha256Digest::new("a".repeat(64)).expect("fixture digest")
    }

    fn component(name: &str) -> ComponentIdentity {
        ComponentIdentity {
            id: id("component_id", name),
            revision: id("component_revision", "r1"),
            digest: digest(),
        }
    }

    #[test]
    fn attempt_relations_are_explicit_and_well_formed() {
        let invalid = AttemptIdentity {
            group_id: id("attempt_group", "group-1"),
            ordinal: 0,
            relation: AttemptRelation::Fallback,
            prior_attempt_id: None,
            reason: None,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn shadow_dispatch_needs_independent_permission() {
        let mut permissions = PermissionSet::deny_all();
        permissions.executor_dispatch = PermissionDecision::Allow;
        let grant = ExecutionGrant {
            envelope: crate::test_support::envelope("grant-1", 1),
            dispatch_decision_id: id("dispatch_decision_id", "dispatch-1"),
            context_pack_id: id("context_pack_id", "context-1"),
            destination: DestinationIdentity {
                destination: id("destination", "local-executor"),
                operator_trust_domain: id("trust_domain", "developer-local"),
                peer_identity: id("peer_identity", "peer-1"),
            },
            executor: ExecutorSelection::CapabilityAlias(id("capability", "edit.fast")),
            protocol: component("protocol"),
            purpose: PredictionPurpose::ShadowEvaluation,
            visibility: VisibilityMode::Shadow,
            allowed_content_classes: vec![DataClass::P3SourceBearing],
            authorized_content: vec![
                ContentReference::new(
                    id("content_id", "context-1"),
                    id("purpose_scope", "session-1"),
                    digest(),
                    16,
                    DataClass::P3SourceBearing,
                    crate::PersistenceClass::MemoryOnly,
                )
                .expect("content"),
            ],
            protected_content: ProtectedContentResult::Clear,
            permissions,
            effective_policy_digest: digest(),
            issuer: id("issuer", "local-policy"),
            issued_at: MonotonicTick::new(5),
            lifetime: GrantLifetime::OneShot,
            decision: GrantDecision::Granted,
            reason_codes: Vec::new(),
        };

        assert_eq!(
            grant.validate(),
            Err(crate::ContractError::InvalidState(
                "shadow dispatch requires an independent shadow grant"
            ))
        );
    }

    #[test]
    fn runtime_read_and_dispatch_grants_cannot_borrow_each_others_authority() {
        let content = ContentReference::new(
            id("content_id", "buffer-1"),
            id("purpose_scope", "session-1"),
            digest(),
            16,
            DataClass::P3SourceBearing,
            crate::PersistenceClass::MemoryOnly,
        )
        .expect("content");
        let mut permissions = PermissionSet::deny_all();
        permissions.runtime_read = PermissionDecision::Allow;
        permissions.executor_dispatch = PermissionDecision::Allow;
        let read_grant = RuntimeReadGrant {
            envelope: crate::test_support::envelope("read-grant-1", 1),
            snapshot_id: id("snapshot_id", "snapshot-1"),
            authorized_content: vec![content],
            purpose: PredictionPurpose::NextEdit,
            permissions,
            protected_content: ProtectedContentResult::Clear,
            effective_policy_digest: digest(),
            issuer: id("issuer", "local-policy"),
            issued_at: MonotonicTick::new(5),
            lifetime: GrantLifetime::OneShot,
            decision: GrantDecision::Granted,
            reason_codes: Vec::new(),
        };

        assert_eq!(
            read_grant.validate(),
            Err(crate::ContractError::InvalidState(
                "runtime-read grant cannot authorize an adjacent purpose"
            ))
        );
    }

    #[test]
    fn dispatch_grant_binds_the_exact_content_class_manifest() {
        let mut permissions = PermissionSet::deny_all();
        permissions.executor_dispatch = PermissionDecision::Allow;
        let grant = ExecutionGrant {
            envelope: crate::test_support::envelope("grant-1", 1),
            dispatch_decision_id: id("dispatch_decision_id", "dispatch-1"),
            context_pack_id: id("context_pack_id", "context-1"),
            destination: DestinationIdentity {
                destination: id("destination", "local-executor"),
                operator_trust_domain: id("trust_domain", "developer-local"),
                peer_identity: id("peer_identity", "peer-1"),
            },
            executor: ExecutorSelection::CapabilityAlias(id("capability", "edit.fast")),
            protocol: component("protocol"),
            purpose: PredictionPurpose::NextEdit,
            visibility: VisibilityMode::Visible,
            allowed_content_classes: vec![DataClass::P2DerivedEditMetadata],
            authorized_content: vec![
                ContentReference::new(
                    id("content_id", "context-1"),
                    id("purpose_scope", "session-1"),
                    digest(),
                    16,
                    DataClass::P3SourceBearing,
                    crate::PersistenceClass::MemoryOnly,
                )
                .expect("content"),
            ],
            protected_content: ProtectedContentResult::Clear,
            permissions,
            effective_policy_digest: digest(),
            issuer: id("issuer", "local-policy"),
            issued_at: MonotonicTick::new(5),
            lifetime: GrantLifetime::OneShot,
            decision: GrantDecision::Granted,
            reason_codes: Vec::new(),
        };

        assert_eq!(
            grant.validate(),
            Err(crate::ContractError::AggregateMismatch(
                "allowed_content_classes"
            ))
        );
    }
}
