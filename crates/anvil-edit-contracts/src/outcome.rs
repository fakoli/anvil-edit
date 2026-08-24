use crate::{
    DocumentRevisionRef, DurationMicros, Identifier, MonotonicTick, RecordEnvelope, Sha256Digest,
};

/// Human or editor outcome attributed to a presentation or application.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum OutcomeKind {
    /// Complete candidate acceptance gesture.
    Accepted,
    /// Bounded subset accepted.
    PartiallyAccepted,
    /// Suggestion dismissed without an explicit quality judgment.
    Dismissed,
    /// Suggestion remained unacted on until superseded under a declared policy.
    IgnoredUntilSuperseded,
    /// Developer explicitly rejected the suggestion.
    ExplicitlyRejected,
    /// Attributable accepted edit was undone.
    Undone,
    /// Attributable accepted content was substantially rewritten.
    Rewritten,
    /// Attributable content was present at a save observation.
    Saved,
    /// Attributable content was correlated with a commit.
    CommitCorrelated,
    /// Later editor state no longer permits reliable attribution.
    AttributionLost,
}

/// Confidence assigned by a declared attribution policy.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AttributionConfidence {
    /// Direct editor event and exact revision evidence.
    Exact,
    /// Strong bounded inference under the declared window.
    Probable,
    /// Multiple causes remain plausible.
    Ambiguous,
}

/// Source-free distance measures for accepted or retained edits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EditDistanceMetrics {
    /// Normalized changed character count when permitted.
    pub changed_characters: u64,
    /// Normalized changed token count when permitted.
    pub changed_tokens: Option<u64>,
}

/// Append-only human/editor observation linked to an attempted interaction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservedOutcome {
    /// Common causal envelope.
    pub envelope: RecordEnvelope,
    /// Source presentation attempt when applicable.
    pub presentation_id: Option<Identifier>,
    /// Source application attempt when applicable.
    pub application_id: Option<Identifier>,
    /// Candidate being observed when applicable.
    pub candidate_id: Option<Identifier>,
    /// Outcome classification.
    pub kind: OutcomeKind,
    /// Document revision before the attributable action.
    pub revision_before: Option<DocumentRevisionRef>,
    /// Document revision after the attributable action.
    pub revision_after: Option<DocumentRevisionRef>,
    /// Observation tick on the producer's monotonic clock.
    pub observed_at: MonotonicTick,
    /// Immutable attribution-window and policy identity.
    pub attribution_policy: crate::ComponentIdentity,
    /// Confidence or ambiguity of attribution.
    pub attribution_confidence: AttributionConfidence,
    /// Accepted or retained content digest when policy permits it.
    pub retained_content_digest: Option<Sha256Digest>,
    /// Source-free distance measures when policy permits them.
    pub distance: Option<EditDistanceMetrics>,
}

impl ObservedOutcome {
    /// Validates that an outcome has an attributable lifecycle anchor.
    pub fn validate(&self) -> Result<(), crate::ContractError> {
        if self.presentation_id.is_none()
            && self.application_id.is_none()
            && self.candidate_id.is_none()
        {
            return Err(crate::ContractError::InvalidState(
                "observed outcome must name a presentation, application, or candidate",
            ));
        }
        if let Some(revision) = &self.revision_before {
            revision.validate()?;
        }
        if let Some(revision) = &self.revision_after {
            revision.validate()?;
        }
        Ok(())
    }
}

/// Definition of an outcome-survival checkpoint.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum SurvivalCheckpoint {
    /// Fixed duration after successful application.
    After(DurationMicros),
    /// Next attributable save.
    Save,
    /// Later commit correlation under a declared policy.
    Commit,
}

/// Why an intended survival observation could not be fully observed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum CensoringStatus {
    /// Checkpoint was fully observed.
    Observed,
    /// Session ended or observation window closed before the checkpoint.
    RightCensored,
    /// Attribution was lost before the checkpoint.
    AttributionLost,
}

/// Retention result observed at a declared survival checkpoint.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum SurvivalStatus {
    /// Accepted content remained substantially present.
    Survived,
    /// Accepted content remained but was substantially rewritten.
    Rewritten,
    /// Accepted content was removed.
    Removed,
    /// Checkpoint was right-censored.
    RightCensored,
    /// Attribution could not be maintained.
    AttributionLost,
}

/// Append-only survival evidence attached to a successful application.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurvivalObservation {
    /// Common causal envelope.
    pub envelope: RecordEnvelope,
    /// Source application attempt.
    pub application_id: Identifier,
    /// Source candidate.
    pub candidate_id: Identifier,
    /// Checkpoint definition.
    pub checkpoint: SurvivalCheckpoint,
    /// Scheduled checkpoint tick when duration-based.
    pub scheduled_at: Option<MonotonicTick>,
    /// Actual observation tick.
    pub observed_at: MonotonicTick,
    /// Censoring state.
    pub censoring: CensoringStatus,
    /// Retained-content digest when permitted.
    pub retained_content_digest: Option<Sha256Digest>,
    /// Source-free retained-content distance when permitted.
    pub distance: Option<EditDistanceMetrics>,
    /// Save or commit correlation identity when permitted and available.
    pub downstream_correlation_id: Option<Identifier>,
    /// Confidence assigned to downstream correlation.
    pub correlation_confidence: Option<AttributionConfidence>,
    /// Terminal survival state.
    pub status: SurvivalStatus,
}

impl SurvivalObservation {
    /// Validates checkpoint scheduling, censoring, and correlation shape.
    pub fn validate(&self) -> Result<(), crate::ContractError> {
        let duration_checkpoint = matches!(self.checkpoint, SurvivalCheckpoint::After(_));
        if duration_checkpoint != self.scheduled_at.is_some() {
            return Err(crate::ContractError::InvalidState(
                "only duration checkpoints carry a scheduled monotonic tick",
            ));
        }
        let censoring_matches = matches!(
            (self.censoring, self.status),
            (CensoringStatus::Observed, SurvivalStatus::Survived)
                | (CensoringStatus::Observed, SurvivalStatus::Rewritten)
                | (CensoringStatus::Observed, SurvivalStatus::Removed)
                | (
                    CensoringStatus::RightCensored,
                    SurvivalStatus::RightCensored
                )
                | (
                    CensoringStatus::AttributionLost,
                    SurvivalStatus::AttributionLost
                )
        );
        if !censoring_matches {
            return Err(crate::ContractError::InvalidState(
                "survival status and censoring state must agree",
            ));
        }
        if self.downstream_correlation_id.is_some() != self.correlation_confidence.is_some() {
            return Err(crate::ContractError::InvalidState(
                "downstream correlation identity and confidence must appear together",
            ));
        }
        Ok(())
    }
}
