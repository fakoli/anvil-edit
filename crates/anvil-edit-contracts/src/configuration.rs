use std::error::Error;
use std::fmt;

/// The semantic contract version implemented by this foundation slice.
pub const FOUNDATION_CONTRACT_VERSION: ContractVersion = ContractVersion::new(0, 1);

const MAX_IDENTITY_BYTES: usize = 256;

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
}

impl fmt::Display for ContractVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}", self.major, self.minor)
    }
}

/// The source of an active configuration snapshot.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ConfigurationMode {
    /// Configuration was resolved locally without a fleet controller.
    Standalone,
}

/// A structurally valid lowercase SHA-256 digest.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Sha256Digest(String);

impl Sha256Digest {
    /// Validates and stores a lowercase 64-character hexadecimal digest.
    pub fn new(value: impl Into<String>) -> Result<Self, ConfigurationError> {
        let value = value.into();
        let valid = value.len() == 64
            && value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));

        if valid {
            Ok(Self(value))
        } else {
            Err(ConfigurationError::InvalidSha256Digest)
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
    type Error = ConfigurationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// The foundation subset of configuration identity pinned by one request.
///
/// This is not yet the complete canonical `ConfigurationSnapshot` contract.
/// It isolates the stable identifier, revision, digest, and local mode needed
/// to prove pinning while the full schema and transport remain undecided.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigurationIdentity {
    contract_version: ContractVersion,
    snapshot_id: String,
    revision: String,
    digest: Sha256Digest,
    mode: ConfigurationMode,
}

impl ConfigurationIdentity {
    /// Creates a structurally valid standalone configuration identity.
    ///
    /// This validates identity shape only. It does not establish local policy
    /// authorization, artifact provenance, deployment, or promotion.
    pub fn standalone(
        snapshot_id: impl Into<String>,
        revision: impl Into<String>,
        digest: Sha256Digest,
    ) -> Result<Self, ConfigurationError> {
        let snapshot_id = validate_identity("snapshot_id", snapshot_id.into())?;
        let revision = validate_identity("revision", revision.into())?;

        Ok(Self {
            contract_version: FOUNDATION_CONTRACT_VERSION,
            snapshot_id,
            revision,
            digest,
            mode: ConfigurationMode::Standalone,
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
        &self.snapshot_id
    }

    /// Returns the immutable configuration revision.
    #[must_use]
    pub fn revision(&self) -> &str {
        &self.revision
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

/// Structural configuration identity errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConfigurationError {
    /// A required identity field was empty or contained only whitespace.
    EmptyField(&'static str),
    /// An identity exceeded the foundation's structural byte bound.
    FieldTooLong(&'static str),
    /// An identity contained a control character unsafe for logs or evidence.
    ControlCharacter(&'static str),
    /// A digest was not lowercase, hexadecimal, and exactly 64 characters.
    InvalidSha256Digest,
}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField(field) => write!(formatter, "{field} must not be empty"),
            Self::FieldTooLong(field) => {
                write!(
                    formatter,
                    "{field} must not exceed {MAX_IDENTITY_BYTES} bytes"
                )
            }
            Self::ControlCharacter(field) => {
                write!(formatter, "{field} must not contain control characters")
            }
            Self::InvalidSha256Digest => formatter
                .write_str("configuration digest must be 64 lowercase hexadecimal characters"),
        }
    }
}

impl Error for ConfigurationError {}

fn validate_identity(field: &'static str, value: String) -> Result<String, ConfigurationError> {
    if value.trim().is_empty() {
        Err(ConfigurationError::EmptyField(field))
    } else if value.len() > MAX_IDENTITY_BYTES {
        Err(ConfigurationError::FieldTooLong(field))
    } else if value.chars().any(char::is_control) {
        Err(ConfigurationError::ControlCharacter(field))
    } else {
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    #[test]
    fn accepts_structurally_valid_standalone_identity() {
        let snapshot = ConfigurationIdentity::standalone(
            "standalone/default",
            "foundation-r1",
            Sha256Digest::new(DIGEST).expect("fixture digest is valid"),
        )
        .expect("fixture snapshot is valid");

        assert_eq!(snapshot.contract_version(), ContractVersion::new(0, 1));
        assert_eq!(snapshot.snapshot_id(), "standalone/default");
        assert_eq!(snapshot.revision(), "foundation-r1");
        assert_eq!(snapshot.digest().as_str(), DIGEST);
        assert_eq!(snapshot.mode(), ConfigurationMode::Standalone);
    }

    #[test]
    fn rejects_empty_identity_fields() {
        let error = ConfigurationIdentity::standalone(
            "   ",
            "foundation-r1",
            Sha256Digest::new(DIGEST).expect("fixture digest is valid"),
        )
        .expect_err("blank identity must fail");

        assert_eq!(error, ConfigurationError::EmptyField("snapshot_id"));
    }

    #[test]
    fn rejects_noncanonical_digest() {
        assert_eq!(
            Sha256Digest::new("A".repeat(64)),
            Err(ConfigurationError::InvalidSha256Digest)
        );
        assert_eq!(
            Sha256Digest::new("abc"),
            Err(ConfigurationError::InvalidSha256Digest)
        );
    }

    #[test]
    fn rejects_unsafe_or_unbounded_identity_text() {
        let digest = || Sha256Digest::new(DIGEST).expect("fixture digest is valid");

        assert_eq!(
            ConfigurationIdentity::standalone("snapshot\nforged", "r1", digest()),
            Err(ConfigurationError::ControlCharacter("snapshot_id"))
        );
        assert_eq!(
            ConfigurationIdentity::standalone("a".repeat(257), "r1", digest()),
            Err(ConfigurationError::FieldTooLong("snapshot_id"))
        );
    }
}
