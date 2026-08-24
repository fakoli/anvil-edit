use std::collections::BTreeSet;

use crate::{
    ComponentIdentity, ContentReference, ContractError, DocumentRevision, DocumentRevisionRef,
    DurationMicros, Identifier, MonotonicTick, PersistenceClass, ReasonCode, RecordEnvelope,
    Sha256Digest, TextRange,
};

/// Capture mode for one editor snapshot.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum SnapshotCaptureClass {
    /// Snapshot and source handles exist only for the bounded live operation.
    Ephemeral,
    /// Source-free snapshot metadata may be retained.
    MetadataOnly,
    /// Governed source-bearing capture was separately enabled.
    GovernedSourceEnabled,
}

/// Immutable editor state from which prediction work may be constructed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditorSnapshot {
    /// Common causal envelope.
    pub envelope: RecordEnvelope,
    /// Exact active document revision.
    pub active_document: DocumentRevision,
    /// Ordered cursor and selection state.
    pub selections: Vec<crate::CursorSelection>,
    /// Editor language identifier.
    pub language_id: Identifier,
    /// Visible editor ranges.
    pub visible_ranges: Vec<TextRange>,
    /// Ordered recent edit record identifiers.
    pub recent_edit_ids: Vec<Identifier>,
    /// Permitted open or recently visited document revisions.
    pub related_documents: Vec<DocumentRevisionRef>,
    /// Permitted diagnostics or semantic-reference handles.
    pub semantic_inputs: Vec<ContentReference>,
    /// Content handle for every source-bearing input available to this snapshot.
    pub content_inputs: Vec<ContentReference>,
    /// Snapshot capture and retention class.
    pub capture_class: SnapshotCaptureClass,
    /// Permitted persistence of source-bearing snapshot bytes.
    pub source_persistence: PersistenceClass,
}

impl EditorSnapshot {
    /// Validates exact editor state and source-retention boundaries.
    pub fn validate(&self) -> Result<(), ContractError> {
        self.active_document.validate()?;
        if self.selections.is_empty()
            || self
                .selections
                .iter()
                .filter(|selection| selection.primary)
                .count()
                != 1
        {
            return Err(ContractError::InvalidState(
                "editor snapshot must contain exactly one primary selection",
            ));
        }
        if self.capture_class != SnapshotCaptureClass::GovernedSourceEnabled
            && self.source_persistence != PersistenceClass::MemoryOnly
        {
            return Err(ContractError::InvalidState(
                "ephemeral or metadata-only capture cannot persist source bytes",
            ));
        }
        for revision in &self.related_documents {
            revision.validate()?;
        }
        for (field, content) in [
            ("content_inputs", &self.content_inputs),
            ("semantic_inputs", &self.semantic_inputs),
        ] {
            let unique = content
                .iter()
                .map(|item| (item.purpose_scope(), item.id()))
                .collect::<BTreeSet<_>>();
            if unique.len() != content.len() {
                return Err(ContractError::DuplicateReference(field));
            }
        }
        Ok(())
    }
}

/// Reason an adapter emitted a prediction opportunity.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum TriggerKind {
    /// Bounded typing pause.
    TypingPause,
    /// A prior edit was applied.
    EditApplied,
    /// Cursor or selection moved materially.
    CursorJump,
    /// Diagnostics changed.
    DiagnosticChange,
    /// Developer explicitly requested a hidden prediction.
    ExplicitReveal,
}

/// Cheap gate outcome for one opportunity.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EligibilityDecision {
    /// The opportunity may continue to context compilation.
    Eligible,
    /// The opportunity terminates before context compilation.
    Ineligible,
}

/// Source-free scalar captured by the cheap opportunity gate.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PolicySignalValue {
    /// Boolean gate signal.
    Boolean(bool),
    /// Signed integer gate signal.
    Signed(i64),
    /// Unsigned integer gate signal.
    Unsigned(u64),
    /// Bounded categorical value.
    Category(Identifier),
}

/// Named source-free signal evaluated by the opportunity gate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicySignal {
    /// Signal identity.
    pub name: Identifier,
    /// Signal value.
    pub value: PolicySignalValue,
}

/// Immutable observation that an editor state may warrant prediction work.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PredictionOpportunity {
    /// Common causal envelope.
    pub envelope: RecordEnvelope,
    /// Source editor snapshot.
    pub snapshot_id: Identifier,
    /// Trigger classification.
    pub trigger: TriggerKind,
    /// Trigger tick on the producer's monotonic clock.
    pub triggered_at: MonotonicTick,
    /// Remaining opportunity-to-render budget.
    pub render_budget: DurationMicros,
    /// Cheap eligibility outcome.
    pub eligibility: EligibilityDecision,
    /// Bounded source-free reasons for eligibility or abstention.
    pub reason_codes: Vec<ReasonCode>,
    /// Prior opportunity superseded by this one, when any.
    pub superseded_opportunity_id: Option<Identifier>,
    /// Cheap policy signals used by the gate.
    pub signals: Vec<PolicySignal>,
}

/// Origin category for one selected context item.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ContextSourceKind {
    /// Active buffer prefix or suffix.
    ActiveDocument,
    /// Recent edit history.
    RecentEdit,
    /// Visible or recently visited document region.
    RelatedDocument,
    /// Language-service definition or reference.
    SemanticReference,
    /// Diagnostic information.
    Diagnostic,
    /// Bounded optional Anvil task context.
    TaskContext,
}

/// Required response when a context dependency changes.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FreshnessRole {
    /// Changed dependency makes application stale.
    ApplicationCritical,
    /// Changed dependency suppresses presentation.
    DisplayCritical,
    /// Changed dependency is recorded as drift only.
    Advisory,
}

/// One bounded, reasoned context input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextItem {
    /// Context source category.
    pub source_kind: ContextSourceKind,
    /// Source-free handle to selected content.
    pub content: ContentReference,
    /// Bounded reason the item was included.
    pub inclusion_reason: ReasonCode,
    /// Model-token count under the selected tokenizer/protocol.
    pub token_count: u64,
    /// Exact source revision when the item comes from a document.
    pub source_revision: Option<DocumentRevisionRef>,
    /// Freshness behavior required downstream.
    pub freshness_role: FreshnessRole,
}

/// Optional bounded task fields selected without embedding their source text.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskContextSelection {
    /// Digest of the complete permitted task input.
    pub task_context_digest: Sha256Digest,
    /// Names of fields selected into context.
    pub selected_fields: Vec<Identifier>,
}

/// A bounded request input compiled under one immutable context policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextPack {
    /// Common causal envelope.
    pub envelope: RecordEnvelope,
    /// Source snapshot identifier.
    pub snapshot_id: Identifier,
    /// Pre-context local runtime-read authorization consumed by compilation.
    pub runtime_read_grant_id: Identifier,
    /// Exact active document revision.
    pub active_document: DocumentRevisionRef,
    /// Immutable context-policy identity.
    pub context_policy: ComponentIdentity,
    /// Ordered context items.
    pub items: Vec<ContextItem>,
    /// Declared total model-token count.
    pub total_tokens: u64,
    /// Declared total content byte count.
    pub total_bytes: u64,
    /// Redaction and omission reason codes.
    pub redaction_results: Vec<ReasonCode>,
    /// Optional bounded task-context selection.
    pub task_context: Option<TaskContextSelection>,
}

impl ContextPack {
    /// Verifies that declared byte and token totals equal the selected items.
    pub fn validate_totals(&self) -> Result<(), ContractError> {
        self.active_document.validate()?;
        for item in &self.items {
            if !matches!(
                item.content.data_class(),
                crate::DataClass::P3SourceBearing | crate::DataClass::P4HighlySensitive
            ) {
                return Err(ContractError::InvalidState(
                    "model context content must be classified P3 or P4",
                ));
            }
            if item.source_kind != ContextSourceKind::TaskContext && item.source_revision.is_none()
            {
                return Err(ContractError::InvalidState(
                    "document-derived context must name its exact source revision",
                ));
            }
        }
        let tokens = self
            .items
            .iter()
            .try_fold(0_u64, |total, item| total.checked_add(item.token_count));
        if tokens != Some(self.total_tokens) {
            return Err(ContractError::AggregateMismatch("total_tokens"));
        }

        let bytes = self.items.iter().try_fold(0_u64, |total, item| {
            total.checked_add(item.content.byte_length())
        });
        if bytes != Some(self.total_bytes) {
            return Err(ContractError::AggregateMismatch("total_bytes"));
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
                position_encoding: crate::PositionEncoding::Utf16,
                line_ending: crate::LineEnding::Lf,
                terminal_newline: crate::TerminalNewline::Present,
                range_end_semantics: crate::RangeEndSemantics::HalfOpen,
                canonicalization: crate::TextCanonicalization::Utf8Bytes,
            },
            full_buffer_byte_length: 12,
            full_buffer_digest: digest('b'),
        }
    }

    #[test]
    fn context_pack_rejects_incorrect_aggregate_counts() {
        let content = ContentReference::new(
            id("content_id", "context-1"),
            id("purpose_scope", "session-1"),
            digest('a'),
            12,
            DataClass::P3SourceBearing,
            PersistenceClass::MemoryOnly,
        )
        .expect("content reference");

        let item = ContextItem {
            source_kind: ContextSourceKind::ActiveDocument,
            content,
            inclusion_reason: ReasonCode::new("active-window").expect("reason"),
            token_count: 3,
            source_revision: Some(document_ref()),
            freshness_role: FreshnessRole::ApplicationCritical,
        };

        // The envelope and remaining identifiers are irrelevant to aggregate validation.
        let pack = ContextPack {
            envelope: crate::test_support::envelope("context-pack-1", 1),
            snapshot_id: id("snapshot_id", "snapshot-1"),
            runtime_read_grant_id: id("runtime_read_grant_id", "read-grant-1"),
            active_document: document_ref(),
            context_policy: ComponentIdentity {
                id: id("component_id", "context-policy"),
                revision: id("component_revision", "r1"),
                digest: digest('c'),
            },
            items: vec![item],
            total_tokens: 4,
            total_bytes: 12,
            redaction_results: Vec::new(),
            task_context: None,
        };

        assert_eq!(
            pack.validate_totals(),
            Err(ContractError::AggregateMismatch("total_tokens"))
        );
    }
}
