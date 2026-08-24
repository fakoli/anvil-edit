use std::collections::BTreeSet;

use crate::{
    ContractError, ContractVersion, FOUNDATION_CONTRACT_VERSION, Identifier, MonotonicTick,
    WallClockMicros,
};

/// How a producer obtained the record.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum CaptureMode {
    /// Observed during a live editor session.
    Live,
    /// Reconstructed by deterministic replay.
    Replay,
    /// Created by a synthetic contract fixture.
    Synthetic,
    /// Imported from a separately governed artifact.
    Imported,
}

/// Source identity attached to every durable lifecycle record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Provenance {
    /// Logical producer name.
    pub producer: Identifier,
    /// Immutable producer implementation revision.
    pub producer_revision: Identifier,
    /// Capture mode that produced the record.
    pub capture_mode: CaptureMode,
}

/// One producer's wall and monotonic observations for a record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProducerPosition {
    occurred_at_wall: WallClockMicros,
    monotonic_tick: MonotonicTick,
    monotonic_clock_id: Identifier,
    producer_instance_id: Identifier,
    producer_sequence: u64,
}

impl ProducerPosition {
    /// Creates a position on one producer instance and monotonic clock epoch.
    pub fn new(
        occurred_at_wall: WallClockMicros,
        monotonic_tick: MonotonicTick,
        monotonic_clock_id: Identifier,
        producer_instance_id: Identifier,
        producer_sequence: u64,
    ) -> Result<Self, ContractError> {
        if producer_sequence == 0 {
            return Err(ContractError::InvalidProducerSequence);
        }

        Ok(Self {
            occurred_at_wall,
            monotonic_tick,
            monotonic_clock_id,
            producer_instance_id,
            producer_sequence,
        })
    }

    /// Returns the producer wall-clock observation.
    #[must_use]
    pub const fn occurred_at_wall(&self) -> WallClockMicros {
        self.occurred_at_wall
    }

    /// Returns the producer-local monotonic tick.
    #[must_use]
    pub const fn monotonic_tick(&self) -> MonotonicTick {
        self.monotonic_tick
    }

    /// Returns the monotonic clock/process epoch identifier.
    #[must_use]
    pub const fn monotonic_clock_id(&self) -> &Identifier {
        &self.monotonic_clock_id
    }

    /// Returns the producer process or adapter instance.
    #[must_use]
    pub const fn producer_instance_id(&self) -> &Identifier {
        &self.producer_instance_id
    }

    /// Returns the strictly increasing sequence within the producer instance.
    #[must_use]
    pub const fn producer_sequence(&self) -> u64 {
        self.producer_sequence
    }
}

/// Purpose-scoped session and repository correlation identifiers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordCorrelation {
    /// Local editing-session identifier scoped to the declared purpose.
    pub session_id: Identifier,
    /// Pseudonymous repository identifier scoped to the declared purpose.
    pub repository_id: Identifier,
}

/// Common causal envelope carried by every durable domain record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordEnvelope {
    semantic_version: ContractVersion,
    id: Identifier,
    producer_position: ProducerPosition,
    ingested_at_wall: Option<WallClockMicros>,
    caused_by: Vec<Identifier>,
    supersedes: Option<Identifier>,
    idempotency_key: Identifier,
    correlation: RecordCorrelation,
    provenance: Provenance,
}

impl RecordEnvelope {
    /// Creates a source-free envelope without causal parents.
    pub fn new(
        id: Identifier,
        producer_position: ProducerPosition,
        idempotency_key: Identifier,
        correlation: RecordCorrelation,
        provenance: Provenance,
    ) -> Self {
        Self {
            semantic_version: FOUNDATION_CONTRACT_VERSION,
            id,
            producer_position,
            ingested_at_wall: None,
            caused_by: Vec::new(),
            supersedes: None,
            idempotency_key,
            correlation,
            provenance,
        }
    }

    /// Attaches a receiver wall-clock observation.
    #[must_use]
    pub fn with_ingested_at_wall(mut self, ingested_at_wall: WallClockMicros) -> Self {
        self.ingested_at_wall = Some(ingested_at_wall);
        self
    }

    /// Attaches causal parents and an optional superseded record.
    pub fn with_causality(
        mut self,
        caused_by: Vec<Identifier>,
        supersedes: Option<Identifier>,
    ) -> Result<Self, ContractError> {
        if caused_by.iter().any(|parent| parent == &self.id) {
            return Err(ContractError::SelfReference("caused_by"));
        }
        if supersedes.as_ref() == Some(&self.id) {
            return Err(ContractError::SelfReference("supersedes"));
        }

        let unique = caused_by.iter().collect::<BTreeSet<_>>();
        if unique.len() != caused_by.len() {
            return Err(ContractError::DuplicateReference("caused_by"));
        }

        self.caused_by = caused_by;
        self.supersedes = supersedes;
        Ok(self)
    }

    /// Returns the semantic contract version.
    ///
    /// A future serialized envelope carries its own independently versioned
    /// `schema_version`; this value does not select that representation.
    #[must_use]
    pub const fn semantic_version(&self) -> ContractVersion {
        self.semantic_version
    }

    /// Returns the immutable record identifier.
    #[must_use]
    pub const fn id(&self) -> &Identifier {
        &self.id
    }

    /// Returns the producer's clock and sequence observations.
    #[must_use]
    pub const fn producer_position(&self) -> &ProducerPosition {
        &self.producer_position
    }

    /// Returns the receiver wall-clock observation, when the record crossed a boundary.
    #[must_use]
    pub const fn ingested_at_wall(&self) -> Option<WallClockMicros> {
        self.ingested_at_wall
    }

    /// Returns the immutable causal parent identifiers.
    #[must_use]
    pub fn caused_by(&self) -> &[Identifier] {
        &self.caused_by
    }

    /// Returns the record superseded by this correction, when any.
    #[must_use]
    pub const fn supersedes(&self) -> Option<&Identifier> {
        self.supersedes.as_ref()
    }

    /// Returns the producer-scoped duplicate-suppression key.
    #[must_use]
    pub const fn idempotency_key(&self) -> &Identifier {
        &self.idempotency_key
    }

    /// Returns the purpose-scoped correlation identifiers.
    #[must_use]
    pub const fn correlation(&self) -> &RecordCorrelation {
        &self.correlation
    }

    /// Returns producer and capture provenance.
    #[must_use]
    pub const fn provenance(&self) -> &Provenance {
        &self.provenance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(field: &'static str, value: &str) -> Identifier {
        Identifier::new(field, value).expect("fixture identifier")
    }

    fn envelope(record_id: &str) -> RecordEnvelope {
        RecordEnvelope::new(
            id("record_id", record_id),
            ProducerPosition::new(
                WallClockMicros::new(1),
                MonotonicTick::new(10),
                id("clock_id", "clock-1"),
                id("producer_instance_id", "adapter-1"),
                1,
            )
            .expect("producer position"),
            id("idempotency_key", "dedupe-1"),
            RecordCorrelation {
                session_id: id("session_id", "session-1"),
                repository_id: id("repository_id", "repo-1"),
            },
            Provenance {
                producer: id("producer", "fixture"),
                producer_revision: id("producer_revision", "r1"),
                capture_mode: CaptureMode::Synthetic,
            },
        )
    }

    #[test]
    fn rejects_duplicate_or_self_causal_links() {
        let duplicate = envelope("record-1").with_causality(
            vec![id("parent", "parent-1"), id("parent", "parent-1")],
            None,
        );
        assert_eq!(
            duplicate,
            Err(ContractError::DuplicateReference("caused_by"))
        );

        let self_link = envelope("record-1").with_causality(vec![id("parent", "record-1")], None);
        assert_eq!(self_link, Err(ContractError::SelfReference("caused_by")));
    }
}
