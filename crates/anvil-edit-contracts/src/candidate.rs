use std::collections::BTreeSet;

use crate::{
    CheckResult, ContentReference, ContractError, DataClass, DocumentRevisionRef, DurationMicros,
    Identifier, MonotonicTick, PositionEncoding, RangeEndSemantics, ReasonCode, RecordEnvelope,
    Sha256Digest, TextPosition, TextRange,
};

/// One normalized replacement against an exact base revision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizedTextEdit {
    /// Exact base document revision and digest.
    pub base_document: DocumentRevisionRef,
    /// Half-open replacement range in the declared encoding.
    pub range: TextRange,
    /// Position encoding for the range.
    pub position_encoding: PositionEncoding,
    /// Range endpoint semantics.
    pub range_end_semantics: RangeEndSemantics,
    /// Source-free handle to replacement bytes.
    pub replacement: ContentReference,
}

impl NormalizedTextEdit {
    /// Validates that replacement bytes are classified as source-bearing or protected.
    pub fn validate(&self) -> Result<(), ContractError> {
        self.base_document.validate()?;
        if matches!(
            self.replacement.data_class(),
            DataClass::P3SourceBearing | DataClass::P4HighlySensitive
        ) {
            if self.position_encoding != self.base_document.text_model.position_encoding {
                return Err(ContractError::InvalidState(
                    "edit position encoding must match its base document revision",
                ));
            }
            if self.range_end_semantics != self.base_document.text_model.range_end_semantics {
                return Err(ContractError::InvalidState(
                    "edit range semantics must match its base document revision",
                ));
            }
            Ok(())
        } else {
            Err(ContractError::InvalidState(
                "replacement content must be classified P3 or P4",
            ))
        }
    }
}

/// Optional next focus location proposed with a candidate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NextFocusLocation {
    /// Target document revision.
    pub document: DocumentRevisionRef,
    /// Proposed focus position.
    pub position: TextPosition,
}

/// Deterministic interpretation of multiple edits against one base revision.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EditOrdering {
    /// Every range addresses the unchanged base revision; edits are listed in
    /// semantic result order when equal-position insertions occur.
    BaseRelativeAsListed,
}

/// Bounded validation results for one normalized candidate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateValidation {
    /// Native protocol parse result.
    pub protocol_parse: CheckResult,
    /// Range and overlap validation.
    pub ranges: CheckResult,
    /// Configured target-scope validation.
    pub scope: CheckResult,
    /// Syntax validation when available.
    pub syntax: CheckResult,
    /// Diagnostic impact validation when available.
    pub diagnostics: CheckResult,
    /// Secret and protected-content validation.
    pub protected_content: CheckResult,
    /// Unsafe control and bidirectional-character validation.
    pub unicode_controls: CheckResult,
}

/// Terminal state of candidate generation and normalization.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum CandidateStatus {
    /// Candidate contains one or more valid normalized edits.
    Valid,
    /// Native output produced no edit.
    Empty,
    /// Native output violated the protocol contract.
    InvalidProtocol,
    /// One or more ranges were invalid.
    InvalidRange,
    /// Syntax validation failed.
    InvalidSyntax,
    /// Candidate targeted content outside the allowed scope.
    OutOfScope,
    /// Base or critical dependency revision was stale.
    Stale,
    /// Work terminated through cancellation.
    Cancelled,
    /// Render deadline expired.
    Expired,
    /// Generation or normalization failed for another bounded reason.
    Failed,
}

/// Normalized proposed change independent of a model-native response format.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PredictionCandidate {
    /// Common causal envelope.
    pub envelope: RecordEnvelope,
    /// Source inference request.
    pub request_id: Identifier,
    /// Ordered, non-overlapping normalized edits.
    pub edits: Vec<NormalizedTextEdit>,
    /// Deterministic interpretation of the ordered edit list.
    pub edit_ordering: EditOrdering,
    /// Optional proposed next focus location.
    pub next_focus: Option<NextFocusLocation>,
    /// Digest of model-native output when retention policy permits the digest.
    pub native_output_digest: Option<Sha256Digest>,
    /// Bounded validation results.
    pub validation: CandidateValidation,
    /// Generation-completion tick on the declared clock.
    pub generated_at: Option<MonotonicTick>,
    /// Normalization-completion tick on the same clock.
    pub normalized_at: MonotonicTick,
    /// Terminal candidate state.
    pub status: CandidateStatus,
    /// Bounded normalization and validation reasons.
    pub reason_codes: Vec<ReasonCode>,
}

impl PredictionCandidate {
    /// Enforces v0 same-document and non-overlap rules for valid candidates.
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.status == CandidateStatus::Valid && self.edits.is_empty() {
            return Err(ContractError::InvalidState(
                "valid candidate must contain at least one edit",
            ));
        }
        for edit in &self.edits {
            edit.validate()?;
        }
        let Some(first) = self.edits.first() else {
            return Ok(());
        };
        if self
            .edits
            .iter()
            .any(|edit| edit.base_document != first.base_document)
        {
            return Err(ContractError::MixedDocumentEdits);
        }

        let mut ranges = self
            .edits
            .iter()
            .map(|edit| &edit.range)
            .collect::<Vec<_>>();
        ranges.sort_by_key(|range| (range.start(), range.end()));
        if ranges.windows(2).any(|pair| pair[0].overlaps(pair[1])) {
            return Err(ContractError::OverlappingEdits);
        }
        Ok(())
    }
}

/// Display-policy action for a candidate.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PredictionDecisionKind {
    /// Ask the adapter to present the selected candidate.
    Show,
    /// Suppress a valid candidate.
    Suppress,
    /// No candidate was available for consideration.
    NoCandidate,
    /// Candidate was stale before presentation.
    Stale,
    /// Render deadline expired before presentation.
    Expired,
}

/// Presentation capability requested from an editor adapter.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PresentationMode {
    /// Inline preview at the current cursor.
    Inline,
    /// Local diff preview.
    LocalDiff,
    /// Location indicator before preview.
    NextLocation,
    /// Explicit cross-file navigation and diff review.
    CrossFilePreview,
    /// Hidden prediction revealed only by explicit action.
    ExplicitReveal,
}

/// Display-policy decision kept distinct from presentation evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PredictionDecision {
    /// Common causal envelope.
    pub envelope: RecordEnvelope,
    /// Source opportunity.
    pub opportunity_id: Identifier,
    /// Candidate identifiers considered by the policy.
    pub considered_candidate_ids: Vec<Identifier>,
    /// Candidate selected for presentation, present only for `Show`.
    pub selected_candidate_id: Option<Identifier>,
    /// Immutable display-policy identity.
    pub decision_policy: crate::ComponentIdentity,
    /// Terminal display-policy action.
    pub decision: PredictionDecisionKind,
    /// Bounded source-free reasons.
    pub reason_codes: Vec<ReasonCode>,
    /// Source-free score names and scaled integer values.
    pub scores: Vec<crate::PolicySignal>,
    /// Requested presentation mode when showing a candidate.
    pub presentation_mode: Option<PresentationMode>,
    /// Decision tick on the producer's monotonic clock.
    pub decided_at: MonotonicTick,
    /// Remaining render budget.
    pub remaining_budget: DurationMicros,
    /// Optional experiment assignment.
    pub experiment: Option<crate::ExperimentAssignment>,
}

impl PredictionDecision {
    /// Validates candidate selection and presentation intent.
    pub fn validate(&self) -> Result<(), ContractError> {
        let unique = self
            .considered_candidate_ids
            .iter()
            .collect::<BTreeSet<_>>();
        if unique.len() != self.considered_candidate_ids.len() {
            return Err(ContractError::DuplicateReference(
                "considered_candidate_ids",
            ));
        }

        if self.decision == PredictionDecisionKind::Show {
            let selected =
                self.selected_candidate_id
                    .as_ref()
                    .ok_or(ContractError::InvalidState(
                        "show decision must select a candidate",
                    ))?;
            if !self.considered_candidate_ids.contains(selected) {
                return Err(ContractError::InvalidState(
                    "selected candidate must be one of the considered candidates",
                ));
            }
            if self.presentation_mode.is_none() {
                return Err(ContractError::InvalidState(
                    "show decision must select a presentation mode",
                ));
            }
        } else if self.selected_candidate_id.is_some() || self.presentation_mode.is_some() {
            return Err(ContractError::InvalidState(
                "non-show decision cannot select a candidate or presentation mode",
            ));
        }

        if self.decision == PredictionDecisionKind::NoCandidate
            && !self.considered_candidate_ids.is_empty()
        {
            return Err(ContractError::InvalidState(
                "no-candidate decision cannot name considered candidates",
            ));
        }
        Ok(())
    }
}

/// Terminal result of an adapter presentation attempt.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PresentationStatus {
    /// Adapter successfully rendered the suggestion.
    Displayed,
    /// Adapter does not support the requested mode.
    Unsupported,
    /// Target or display-critical dependency was stale.
    Stale,
    /// Render deadline expired.
    Expired,
    /// Adapter failed to render.
    Failed,
    /// Adapter policy independently suppressed presentation.
    SuppressedByAdapter,
}

/// What an adapter actually attempted to render after a show decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PresentationAttempt {
    /// Common causal envelope.
    pub envelope: RecordEnvelope,
    /// Source display decision.
    pub decision_id: Identifier,
    /// Selected candidate.
    pub candidate_id: Identifier,
    /// Adapter capability identity used for the attempt.
    pub adapter_capability: Identifier,
    /// Requested presentation mode.
    pub requested_mode: PresentationMode,
    /// Fresh target revisions checked before rendering.
    pub target_revisions: Vec<DocumentRevisionRef>,
    /// Display-critical dependency revisions checked before rendering.
    pub display_critical_revisions: Vec<DocumentRevisionRef>,
    /// Attempt tick on the adapter's monotonic clock.
    pub attempted_at: MonotonicTick,
    /// Adapter render duration.
    pub render_duration: DurationMicros,
    /// Remaining deadline budget after rendering.
    pub remaining_budget: DurationMicros,
    /// Terminal adapter result.
    pub status: PresentationStatus,
    /// Bounded adapter reason.
    pub reason_code: Option<ReasonCode>,
    /// Digest of rendered content when policy permits it.
    pub rendered_content_digest: Option<Sha256Digest>,
}

/// User gesture attributed to a candidate presentation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum GestureKind {
    /// Accept the complete suggestion.
    AcceptAll,
    /// Accept a bounded subset of the suggestion.
    AcceptPartial,
    /// Explicitly reject the suggestion.
    Reject,
    /// Dismiss without an explicit quality judgment.
    Dismiss,
}

/// Transaction semantics requested from an editor adapter.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum TransactionMode {
    /// v0 compare-and-apply against one exact document revision.
    SingleDocumentConditional,
    /// Future editor-proven atomic compare-and-apply across documents.
    MultiDocumentAtomic,
    /// Explicit one-document-at-a-time review flow.
    PerDocumentReview,
}

/// Terminal result of an editor application attempt.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ApplicationStatus {
    /// Every attempted edit was applied.
    Applied,
    /// Some edits were applied in a declared non-atomic transaction.
    PartiallyApplied,
    /// Expected target revision did not match.
    Stale,
    /// Policy or user authority denied application.
    Denied,
    /// Adapter cannot provide the requested transaction.
    Unsupported,
    /// Adapter failed during the attempt.
    Failed,
}

/// Editor-owned conditional transaction after an attributable user gesture.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplicationAttempt {
    /// Common causal envelope.
    pub envelope: RecordEnvelope,
    /// Source presentation attempt.
    pub presentation_id: Identifier,
    /// Source display decision.
    pub decision_id: Identifier,
    /// Selected candidate.
    pub candidate_id: Identifier,
    /// Attributed user gesture.
    pub gesture: GestureKind,
    /// Immutable attribution-policy identity.
    pub attribution_policy: crate::ComponentIdentity,
    /// Exact expected target revisions.
    pub expected_revisions: Vec<DocumentRevisionRef>,
    /// Declared editor transaction semantics.
    pub transaction_mode: TransactionMode,
    /// Ordered replacement digests attempted.
    pub attempted_edit_digests: Vec<Sha256Digest>,
    /// Post-application revisions observed by the adapter.
    pub resulting_revisions: Vec<DocumentRevisionRef>,
    /// Terminal transaction result.
    pub status: ApplicationStatus,
    /// Bounded adapter reason codes.
    pub reason_codes: Vec<ReasonCode>,
}

impl ApplicationAttempt {
    /// Enforces the v0 one-document conditional-application boundary.
    pub fn validate_v0(&self) -> Result<(), ContractError> {
        if !matches!(
            self.gesture,
            GestureKind::AcceptAll | GestureKind::AcceptPartial
        ) {
            return Err(ContractError::InvalidState(
                "application attempt requires an acceptance gesture",
            ));
        }
        if self.transaction_mode != TransactionMode::SingleDocumentConditional {
            return Err(ContractError::InvalidState(
                "v0 supports only single-document conditional application",
            ));
        }
        if self.expected_revisions.len() != 1 {
            return Err(ContractError::InvalidState(
                "single-document transaction requires exactly one expected revision",
            ));
        }
        self.expected_revisions[0].validate()?;
        for revision in &self.resulting_revisions {
            revision.validate()?;
        }
        if self.attempted_edit_digests.is_empty() {
            return Err(ContractError::InvalidState(
                "application attempt must name at least one replacement digest",
            ));
        }
        if self.status == ApplicationStatus::PartiallyApplied {
            return Err(ContractError::InvalidState(
                "single-document conditional application cannot report partial success",
            ));
        }
        if self.status == ApplicationStatus::Applied && self.resulting_revisions.len() != 1 {
            return Err(ContractError::InvalidState(
                "applied single-document transaction requires one resulting revision",
            ));
        }
        if self.status == ApplicationStatus::Applied
            && self.resulting_revisions[0].document != self.expected_revisions[0].document
        {
            return Err(ContractError::InvalidState(
                "resulting revision must preserve the expected document identity",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DataClass, PersistenceClass};

    fn id(field: &'static str, value: &str) -> Identifier {
        Identifier::new(field, value).expect("fixture identifier")
    }

    fn digest(character: char) -> Sha256Digest {
        Sha256Digest::new(character.to_string().repeat(64)).expect("fixture digest")
    }

    fn replacement(name: &str) -> ContentReference {
        ContentReference::new(
            id("content_id", name),
            id("purpose_scope", "session-1"),
            digest('c'),
            2,
            DataClass::P3SourceBearing,
            PersistenceClass::MemoryOnly,
        )
        .expect("replacement")
    }

    fn document_ref() -> DocumentRevisionRef {
        DocumentRevisionRef {
            id: id("document_revision_id", "document-r1"),
            document: crate::AdapterDocumentIdentity {
                adapter_type: id("adapter_type", "fixture-editor"),
                adapter_instance: id("adapter_instance", "adapter-1"),
                workspace_instance: id("workspace_instance", "workspace-1"),
                uri_scheme: crate::UriScheme::new("file").expect("scheme"),
                logical_uri: ContentReference::new(
                    id("content_id", "logical-uri-1"),
                    id("purpose_scope", "session-1"),
                    digest('d'),
                    24,
                    DataClass::P3SourceBearing,
                    PersistenceClass::MemoryOnly,
                )
                .expect("logical URI"),
                document_incarnation: id("document_incarnation", "incarnation-1"),
            },
            editor_version: crate::EditorDocumentVersion {
                namespace: id("version_namespace", "fixture-buffer"),
                value: id("editor_version", "1"),
            },
            text_model: crate::DocumentTextModel {
                position_encoding: PositionEncoding::Utf16,
                line_ending: crate::LineEnding::Lf,
                terminal_newline: crate::TerminalNewline::Present,
                range_end_semantics: RangeEndSemantics::HalfOpen,
                canonicalization: crate::TextCanonicalization::Utf8Bytes,
            },
            full_buffer_byte_length: 12,
            full_buffer_digest: digest('a'),
        }
    }

    fn edit(start: u32, end: u32) -> NormalizedTextEdit {
        NormalizedTextEdit {
            base_document: document_ref(),
            range: TextRange::new(
                TextPosition {
                    line: 0,
                    character: start,
                },
                TextPosition {
                    line: 0,
                    character: end,
                },
            )
            .expect("range"),
            position_encoding: PositionEncoding::Utf16,
            range_end_semantics: RangeEndSemantics::HalfOpen,
            replacement: replacement(&format!("replacement-{start}")),
        }
    }

    fn candidate(edits: Vec<NormalizedTextEdit>) -> PredictionCandidate {
        PredictionCandidate {
            envelope: crate::test_support::envelope("candidate-1", 1),
            request_id: id("request_id", "request-1"),
            edits,
            edit_ordering: EditOrdering::BaseRelativeAsListed,
            next_focus: None,
            native_output_digest: None,
            validation: CandidateValidation {
                protocol_parse: CheckResult::Passed,
                ranges: CheckResult::Passed,
                scope: CheckResult::Passed,
                syntax: CheckResult::NotRun,
                diagnostics: CheckResult::NotRun,
                protected_content: CheckResult::Passed,
                unicode_controls: CheckResult::Passed,
            },
            generated_at: Some(MonotonicTick::new(10)),
            normalized_at: MonotonicTick::new(11),
            status: CandidateStatus::Valid,
            reason_codes: Vec::new(),
        }
    }

    #[test]
    fn rejects_overlapping_normalized_edits() {
        assert_eq!(
            candidate(vec![edit(0, 3), edit(2, 4)]).validate(),
            Err(ContractError::OverlappingEdits)
        );
        assert!(candidate(vec![edit(0, 2), edit(2, 4)]).validate().is_ok());
    }

    #[test]
    fn rejects_edit_coordinates_that_do_not_match_the_exact_revision() {
        let mut mismatched = edit(0, 2);
        mismatched.position_encoding = PositionEncoding::Utf8;

        assert_eq!(
            candidate(vec![mismatched]).validate(),
            Err(ContractError::InvalidState(
                "edit position encoding must match its base document revision"
            ))
        );
    }

    #[test]
    fn v0_application_cannot_imply_multi_document_or_partial_atomicity() {
        let attempt = ApplicationAttempt {
            envelope: crate::test_support::envelope("apply-1", 1),
            presentation_id: id("presentation_id", "presentation-1"),
            decision_id: id("decision_id", "decision-1"),
            candidate_id: id("candidate_id", "candidate-1"),
            gesture: GestureKind::AcceptAll,
            attribution_policy: crate::test_support::component("attribution-policy"),
            expected_revisions: vec![document_ref()],
            transaction_mode: TransactionMode::SingleDocumentConditional,
            attempted_edit_digests: vec![digest('c')],
            resulting_revisions: Vec::new(),
            status: ApplicationStatus::PartiallyApplied,
            reason_codes: Vec::new(),
        };

        assert_eq!(
            attempt.validate_v0(),
            Err(ContractError::InvalidState(
                "single-document conditional application cannot report partial success"
            ))
        );
    }
}
