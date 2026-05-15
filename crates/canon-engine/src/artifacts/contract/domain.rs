/// Artifact requirements for domain-class modes: DomainLanguage, DomainModel.
use crate::domain::artifact::{ArtifactFormat, ArtifactRequirement};
use crate::domain::gate::GateKind;

use super::sections::*;
use super::{requirement, requirement_with_format};

// ── Artifact file-name constants ──────────────────────────────────────────────

// DomainLanguage
const LANGUAGE_OVERVIEW_MD: &str = "language-overview.md";
const DOMAIN_GLOSSARY_MD: &str = "domain-glossary.md";
const PREFERRED_LANGUAGE_MD: &str = "preferred-language.md";
const LANGUAGE_CONFLICTS_MD: &str = "language-conflicts.md";
const CONTEXTUAL_MEANINGS_MD: &str = "contextual-meanings.md";
const BUSINESS_LANGUAGE_RULES_MD: &str = "business-language-rules.md";
const CODE_AND_API_VOCABULARY_MD: &str = "code-and-api-vocabulary.md";
const DOWNSTREAM_LANGUAGE_GUIDANCE_MD: &str = "downstream-language-guidance.md";
const LANGUAGE_DECISION_RECORD_MD: &str = "language-decision-record.md";
const DOMAIN_LANGUAGE_AI_PROVENANCE_MD: &str = "ai-provenance.md";

// DomainModel
const MODEL_OVERVIEW_MD: &str = "model-overview.md";
const CONCEPT_CATALOG_MD: &str = "concept-catalog.md";
const RELATIONSHIP_MAP_MD: &str = "relationship-map.md";
const BOUNDED_CONTEXT_MAP_MD: &str = "bounded-context-map.md";
const LIFECYCLE_AND_STATE_MODEL_MD: &str = "lifecycle-and-state-model.md";
const DOMAIN_INVARIANTS_MD: &str = "domain-invariants.md";
const POLICY_AND_CONSTRAINT_RULES_MD: &str = "policy-and-constraint-rules.md";
const FEATURE_IMPACT_RULES_MD: &str = "feature-impact-rules.md";
const CODE_DATA_ALIGNMENT_MD: &str = "code-data-alignment.md";
const MODEL_GAPS_AND_RISKS_MD: &str = "model-gaps-and-risks.md";
const DOWNSTREAM_MODEL_GUIDANCE_MD: &str = "downstream-model-guidance.md";
const DOMAIN_MODEL_JSON: &str = "domain-model.json";
const DOMAIN_MODEL_AI_PROVENANCE_MD: &str = "ai-provenance.md";

// ── Mode contracts ────────────────────────────────────────────────────────────

/// Returns the artifact requirements for the [`DomainLanguage`](crate::domain::mode::Mode::DomainLanguage) mode.
pub(super) fn domain_language() -> Vec<ArtifactRequirement> {
    vec![
        requirement(
            LANGUAGE_OVERVIEW_MD,
            &[SUMMARY, DOMAIN_SCOPE, "Language Maturity", UPSTREAM_SOURCES, DOWNSTREAM_CONSUMERS],
            &[GateKind::Risk, GateKind::Architecture],
        ),
        requirement(
            DOMAIN_GLOSSARY_MD,
            &[SUMMARY, "Glossary Entries", "Source References", "Open Gaps"],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            PREFERRED_LANGUAGE_MD,
            &[SUMMARY, "Canonical Terms", "Deprecated Synonyms", "Migration Notes"],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            LANGUAGE_CONFLICTS_MD,
            &[SUMMARY, "Conflict Inventory", "Resolution Status", "Escalation Triggers"],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
        requirement(
            CONTEXTUAL_MEANINGS_MD,
            &[SUMMARY, "Context-Dependent Terms", "Disambiguation Rules", "Usage Examples"],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            BUSINESS_LANGUAGE_RULES_MD,
            &[SUMMARY, "Naming Conventions", "Domain Boundaries", "Enforcement Guidance"],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            CODE_AND_API_VOCABULARY_MD,
            &[SUMMARY, "Code Naming Patterns", "API Surface Terms", "Alignment Gaps"],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            DOWNSTREAM_LANGUAGE_GUIDANCE_MD,
            &[SUMMARY, CONSUMER_MODES, HANDOFF_EXPECTATIONS, ADOPTION_RISKS],
            &[GateKind::ReleaseReadiness],
        ),
        requirement(
            LANGUAGE_DECISION_RECORD_MD,
            &[
                SUMMARY,
                DECISION_DRIVERS,
                "Options Considered",
                DECISION_EVIDENCE,
                RECOMMENDATION,
                CONSEQUENCES,
            ],
            &[GateKind::Architecture, GateKind::Risk],
        ),
        requirement(
            DOMAIN_LANGUAGE_AI_PROVENANCE_MD,
            &[SUMMARY, GENERATION_LINEAGE, HUMAN_AUTHORED_SECTIONS, CONFIDENCE_POSTURE],
            &[GateKind::ReleaseReadiness],
        ),
    ]
}

/// Returns the artifact requirements for the [`DomainModel`](crate::domain::mode::Mode::DomainModel) mode.
pub(super) fn domain_model() -> Vec<ArtifactRequirement> {
    vec![
        requirement(
            MODEL_OVERVIEW_MD,
            &[SUMMARY, DOMAIN_SCOPE, "Model Maturity", UPSTREAM_SOURCES, DOWNSTREAM_CONSUMERS],
            &[GateKind::Risk, GateKind::Architecture],
        ),
        requirement(
            CONCEPT_CATALOG_MD,
            &[SUMMARY, "Concepts", "Ownership Boundaries", "Open Gaps"],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            RELATIONSHIP_MAP_MD,
            &[SUMMARY, "Relationships", "Cardinality Rules", "Boundary Crossings"],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            BOUNDED_CONTEXT_MAP_MD,
            &[SUMMARY, "Bounded Contexts", "Context Relationships", "Integration Seams"],
            &[GateKind::Architecture, GateKind::Risk],
        ),
        requirement(
            LIFECYCLE_AND_STATE_MODEL_MD,
            &[SUMMARY, "Entity Lifecycles", "State Transitions", "Invariant Guards"],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            DOMAIN_INVARIANTS_MD,
            &[SUMMARY, "Invariants", "Enforcement Points", "Violation Consequences"],
            &[GateKind::Architecture, GateKind::Risk],
        ),
        requirement(
            POLICY_AND_CONSTRAINT_RULES_MD,
            &[SUMMARY, "Business Policies", "Constraint Rules", "Exception Handling"],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            FEATURE_IMPACT_RULES_MD,
            &[SUMMARY, "Impact Rules", "Affected Concepts", "Downstream Effects"],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
        requirement(
            CODE_DATA_ALIGNMENT_MD,
            &[SUMMARY, "Code Mapping", "Data Store Mapping", "Alignment Gaps"],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            MODEL_GAPS_AND_RISKS_MD,
            &[SUMMARY, "Model Gaps", "Risk Signals", "Recommended Follow-Ups"],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
        requirement(
            DOWNSTREAM_MODEL_GUIDANCE_MD,
            &[SUMMARY, CONSUMER_MODES, HANDOFF_EXPECTATIONS, ADOPTION_RISKS],
            &[GateKind::ReleaseReadiness],
        ),
        requirement_with_format(
            DOMAIN_MODEL_JSON,
            ArtifactFormat::Json,
            &[],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            DOMAIN_MODEL_AI_PROVENANCE_MD,
            &[SUMMARY, GENERATION_LINEAGE, HUMAN_AUTHORED_SECTIONS, CONFIDENCE_POSTURE],
            &[GateKind::ReleaseReadiness],
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_language_has_expected_artifact_count() {
        assert_eq!(domain_language().len(), 10);
    }

    #[test]
    fn domain_language_primary_is_overview() {
        assert_eq!(domain_language()[0].file_name, LANGUAGE_OVERVIEW_MD);
    }

    #[test]
    fn domain_language_all_artifacts_are_required() {
        assert!(domain_language().iter().all(|r| r.required));
    }

    #[test]
    fn domain_model_has_expected_artifact_count() {
        assert_eq!(domain_model().len(), 13);
    }

    #[test]
    fn domain_model_primary_is_overview() {
        assert_eq!(domain_model()[0].file_name, MODEL_OVERVIEW_MD);
    }

    #[test]
    fn domain_model_includes_json_artifact() {
        assert!(domain_model().iter().any(|r| r.file_name == DOMAIN_MODEL_JSON));
    }

    #[test]
    fn domain_model_json_artifact_is_required() {
        let json = domain_model().into_iter().find(|r| r.file_name == DOMAIN_MODEL_JSON).unwrap();
        assert!(json.required);
    }
}
