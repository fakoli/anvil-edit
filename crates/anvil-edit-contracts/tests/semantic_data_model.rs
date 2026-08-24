use std::collections::BTreeSet;

use anvil_edit_contracts::*;

fn id(field: &'static str, value: &str) -> Identifier {
    Identifier::new(field, value).expect("fixture identifier")
}

fn digest(character: char) -> Sha256Digest {
    Sha256Digest::new(character.to_string().repeat(64)).expect("fixture digest")
}

fn envelope(record_id: &str, sequence: u64, parent: Option<&str>) -> RecordEnvelope {
    let envelope = RecordEnvelope::new(
        id("record_id", record_id),
        ProducerPosition::new(
            WallClockMicros::new(i64::try_from(sequence).expect("small sequence")),
            MonotonicTick::new(sequence * 10),
            id("clock_id", "fixture-clock"),
            id("producer_instance_id", "fixture-producer"),
            sequence,
        )
        .expect("producer position"),
        id("idempotency_key", &format!("dedupe-{sequence}")),
        RecordCorrelation {
            session_id: id("session_id", "session-1"),
            repository_id: id("repository_id", "repository-1"),
        },
        Provenance {
            producer: id("producer", "semantic-data-model-fixture"),
            producer_revision: id("producer_revision", "r1"),
            capture_mode: CaptureMode::Synthetic,
        },
    );

    envelope
        .with_causality(
            parent
                .into_iter()
                .map(|value| id("caused_by", value))
                .collect(),
            None,
        )
        .expect("fixture causality")
}

fn component(name: &str, character: char) -> ComponentIdentity {
    ComponentIdentity {
        id: id("component_id", name),
        revision: id("component_revision", "r1"),
        digest: digest(character),
    }
}

fn content(name: &str, character: char, bytes: u64) -> ContentReference {
    ContentReference::new(
        id("content_id", name),
        id("purpose_scope", "session-1"),
        digest(character),
        bytes,
        DataClass::P3SourceBearing,
        PersistenceClass::MemoryOnly,
    )
    .expect("content reference")
}

fn configuration_components() -> Vec<ConfigurationComponent> {
    [
        ConfigurationComponentKind::PredictionPolicy,
        ConfigurationComponentKind::ContextPolicy,
        ConfigurationComponentKind::DisplayPolicy,
        ConfigurationComponentKind::RoutingPolicy,
        ConfigurationComponentKind::AuthorizationPolicy,
        ConfigurationComponentKind::PromptProtocol,
        ConfigurationComponentKind::CapabilityPack,
        ConfigurationComponentKind::NormalizationPolicy,
    ]
    .into_iter()
    .zip(['1', '2', '3', '4', '5', '6', '7', '8'])
    .map(|(kind, character)| ConfigurationComponent {
        kind,
        identity: component(&format!("{kind:?}"), character),
    })
    .collect()
}

fn document_revision(record_id: &str, sequence: u64, parent: Option<&str>) -> DocumentRevision {
    let revision = DocumentRevision {
        envelope: envelope(record_id, sequence, parent),
        document: AdapterDocumentIdentity {
            adapter_type: id("adapter_type", "reference-editor"),
            adapter_instance: id("adapter_instance", "adapter-1"),
            workspace_instance: id("workspace_instance", "workspace-1"),
            uri_scheme: UriScheme::new("file").expect("URI scheme"),
            logical_uri: content("logical-uri-1", '9', 32),
            document_incarnation: id("document_incarnation", "incarnation-1"),
        },
        editor_version: EditorDocumentVersion {
            namespace: id("version_namespace", "reference-editor-buffer"),
            value: id("editor_version", &format!("version-{sequence}")),
        },
        text_model: DocumentTextModel {
            position_encoding: PositionEncoding::Utf16,
            line_ending: LineEnding::Lf,
            terminal_newline: TerminalNewline::Present,
            range_end_semantics: RangeEndSemantics::HalfOpen,
            canonicalization: TextCanonicalization::Utf8Bytes,
        },
        full_buffer_byte_length: 128,
        full_buffer_digest: digest(if sequence == 2 { 'a' } else { 'b' }),
        source_persistence: PersistenceClass::MemoryOnly,
    };
    revision.validate().expect("document revision");
    revision
}

#[test]
fn complete_semantic_lifecycle_preserves_record_boundaries_and_causality() {
    let configuration = ConfigurationSnapshot::new(
        envelope("configuration-1", 1, None),
        ConfigurationIdentity::standalone("configuration-1", "r1", digest('0'))
            .expect("configuration identity"),
        ConfigurationSource {
            provider: id("provider", "standalone-fixture"),
            provider_revision: id("provider_revision", "r1"),
            activation_attempt_id: id("activation_attempt_id", "activation-1"),
            previous_snapshot_id: None,
            desired: None,
        },
        configuration_components(),
        digest('f'),
        false,
        None,
    )
    .expect("configuration snapshot");
    let configuration_identity = configuration.identity().clone();

    let document = document_revision("document-r1", 2, Some("configuration-1"));
    let document_ref = DocumentRevisionRef::from(&document);

    let active_buffer = content("active-buffer-1", 'c', 128);
    let snapshot = EditorSnapshot {
        envelope: envelope("snapshot-1", 3, Some("document-r1")),
        active_document: document.clone(),
        selections: vec![CursorSelection {
            range: TextRange::new(
                TextPosition {
                    line: 3,
                    character: 4,
                },
                TextPosition {
                    line: 3,
                    character: 4,
                },
            )
            .expect("cursor"),
            primary: true,
        }],
        language_id: id("language_id", "rust"),
        visible_ranges: vec![
            TextRange::new(
                TextPosition {
                    line: 0,
                    character: 0,
                },
                TextPosition {
                    line: 20,
                    character: 0,
                },
            )
            .expect("visible range"),
        ],
        recent_edit_ids: Vec::new(),
        related_documents: Vec::new(),
        semantic_inputs: Vec::new(),
        content_inputs: vec![active_buffer.clone()],
        capture_class: SnapshotCaptureClass::MetadataOnly,
        source_persistence: PersistenceClass::MemoryOnly,
    };
    snapshot.validate().expect("editor snapshot");

    let opportunity = PredictionOpportunity {
        envelope: envelope("opportunity-1", 4, Some("snapshot-1")),
        snapshot_id: id("snapshot_id", "snapshot-1"),
        trigger: TriggerKind::TypingPause,
        triggered_at: MonotonicTick::new(40),
        render_budget: DurationMicros::new(500_000),
        eligibility: EligibilityDecision::Eligible,
        reason_codes: vec![ReasonCode::new("typing-pause").expect("reason")],
        superseded_opportunity_id: None,
        signals: vec![PolicySignal {
            name: id("signal_name", "typing_velocity"),
            value: PolicySignalValue::Unsigned(12),
        }],
    };

    let mut runtime_permissions = PermissionSet::deny_all();
    runtime_permissions.runtime_read = PermissionDecision::Allow;
    let runtime_read_grant = RuntimeReadGrant {
        envelope: envelope("read-grant-1", 5, Some("opportunity-1")),
        snapshot_id: id("snapshot_id", "snapshot-1"),
        authorized_content: vec![active_buffer],
        purpose: PredictionPurpose::NextEdit,
        permissions: runtime_permissions,
        protected_content: ProtectedContentResult::Clear,
        effective_policy_digest: digest('f'),
        issuer: id("issuer", "local-authorization"),
        issued_at: MonotonicTick::new(50),
        lifetime: GrantLifetime::OneShot,
        decision: GrantDecision::Granted,
        reason_codes: Vec::new(),
    };
    runtime_read_grant.validate().expect("runtime-read grant");

    let active_window = content("active-window-1", 'd', 64);
    let context_item = ContextItem {
        source_kind: ContextSourceKind::ActiveDocument,
        content: active_window.clone(),
        inclusion_reason: ReasonCode::new("active-window").expect("reason"),
        token_count: 16,
        source_revision: Some(document_ref.clone()),
        freshness_role: FreshnessRole::ApplicationCritical,
    };
    let context = ContextPack {
        envelope: envelope("context-1", 6, Some("read-grant-1")),
        snapshot_id: id("snapshot_id", "snapshot-1"),
        runtime_read_grant_id: id("runtime_read_grant_id", "read-grant-1"),
        active_document: document_ref.clone(),
        context_policy: component("context-policy", '2'),
        total_tokens: context_item.token_count,
        total_bytes: context_item.content.byte_length(),
        items: vec![context_item],
        redaction_results: Vec::new(),
        task_context: None,
    };
    context.validate_totals().expect("context totals");

    let attempt = AttemptIdentity {
        group_id: id("attempt_group_id", "attempt-group-1"),
        ordinal: 0,
        relation: AttemptRelation::Initial,
        prior_attempt_id: None,
        reason: None,
    };
    attempt.validate().expect("initial attempt");
    let dispatch = DispatchDecision {
        envelope: envelope("dispatch-1", 7, Some("context-1")),
        opportunity_id: id("opportunity_id", "opportunity-1"),
        configuration: configuration_identity.clone(),
        prediction_policy: component("prediction-policy", '1'),
        context_policy: component("context-policy", '2'),
        protocol: component("protocol", '6'),
        routing_policy: component("routing-policy", '4'),
        action: DispatchAction::Dispatch,
        executor: Some(ExecutorSelection::CapabilityAlias(id(
            "capability_alias",
            "edit.fast",
        ))),
        purpose: PredictionPurpose::NextEdit,
        visibility: VisibilityMode::Visible,
        budgets: RelativeBudgets {
            queue: DurationMicros::new(50_000),
            inference: DurationMicros::new(350_000),
            render: DurationMicros::new(500_000),
        },
        reason_codes: vec![ReasonCode::new("fast-baseline").expect("reason")],
        attempt: attempt.clone(),
        experiment: None,
    };
    dispatch.validate().expect("dispatch decision");

    let mut permissions = PermissionSet::deny_all();
    permissions.executor_dispatch = PermissionDecision::Allow;
    let grant = ExecutionGrant {
        envelope: envelope("grant-1", 8, Some("dispatch-1")),
        dispatch_decision_id: id("dispatch_decision_id", "dispatch-1"),
        context_pack_id: id("context_pack_id", "context-1"),
        destination: DestinationIdentity {
            destination: id("destination", "local-executor"),
            operator_trust_domain: id("operator_trust_domain", "developer-local"),
            peer_identity: id("peer_identity", "executor-peer-1"),
        },
        executor: ExecutorSelection::CapabilityAlias(id("capability_alias", "edit.fast")),
        protocol: component("protocol", '6'),
        purpose: PredictionPurpose::NextEdit,
        visibility: VisibilityMode::Visible,
        allowed_content_classes: vec![DataClass::P3SourceBearing],
        authorized_content: vec![active_window],
        protected_content: ProtectedContentResult::Clear,
        permissions,
        effective_policy_digest: digest('f'),
        issuer: id("issuer", "local-authorization"),
        issued_at: MonotonicTick::new(80),
        lifetime: GrantLifetime::OneShot,
        decision: GrantDecision::Granted,
        reason_codes: Vec::new(),
    };
    grant.validate().expect("execution grant");

    let request = PredictionRequest {
        envelope: envelope("request-1", 9, Some("grant-1")),
        request_correlation_id: id("request_correlation_id", "request-correlation-1"),
        opportunity_id: id("opportunity_id", "opportunity-1"),
        context_pack_id: id("context_pack_id", "context-1"),
        configuration: configuration_identity,
        prediction_policy: component("prediction-policy", '1'),
        protocol: component("protocol", '6'),
        executor: ExecutorSelection::CapabilityAlias(id("capability_alias", "edit.fast")),
        output_budget: OutputBudget {
            max_tokens: 64,
            max_edits: 4,
            max_replacement_bytes: 4_096,
        },
        generation: GenerationSettings {
            deterministic: true,
            seed: Some(7),
            settings_digest: digest('e'),
        },
        dispatch_decision_id: id("dispatch_decision_id", "dispatch-1"),
        execution_grant_id: id("execution_grant_id", "grant-1"),
        budgets: dispatch.budgets,
        dispatched_at: MonotonicTick::new(90),
        cancellation_id: id("cancellation_id", "cancel-1"),
        attempt,
        experiment: None,
    };
    request.validate().expect("prediction request");

    let serving = ServingObservation {
        envelope: envelope("serving-1", 10, Some("request-1")),
        request_correlation_id: id("request_correlation_id", "request-correlation-1"),
        model_repository: Some(id("model_repository", "example/model")),
        model_revision: Some(id("model_revision", "immutable-r1")),
        tokenizer: Some(component("tokenizer", 'a')),
        prompt_template: Some(component("prompt-template", 'b')),
        quantization_digest: Some(digest('c')),
        runtime_revision: Some(id("runtime_revision", "runtime-r1")),
        runtime_flags_digest: Some(digest('d')),
        hardware_class: Some(id("hardware_class", "local-gpu")),
        executor_identity: Some(id("executor_identity", "executor-1")),
        durations: ServingDurations {
            queue: Some(DurationMicros::new(1_000)),
            time_to_first_token: Some(DurationMicros::new(40_000)),
            decode: Some(DurationMicros::new(20_000)),
            total: Some(DurationMicros::new(61_000)),
        },
        generated_tokens: Some(12),
        cache_state: CacheState::WarmExact,
        status: ServingStatus::Completed,
        reason_codes: Vec::new(),
    };

    let candidate = PredictionCandidate {
        envelope: envelope("candidate-1", 11, Some("request-1")),
        request_id: id("request_id", "request-1"),
        edits: vec![NormalizedTextEdit {
            base_document: document_ref.clone(),
            range: TextRange::new(
                TextPosition {
                    line: 3,
                    character: 4,
                },
                TextPosition {
                    line: 3,
                    character: 4,
                },
            )
            .expect("edit range"),
            position_encoding: PositionEncoding::Utf16,
            range_end_semantics: RangeEndSemantics::HalfOpen,
            replacement: content("replacement-1", 'e', 2),
        }],
        edit_ordering: EditOrdering::BaseRelativeAsListed,
        next_focus: None,
        native_output_digest: Some(digest('f')),
        validation: CandidateValidation {
            protocol_parse: CheckResult::Passed,
            ranges: CheckResult::Passed,
            scope: CheckResult::Passed,
            syntax: CheckResult::Passed,
            diagnostics: CheckResult::NotRun,
            protected_content: CheckResult::Passed,
            unicode_controls: CheckResult::Passed,
        },
        generated_at: Some(MonotonicTick::new(100)),
        normalized_at: MonotonicTick::new(110),
        status: CandidateStatus::Valid,
        reason_codes: Vec::new(),
    };
    candidate.validate().expect("candidate");

    let decision = PredictionDecision {
        envelope: envelope("decision-1", 12, Some("candidate-1")),
        opportunity_id: id("opportunity_id", "opportunity-1"),
        considered_candidate_ids: vec![id("candidate_id", "candidate-1")],
        selected_candidate_id: Some(id("candidate_id", "candidate-1")),
        decision_policy: component("display-policy", '3'),
        decision: PredictionDecisionKind::Show,
        reason_codes: vec![ReasonCode::new("within-deadline").expect("reason")],
        scores: Vec::new(),
        presentation_mode: Some(PresentationMode::Inline),
        decided_at: MonotonicTick::new(120),
        remaining_budget: DurationMicros::new(300_000),
        experiment: None,
    };
    decision.validate().expect("display decision");

    let presentation = PresentationAttempt {
        envelope: envelope("presentation-1", 13, Some("decision-1")),
        decision_id: id("decision_id", "decision-1"),
        candidate_id: id("candidate_id", "candidate-1"),
        adapter_capability: id("adapter_capability", "inline-preview"),
        requested_mode: PresentationMode::Inline,
        target_revisions: vec![document_ref.clone()],
        display_critical_revisions: Vec::new(),
        attempted_at: MonotonicTick::new(130),
        render_duration: DurationMicros::new(2_000),
        remaining_budget: DurationMicros::new(298_000),
        status: PresentationStatus::Displayed,
        reason_code: None,
        rendered_content_digest: Some(digest('e')),
    };

    let resulting_document = document_revision("document-r2", 17, Some("application-1"));
    let resulting_revision = DocumentRevisionRef::from(&resulting_document);
    let application = ApplicationAttempt {
        envelope: envelope("application-1", 14, Some("presentation-1")),
        presentation_id: id("presentation_id", "presentation-1"),
        decision_id: id("decision_id", "decision-1"),
        candidate_id: id("candidate_id", "candidate-1"),
        gesture: GestureKind::AcceptAll,
        attribution_policy: component("attribution-policy", '9'),
        expected_revisions: vec![document_ref.clone()],
        transaction_mode: TransactionMode::SingleDocumentConditional,
        attempted_edit_digests: vec![digest('e')],
        resulting_revisions: vec![resulting_revision.clone()],
        status: ApplicationStatus::Applied,
        reason_codes: Vec::new(),
    };
    application.validate_v0().expect("v0 application");

    let outcome = ObservedOutcome {
        envelope: envelope("outcome-1", 15, Some("application-1")),
        presentation_id: Some(id("presentation_id", "presentation-1")),
        application_id: Some(id("application_id", "application-1")),
        candidate_id: Some(id("candidate_id", "candidate-1")),
        kind: OutcomeKind::Accepted,
        revision_before: Some(document_ref),
        revision_after: Some(resulting_revision),
        observed_at: MonotonicTick::new(150),
        attribution_policy: component("attribution-policy", '9'),
        attribution_confidence: AttributionConfidence::Exact,
        retained_content_digest: Some(digest('e')),
        distance: Some(EditDistanceMetrics {
            changed_characters: 2,
            changed_tokens: Some(1),
        }),
    };
    outcome.validate().expect("observed outcome");

    let survival = SurvivalObservation {
        envelope: envelope("survival-1", 16, Some("application-1")),
        application_id: id("application_id", "application-1"),
        candidate_id: id("candidate_id", "candidate-1"),
        checkpoint: SurvivalCheckpoint::After(DurationMicros::new(300_000_000)),
        scheduled_at: Some(MonotonicTick::new(300_000_130)),
        observed_at: MonotonicTick::new(300_000_140),
        censoring: CensoringStatus::Observed,
        retained_content_digest: Some(digest('e')),
        distance: Some(EditDistanceMetrics {
            changed_characters: 0,
            changed_tokens: Some(0),
        }),
        downstream_correlation_id: None,
        correlation_confidence: None,
        status: SurvivalStatus::Survived,
    };
    survival.validate().expect("survival observation");

    let records = vec![
        LifecycleRecord::ConfigurationSnapshot(configuration),
        LifecycleRecord::DocumentRevision(document),
        LifecycleRecord::EditorSnapshot(snapshot),
        LifecycleRecord::PredictionOpportunity(opportunity),
        LifecycleRecord::RuntimeReadGrant(runtime_read_grant),
        LifecycleRecord::ContextPack(context),
        LifecycleRecord::DispatchDecision(dispatch),
        LifecycleRecord::ExecutionGrant(grant),
        LifecycleRecord::PredictionRequest(request),
        LifecycleRecord::ServingObservation(serving),
        LifecycleRecord::PredictionCandidate(candidate),
        LifecycleRecord::PredictionDecision(decision),
        LifecycleRecord::PresentationAttempt(presentation),
        LifecycleRecord::ApplicationAttempt(application),
        LifecycleRecord::ObservedOutcome(outcome),
        LifecycleRecord::SurvivalObservation(survival),
    ];

    assert_eq!(records.len(), 16);
    let kinds = records
        .iter()
        .map(LifecycleRecord::kind)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        kinds.len(),
        records.len(),
        "record roles must remain distinct"
    );
    assert!(
        records
            .iter()
            .skip(1)
            .all(|record| !record.envelope().caused_by().is_empty()),
        "every non-root fixture record keeps explicit causality"
    );
    assert!(records.iter().all(|record| {
        record
            .envelope()
            .semantic_version()
            .ensure_compatible_with(FOUNDATION_CONTRACT_VERSION)
            .is_ok()
    }));
}
