use crate::domain::artifact::artifact_slug;
use crate::orchestrator::service::context_parse::truncate_context_excerpt;

use super::shared::{extract_authored_section_or_marker, render_authored_artifact};
use super::{AuthoredSectionSpec, render_markdown};

pub fn render_domain_language_artifact(file_name: &str, brief_summary: &str) -> String {
    let file_name = artifact_slug(file_name);
    let normalized = brief_summary.to_lowercase();
    let domain_scope = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Domain Scope",
        &[],
        &["domain scope"],
    )
    .unwrap_or_else(|| "domain scope not yet authored".to_string());
    let summary = format!(
        "Bounded domain-language packet for {}.",
        truncate_context_excerpt(&domain_scope, 120)
    );

    match file_name {
        "language-overview.md" => render_authored_artifact(
            "Language Overview",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Domain Scope", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Language Maturity", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Upstream Sources", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Downstream Consumers", aliases: &[] },
            ],
        ),
        "domain-glossary.md" => render_authored_artifact(
            "Domain Glossary",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Glossary Entries", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Source References", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Open Gaps", aliases: &[] },
            ],
        ),
        "preferred-language.md" => render_authored_artifact(
            "Preferred Language",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Canonical Terms", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Deprecated Synonyms", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Migration Notes", aliases: &[] },
            ],
        ),
        "language-conflicts.md" => render_authored_artifact(
            "Language Conflicts",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Conflict Inventory", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Resolution Status", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Escalation Triggers", aliases: &[] },
            ],
        ),
        "contextual-meanings.md" => render_authored_artifact(
            "Contextual Meanings",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Context-Dependent Terms", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Disambiguation Rules", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Usage Examples", aliases: &[] },
            ],
        ),
        "business-language-rules.md" => render_authored_artifact(
            "Business Language Rules",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Naming Conventions", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Domain Boundaries", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Enforcement Guidance", aliases: &[] },
            ],
        ),
        "code-and-api-vocabulary.md" => render_authored_artifact(
            "Code And API Vocabulary",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Code Naming Patterns", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "API Surface Terms", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Alignment Gaps", aliases: &[] },
            ],
        ),
        "downstream-language-guidance.md" => render_authored_artifact(
            "Downstream Language Guidance",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Consumer Modes", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Handoff Expectations", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Adoption Risks", aliases: &[] },
            ],
        ),
        "language-decision-record.md" => render_authored_artifact(
            "Language Decision Record",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Decision Drivers", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Options Considered", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Decision Evidence", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Recommendation", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Consequences", aliases: &[] },
            ],
        ),
        "ai-provenance.md" => render_authored_artifact(
            "AI Provenance",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Generation Lineage", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Human Authored Sections", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Confidence Posture", aliases: &[] },
            ],
        ),
        other => render_markdown(other, brief_summary),
    }
}

/// Renders a domain model mode artifact for the given filename slug.
pub fn render_domain_model_artifact(file_name: &str, brief_summary: &str) -> String {
    let file_name = artifact_slug(file_name);
    let normalized = brief_summary.to_lowercase();
    let domain_scope = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Domain Scope",
        &[],
        &["domain scope"],
    )
    .unwrap_or_else(|| "domain scope not yet authored".to_string());
    let summary = format!(
        "Bounded domain-model packet for {}.",
        truncate_context_excerpt(&domain_scope, 120)
    );

    match file_name {
        "model-overview.md" => render_authored_artifact(
            "Model Overview",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Domain Scope", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Model Maturity", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Upstream Sources", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Downstream Consumers", aliases: &[] },
            ],
        ),
        "concept-catalog.md" => render_authored_artifact(
            "Concept Catalog",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Concepts", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Ownership Boundaries", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Open Gaps", aliases: &[] },
            ],
        ),
        "relationship-map.md" => render_authored_artifact(
            "Relationship Map",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Relationships", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Cardinality Rules", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Boundary Crossings", aliases: &[] },
            ],
        ),
        "bounded-context-map.md" => render_authored_artifact(
            "Bounded Context Map",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Bounded Contexts", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Context Relationships", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Integration Seams", aliases: &[] },
            ],
        ),
        "lifecycle-and-state-model.md" => render_authored_artifact(
            "Lifecycle And State Model",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Entity Lifecycles", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "State Transitions", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Invariant Guards", aliases: &[] },
            ],
        ),
        "domain-invariants.md" => render_authored_artifact(
            "Domain Invariants",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Invariants", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Enforcement Points", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Violation Consequences", aliases: &[] },
            ],
        ),
        "policy-and-constraint-rules.md" => render_authored_artifact(
            "Policy And Constraint Rules",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Business Policies", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Constraint Rules", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Exception Handling", aliases: &[] },
            ],
        ),
        "feature-impact-rules.md" => render_authored_artifact(
            "Feature Impact Rules",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Impact Rules", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Affected Concepts", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Downstream Effects", aliases: &[] },
            ],
        ),
        "code-data-alignment.md" => render_authored_artifact(
            "Code Data Alignment",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Code Mapping", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Data Store Mapping", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Alignment Gaps", aliases: &[] },
            ],
        ),
        "model-gaps-and-risks.md" => render_authored_artifact(
            "Model Gaps And Risks",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Model Gaps", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Risk Signals", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Recommended Follow-Ups", aliases: &[] },
            ],
        ),
        "downstream-model-guidance.md" => render_authored_artifact(
            "Downstream Model Guidance",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Consumer Modes", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Handoff Expectations", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Adoption Risks", aliases: &[] },
            ],
        ),
        "domain-model.json" => render_domain_model_json(brief_summary),
        "ai-provenance.md" => render_authored_artifact(
            "AI Provenance",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Generation Lineage", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Human Authored Sections", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Confidence Posture", aliases: &[] },
            ],
        ),
        other => render_markdown(other, brief_summary),
    }
}

fn render_domain_model_json(brief_summary: &str) -> String {
    let normalized = brief_summary.to_lowercase();
    let domain_scope = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Domain Scope",
        &[],
        &["domain scope"],
    )
    .unwrap_or_else(|| "not yet authored".to_string());

    format!(
        "{{\n  \"schema_version\": \"1\",\n  \"domain_scope\": {},\n  \"concepts\": [],\n  \"relationships\": [],\n  \"invariants\": [],\n  \"feature_impact_rules\": []\n}}",
        serde_json::to_string(&truncate_context_excerpt(&domain_scope, 200))
            .unwrap_or_else(|_| "\"\"".to_string())
    )
}
