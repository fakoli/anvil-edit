//! Semantic domain model shared by Anvil Edit components.
//!
//! These Rust types define lifecycle meaning and critical structural
//! invariants. They deliberately contain no wire encoding, IPC transport,
//! generated bindings, database mapping, or source-bearing payload bytes.
//! Those boundaries remain independently versioned product decisions.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod candidate;
mod capture;
mod configuration;
mod dispatch;
mod document;
mod envelope;
mod outcome;
mod primitives;
mod record;

pub use candidate::{
    ApplicationAttempt, ApplicationStatus, CandidateStatus, CandidateValidation, EditOrdering,
    GestureKind, NextFocusLocation, NormalizedTextEdit, PredictionCandidate, PredictionDecision,
    PredictionDecisionKind, PresentationAttempt, PresentationMode, PresentationStatus,
    TransactionMode,
};
pub use capture::{
    ContextItem, ContextPack, ContextSourceKind, EditorSnapshot, EligibilityDecision,
    FreshnessRole, PolicySignal, PolicySignalValue, PredictionOpportunity, SnapshotCaptureClass,
    TaskContextSelection, TriggerKind,
};
pub use configuration::{
    ComponentIdentity, ConfigurationComponent, ConfigurationComponentKind, ConfigurationError,
    ConfigurationIdentity, ConfigurationLifecycleObservation, ConfigurationLifecycleState,
    ConfigurationMode, ConfigurationReconciliationObservation, ConfigurationSnapshot,
    ConfigurationSource, DesiredConfigurationProvenance, ReconciliationChecks,
    ReconciliationOutcome,
};
pub use dispatch::{
    AttemptIdentity, AttemptRelation, CacheState, DestinationIdentity, DispatchAction,
    DispatchDecision, ExecutionGrant, ExecutorSelection, ExperimentAssignment, GenerationSettings,
    GrantDecision, GrantLifetime, OutputBudget, PermissionDecision, PermissionSet,
    PredictionPurpose, PredictionRequest, ProtectedContentResult, RelativeBudgets,
    RuntimeReadGrant, ServingDurations, ServingObservation, ServingStatus, VisibilityMode,
};
pub use document::{
    AdapterDocumentIdentity, CursorSelection, DocumentRevision, DocumentRevisionRef,
    DocumentTextModel, EditorDocumentVersion, LineEnding, PositionEncoding, RangeEndSemantics,
    TerminalNewline, TextCanonicalization, TextPosition, TextRange, UriScheme,
};
pub use envelope::{CaptureMode, ProducerPosition, Provenance, RecordCorrelation, RecordEnvelope};
pub use outcome::{
    AttributionConfidence, CensoringStatus, EditDistanceMetrics, ObservedOutcome, OutcomeKind,
    SurvivalCheckpoint, SurvivalObservation, SurvivalStatus,
};
pub use primitives::{
    CheckResult, ContentReference, ContractError, ContractVersion, DataClass, DurationMicros,
    FOUNDATION_CONTRACT_VERSION, Identifier, MonotonicTick, PersistenceClass, ReasonCode,
    Sha256Digest, WallClockMicros,
};
pub use record::{LifecycleRecord, RecordKind};

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;

    pub(crate) fn identifier(field: &'static str, value: &str) -> Identifier {
        Identifier::new(field, value).expect("fixture identifier")
    }

    pub(crate) fn digest(character: char) -> Sha256Digest {
        Sha256Digest::new(character.to_string().repeat(64)).expect("fixture digest")
    }

    pub(crate) fn envelope(record_id: &str, sequence: u64) -> RecordEnvelope {
        RecordEnvelope::new(
            identifier("record_id", record_id),
            ProducerPosition::new(
                WallClockMicros::new(i64::try_from(sequence).expect("small sequence")),
                MonotonicTick::new(sequence),
                identifier("clock_id", "clock-1"),
                identifier("producer_instance_id", "fixture-producer-1"),
                sequence,
            )
            .expect("producer position"),
            identifier("idempotency_key", &format!("dedupe-{sequence}")),
            RecordCorrelation {
                session_id: identifier("session_id", "session-1"),
                repository_id: identifier("repository_id", "repository-1"),
            },
            Provenance {
                producer: identifier("producer", "contract-fixture"),
                producer_revision: identifier("producer_revision", "r1"),
                capture_mode: CaptureMode::Synthetic,
            },
        )
    }

    pub(crate) fn component(name: &str) -> ComponentIdentity {
        ComponentIdentity {
            id: identifier("component_id", name),
            revision: identifier("component_revision", "r1"),
            digest: digest('a'),
        }
    }
}
