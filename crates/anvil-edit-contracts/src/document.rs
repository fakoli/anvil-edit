use crate::{
    ContentReference, ContractError, DataClass, Identifier, PersistenceClass, RecordEnvelope,
    Sha256Digest,
};

/// A normalized lowercase URI scheme without a filesystem assumption.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UriScheme(String);

impl UriScheme {
    /// Validates and stores a URI scheme.
    pub fn new(value: impl Into<String>) -> Result<Self, ContractError> {
        let value = value.into();
        let valid = !value.is_empty()
            && value
                .bytes()
                .next()
                .is_some_and(|byte| byte.is_ascii_alphabetic())
            && value
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'-' | b'.'));
        if !valid {
            return Err(ContractError::InvalidUriScheme);
        }
        Ok(Self(value.to_ascii_lowercase()))
    }

    /// Returns the normalized URI scheme.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Identity assigned by an editor adapter to one logical document incarnation.
///
/// The logical URI is represented by a governed content handle because paths
/// and editor URIs are P3 data. Actual URI bytes are not embedded in durable
/// lifecycle records.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterDocumentIdentity {
    /// Adapter implementation type.
    pub adapter_type: Identifier,
    /// Adapter process or extension-host instance.
    pub adapter_instance: Identifier,
    /// Editor workspace instance.
    pub workspace_instance: Identifier,
    /// Logical document URI scheme.
    pub uri_scheme: UriScheme,
    /// Purpose-scoped handle to the source-bearing logical URI.
    pub logical_uri: ContentReference,
    /// Incarnation changed across reopen or logical replacement when continuity is unknown.
    pub document_incarnation: Identifier,
}

impl AdapterDocumentIdentity {
    /// Validates that the logical URI is classified as source-bearing or protected.
    pub fn validate(&self) -> Result<(), ContractError> {
        if matches!(
            self.logical_uri.data_class(),
            DataClass::P3SourceBearing | DataClass::P4HighlySensitive
        ) {
            Ok(())
        } else {
            Err(ContractError::InvalidState(
                "logical URI content must be classified P3 or P4",
            ))
        }
    }
}

/// An editor-native version value interpreted only within its namespace.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditorDocumentVersion {
    /// Namespace defining how the version value is interpreted.
    pub namespace: Identifier,
    /// Opaque editor-native version value.
    pub value: Identifier,
}

/// Position-unit semantics for editor ranges.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PositionEncoding {
    /// Character offsets count UTF-8 bytes.
    Utf8,
    /// Character offsets count UTF-16 code units.
    Utf16,
    /// Character offsets count Unicode scalar values.
    UnicodeScalar,
}

/// Line-ending representation of the exact editor buffer.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum LineEnding {
    /// Line feed.
    Lf,
    /// Carriage return followed by line feed.
    CrLf,
    /// Carriage return.
    Cr,
    /// Buffer contains more than one line-ending representation.
    Mixed,
}

/// Terminal-newline state of the exact editor buffer.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TerminalNewline {
    /// Buffer ends with its declared line ending.
    Present,
    /// Buffer does not end with a line ending.
    Absent,
}

/// Canonicalization applied before computing the full-buffer digest.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum TextCanonicalization {
    /// Hash the editor buffer bytes exactly as represented.
    ExactBytes,
    /// Hash UTF-8 bytes after converting editor-native text without other normalization.
    Utf8Bytes,
}

/// End-point semantics for ranges.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RangeEndSemantics {
    /// Start is included and end is excluded.
    HalfOpen,
}

/// Text and range semantics required to interpret a document revision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocumentTextModel {
    /// Position-unit encoding.
    pub position_encoding: PositionEncoding,
    /// Buffer line-ending representation.
    pub line_ending: LineEnding,
    /// Terminal-newline state.
    pub terminal_newline: TerminalNewline,
    /// Range endpoint semantics.
    pub range_end_semantics: RangeEndSemantics,
    /// Digest canonicalization.
    pub canonicalization: TextCanonicalization,
}

/// Portable identity for one exact editor buffer state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocumentRevision {
    /// Common causal envelope; its identifier is the revision identifier.
    pub envelope: RecordEnvelope,
    /// Editor and logical document identity.
    pub document: AdapterDocumentIdentity,
    /// Editor-native version and namespace.
    pub editor_version: EditorDocumentVersion,
    /// Text, digest, and range interpretation.
    pub text_model: DocumentTextModel,
    /// Full canonical buffer length in bytes.
    pub full_buffer_byte_length: u64,
    /// Digest over the complete canonical buffer.
    pub full_buffer_digest: Sha256Digest,
    /// Permitted persistence of the underlying source bytes.
    pub source_persistence: PersistenceClass,
}

impl DocumentRevision {
    /// Validates document identity and protected-content persistence.
    pub fn validate(&self) -> Result<(), ContractError> {
        self.document.validate()?;
        if self.document.logical_uri.data_class() == DataClass::P4HighlySensitive
            && self.source_persistence != PersistenceClass::MemoryOnly
        {
            return Err(ContractError::InvalidState(
                "protected document source must remain memory-only",
            ));
        }
        Ok(())
    }

    /// Returns the immutable document-revision record identifier.
    #[must_use]
    pub const fn id(&self) -> &Identifier {
        self.envelope.id()
    }
}

/// Self-contained exact revision reference used by downstream records.
///
/// Downstream consumers may receive records before the referenced
/// `DocumentRevision`. The reference therefore repeats the source-free
/// identity and text semantics required for stale fencing instead of assuming
/// a database join is immediately available.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocumentRevisionRef {
    /// Document-revision record identifier.
    pub id: Identifier,
    /// Editor and logical document identity.
    pub document: AdapterDocumentIdentity,
    /// Editor-native version and namespace.
    pub editor_version: EditorDocumentVersion,
    /// Text, digest, and range interpretation.
    pub text_model: DocumentTextModel,
    /// Full canonical buffer length in bytes.
    pub full_buffer_byte_length: u64,
    /// Full canonical buffer digest.
    pub full_buffer_digest: Sha256Digest,
}

impl DocumentRevisionRef {
    /// Validates the repeated document identity used for fencing.
    pub fn validate(&self) -> Result<(), ContractError> {
        self.document.validate()
    }
}

impl From<&DocumentRevision> for DocumentRevisionRef {
    fn from(revision: &DocumentRevision) -> Self {
        Self {
            id: revision.id().clone(),
            document: revision.document.clone(),
            editor_version: revision.editor_version.clone(),
            text_model: revision.text_model.clone(),
            full_buffer_byte_length: revision.full_buffer_byte_length,
            full_buffer_digest: revision.full_buffer_digest.clone(),
        }
    }
}

/// A zero-based editor text position in the declared position encoding.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TextPosition {
    /// Zero-based line number.
    pub line: u32,
    /// Zero-based character unit within the line.
    pub character: u32,
}

/// A valid ordered text range.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TextRange {
    start: TextPosition,
    end: TextPosition,
}

impl TextRange {
    /// Creates an ordered range.
    pub fn new(start: TextPosition, end: TextPosition) -> Result<Self, ContractError> {
        if end < start {
            Err(ContractError::InvalidRange)
        } else {
            Ok(Self { start, end })
        }
    }

    /// Returns the inclusive start position.
    #[must_use]
    pub const fn start(&self) -> TextPosition {
        self.start
    }

    /// Returns the exclusive end position.
    #[must_use]
    pub const fn end(&self) -> TextPosition {
        self.end
    }

    /// Returns true when this half-open range overlaps another range.
    #[must_use]
    pub fn overlaps(&self, other: &Self) -> bool {
        self.start < other.end && other.start < self.end
    }
}

/// One cursor or selection captured in an editor snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CursorSelection {
    /// Selection range; an empty range represents a caret.
    pub range: TextRange,
    /// Whether this is the editor's primary selection.
    pub primary: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_uri_scheme_and_half_open_ranges() {
        assert_eq!(
            UriScheme::new("VSCODE-UNTITLED").expect("scheme").as_str(),
            "vscode-untitled"
        );
        assert_eq!(
            UriScheme::new("1file"),
            Err(ContractError::InvalidUriScheme)
        );

        let range = TextRange::new(
            TextPosition {
                line: 1,
                character: 2,
            },
            TextPosition {
                line: 1,
                character: 1,
            },
        );
        assert_eq!(range, Err(ContractError::InvalidRange));
    }

    #[test]
    fn adjacent_half_open_ranges_do_not_overlap() {
        let first = TextRange::new(
            TextPosition {
                line: 0,
                character: 0,
            },
            TextPosition {
                line: 0,
                character: 2,
            },
        )
        .expect("first range");
        let second = TextRange::new(
            TextPosition {
                line: 0,
                character: 2,
            },
            TextPosition {
                line: 0,
                character: 4,
            },
        )
        .expect("second range");

        assert!(!first.overlaps(&second));
    }
}
