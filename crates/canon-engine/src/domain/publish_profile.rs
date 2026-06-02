use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::domain::mode::{GovernedExpertiseKind, IntendedPersona, Mode, StageRoleHint};
use crate::domain::policy::{AuthorityZone, ChangeClass, RiskClass, UsageZone};

mod authority;
mod publication;
mod semantic;

pub use authority::{
    ADAPTIVE_GOVERNANCE_V1_CONTRACT_LINE, AUTHORITY_GOVERNANCE_V1_CONTRACT_LINE,
    AdaptiveGovernanceState, AdaptiveGovernanceV1Envelope, AdaptiveGovernanceV1RuntimeInputs,
    AdaptiveRolloutProfile, AuthorityApprovalState, AuthorityGovernanceV1Envelope,
    AuthorityGovernanceV1RuntimeInputs, AuthorityPacketReadiness, AuthorityRiskClass,
};
pub use publication::{
    CANON_PRODUCER, GOVERNED_EXPERTISE_INPUT_CONTRACT_VERSION, LineageMetadata,
    ManagedBlockDescriptor, ModePromotionPolicy, OPTIONAL_V1_LINEAGE_FIELDS,
    PROJECT_MEMORY_CONTRACT_VERSION, PROJECT_MEMORY_MANAGED_BLOCK_MARKER,
    PROJECT_MEMORY_PACKET_METADATA_FILE_NAME, PromotionState, PublishProfile,
    PublishProfilesPolicy, REQUIRED_V1_LINEAGE_FIELDS, UpdateStrategy,
};
pub use semantic::{
    ArtifactIndexingMetadata, ArtifactMetadataCarrier, ExpertiseInputMetadata,
    IndexableArtifactClass, PublicationTargetClass, SEMANTIC_ARTIFACT_CONTRACT_LINE_V1,
    SemanticArtifactDescriptor, SemanticEligibilityState, SemanticProvenanceBoundary,
    classify_governed_expertise_input, indexable_artifact_class_for_publication,
    normalize_domain_families,
};

#[cfg(test)]
mod authority_governance_tests {
    use super::{
        ADAPTIVE_GOVERNANCE_V1_CONTRACT_LINE, AUTHORITY_GOVERNANCE_V1_CONTRACT_LINE,
        AdaptiveGovernanceState, AdaptiveGovernanceV1Envelope, AdaptiveGovernanceV1RuntimeInputs,
        AdaptiveRolloutProfile, ArtifactIndexingMetadata, ArtifactMetadataCarrier,
        AuthorityApprovalState, AuthorityGovernanceV1Envelope, AuthorityGovernanceV1RuntimeInputs,
        AuthorityPacketReadiness, AuthorityRiskClass, IndexableArtifactClass,
        PublicationTargetClass, SEMANTIC_ARTIFACT_CONTRACT_LINE_V1, SemanticArtifactDescriptor,
        SemanticEligibilityState, SemanticProvenanceBoundary, UpdateStrategy,
    };
    use crate::domain::mode::{IntendedPersona, Mode};
    use crate::domain::policy::{AuthorityZone, ChangeClass, RiskClass, UsageZone};

    #[test]
    fn authority_governance_envelope_derives_required_fields() {
        let envelope = AuthorityGovernanceV1Envelope::from_runtime_inputs(
            AuthorityGovernanceV1RuntimeInputs {
                mode: Mode::Architecture,
                risk: RiskClass::SystemicImpact,
                zone: UsageZone::Yellow,
                approval_state: AuthorityApprovalState::Requested,
                packet_readiness: AuthorityPacketReadiness::Incomplete,
                primary_artifact: Some("01-architecture-summary.md".to_string()),
                artifact_order: vec!["01-architecture-summary.md".to_string()],
                promotion_refs: Vec::new(),
            },
        );

        assert_eq!(envelope.contract_line, AUTHORITY_GOVERNANCE_V1_CONTRACT_LINE);
        assert_eq!(envelope.authority_zone, AuthorityZone::Restricted);
        assert_eq!(envelope.change_class, ChangeClass::SystemicImpact);
        assert_eq!(envelope.intended_persona, IntendedPersona::SystemArchitect);
        assert_eq!(envelope.packet_readiness, AuthorityPacketReadiness::Incomplete);
        assert_eq!(envelope.risk, AuthorityRiskClass::SystemicImpact);
        assert!(!envelope.persona_anti_behaviors.is_empty());
    }

    #[test]
    fn authority_governance_envelope_omits_optional_fields_when_empty() {
        let envelope = AuthorityGovernanceV1Envelope::from_runtime_inputs(
            AuthorityGovernanceV1RuntimeInputs {
                mode: Mode::Requirements,
                risk: RiskClass::LowImpact,
                zone: UsageZone::Green,
                approval_state: AuthorityApprovalState::NotNeeded,
                packet_readiness: AuthorityPacketReadiness::Reusable,
                primary_artifact: None,
                artifact_order: Vec::new(),
                promotion_refs: Vec::new(),
            },
        );

        let value = serde_json::to_value(&envelope).unwrap();
        assert_eq!(value["authority_zone"], "green");
        assert_eq!(value["change_class"], "low-impact");
        assert_eq!(value["risk"], "low-impact");
        assert!(value.get("primary_artifact").is_none());
        assert!(value.get("promotion_refs").is_none());
    }

    #[test]
    fn adaptive_governance_envelope_derives_guided_advisory_companion() {
        let envelope =
            AdaptiveGovernanceV1Envelope::from_runtime_inputs(AdaptiveGovernanceV1RuntimeInputs {
                risk: RiskClass::BoundedImpact,
                zone: UsageZone::Yellow,
                approval_state: AuthorityApprovalState::NotNeeded,
                packet_readiness: AuthorityPacketReadiness::Reusable,
            });

        assert_eq!(envelope.contract_line, ADAPTIVE_GOVERNANCE_V1_CONTRACT_LINE);
        assert_eq!(envelope.governance_state, AdaptiveGovernanceState::Advisory);
        assert_eq!(envelope.rollout_profile, AdaptiveRolloutProfile::Guided);
        assert!(envelope.state_rationale.is_none());
        assert!(envelope.profile_rationale.is_none());
    }

    #[test]
    fn artifact_indexing_metadata_maps_managed_surfaces_from_publication() {
        let metadata = ArtifactIndexingMetadata::for_publication(
            PublicationTargetClass::Stable,
            UpdateStrategy::ManagedBlocks,
        )
        .unwrap();

        assert_eq!(metadata.artifact_class, IndexableArtifactClass::ManagedSurface);
        assert_eq!(metadata.metadata_carrier, ArtifactMetadataCarrier::ManagedSurfaceEnvelope);
        assert!(metadata.validate().is_ok());
    }

    #[test]
    fn artifact_indexing_metadata_rejects_ambiguous_publication_mappings() {
        let error = ArtifactIndexingMetadata::for_publication(
            PublicationTargetClass::Proposal,
            UpdateStrategy::ManagedBlocks,
        )
        .unwrap_err();

        assert!(error.contains("unsupported artifact indexing mapping"));
    }

    #[test]
    fn artifact_indexing_metadata_maps_index_surfaces_from_publication() {
        let metadata = ArtifactIndexingMetadata::for_publication(
            PublicationTargetClass::Index,
            UpdateStrategy::AppendOnlyIndex,
        )
        .unwrap();

        assert_eq!(metadata.artifact_class, IndexableArtifactClass::IndexSurface);
        assert_eq!(metadata.metadata_carrier, ArtifactMetadataCarrier::PacketMetadataSidecar);
        assert!(metadata.validate().is_ok());
    }

    #[test]
    fn artifact_indexing_metadata_validate_rejects_misaligned_discovery_rule() {
        let error = ArtifactIndexingMetadata {
            artifact_class: IndexableArtifactClass::ManagedSurface,
            metadata_carrier: ArtifactMetadataCarrier::ManagedSurfaceEnvelope,
            discovery_rule: "wrong-rule".to_string(),
        }
        .validate()
        .unwrap_err();

        assert!(error.contains("requires discovery rule"));
    }

    #[test]
    fn semantic_artifact_descriptor_requires_provenance_when_eligible() {
        let error = SemanticArtifactDescriptor {
            semantic_contract_line: SEMANTIC_ARTIFACT_CONTRACT_LINE_V1.to_string(),
            semantic_eligibility: SemanticEligibilityState::Eligible,
            semantic_provenance_boundary: None,
            semantic_provenance_ref: None,
            semantic_labels: vec!["project-memory".to_string()],
            semantic_exclusion_reason: None,
        }
        .validate()
        .unwrap_err();

        assert!(error.contains("semantic_provenance_boundary"));
    }

    #[test]
    fn semantic_artifact_descriptor_requires_provenance_for_excluded_surfaces() {
        let error = SemanticArtifactDescriptor {
            semantic_contract_line: SEMANTIC_ARTIFACT_CONTRACT_LINE_V1.to_string(),
            semantic_eligibility: SemanticEligibilityState::Excluded,
            semantic_provenance_boundary: None,
            semantic_provenance_ref: None,
            semantic_labels: Vec::new(),
            semantic_exclusion_reason: Some("excluded from semantic retrieval".to_string()),
        }
        .validate()
        .unwrap_err();

        assert!(error.contains("semantic_provenance_boundary"));
    }

    #[test]
    fn semantic_artifact_descriptor_accepts_excluded_surface_with_provenance() {
        let descriptor = SemanticArtifactDescriptor {
            semantic_contract_line: SEMANTIC_ARTIFACT_CONTRACT_LINE_V1.to_string(),
            semantic_eligibility: SemanticEligibilityState::Excluded,
            semantic_provenance_boundary: Some(SemanticProvenanceBoundary::Surface),
            semantic_provenance_ref: Some("tech-docs/project/open-risks.md".to_string()),
            semantic_labels: vec!["visibility-only".to_string()],
            semantic_exclusion_reason: Some(
                "index surfaces stay excluded from retrieval".to_string(),
            ),
        };

        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn semantic_artifact_descriptor_accepts_eligible_managed_block_shape() {
        let descriptor = SemanticArtifactDescriptor {
            semantic_contract_line: SEMANTIC_ARTIFACT_CONTRACT_LINE_V1.to_string(),
            semantic_eligibility: SemanticEligibilityState::Eligible,
            semantic_provenance_boundary: Some(SemanticProvenanceBoundary::ManagedBlock),
            semantic_provenance_ref: Some(
                "tech-docs/project/overview.md#managed-block-1".to_string(),
            ),
            semantic_labels: vec!["project-memory".to_string(), "overview".to_string()],
            semantic_exclusion_reason: None,
        };

        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn indexable_artifact_class_for_publication_rejects_pending_append_only_mapping() {
        let error = super::indexable_artifact_class_for_publication(
            PublicationTargetClass::Pending,
            UpdateStrategy::AppendOnlyIndex,
        )
        .unwrap_err();

        assert!(error.contains("unsupported artifact indexing mapping"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn promotion_state_serde_round_trip() {
        for state in [
            PromotionState::Auto,
            PromotionState::AutoIfApproved,
            PromotionState::PendingIndex,
            PromotionState::IndexOnly,
            PromotionState::EvidenceOnly,
            PromotionState::Manual,
        ] {
            let json = serde_json::to_string(&state).unwrap();
            let back: PromotionState = serde_json::from_str(&json).unwrap();
            assert_eq!(state, back, "round-trip failed for {state:?}");
        }
    }

    #[test]
    fn promotion_state_kebab_case_serialization() {
        assert_eq!(
            serde_json::to_string(&PromotionState::AutoIfApproved).unwrap(),
            "\"auto-if-approved\""
        );
        assert_eq!(
            serde_json::to_string(&PromotionState::PendingIndex).unwrap(),
            "\"pending-index\""
        );
        assert_eq!(
            serde_json::to_string(&PromotionState::EvidenceOnly).unwrap(),
            "\"evidence-only\""
        );
    }

    #[test]
    fn update_strategy_serde_round_trip() {
        for strategy in [
            UpdateStrategy::ManagedBlocks,
            UpdateStrategy::ProposalFiles,
            UpdateStrategy::AppendOnlyIndex,
        ] {
            let json = serde_json::to_string(&strategy).unwrap();
            let back: UpdateStrategy = serde_json::from_str(&json).unwrap();
            assert_eq!(strategy, back);
        }
    }

    #[test]
    fn publish_profile_from_str() {
        assert_eq!(
            "project-memory".parse::<PublishProfile>().unwrap(),
            PublishProfile::ProjectMemory
        );
        assert!("unknown".parse::<PublishProfile>().is_err());
    }

    #[test]
    fn publish_profile_serde_round_trip() {
        let json = serde_json::to_string(&PublishProfile::ProjectMemory).unwrap();
        assert_eq!(json, "\"project-memory\"");
        let back: PublishProfile = serde_json::from_str(&json).unwrap();
        assert_eq!(back, PublishProfile::ProjectMemory);
    }

    #[test]
    fn indexable_artifact_classes_have_stable_carriers_and_rules() {
        let managed_surface = IndexableArtifactClass::ManagedSurface;
        assert_eq!(managed_surface.as_str(), "managed-surface");
        assert_eq!(
            managed_surface.metadata_carrier(),
            ArtifactMetadataCarrier::ManagedSurfaceEnvelope
        );
        assert!(managed_surface.discovery_rule().contains("managed-block"));

        let packet_backed = [
            IndexableArtifactClass::ProposalArtifact,
            IndexableArtifactClass::EvidenceBundle,
            IndexableArtifactClass::IndexSurface,
        ];

        for artifact_class in packet_backed {
            assert_eq!(
                artifact_class.metadata_carrier(),
                ArtifactMetadataCarrier::PacketMetadataSidecar
            );
            assert!(
                artifact_class.discovery_rule().contains(PROJECT_MEMORY_PACKET_METADATA_FILE_NAME)
            );
        }
    }

    #[test]
    fn indexable_artifact_class_inventory_is_explicit() {
        let actual = IndexableArtifactClass::all()
            .iter()
            .map(|artifact_class| artifact_class.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            actual,
            vec!["managed-surface", "proposal-artifact", "evidence-bundle", "index-surface",]
        );
    }

    #[test]
    fn lineage_metadata_serde_round_trip() {
        let meta = LineageMetadata {
            contract_version: PROJECT_MEMORY_CONTRACT_VERSION.into(),
            producer: CANON_PRODUCER.into(),
            source_ref: "canon-run:run-abc".into(),
            source_artifacts: vec!["artifact-1.md".into()],
            promotion_state: PromotionState::Auto,
            promoted_at: "2026-05-13T00:00:00Z".into(),
            content_digest: "sha256:abc123".into(),
            mode: Some("architecture".into()),
            stage: Some("architecture".into()),
            owner: Some("Owner <owner@example.com>".into()),
            risk: Some("bounded-impact".into()),
            zone: Some("yellow".into()),
            approval_state: Some("approved".into()),
            packet_readiness: Some("complete".into()),
            promotion_profile: Some(PublishProfile::ProjectMemory),
        };
        let json = serde_json::to_string(&meta).unwrap();
        let back: LineageMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(meta, back);
    }

    #[test]
    fn lineage_metadata_deserializes_legacy_aliases() {
        let legacy_json = r#"{
            "contract_version": "0.1.0",
            "producer": "canon",
            "source_run": "run-abc",
            "source_artifacts": ["artifact-1.md"],
            "promotion_state": "auto-if-approved",
            "published_at": "2026-05-13T00:00:00Z",
            "content_digest": "sha256:legacy",
            "mode": "architecture",
            "approval_state": "Completed",
            "readiness": "complete",
            "profile": "project-memory"
        }"#;

        let back: LineageMetadata = serde_json::from_str(legacy_json).unwrap();
        assert_eq!(back.source_ref, "run-abc");
        assert_eq!(back.promoted_at, "2026-05-13T00:00:00Z");
        assert_eq!(back.promotion_state, PromotionState::AutoIfApproved);
        assert_eq!(back.packet_readiness.as_deref(), Some("complete"));
        assert_eq!(back.promotion_profile, Some(PublishProfile::ProjectMemory));
    }

    #[test]
    fn managed_block_descriptor_renders_v1_marker() {
        let descriptor = ManagedBlockDescriptor::canon("canon-run:R-test");
        assert_eq!(
            descriptor.start_marker(),
            "<!-- project-memory:managed:start producer=\"canon\" source_ref=\"canon-run:R-test\" contract_version=\"v1\" -->"
        );
        assert_eq!(ManagedBlockDescriptor::end_marker(), "<!-- project-memory:managed:end -->");
    }

    #[test]
    fn lineage_field_lists_match_v1_contract() {
        assert_eq!(LineageMetadata::required_field_names(), REQUIRED_V1_LINEAGE_FIELDS);
        assert_eq!(LineageMetadata::optional_field_names(), OPTIONAL_V1_LINEAGE_FIELDS);
    }

    #[test]
    fn promotion_state_surface_targeting() {
        assert!(PromotionState::Auto.targets_stable_surface());
        assert!(PromotionState::AutoIfApproved.targets_stable_surface());
        assert!(!PromotionState::PendingIndex.targets_stable_surface());
        assert!(!PromotionState::IndexOnly.targets_stable_surface());
        assert!(!PromotionState::EvidenceOnly.targets_stable_surface());
        assert!(!PromotionState::Manual.targets_stable_surface());

        assert!(PromotionState::EvidenceOnly.targets_evidence_only());
        assert!(!PromotionState::Auto.targets_evidence_only());

        assert!(PromotionState::PendingIndex.targets_pending_surface());
        assert!(PromotionState::IndexOnly.targets_pending_surface());
        assert!(!PromotionState::Auto.targets_pending_surface());
    }

    #[test]
    fn publish_profiles_policy_serde() {
        let policy = PublishProfilesPolicy {
            contract_version: PROJECT_MEMORY_CONTRACT_VERSION.into(),
            profiles: vec![ModePromotionPolicy {
                mode: "architecture".into(),
                default_promotion_state: PromotionState::AutoIfApproved,
                default_update_strategy: UpdateStrategy::ManagedBlocks,
            }],
        };
        let json = serde_json::to_string(&policy).unwrap();
        let back: PublishProfilesPolicy = serde_json::from_str(&json).unwrap();
        assert_eq!(policy, back);
    }

    #[test]
    fn publish_profile_display() {
        assert_eq!(PublishProfile::ProjectMemory.to_string(), "project-memory");
    }

    #[test]
    fn promotion_state_as_str_and_display() {
        let cases = [
            (PromotionState::Auto, "auto"),
            (PromotionState::AutoIfApproved, "auto-if-approved"),
            (PromotionState::PendingIndex, "pending-index"),
            (PromotionState::IndexOnly, "index-only"),
            (PromotionState::EvidenceOnly, "evidence-only"),
            (PromotionState::Manual, "manual"),
        ];
        for (state, expected) in cases {
            assert_eq!(state.as_str(), expected);
            assert_eq!(state.to_string(), expected);
        }
    }

    #[test]
    fn update_strategy_as_str_and_display() {
        let cases = [
            (UpdateStrategy::ManagedBlocks, "managed-blocks"),
            (UpdateStrategy::ProposalFiles, "proposal-files"),
            (UpdateStrategy::AppendOnlyIndex, "append-only-index"),
        ];
        for (strategy, expected) in cases {
            assert_eq!(strategy.as_str(), expected);
            assert_eq!(strategy.to_string(), expected);
        }
    }

    #[test]
    fn metadata_carrier_serde_round_trip() {
        let json = serde_json::to_string(&ArtifactMetadataCarrier::PacketMetadataSidecar).unwrap();
        assert_eq!(json, "\"packet-metadata-sidecar\"");
        let back: ArtifactMetadataCarrier = serde_json::from_str(&json).unwrap();
        assert_eq!(back, ArtifactMetadataCarrier::PacketMetadataSidecar);
    }

    #[test]
    fn metadata_and_publication_target_strings_are_stable() {
        for (carrier, expected) in [
            (ArtifactMetadataCarrier::ManagedSurfaceEnvelope, "managed-surface-envelope"),
            (ArtifactMetadataCarrier::PacketMetadataSidecar, "packet-metadata-sidecar"),
        ] {
            assert_eq!(carrier.as_str(), expected);
            assert_eq!(carrier.to_string(), expected);
            assert!(!carrier.discovery_rule().is_empty());
        }

        for (artifact_class, expected, carrier) in [
            (
                IndexableArtifactClass::ManagedSurface,
                "managed-surface",
                ArtifactMetadataCarrier::ManagedSurfaceEnvelope,
            ),
            (
                IndexableArtifactClass::ProposalArtifact,
                "proposal-artifact",
                ArtifactMetadataCarrier::PacketMetadataSidecar,
            ),
            (
                IndexableArtifactClass::EvidenceBundle,
                "evidence-bundle",
                ArtifactMetadataCarrier::PacketMetadataSidecar,
            ),
            (
                IndexableArtifactClass::IndexSurface,
                "index-surface",
                ArtifactMetadataCarrier::PacketMetadataSidecar,
            ),
        ] {
            assert_eq!(artifact_class.as_str(), expected);
            assert_eq!(artifact_class.to_string(), expected);
            assert_eq!(artifact_class.metadata_carrier(), carrier);
            assert_eq!(artifact_class.discovery_rule(), carrier.discovery_rule());
        }

        for (target_class, expected) in [
            (PublicationTargetClass::Stable, "stable"),
            (PublicationTargetClass::Pending, "pending"),
            (PublicationTargetClass::Proposal, "proposal"),
            (PublicationTargetClass::Evidence, "evidence"),
            (PublicationTargetClass::Index, "index"),
        ] {
            assert_eq!(target_class.as_str(), expected);
            assert_eq!(target_class.to_string(), expected);
        }
    }

    #[test]
    fn publication_target_class_maps_project_memory_outcomes() {
        assert_eq!(
            PublicationTargetClass::for_publication(
                PromotionState::Auto,
                UpdateStrategy::ManagedBlocks,
            ),
            PublicationTargetClass::Stable
        );
        assert_eq!(
            PublicationTargetClass::for_publication(
                PromotionState::PendingIndex,
                UpdateStrategy::ManagedBlocks,
            ),
            PublicationTargetClass::Pending
        );
        assert_eq!(
            PublicationTargetClass::for_publication(
                PromotionState::PendingIndex,
                UpdateStrategy::ProposalFiles,
            ),
            PublicationTargetClass::Proposal
        );
        assert_eq!(
            PublicationTargetClass::for_publication(
                PromotionState::EvidenceOnly,
                UpdateStrategy::AppendOnlyIndex,
            ),
            PublicationTargetClass::Evidence
        );
        assert_eq!(
            PublicationTargetClass::for_publication(
                PromotionState::IndexOnly,
                UpdateStrategy::AppendOnlyIndex,
            ),
            PublicationTargetClass::Index
        );
        assert_eq!(
            PublicationTargetClass::for_publication(
                PromotionState::Auto,
                UpdateStrategy::AppendOnlyIndex,
            ),
            PublicationTargetClass::Pending
        );
    }

    #[test]
    fn expertise_input_metadata_normalizes_domain_families() {
        let metadata = classify_governed_expertise_input(
            Mode::DomainLanguage,
            vec!["systems".to_string(), String::new(), "systems".to_string()],
        )
        .expect("supported expertise metadata");

        assert_eq!(metadata.expertise_kind, GovernedExpertiseKind::DomainLanguage);
        assert_eq!(metadata.domain_families, vec!["systems".to_string()]);
        assert!(
            classify_governed_expertise_input(Mode::Review, vec!["systems".to_string()]).is_none()
        );
        assert!(classify_governed_expertise_input(Mode::DomainModel, Vec::new()).is_none());

        let unnormalized = ExpertiseInputMetadata {
            expertise_kind: GovernedExpertiseKind::DomainModel,
            domain_families: vec![
                " bounded-contexts ".to_string(),
                String::new(),
                "bounded-contexts".to_string(),
                "services".to_string(),
            ],
        };
        assert_eq!(
            unnormalized.normalized(),
            Some(ExpertiseInputMetadata {
                expertise_kind: GovernedExpertiseKind::DomainModel,
                domain_families: vec!["bounded-contexts".to_string(), "services".to_string(),],
            })
        );

        let empty = ExpertiseInputMetadata {
            expertise_kind: GovernedExpertiseKind::DomainLanguage,
            domain_families: vec!["   ".to_string()],
        };
        assert_eq!(empty.normalized(), None);
    }

    #[test]
    fn adaptive_governance_state_as_str_and_display_cover_all_variants() {
        let cases = [
            (AdaptiveGovernanceState::Advisory, "advisory"),
            (AdaptiveGovernanceState::Catch, "catch"),
            (AdaptiveGovernanceState::Rule, "rule"),
            (AdaptiveGovernanceState::Hook, "hook"),
        ];
        for (state, expected) in cases {
            assert_eq!(state.as_str(), expected);
            assert_eq!(state.to_string(), expected);
        }
    }

    #[test]
    fn adaptive_rollout_profile_as_str_and_display_cover_all_variants() {
        let cases = [
            (AdaptiveRolloutProfile::Minimal, "minimal"),
            (AdaptiveRolloutProfile::Guided, "guided"),
            (AdaptiveRolloutProfile::Governed, "governed"),
            (AdaptiveRolloutProfile::Strict, "strict"),
        ];
        for (profile, expected) in cases {
            assert_eq!(profile.as_str(), expected);
            assert_eq!(profile.to_string(), expected);
        }
    }

    #[test]
    fn semantic_eligibility_state_as_str_and_display_cover_both_variants() {
        assert_eq!(SemanticEligibilityState::Eligible.as_str(), "eligible");
        assert_eq!(SemanticEligibilityState::Excluded.as_str(), "excluded");
        assert_eq!(SemanticEligibilityState::Eligible.to_string(), "eligible");
        assert_eq!(SemanticEligibilityState::Excluded.to_string(), "excluded");
    }

    #[test]
    fn semantic_provenance_boundary_as_str_and_display_cover_all_variants() {
        let cases = [
            (SemanticProvenanceBoundary::Surface, "surface"),
            (SemanticProvenanceBoundary::ManagedBlock, "managed_block"),
            (SemanticProvenanceBoundary::Section, "section"),
        ];
        for (boundary, expected) in cases {
            assert_eq!(boundary.as_str(), expected);
            assert_eq!(boundary.to_string(), expected);
        }
    }

    #[test]
    fn semantic_artifact_descriptor_validate_rejects_empty_contract_line() {
        let descriptor = SemanticArtifactDescriptor {
            semantic_contract_line: "   ".to_string(),
            semantic_eligibility: SemanticEligibilityState::Eligible,
            semantic_provenance_boundary: Some(SemanticProvenanceBoundary::Surface),
            semantic_provenance_ref: Some("tech-docs/architecture.md".to_string()),
            semantic_labels: vec![],
            semantic_exclusion_reason: None,
        };
        let error = descriptor.validate().expect_err("empty contract line should fail");
        assert!(error.contains("semantic_contract_line"));
    }

    #[test]
    fn semantic_artifact_descriptor_validate_rejects_empty_label() {
        let descriptor = SemanticArtifactDescriptor {
            semantic_contract_line: "architecture-v1".to_string(),
            semantic_eligibility: SemanticEligibilityState::Eligible,
            semantic_provenance_boundary: Some(SemanticProvenanceBoundary::Surface),
            semantic_provenance_ref: Some("tech-docs/architecture.md".to_string()),
            semantic_labels: vec!["  ".to_string()], // empty after trim
            semantic_exclusion_reason: None,
        };
        let error = descriptor.validate().expect_err("empty label should fail");
        assert!(error.contains("labels"));
    }

    #[test]
    fn semantic_artifact_descriptor_validate_rejects_empty_provenance_ref() {
        let descriptor = SemanticArtifactDescriptor {
            semantic_contract_line: "architecture-v1".to_string(),
            semantic_eligibility: SemanticEligibilityState::Eligible,
            semantic_provenance_boundary: Some(SemanticProvenanceBoundary::Section),
            semantic_provenance_ref: Some("   ".to_string()), // empty after trim
            semantic_labels: vec![],
            semantic_exclusion_reason: None,
        };
        let error = descriptor.validate().expect_err("empty provenance_ref should fail");
        assert!(error.contains("semantic_provenance_ref"));
    }

    #[test]
    fn semantic_artifact_descriptor_validate_rejects_empty_exclusion_reason() {
        let descriptor = SemanticArtifactDescriptor {
            semantic_contract_line: "architecture-v1".to_string(),
            semantic_eligibility: SemanticEligibilityState::Excluded,
            semantic_provenance_boundary: Some(SemanticProvenanceBoundary::Surface),
            semantic_provenance_ref: Some("tech-docs/architecture.md".to_string()),
            semantic_labels: vec![],
            semantic_exclusion_reason: Some("   ".to_string()), // provided but empty
        };
        let error = descriptor.validate().expect_err("empty exclusion reason should fail");
        assert!(error.contains("exclusion reason"));
    }
}
