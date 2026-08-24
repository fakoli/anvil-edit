use crate::{
    ApplicationAttempt, ConfigurationLifecycleObservation, ConfigurationReconciliationObservation,
    ConfigurationSnapshot, ContextPack, DispatchDecision, DocumentRevision, EditorSnapshot,
    ExecutionGrant, ObservedOutcome, PredictionCandidate, PredictionDecision,
    PredictionOpportunity, PredictionRequest, PresentationAttempt, RecordEnvelope,
    RuntimeReadGrant, ServingObservation, SurvivalObservation,
};

/// Stable semantic category for a durable lifecycle record.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum RecordKind {
    /// Complete immutable configuration snapshot.
    ConfigurationSnapshot,
    /// Configuration lifecycle observation.
    ConfigurationLifecycleObservation,
    /// Managed-configuration reconciliation observation.
    ConfigurationReconciliationObservation,
    /// Exact editor document revision.
    DocumentRevision,
    /// Immutable editor snapshot.
    EditorSnapshot,
    /// Prediction opportunity.
    PredictionOpportunity,
    /// Bounded compiled context pack.
    ContextPack,
    /// Local pre-context runtime-read authorization.
    RuntimeReadGrant,
    /// Pre-request policy decision.
    DispatchDecision,
    /// Pre-serialization authorization grant.
    ExecutionGrant,
    /// Explicit inference attempt.
    PredictionRequest,
    /// Executor-owned identity and timing evidence.
    ServingObservation,
    /// Normalized candidate.
    PredictionCandidate,
    /// Candidate-display policy decision.
    PredictionDecision,
    /// Adapter presentation attempt.
    PresentationAttempt,
    /// Editor conditional-application attempt.
    ApplicationAttempt,
    /// Human/editor outcome observation.
    ObservedOutcome,
    /// Durable-outcome checkpoint observation.
    SurvivalObservation,
}

/// Closed foundation union for all durable semantic lifecycle records.
///
/// A future wire union must preserve these distinctions and version itself
/// independently; this Rust enum does not choose that representation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum LifecycleRecord {
    /// Complete immutable configuration snapshot.
    ConfigurationSnapshot(ConfigurationSnapshot),
    /// Configuration lifecycle observation.
    ConfigurationLifecycleObservation(ConfigurationLifecycleObservation),
    /// Managed-configuration reconciliation observation.
    ConfigurationReconciliationObservation(ConfigurationReconciliationObservation),
    /// Exact editor document revision.
    DocumentRevision(DocumentRevision),
    /// Immutable editor snapshot.
    EditorSnapshot(EditorSnapshot),
    /// Prediction opportunity.
    PredictionOpportunity(PredictionOpportunity),
    /// Bounded compiled context pack.
    ContextPack(ContextPack),
    /// Local pre-context runtime-read authorization.
    RuntimeReadGrant(RuntimeReadGrant),
    /// Pre-request policy decision.
    DispatchDecision(DispatchDecision),
    /// Pre-serialization authorization grant.
    ExecutionGrant(ExecutionGrant),
    /// Explicit inference attempt.
    PredictionRequest(PredictionRequest),
    /// Executor-owned identity and timing evidence.
    ServingObservation(ServingObservation),
    /// Normalized candidate.
    PredictionCandidate(PredictionCandidate),
    /// Candidate-display policy decision.
    PredictionDecision(PredictionDecision),
    /// Adapter presentation attempt.
    PresentationAttempt(PresentationAttempt),
    /// Editor conditional-application attempt.
    ApplicationAttempt(ApplicationAttempt),
    /// Human/editor outcome observation.
    ObservedOutcome(ObservedOutcome),
    /// Durable-outcome checkpoint observation.
    SurvivalObservation(SurvivalObservation),
}

impl LifecycleRecord {
    /// Returns the stable semantic record category.
    #[must_use]
    pub const fn kind(&self) -> RecordKind {
        match self {
            Self::ConfigurationSnapshot(_) => RecordKind::ConfigurationSnapshot,
            Self::ConfigurationLifecycleObservation(_) => {
                RecordKind::ConfigurationLifecycleObservation
            }
            Self::ConfigurationReconciliationObservation(_) => {
                RecordKind::ConfigurationReconciliationObservation
            }
            Self::DocumentRevision(_) => RecordKind::DocumentRevision,
            Self::EditorSnapshot(_) => RecordKind::EditorSnapshot,
            Self::PredictionOpportunity(_) => RecordKind::PredictionOpportunity,
            Self::ContextPack(_) => RecordKind::ContextPack,
            Self::RuntimeReadGrant(_) => RecordKind::RuntimeReadGrant,
            Self::DispatchDecision(_) => RecordKind::DispatchDecision,
            Self::ExecutionGrant(_) => RecordKind::ExecutionGrant,
            Self::PredictionRequest(_) => RecordKind::PredictionRequest,
            Self::ServingObservation(_) => RecordKind::ServingObservation,
            Self::PredictionCandidate(_) => RecordKind::PredictionCandidate,
            Self::PredictionDecision(_) => RecordKind::PredictionDecision,
            Self::PresentationAttempt(_) => RecordKind::PresentationAttempt,
            Self::ApplicationAttempt(_) => RecordKind::ApplicationAttempt,
            Self::ObservedOutcome(_) => RecordKind::ObservedOutcome,
            Self::SurvivalObservation(_) => RecordKind::SurvivalObservation,
        }
    }

    /// Returns the common causal envelope without collapsing record semantics.
    #[must_use]
    pub const fn envelope(&self) -> &RecordEnvelope {
        match self {
            Self::ConfigurationSnapshot(value) => value.envelope(),
            Self::ConfigurationLifecycleObservation(value) => &value.envelope,
            Self::ConfigurationReconciliationObservation(value) => &value.envelope,
            Self::DocumentRevision(value) => &value.envelope,
            Self::EditorSnapshot(value) => &value.envelope,
            Self::PredictionOpportunity(value) => &value.envelope,
            Self::ContextPack(value) => &value.envelope,
            Self::RuntimeReadGrant(value) => &value.envelope,
            Self::DispatchDecision(value) => &value.envelope,
            Self::ExecutionGrant(value) => &value.envelope,
            Self::PredictionRequest(value) => &value.envelope,
            Self::ServingObservation(value) => &value.envelope,
            Self::PredictionCandidate(value) => &value.envelope,
            Self::PredictionDecision(value) => &value.envelope,
            Self::PresentationAttempt(value) => &value.envelope,
            Self::ApplicationAttempt(value) => &value.envelope,
            Self::ObservedOutcome(value) => &value.envelope,
            Self::SurvivalObservation(value) => &value.envelope,
        }
    }
}
