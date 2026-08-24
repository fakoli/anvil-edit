use std::error::Error;
use std::fmt;

/// The semantic contract version implemented by the foundation data model.
///
/// This is not a wire-format version. O003 still owns concrete serialization,
/// transport, and durable-schema versioning.
pub const FOUNDATION_CONTRACT_VERSION: ContractVersion = ContractVersion::new(0, 2);

const MAX_IDENTIFIER_BYTES: usize = 256;

/// A major/minor semantic contract version.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ContractVersion {
    major: u16,
    minor: u16,
}

impl ContractVersion {
    /// Creates a semantic contract version.
    #[must_use]
    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    /// Returns the incompatible-change version.
    #[must_use]
    pub const fn major(self) -> u16 {
        self.major
    }

    /// Returns the backwards-compatible-change version.
    #[must_use]
    pub const fn minor(self) -> u16 {
        self.minor
    }

    /// Rejects an object whose semantic major version is not supported.
    pub fn ensure_compatible_with(self, supported: Self) -> Result<(), ContractError> {
        if self.major == supported.major {
            Ok(())
        } else {
            Err(ContractError::UnsupportedMajorVersion {
                found: self.major,
                supported: supported.major,
            })
        }
    }
}

impl fmt::Display for ContractVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}", self.major, self.minor)
    }
}

/// A bounded, non-empty identifier safe to include in source-free evidence.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Identifier(String);

impl Identifier {
    /// Validates and stores an opaque identifier.
    pub fn new(field: &'static str, value: impl Into<String>) -> Result<Self, ContractError> {
        validate_text(field, value.into()).map(Self)
    }

    /// Returns the identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A bounded machine-readable reason code.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ReasonCode(Identifier);

impl ReasonCode {
    /// Creates a reason code from bounded identifier text.
    pub fn new(value: impl Into<String>) -> Result<Self, ContractError> {
        Identifier::new("reason_code", value).map(Self)
    }

    /// Returns the reason-code text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// A structurally valid lowercase SHA-256 digest.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Sha256Digest(String);

impl Sha256Digest {
    /// Validates and stores a lowercase 64-character hexadecimal digest.
    pub fn new(value: impl Into<String>) -> Result<Self, ContractError> {
        let value = value.into();
        let valid = value.len() == 64
            && value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));

        if valid {
            Ok(Self(value))
        } else {
            Err(ContractError::InvalidSha256Digest)
        }
    }

    /// Returns the digest text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Sha256Digest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl TryFrom<&str> for Sha256Digest {
    type Error = ContractError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// A wall-clock timestamp expressed as signed microseconds from the Unix epoch.
///
/// Wall time supports human correlation only and never establishes causality.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WallClockMicros(i64);

impl WallClockMicros {
    /// Creates a wall-clock observation.
    #[must_use]
    pub const fn new(value: i64) -> Self {
        Self(value)
    }

    /// Returns signed microseconds from the Unix epoch.
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}

/// A producer-local monotonic clock observation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MonotonicTick(u64);

impl MonotonicTick {
    /// Creates a monotonic clock observation.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the producer-local tick value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// A relative duration budget in microseconds.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DurationMicros(u64);

impl DurationMicros {
    /// Creates a relative duration. Zero represents an exhausted budget.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the duration in microseconds.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Privacy classification for a content-bearing value or reference.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum DataClass {
    /// Source-free configuration.
    P0Configuration,
    /// Source-free operational metadata.
    P1OperationalMetadata,
    /// Derived edit metadata that cannot reconstruct source.
    P2DerivedEditMetadata,
    /// Source-bearing editor, prompt, model, or replacement content.
    P3SourceBearing,
    /// Protected content such as credentials or private keys.
    P4HighlySensitive,
}

/// Where the referenced content bytes are permitted to exist.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum PersistenceClass {
    /// Bytes exist only for the bounded in-memory operation.
    MemoryOnly,
    /// Bytes may exist in a governed local content store.
    GovernedLocal,
    /// Bytes are part of a separately authorized export package.
    AuthorizedExport,
}

/// Result of one bounded structural, policy, compatibility, or validation check.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum CheckResult {
    /// The check completed and passed.
    Passed,
    /// The check completed and failed.
    Failed,
    /// The producer cannot determine the result.
    Unknown,
    /// The check was not applicable to this record.
    NotApplicable,
    /// The check was expected but did not run.
    NotRun,
}

/// A source-free handle to governed or ephemeral content.
///
/// The handle deliberately contains no source text, path, prompt, model
/// output, or replacement bytes. Those bytes live behind a separately
/// authorized runtime or content-store boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentReference {
    id: Identifier,
    purpose_scope: Identifier,
    digest: Sha256Digest,
    byte_length: u64,
    data_class: DataClass,
    persistence: PersistenceClass,
}

impl ContentReference {
    /// Creates a source-free content handle.
    ///
    /// P4 content can only be represented as memory-only; persistence and
    /// export of protected bytes fail closed.
    pub fn new(
        id: Identifier,
        purpose_scope: Identifier,
        digest: Sha256Digest,
        byte_length: u64,
        data_class: DataClass,
        persistence: PersistenceClass,
    ) -> Result<Self, ContractError> {
        if data_class == DataClass::P4HighlySensitive && persistence != PersistenceClass::MemoryOnly
        {
            return Err(ContractError::InvalidState(
                "P4 content must remain memory-only",
            ));
        }

        Ok(Self {
            id,
            purpose_scope,
            digest,
            byte_length,
            data_class,
            persistence,
        })
    }

    /// Returns the purpose-scoped content identifier.
    #[must_use]
    pub const fn id(&self) -> &Identifier {
        &self.id
    }

    /// Returns the purpose scope that prevents global correlation by default.
    #[must_use]
    pub const fn purpose_scope(&self) -> &Identifier {
        &self.purpose_scope
    }

    /// Returns the digest of the referenced bytes.
    #[must_use]
    pub const fn digest(&self) -> &Sha256Digest {
        &self.digest
    }

    /// Returns the referenced byte length.
    #[must_use]
    pub const fn byte_length(&self) -> u64 {
        self.byte_length
    }

    /// Returns the privacy class.
    #[must_use]
    pub const fn data_class(&self) -> DataClass {
        self.data_class
    }

    /// Returns the permitted persistence class.
    #[must_use]
    pub const fn persistence(&self) -> PersistenceClass {
        self.persistence
    }
}

/// Structural semantic-contract failures.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ContractError {
    /// A required text field was empty or contained only whitespace.
    EmptyField(&'static str),
    /// A text field exceeded the structural byte bound.
    FieldTooLong(&'static str),
    /// A text field contained a control character unsafe for evidence.
    ControlCharacter(&'static str),
    /// A digest was not lowercase hexadecimal with exactly 64 characters.
    InvalidSha256Digest,
    /// An object's semantic major version is unsupported.
    UnsupportedMajorVersion {
        /// Major version found on the object.
        found: u16,
        /// Major version supported by the reader.
        supported: u16,
    },
    /// A producer sequence was zero even though sequences start at one.
    InvalidProducerSequence,
    /// A record attempted to identify itself as a causal or superseded parent.
    SelfReference(&'static str),
    /// A list contained the same identifier more than once.
    DuplicateReference(&'static str),
    /// A logical URI scheme was structurally invalid.
    InvalidUriScheme,
    /// A logical URI did not use its declared scheme.
    UriSchemeMismatch,
    /// A text range ended before it started.
    InvalidRange,
    /// A candidate mixed edits from different base documents.
    MixedDocumentEdits,
    /// Two normalized edits overlap.
    OverlappingEdits,
    /// Aggregate counts did not match the contained items.
    AggregateMismatch(&'static str),
    /// A structurally impossible state combination was requested.
    InvalidState(&'static str),
}

impl fmt::Display for ContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField(field) => write!(formatter, "{field} must not be empty"),
            Self::FieldTooLong(field) => {
                write!(
                    formatter,
                    "{field} must not exceed {MAX_IDENTIFIER_BYTES} bytes"
                )
            }
            Self::ControlCharacter(field) => {
                write!(formatter, "{field} must not contain control characters")
            }
            Self::InvalidSha256Digest => {
                formatter.write_str("digest must be 64 lowercase hexadecimal characters")
            }
            Self::UnsupportedMajorVersion { found, supported } => write!(
                formatter,
                "semantic contract major version {found} is incompatible with supported major {supported}"
            ),
            Self::InvalidProducerSequence => {
                formatter.write_str("producer_sequence must start at one")
            }
            Self::SelfReference(field) => {
                write!(formatter, "{field} must not reference the record itself")
            }
            Self::DuplicateReference(field) => {
                write!(formatter, "{field} must not contain duplicates")
            }
            Self::InvalidUriScheme => formatter.write_str("URI scheme is invalid"),
            Self::UriSchemeMismatch => {
                formatter.write_str("logical URI does not use the declared scheme")
            }
            Self::InvalidRange => formatter.write_str("text range end precedes start"),
            Self::MixedDocumentEdits => {
                formatter.write_str("candidate edits must share one base document in v0")
            }
            Self::OverlappingEdits => {
                formatter.write_str("normalized candidate edits must not overlap")
            }
            Self::AggregateMismatch(field) => {
                write!(formatter, "aggregate {field} does not match its items")
            }
            Self::InvalidState(message) => formatter.write_str(message),
        }
    }
}

impl Error for ContractError {}

pub(crate) fn validate_text(field: &'static str, value: String) -> Result<String, ContractError> {
    if value.trim().is_empty() {
        Err(ContractError::EmptyField(field))
    } else if value.len() > MAX_IDENTIFIER_BYTES {
        Err(ContractError::FieldTooLong(field))
    } else if value.chars().any(char::is_control) {
        Err(ContractError::ControlCharacter(field))
    } else {
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    #[test]
    fn rejects_incompatible_semantic_major() {
        assert_eq!(
            ContractVersion::new(2, 0).ensure_compatible_with(ContractVersion::new(1, 4)),
            Err(ContractError::UnsupportedMajorVersion {
                found: 2,
                supported: 1,
            })
        );
    }

    #[test]
    fn protected_content_cannot_be_persisted_or_exported() {
        let result = ContentReference::new(
            Identifier::new("content_id", "content-1").expect("identifier"),
            Identifier::new("purpose_scope", "session-1").expect("scope"),
            Sha256Digest::new(DIGEST).expect("digest"),
            10,
            DataClass::P4HighlySensitive,
            PersistenceClass::GovernedLocal,
        );

        assert_eq!(
            result,
            Err(ContractError::InvalidState(
                "P4 content must remain memory-only"
            ))
        );
    }
}
