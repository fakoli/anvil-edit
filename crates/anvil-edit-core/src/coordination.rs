use std::{error::Error, fmt, sync::Arc};

use anvil_edit_contracts::{ContractError, DocumentRevisionRef};

/// Process-local generation used as a cheap supersession check.
///
/// A generation is meaningful only inside one single-writer revision slot. It
/// never replaces the exact document identity, version, text semantics, length,
/// and digest carried by [`DocumentRevisionRef`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RevisionGeneration(u64);

impl RevisionGeneration {
    const INITIAL: Self = Self(1);

    /// Returns the process-local generation value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    fn next(self) -> Option<Self> {
        self.0.checked_add(1).map(Self)
    }
}

/// Request-local pin combining the fast local generation with the exact fence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevisionFence {
    generation: RevisionGeneration,
    revision: Arc<DocumentRevisionRef>,
}

impl RevisionFence {
    /// Returns the process-local generation captured by this pin.
    #[must_use]
    pub const fn generation(&self) -> RevisionGeneration {
        self.generation
    }

    /// Returns the exact portable revision required for final fencing.
    #[must_use]
    pub fn revision(&self) -> &DocumentRevisionRef {
        &self.revision
    }
}

/// Result of observing a revision in one actor-owned slot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RevisionObservation {
    /// The exact revision was already current, so no cancellation is warranted.
    Unchanged(RevisionGeneration),
    /// A different exact revision superseded the prior pin.
    Advanced {
        /// Generation invalidated by the new observation.
        superseded_generation: RevisionGeneration,
        /// Newly current generation; the owning actor may now signal cancellation.
        current_generation: RevisionGeneration,
    },
}

/// Failure to initialize or advance an actor-owned revision slot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RevisionStateError {
    /// The supplied portable revision failed its structural contract.
    InvalidRevision(ContractError),
    /// One immutable revision identifier was reused with different semantics.
    ConflictingRevisionIdentity,
    /// The process-local generation counter cannot advance safely.
    GenerationExhausted,
}

impl fmt::Display for RevisionStateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRevision(error) => write!(formatter, "invalid document revision: {error}"),
            Self::ConflictingRevisionIdentity => formatter.write_str(
                "document revision identifier was reused with different fencing semantics",
            ),
            Self::GenerationExhausted => {
                formatter.write_str("process-local revision generation is exhausted")
            }
        }
    }
}

impl Error for RevisionStateError {}

impl From<ContractError> for RevisionStateError {
    fn from(value: ContractError) -> Self {
        Self::InvalidRevision(value)
    }
}

/// Exact latest-revision state intended to be owned by one session actor.
///
/// This type deliberately performs no synchronization. The future session
/// coordinator is the single writer; asynchronous workers carry a
/// [`RevisionFence`] back to it. An advanced generation is a cancellation
/// signal, while [`Self::is_current`] remains only a fast guard before the full
/// adapter fence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LatestRevision {
    current: RevisionFence,
}

impl LatestRevision {
    /// Creates a revision slot at generation one after structural validation.
    pub fn new(revision: DocumentRevisionRef) -> Result<Self, RevisionStateError> {
        revision.validate()?;
        Ok(Self {
            current: RevisionFence {
                generation: RevisionGeneration::INITIAL,
                revision: Arc::new(revision),
            },
        })
    }

    /// Pins the current local generation and complete portable revision.
    #[must_use]
    pub fn pin(&self) -> RevisionFence {
        self.current.clone()
    }

    /// Observes an exact revision, advancing only when the full fence changed.
    ///
    /// Reusing one record identifier with different revision semantics is an
    /// integrity conflict rather than a new generation.
    pub fn observe(
        &mut self,
        revision: DocumentRevisionRef,
    ) -> Result<RevisionObservation, RevisionStateError> {
        revision.validate()?;

        if revision == *self.current.revision {
            return Ok(RevisionObservation::Unchanged(self.current.generation));
        }

        if revision.id == self.current.revision.id {
            return Err(RevisionStateError::ConflictingRevisionIdentity);
        }

        let generation = self
            .current
            .generation
            .next()
            .ok_or(RevisionStateError::GenerationExhausted)?;
        let superseded_generation = self.current.generation;
        self.current = RevisionFence {
            generation,
            revision: Arc::new(revision),
        };

        Ok(RevisionObservation::Advanced {
            superseded_generation,
            current_generation: generation,
        })
    }

    /// Returns whether a request-local pin still matches the complete slot.
    ///
    /// This is an in-process fast check. The editor adapter must still compare
    /// the expected portable revision before presentation and application.
    #[must_use]
    pub fn is_current(&self, fence: &RevisionFence) -> bool {
        &self.current == fence
    }
}

#[cfg(test)]
mod tests {
    use anvil_edit_contracts::{
        AdapterDocumentIdentity, ContentReference, DataClass, DocumentTextModel,
        EditorDocumentVersion, Identifier, LineEnding, PersistenceClass, PositionEncoding,
        RangeEndSemantics, Sha256Digest, TerminalNewline, TextCanonicalization, UriScheme,
    };

    use super::*;

    fn id(field: &'static str, value: &str) -> Identifier {
        Identifier::new(field, value).expect("fixture identifier")
    }

    fn digest(character: char) -> Sha256Digest {
        Sha256Digest::new(character.to_string().repeat(64)).expect("fixture digest")
    }

    fn revision(
        record_id: &str,
        editor_version: &str,
        digest_character: char,
    ) -> DocumentRevisionRef {
        DocumentRevisionRef {
            id: id("document_revision_id", record_id),
            document: AdapterDocumentIdentity {
                adapter_type: id("adapter_type", "fixture-editor"),
                adapter_instance: id("adapter_instance", "adapter-1"),
                workspace_instance: id("workspace_instance", "workspace-1"),
                uri_scheme: UriScheme::new("file").expect("scheme"),
                logical_uri: ContentReference::new(
                    id("content_id", "logical-uri-1"),
                    id("purpose_scope", "session-1"),
                    digest('c'),
                    24,
                    DataClass::P3SourceBearing,
                    PersistenceClass::MemoryOnly,
                )
                .expect("logical URI"),
                document_incarnation: id("document_incarnation", "incarnation-1"),
            },
            editor_version: EditorDocumentVersion {
                namespace: id("version_namespace", "fixture-buffer"),
                value: id("editor_version", editor_version),
            },
            text_model: DocumentTextModel {
                position_encoding: PositionEncoding::Utf16,
                line_ending: LineEnding::Lf,
                terminal_newline: TerminalNewline::Present,
                range_end_semantics: RangeEndSemantics::HalfOpen,
                canonicalization: TextCanonicalization::Utf8Bytes,
            },
            full_buffer_byte_length: 12,
            full_buffer_digest: digest(digest_character),
        }
    }

    #[test]
    fn exact_duplicate_does_not_advance_generation() {
        let first = revision("revision-1", "1", 'a');
        let mut latest = LatestRevision::new(first.clone()).expect("valid first revision");

        let observation = latest.observe(first).expect("duplicate is valid");

        assert!(matches!(observation, RevisionObservation::Unchanged(_)));
        assert_eq!(latest.pin().generation().get(), 1);
    }

    #[test]
    fn new_revision_invalidates_the_prior_fast_fence() {
        let mut latest =
            LatestRevision::new(revision("revision-1", "1", 'a')).expect("valid first revision");
        let prior = latest.pin();

        let observation = latest
            .observe(revision("revision-2", "2", 'b'))
            .expect("valid newer revision");

        let RevisionObservation::Advanced {
            superseded_generation,
            current_generation,
        } = observation
        else {
            panic!("revision should advance");
        };
        assert_eq!(superseded_generation, prior.generation());
        assert_eq!(current_generation.get(), 2);
        assert!(!latest.is_current(&prior));
        assert!(latest.is_current(&latest.pin()));
    }

    #[test]
    fn reused_current_record_identity_with_different_digest_is_rejected() {
        let mut latest =
            LatestRevision::new(revision("revision-1", "1", 'a')).expect("valid first revision");

        assert_eq!(
            latest.observe(revision("revision-1", "1", 'b')),
            Err(RevisionStateError::ConflictingRevisionIdentity)
        );
        assert_eq!(latest.pin().generation().get(), 1);
    }

    #[test]
    fn exhausted_generation_fails_without_replacing_the_revision() {
        let mut latest =
            LatestRevision::new(revision("revision-1", "1", 'a')).expect("valid first revision");
        latest.current.generation = RevisionGeneration(u64::MAX);
        let prior = latest.pin();

        assert_eq!(
            latest.observe(revision("revision-2", "2", 'b')),
            Err(RevisionStateError::GenerationExhausted)
        );
        assert!(latest.is_current(&prior));
    }
}
