//! Brief parsing and clarity helpers for inspectable authored modes.
//!
//! Owns mode-specific brief parsers plus the shared clarity-inspection
//! functions (missing context, clarification questions, reasoning signals).

use super::context_parse::{
    condense_context_block, count_markdown_entries, extract_context_list, extract_context_marker,
    first_meaningful_line, infer_discovery_next_phase, render_repo_surface_block,
    split_context_items, truncate_context_excerpt,
};
use super::{ClarificationQuestionSummary, OutputQualitySummary};
use crate::domain::mode::Mode;

// ── Simple list utilities (used by RequirementsBrief and clarity functions) ───

pub(crate) fn default_list(values: Vec<String>, fallback: &str) -> Vec<String> {
    if values.is_empty() { vec![fallback.to_string()] } else { values }
}

pub(crate) fn list_contains_missing_markers(values: &[String]) -> bool {
    values.iter().any(|value| value.contains("NOT CAPTURED"))
}

pub(crate) fn list_contains_missing_decision_markers(values: &[String]) -> bool {
    values.iter().any(|value| value.contains("MISSING AUTHORED DECISION"))
}

pub(crate) fn count_captured_list_items(values: &[String]) -> usize {
    values.iter().filter(|value| !value.contains("NOT CAPTURED")).count()
}

fn has_authored_value(value: &str) -> bool {
    !value.contains("NOT CAPTURED") && !value.contains("MISSING AUTHORED")
}

fn required_context_marker(
    source: &str,
    normalized: &str,
    markers: &[&str],
    fallback: &str,
    max_chars: usize,
) -> String {
    extract_context_marker(source, normalized, markers)
        .map(|value| condense_context_block(&value, max_chars))
        .unwrap_or_else(|| fallback.to_string())
}

// ── SupplyChainAnalysisBrief ────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub(crate) struct SupplyChainAnalysisBrief {
    pub(crate) declared_scope: String,
    pub(crate) licensing_posture: String,
    pub(crate) distribution_model: String,
    pub(crate) ecosystems_in_scope: Vec<String>,
    pub(crate) out_of_scope_components: Vec<String>,
    pub(crate) scanner_decisions: Vec<String>,
    pub(crate) source_refs: Vec<String>,
}

impl SupplyChainAnalysisBrief {
    pub(crate) fn from_context(context_summary: String, source_refs: &[String]) -> Self {
        let normalized = context_summary.to_lowercase();
        let declared_scope = extract_context_marker(
            &context_summary,
            &normalized,
            &["declared scope", "scope"],
        )
        .map(|value| condense_context_block(&value, 320))
        .unwrap_or_else(|| {
            "NOT CAPTURED - Provide a `## Declared Scope` section in the supply-chain input."
                .to_string()
        });
        let licensing_posture = extract_context_marker(
            &context_summary,
            &normalized,
            &["licensing posture", "license posture"],
        )
        .map(|value| condense_context_block(&value, 220))
        .unwrap_or_else(|| {
            "MISSING AUTHORED DECISION - Provide a `## Licensing Posture` section in the supply-chain input."
                .to_string()
        });
        let distribution_model = extract_context_marker(
            &context_summary,
            &normalized,
            &["distribution model"],
        )
        .map(|value| condense_context_block(&value, 220))
        .unwrap_or_else(|| {
            "MISSING AUTHORED DECISION - Provide a `## Distribution Model` section in the supply-chain input."
                .to_string()
        });
        let ecosystems_in_scope = default_list(
            extract_context_list(
                &context_summary,
                &normalized,
                &["ecosystems in scope", "ecosystems", "ecosystem confirmation"],
            ),
            "MISSING AUTHORED DECISION - Provide a `## Ecosystems In Scope` section in the supply-chain input.",
        );
        let out_of_scope_components = default_list(
            extract_context_list(
                &context_summary,
                &normalized,
                &["out of scope components", "out of scope", "excluded"],
            ),
            "MISSING AUTHORED DECISION - Provide a `## Out Of Scope Components` section in the supply-chain input.",
        );
        let scanner_decisions = default_list(
            extract_context_list(
                &context_summary,
                &normalized,
                &["scanner decisions", "non-oss tool policy", "non oss tool policy"],
            ),
            "MISSING AUTHORED DECISION - Provide a `## Scanner Decisions` section that records non-OSS tool policy and any installed, skipped, or replaced scanner choices.",
        );

        Self {
            declared_scope,
            licensing_posture,
            distribution_model,
            ecosystems_in_scope,
            out_of_scope_components,
            scanner_decisions,
            source_refs: source_refs.iter().map(ToString::to_string).collect(),
        }
    }

    pub(crate) fn summary(&self) -> String {
        let mut lines = vec![format!(
            "Declared scope: {}",
            truncate_context_excerpt(&self.declared_scope, 180)
        )];

        if !self.source_refs.is_empty() {
            lines.push(format!("Source inputs: {}", self.source_refs.join(", ")));
        }

        lines.push(format!(
            "Ecosystems in scope: {}",
            count_captured_list_items(&self.ecosystems_in_scope)
        ));

        lines.join("\n")
    }
}

// ── RequirementsBrief ─────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub(crate) struct RequirementsBrief {
    pub(crate) problem: String,
    pub(crate) outcome: String,
    pub(crate) constraints: Vec<String>,
    pub(crate) tradeoffs: Vec<String>,
    pub(crate) out_of_scope: Vec<String>,
    pub(crate) open_questions: Vec<String>,
    pub(crate) source_refs: Vec<String>,
}

impl RequirementsBrief {
    pub(crate) fn from_context(context_summary: String, source_refs: &[String]) -> Self {
        let normalized = context_summary.to_lowercase();
        let problem =
            extract_context_marker(&context_summary, &normalized, &["problem", "intent", "goal"])
                .or_else(|| extract_context_marker(&context_summary, &normalized, &["abstract"]))
                .or_else(|| extract_context_marker(&context_summary, &normalized, &["subject"]))
                .map(|value| condense_context_block(&value, 420))
                .unwrap_or_else(|| {
                    "NOT CAPTURED - Provide a `## Problem` section in the requirements input."
                        .to_string()
                });
        let outcome = extract_context_marker(
            &context_summary,
            &normalized,
            &["outcome", "desired outcome", "success signal", "objective"],
        )
        .map(|value| condense_context_block(&value, 320))
        .unwrap_or_else(|| {
            "NOT CAPTURED - Provide a `## Outcome` section in the requirements input.".to_string()
        });

        let constraints = extract_context_list(
            &context_summary,
            &normalized,
            &["constraints", "constraint", "non-negotiables"],
        );
        let tradeoffs =
            extract_context_list(&context_summary, &normalized, &["tradeoffs", "tradeoff"]);
        let out_of_scope = extract_context_list(
            &context_summary,
            &normalized,
            &["out of scope", "out-of-scope", "scope cuts", "excluded"],
        );
        let open_questions = extract_context_list(
            &context_summary,
            &normalized,
            &["open questions", "questions", "unknowns", "risks"],
        );

        let mut brief = Self {
            problem,
            outcome,
            constraints: default_list(
                constraints,
                "NOT CAPTURED - Provide a `## Constraints` section in the requirements input.",
            ),
            tradeoffs: default_list(
                tradeoffs,
                "NOT CAPTURED - Provide a `## Tradeoffs` section in the requirements input.",
            ),
            out_of_scope: default_list(
                out_of_scope,
                "NOT CAPTURED - Provide a `## Out of Scope` or `## Scope Cuts` section in the requirements input.",
            ),
            open_questions,
            source_refs: source_refs.iter().map(ToString::to_string).collect(),
        };

        if brief.open_questions.is_empty() {
            brief.open_questions.extend(
                prioritized_requirements_clarification_questions(&brief, &context_summary)
                    .into_iter()
                    .take(4)
                    .map(|question| question.prompt),
            );
        }

        if brief.open_questions.is_empty() {
            brief
                .open_questions
                .push("Which downstream mode should consume this packet first?".to_string());
        }

        brief
    }

    pub(crate) fn summary(&self) -> String {
        let mut lines = vec![
            format!("Problem framing: {}", truncate_context_excerpt(&self.problem, 180)),
            format!("Desired outcome: {}", truncate_context_excerpt(&self.outcome, 180)),
        ];

        if !self.source_refs.is_empty() {
            lines.push(format!("Source inputs: {}", self.source_refs.join(", ")));
        }

        lines.join("\n")
    }
}

// ── DiscoveryBrief ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub(crate) struct DiscoveryBrief {
    pub(crate) context_summary: String,
    pub(crate) problem: String,
    pub(crate) constraints: String,
    pub(crate) repo_focus: String,
    pub(crate) unknowns: String,
    pub(crate) next_phase: String,
}

impl DiscoveryBrief {
    pub(crate) fn from_context(context_summary: String, repo_surfaces: &[String]) -> Self {
        let normalized = context_summary.to_lowercase();
        let problem =
            extract_context_marker(&context_summary, &normalized, &["problem", "problem domain"])
                .unwrap_or_else(|| {
                    let line = first_meaningful_line(&context_summary);
                    if line.contains("Bound the problem to") {
                        "NOT CAPTURED - Provide a `## Problem` section in the discovery brief."
                            .to_string()
                    } else {
                        line
                    }
                });
        let constraints = extract_context_marker(
            &context_summary,
            &normalized,
            &["constraints", "constraint", "boundary"],
        )
        .unwrap_or_else(|| {
            "NOT CAPTURED - Provide a `## Constraints` section in the discovery brief.".to_string()
        });
        let repo_focus = extract_context_marker(
            &context_summary,
            &normalized,
            &["repo focus", "repository focus", "current state", "system slice"],
        )
        .unwrap_or_else(|| {
            if repo_surfaces.is_empty() {
                "NOT CAPTURED - Provide a `## Repo Focus` section in the discovery brief."
                    .to_string()
            } else {
                format!(
                    "Ground discovery in these repository surfaces: {}",
                    repo_surfaces.join(", ")
                )
            }
        });
        let unknowns = extract_context_marker(
            &context_summary,
            &normalized,
            &["unknowns", "open questions", "risks"],
        )
        .unwrap_or_else(|| {
            "NOT CAPTURED - Provide an `## Unknowns` section in the discovery brief.".to_string()
        });
        let next_phase = extract_context_marker(
            &context_summary,
            &normalized,
            &["next phase", "handoff", "translation trigger"],
        )
        .unwrap_or_else(|| {
            let inferred = infer_discovery_next_phase(&context_summary);
            if inferred.contains("Translate this packet") {
                "NOT CAPTURED - Provide a `## Next Phase` section in the discovery brief."
                    .to_string()
            } else {
                inferred
            }
        });

        Self { context_summary, problem, constraints, repo_focus, unknowns, next_phase }
    }

    pub(crate) fn generation_prompt(&self, repo_surfaces: &[String]) -> String {
        format!(
            "# Discovery Brief\n\n## Problem\n{}\n\n## Constraints\n{}\n\n## Repo Focus\n{}\n\n## Repo Surface\n{}\n\n## Unknowns\n{}\n\n## Next Phase\n{}",
            self.problem,
            self.constraints,
            self.repo_focus,
            render_repo_surface_block(repo_surfaces),
            self.unknowns,
            self.next_phase,
        )
    }

    pub(crate) fn critique_prompt(
        &self,
        generation_summary: &str,
        repo_surfaces: &[String],
    ) -> String {
        format!(
            "# Discovery Critique Target\n\n## Context Summary\n{}\n\n## Repo Surface\n{}\n\n## Generated Framing\n{}\n\n## Challenge\nCheck whether the generated framing stays anchored to the repository surfaces, preserves the stated constraints, and points to a concrete next governed mode.",
            self.context_summary,
            render_repo_surface_block(repo_surfaces),
            generation_summary,
        )
    }
}

// ── Shared authored-mode clarity ────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AuthoredClarityFamily {
    Planning,
    Execution,
    Assessment,
}

impl AuthoredClarityFamily {
    fn label(self) -> &'static str {
        match self {
            Self::Planning => "planning",
            Self::Execution => "execution",
            Self::Assessment => "assessment",
        }
    }
}

pub(crate) fn authored_clarity_family(mode: Mode) -> AuthoredClarityFamily {
    match mode {
        Mode::SystemShaping | Mode::Architecture | Mode::Change | Mode::Backlog => {
            AuthoredClarityFamily::Planning
        }
        Mode::Implementation | Mode::Refactor | Mode::Migration => AuthoredClarityFamily::Execution,
        Mode::Review
        | Mode::Verification
        | Mode::Incident
        | Mode::SecurityAssessment
        | Mode::SystemAssessment => AuthoredClarityFamily::Assessment,
        Mode::Requirements | Mode::Discovery | Mode::PrReview | Mode::SupplyChainAnalysis => {
            AuthoredClarityFamily::Planning
        }
    }
}

fn authored_primary_markers(family: AuthoredClarityFamily) -> &'static [&'static str] {
    match family {
        AuthoredClarityFamily::Planning => {
            &["system shape", "decision", "delivery intent", "intended change", "problem"]
        }
        AuthoredClarityFamily::Execution => {
            &["task mapping", "refactor scope", "current state", "target state"]
        }
        AuthoredClarityFamily::Assessment => &[
            "review target",
            "claims under test",
            "incident scope",
            "assessment scope",
            "assessment objective",
        ],
    }
}

fn authored_boundary_markers(family: AuthoredClarityFamily) -> &'static [&'static str] {
    match family {
        AuthoredClarityFamily::Planning => &[
            "constraints",
            "boundary decisions",
            "candidate boundaries",
            "change surface",
            "system slice",
            "desired granularity",
            "planning horizon",
            "domain slice",
            "excluded areas",
            "out of scope",
        ],
        AuthoredClarityFamily::Execution => &[
            "bounded changes",
            "mutation bounds",
            "allowed paths",
            "transition boundaries",
            "rollback triggers",
            "fallback paths",
            "refactor scope",
        ],
        AuthoredClarityFamily::Assessment => &[
            "boundary findings",
            "assessment scope",
            "in-scope assets",
            "trust boundaries",
            "incident scope",
            "assessed views",
            "impacted surfaces",
            "scope limits",
            "out of scope",
        ],
    }
}

fn authored_support_markers(family: AuthoredClarityFamily) -> &'static [&'static str] {
    match family {
        AuthoredClarityFamily::Planning => &[
            "rationale",
            "decision evidence",
            "decision drivers",
            "sequencing rationale",
            "source references",
            "validation strategy",
            "independent checks",
        ],
        AuthoredClarityFamily::Execution => &[
            "decision evidence",
            "completion evidence",
            "safety-net evidence",
            "verification checks",
            "independent checks",
            "rollback steps",
            "contract drift",
        ],
        AuthoredClarityFamily::Assessment => &[
            "evidence basis",
            "known facts",
            "observed findings",
            "inferred findings",
            "evidence sources",
            "collection priorities",
            "control families",
            "verified claims",
        ],
    }
}

fn authored_decision_markers(family: AuthoredClarityFamily) -> &'static [&'static str] {
    match family {
        AuthoredClarityFamily::Planning => &[
            "selected boundaries",
            "recommendation",
            "decision record",
            "recommended direction",
            "recommended path",
            "decision",
        ],
        AuthoredClarityFamily::Execution => {
            &["recommendation", "decision", "migration decisions", "approval notes"]
        }
        AuthoredClarityFamily::Assessment => &[
            "final disposition",
            "overall verdict",
            "verification outcome",
            "decision impact",
            "immediate actions",
        ],
    }
}

fn authored_tradeoff_markers(family: AuthoredClarityFamily) -> &'static [&'static str] {
    match family {
        AuthoredClarityFamily::Planning => &[
            "boundary tradeoffs",
            "tradeoffs",
            "consequences",
            "pros",
            "cons",
            "cross-context risks",
            "risk per phase",
        ],
        AuthoredClarityFamily::Execution => &[
            "adoption implications",
            "tradeoff analysis",
            "remaining risks",
            "residual risks",
            "temporary incompatibilities",
        ],
        AuthoredClarityFamily::Assessment => &[
            "accepted risks",
            "reversibility concerns",
            "tradeoffs",
            "likelihood and impact",
            "impact notes",
        ],
    }
}

fn authored_option_markers(family: AuthoredClarityFamily) -> &'static [&'static str] {
    match family {
        AuthoredClarityFamily::Planning => &[
            "structural options",
            "options considered",
            "options",
            "candidate bounded contexts",
            "candidate boundaries",
            "why not the others",
        ],
        AuthoredClarityFamily::Execution => {
            &["candidate frameworks", "options matrix", "parallelizable work", "why not the others"]
        }
        AuthoredClarityFamily::Assessment => &[],
    }
}

fn authored_gap_markers(family: AuthoredClarityFamily) -> &'static [&'static str] {
    match family {
        AuthoredClarityFamily::Planning => &[
            "boundary risks and open questions",
            "unresolved risks",
            "unresolved questions",
            "gaps",
            "open questions",
        ],
        AuthoredClarityFamily::Execution => &[
            "remaining risks",
            "residual risks",
            "deferred decisions",
            "regression findings",
            "feature audit",
        ],
        AuthoredClarityFamily::Assessment => &[
            "missing evidence",
            "evidence gaps",
            "assessment gaps",
            "confidence and unknowns",
            "open findings",
            "risk findings",
            "deferred verification",
            "unobservable surfaces",
        ],
    }
}

fn authored_primary_fallback(family: AuthoredClarityFamily) -> &'static str {
    match family {
        AuthoredClarityFamily::Planning => {
            "NOT CAPTURED - Provide a planning subject such as `## System Shape`, `## Decision`, `## Delivery Intent`, or `## Intended Change`."
        }
        AuthoredClarityFamily::Execution => {
            "NOT CAPTURED - Provide an execution subject such as `## Task Mapping`, `## Refactor Scope`, `## Current State`, or `## Target State`."
        }
        AuthoredClarityFamily::Assessment => {
            "NOT CAPTURED - Provide an assessment target such as `## Review Target`, `## Claims Under Test`, `## Incident Scope`, or `## Assessment Scope`."
        }
    }
}

fn authored_boundary_fallback(family: AuthoredClarityFamily) -> &'static str {
    match family {
        AuthoredClarityFamily::Planning => {
            "NOT CAPTURED - Provide a planning boundary such as `## Constraints`, `## Boundary Decisions`, `## Candidate Boundaries`, or `## Change Surface`."
        }
        AuthoredClarityFamily::Execution => {
            "NOT CAPTURED - Provide a mutation boundary such as `## Bounded Changes`, `## Mutation Bounds`, `## Allowed Paths`, or `## Transition Boundaries`."
        }
        AuthoredClarityFamily::Assessment => {
            "NOT CAPTURED - Provide an assessment boundary such as `## Boundary Findings`, `## Assessment Scope`, `## Assessed Views`, or `## Impacted Surfaces`."
        }
    }
}

fn authored_support_fallback(family: AuthoredClarityFamily) -> &'static str {
    match family {
        AuthoredClarityFamily::Planning => {
            "NOT CAPTURED - Provide explicit rationale or support such as `## Rationale`, `## Decision Evidence`, or `## Decision Drivers`."
        }
        AuthoredClarityFamily::Execution => {
            "NOT CAPTURED - Provide execution support such as `## Decision Evidence`, `## Safety-Net Evidence`, `## Verification Checks`, or `## Rollback Steps`."
        }
        AuthoredClarityFamily::Assessment => {
            "NOT CAPTURED - Provide an evidence basis such as `## Evidence Basis`, `## Known Facts`, `## Observed Findings`, or `## Evidence Sources`."
        }
    }
}

fn authored_decision_fallback(family: AuthoredClarityFamily) -> &'static str {
    match family {
        AuthoredClarityFamily::Planning => {
            "NOT CAPTURED - Provide a `## Recommendation`, `## Selected Boundaries`, or `## Decision` section if the planning packet is already materially closed."
        }
        AuthoredClarityFamily::Execution => {
            "NOT CAPTURED - Provide a `## Recommendation`, `## Decision`, or `## Migration Decisions` section if the execution plan is already decided."
        }
        AuthoredClarityFamily::Assessment => {
            "NOT CAPTURED - Provide a `## Final Disposition`, `## Overall Verdict`, or equivalent conclusion section."
        }
    }
}

fn authored_preserved_fallback() -> &'static str {
    "NOT CAPTURED - Provide a preservation boundary such as `## Preserved Behavior`, `## Guaranteed Compatibility`, or `## Invariant Checks`."
}

fn authored_preserved_markers() -> &'static [&'static str] {
    &[
        "preserved behavior",
        "guaranteed compatibility",
        "coexistence rules",
        "invariant checks",
        "operational constraints",
    ]
}

fn is_authored_gap_question(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.contains('?') {
        return true;
    }

    let normalized = trimmed.to_lowercase();
    [
        "what ", "which ", "how ", "who ", "when ", "where ", "should ", "can ", "is ", "are ",
        "does ", "do ",
    ]
    .iter()
    .any(|prefix| normalized.starts_with(prefix))
}

#[derive(Debug, Clone)]
pub(crate) struct AuthoredModeBrief {
    pub(crate) mode: Mode,
    pub(crate) family: AuthoredClarityFamily,
    pub(crate) primary_subject: String,
    pub(crate) boundary: String,
    pub(crate) support_evidence: String,
    pub(crate) decision_state: String,
    pub(crate) preserved_boundary: String,
    pub(crate) options: Vec<String>,
    pub(crate) tradeoffs: Vec<String>,
    pub(crate) questions_or_gaps: Vec<String>,
    pub(crate) source_refs: Vec<String>,
}

impl AuthoredModeBrief {
    pub(crate) fn from_context(
        mode: Mode,
        context_summary: String,
        source_refs: &[String],
    ) -> Self {
        let family = authored_clarity_family(mode);
        let normalized = context_summary.to_lowercase();

        Self {
            mode,
            family,
            primary_subject: required_context_marker(
                &context_summary,
                &normalized,
                authored_primary_markers(family),
                authored_primary_fallback(family),
                320,
            ),
            boundary: required_context_marker(
                &context_summary,
                &normalized,
                authored_boundary_markers(family),
                authored_boundary_fallback(family),
                320,
            ),
            support_evidence: required_context_marker(
                &context_summary,
                &normalized,
                authored_support_markers(family),
                authored_support_fallback(family),
                320,
            ),
            decision_state: required_context_marker(
                &context_summary,
                &normalized,
                authored_decision_markers(family),
                authored_decision_fallback(family),
                260,
            ),
            preserved_boundary: if matches!(family, AuthoredClarityFamily::Execution) {
                required_context_marker(
                    &context_summary,
                    &normalized,
                    authored_preserved_markers(),
                    authored_preserved_fallback(),
                    260,
                )
            } else {
                "NOT APPLICABLE".to_string()
            },
            options: extract_context_list(
                &context_summary,
                &normalized,
                authored_option_markers(family),
            ),
            tradeoffs: extract_context_list(
                &context_summary,
                &normalized,
                authored_tradeoff_markers(family),
            ),
            questions_or_gaps: extract_context_list(
                &context_summary,
                &normalized,
                authored_gap_markers(family),
            ),
            source_refs: source_refs.iter().map(ToString::to_string).collect(),
        }
    }

    pub(crate) fn materially_closed(&self) -> bool {
        has_authored_value(&self.decision_state) && count_captured_list_items(&self.options) <= 1
    }

    pub(crate) fn weak_reasoning(&self) -> bool {
        if !has_authored_value(&self.boundary) || !has_authored_value(&self.support_evidence) {
            return true;
        }

        match self.family {
            AuthoredClarityFamily::Planning => {
                count_captured_list_items(&self.options) == 0
                    && !has_authored_value(&self.decision_state)
            }
            AuthoredClarityFamily::Execution => !has_authored_value(&self.preserved_boundary),
            AuthoredClarityFamily::Assessment => {
                matches!(self.mode, Mode::Review | Mode::Verification)
                    && !has_authored_value(&self.decision_state)
            }
        }
    }

    pub(crate) fn summary(&self) -> String {
        let mut lines = match self.family {
            AuthoredClarityFamily::Planning => vec![
                format!("Planning focus: {}", truncate_context_excerpt(&self.primary_subject, 180)),
                format!("Boundary: {}", truncate_context_excerpt(&self.boundary, 180)),
                if has_authored_value(&self.decision_state) {
                    format!(
                        "Decision posture: {}",
                        truncate_context_excerpt(&self.decision_state, 180)
                    )
                } else {
                    format!(
                        "Decision posture: {} option signal(s), {} tradeoff signal(s)",
                        count_captured_list_items(&self.options),
                        count_captured_list_items(&self.tradeoffs)
                    )
                },
            ],
            AuthoredClarityFamily::Execution => vec![
                format!(
                    "Execution focus: {}",
                    truncate_context_excerpt(&self.primary_subject, 180)
                ),
                format!("Mutation boundary: {}", truncate_context_excerpt(&self.boundary, 180)),
                format!(
                    "Preservation posture: {}",
                    truncate_context_excerpt(&self.preserved_boundary, 180)
                ),
            ],
            AuthoredClarityFamily::Assessment => vec![
                format!(
                    "Assessment target: {}",
                    truncate_context_excerpt(&self.primary_subject, 180)
                ),
                format!("Scope boundary: {}", truncate_context_excerpt(&self.boundary, 180)),
                if has_authored_value(&self.decision_state) {
                    format!(
                        "Review posture: {}",
                        truncate_context_excerpt(&self.decision_state, 180)
                    )
                } else {
                    format!(
                        "Evidence basis: {}",
                        truncate_context_excerpt(&self.support_evidence, 180)
                    )
                },
            ],
        };

        if !self.source_refs.is_empty() {
            lines.push(format!("Source inputs: {}", self.source_refs.join(", ")));
        }

        lines.join("\n")
    }
}

pub(crate) fn authored_mode_missing_context(brief: &AuthoredModeBrief) -> Vec<String> {
    let mut missing = Vec::new();

    if !has_authored_value(&brief.primary_subject) {
        missing.push(match brief.family {
            AuthoredClarityFamily::Planning => {
                "Planning intent is missing; Canon cannot tell what bounded problem or decision this packet is meant to drive.".to_string()
            }
            AuthoredClarityFamily::Execution => {
                "Execution target is missing; Canon cannot tell which bounded change, refactor, or migration surface this packet controls.".to_string()
            }
            AuthoredClarityFamily::Assessment => {
                "Assessment target is missing; Canon cannot tell what claim, incident, or system slice this packet is evaluating.".to_string()
            }
        });
    }

    if !has_authored_value(&brief.boundary) {
        missing.push(match brief.family {
            AuthoredClarityFamily::Planning => {
                "Planning boundary is missing; the packet does not yet state the scope, slice, or constraint Canon must preserve.".to_string()
            }
            AuthoredClarityFamily::Execution => {
                "Mutation boundary is missing; execution output would otherwise overreach beyond the authored scope.".to_string()
            }
            AuthoredClarityFamily::Assessment => {
                "Assessment boundary is missing; the packet does not yet say which evidence surface is actually in scope.".to_string()
            }
        });
    }

    if !has_authored_value(&brief.support_evidence) {
        missing.push(match brief.family {
            AuthoredClarityFamily::Planning => {
                "Planning support is missing; the packet has no explicit rationale or decision evidence anchoring its direction.".to_string()
            }
            AuthoredClarityFamily::Execution => {
                "Execution evidence is missing; the packet lacks safety-net, rollback, or validation support for the proposed work.".to_string()
            }
            AuthoredClarityFamily::Assessment => {
                "Evidence basis is missing; assessment output would otherwise rely on inferred confidence instead of authored support.".to_string()
            }
        });
    }

    match brief.family {
        AuthoredClarityFamily::Planning => {
            if count_captured_list_items(&brief.options) == 0
                && !has_authored_value(&brief.decision_state)
            {
                missing.push(
                    "Decision posture is shallow; the packet names neither viable options nor a materially closed direction.".to_string(),
                );
            } else if count_captured_list_items(&brief.tradeoffs) == 0 && !brief.materially_closed()
            {
                missing.push(
                    "Tradeoff evidence is missing; planning output would otherwise overstate certainty.".to_string(),
                );
            }
        }
        AuthoredClarityFamily::Execution => {
            if !has_authored_value(&brief.preserved_boundary) {
                missing.push(
                    "Preserved behavior or compatibility boundary is missing; execution output would weaken change-preservation guarantees.".to_string(),
                );
            }
        }
        AuthoredClarityFamily::Assessment => {
            if matches!(brief.mode, Mode::Review | Mode::Verification)
                && !has_authored_value(&brief.decision_state)
            {
                missing.push(
                    "Disposition or verdict is missing; review-family output would otherwise imply a conclusion that is not yet authored.".to_string(),
                );
            }
        }
    }

    missing
}

pub(crate) fn prioritized_authored_mode_clarification_questions(
    brief: &AuthoredModeBrief,
) -> Vec<ClarificationQuestionSummary> {
    let mut questions = Vec::new();

    if !has_authored_value(&brief.primary_subject) {
        let prompt = match brief.family {
            AuthoredClarityFamily::Planning => {
                "What concrete planning target or decision should this packet drive?"
            }
            AuthoredClarityFamily::Execution => {
                "Which implementation, refactor, or migration surface is actually in scope?"
            }
            AuthoredClarityFamily::Assessment => {
                "What exact review, verification, incident, or assessment target is under examination?"
            }
        };

        push_clarification_question(
            &mut questions,
            "clarify-authored-target",
            prompt,
            "Without a bounded target, Canon cannot tell whether the packet is actually reasoning about the intended surface.",
            authored_primary_fallback(brief.family),
        );
    }

    if !has_authored_value(&brief.boundary) {
        let prompt = match brief.family {
            AuthoredClarityFamily::Planning => {
                "Which boundary, slice, or scope limit must this packet preserve?"
            }
            AuthoredClarityFamily::Execution => {
                "Which paths, mutation bounds, or transition boundaries are explicitly allowed?"
            }
            AuthoredClarityFamily::Assessment => {
                "Which evidence surfaces are in scope, and which ones are explicitly excluded?"
            }
        };

        push_clarification_question(
            &mut questions,
            "clarify-authored-boundary",
            prompt,
            "Boundaries keep the packet honest about what Canon is allowed to interpret or recommend.",
            authored_boundary_fallback(brief.family),
        );
    }

    if !has_authored_value(&brief.support_evidence) {
        let prompt = match brief.family {
            AuthoredClarityFamily::Planning => {
                "What explicit rationale or evidence supports the chosen planning direction?"
            }
            AuthoredClarityFamily::Execution => {
                "What safety-net, validation, or rollback evidence makes this execution plan safe?"
            }
            AuthoredClarityFamily::Assessment => {
                "What evidence basis supports this packet instead of inferred reasoning?"
            }
        };

        push_clarification_question(
            &mut questions,
            "clarify-authored-support",
            prompt,
            "Without explicit support, the packet risks sounding more grounded than it actually is.",
            authored_support_fallback(brief.family),
        );
    }

    match brief.family {
        AuthoredClarityFamily::Planning => {
            if count_captured_list_items(&brief.options) == 0
                && !has_authored_value(&brief.decision_state)
            {
                if matches!(brief.mode, Mode::Architecture) {
                    push_architecture_clarification_question(
                        &mut questions,
                        "clarify-authored-decision-posture",
                        "Which options were considered, or is the decision already materially closed?",
                        "Architecture packets should either preserve viable alternatives or say directly that the decision is already closed.",
                        authored_decision_fallback(brief.family),
                    );
                } else {
                    push_clarification_question(
                        &mut questions,
                        "clarify-authored-decision-posture",
                        "Which options were considered, or is the decision already materially closed?",
                        "Planning packets should either preserve viable alternatives or say directly that the decision is already closed.",
                        authored_decision_fallback(brief.family),
                    );
                }
            }

            if matches!(brief.mode, Mode::Architecture)
                && count_captured_list_items(&brief.tradeoffs) == 0
                && !brief.materially_closed()
            {
                push_architecture_clarification_question(
                    &mut questions,
                    "clarify-authored-tradeoffs",
                    "Which tradeoffs would actually change the architectural choice?",
                    "Architecture clarification should stay limited to tradeoffs that could change the structural decision or next-mode routing.",
                    "No authored `## Pros`, `## Cons`, `## Tradeoffs`, or equivalent tradeoff section was detected in the supplied architecture brief.",
                );
            }
        }
        AuthoredClarityFamily::Execution => {
            if !has_authored_value(&brief.preserved_boundary) {
                push_clarification_question(
                    &mut questions,
                    "clarify-authored-preservation",
                    "Which behavior, compatibility, or invariant guarantees must remain intact?",
                    "Execution packets need an explicit preservation boundary before they can claim grounded reasoning.",
                    authored_preserved_fallback(),
                );
            }
        }
        AuthoredClarityFamily::Assessment => {
            if matches!(brief.mode, Mode::Review | Mode::Verification)
                && !has_authored_value(&brief.decision_state)
            {
                push_clarification_question(
                    &mut questions,
                    "clarify-authored-disposition",
                    "What disposition or verdict is actually justified by the authored evidence?",
                    "Review-family packets should not imply approval, contradiction, or rejection without an authored conclusion.",
                    authored_decision_fallback(brief.family),
                );
            }
        }
    }

    for (index, gap) in brief.questions_or_gaps.iter().enumerate() {
        if !is_authored_gap_question(gap) {
            continue;
        }

        let prompt = question_prompt(gap);
        if matches!(brief.mode, Mode::Architecture) {
            push_architecture_clarification_question(
                &mut questions,
                &format!("authored-gap-question-{}", index + 1),
                &prompt,
                "This unresolved architecture question is already authored in the packet and should stay visible before a governed run starts.",
                "Captured from the authored gaps, open questions, or evidence-gap section.",
            );
        } else {
            push_clarification_question(
                &mut questions,
                &format!("authored-gap-question-{}", index + 1),
                &prompt,
                "This unresolved question is already authored in the packet and should stay visible before a governed run starts.",
                "Captured from the authored gaps, open questions, or evidence-gap section.",
            );
        }
    }

    questions.truncate(5);
    questions
}

pub(crate) fn authored_mode_reasoning_signals(
    source_inputs: &[String],
    brief: &AuthoredModeBrief,
) -> Vec<String> {
    let mut signals = Vec::new();

    signals.push(format!(
        "Detected {} authored input surface(s): {}.",
        source_inputs.len(),
        if source_inputs.is_empty() {
            "no-authored-source-inputs-recorded".to_string()
        } else {
            source_inputs.join(", ")
        }
    ));
    signals.push(match brief.family {
        AuthoredClarityFamily::Planning => format!(
            "Captured {} option signal(s), {} tradeoff signal(s), and {} gap or question signal(s).",
            count_captured_list_items(&brief.options),
            count_captured_list_items(&brief.tradeoffs),
            count_captured_list_items(&brief.questions_or_gaps)
        ),
        AuthoredClarityFamily::Execution => format!(
            "Captured preservation posture `{}`, {} tradeoff signal(s), and {} gap or risk signal(s).",
            if has_authored_value(&brief.preserved_boundary) {
                truncate_context_excerpt(&brief.preserved_boundary, 72)
            } else {
                "NOT CAPTURED".to_string()
            },
            count_captured_list_items(&brief.tradeoffs),
            count_captured_list_items(&brief.questions_or_gaps)
        ),
        AuthoredClarityFamily::Assessment => format!(
            "Captured {} authored gap signal(s) with review posture `{}`.",
            count_captured_list_items(&brief.questions_or_gaps),
            if has_authored_value(&brief.decision_state) {
                truncate_context_excerpt(&brief.decision_state, 72)
            } else {
                "NOT CAPTURED".to_string()
            }
        ),
    });

    if brief.materially_closed() {
        signals.push(format!(
            "The authored packet already materially closes the decision around `{}`; Canon should preserve that closure instead of inventing balanced alternatives.",
            truncate_context_excerpt(&brief.decision_state, 96)
        ));
    } else if brief.weak_reasoning() {
        signals.push(match brief.family {
            AuthoredClarityFamily::Planning => {
                "Headings are present but the planning packet still lacks enough boundary, evidence, or decision support to justify strong reasoning.".to_string()
            }
            AuthoredClarityFamily::Execution => {
                "The execution packet still lacks enough preservation or safety-net support to read as grounded reasoning.".to_string()
            }
            AuthoredClarityFamily::Assessment => {
                "The assessment packet still lacks enough evidence or conclusion support to read as review-ready reasoning.".to_string()
            }
        });
    } else {
        signals.push(match brief.family {
            AuthoredClarityFamily::Planning => {
                "The planning packet contains bounded intent, explicit support, and enough decision posture to reason without inventing extra balance.".to_string()
            }
            AuthoredClarityFamily::Execution => {
                "The execution packet contains bounded scope, preservation posture, and supporting evidence for grounded reasoning.".to_string()
            }
            AuthoredClarityFamily::Assessment => {
                "The assessment packet contains a bounded target, evidence basis, and review posture strong enough for grounded reasoning.".to_string()
            }
        });
    }

    signals
}

pub(crate) fn clarity_output_quality(
    materially_closed: bool,
    missing_context: &[String],
    clarification_questions: &[ClarificationQuestionSummary],
    reasoning_signals: &[String],
) -> OutputQualitySummary {
    let posture = if !missing_context.is_empty() {
        "structurally-complete"
    } else if clarification_questions.is_empty() {
        "publishable"
    } else {
        "materially-useful"
    };

    let mut evidence_signals = reasoning_signals.iter().take(2).cloned().collect::<Vec<_>>();
    if materially_closed {
        evidence_signals.push(
            "The authored packet is materially closed around one viable direction and keeps that closure explicit.".to_string(),
        );
    } else if missing_context.is_empty() {
        evidence_signals.push(
            "No explicit missing-context markers remain in the inspected packet.".to_string(),
        );
    }
    if evidence_signals.is_empty() {
        evidence_signals.push(
            "The inspect surface computed a bounded output-quality posture from the authored packet."
                .to_string(),
        );
    }

    let mut downgrade_reasons = missing_context.to_vec();
    if !clarification_questions.is_empty() {
        downgrade_reasons.push(format!(
            "{} clarification question(s) remain before Canon should treat this packet as unambiguously publishable.",
            clarification_questions.len()
        ));
    }

    OutputQualitySummary {
        posture: posture.to_string(),
        materially_closed,
        evidence_signals,
        downgrade_reasons,
    }
}

pub(crate) fn authored_mode_output_quality(
    brief: &AuthoredModeBrief,
    missing_context: &[String],
    clarification_questions: &[ClarificationQuestionSummary],
    reasoning_signals: &[String],
) -> OutputQualitySummary {
    clarity_output_quality(
        brief.materially_closed(),
        missing_context,
        clarification_questions,
        reasoning_signals,
    )
}

pub(crate) fn authored_mode_recommended_focus(
    brief: &AuthoredModeBrief,
    missing_context: &[String],
    clarification_questions: &[ClarificationQuestionSummary],
) -> String {
    if !missing_context.is_empty() {
        if matches!(brief.mode, Mode::Architecture)
            && let Some(reroute) = architecture_reroute_guidance(brief)
        {
            return reroute;
        }

        return match brief.family {
            AuthoredClarityFamily::Planning => {
                "Resolve the missing planning boundaries and evidence before treating this packet as reasoning-ready downstream input.".to_string()
            }
            AuthoredClarityFamily::Execution => {
                "Resolve the missing execution boundary, preservation, and safety-net support before using this packet to justify mutation.".to_string()
            }
            AuthoredClarityFamily::Assessment => {
                "Resolve the missing evidence or scope boundaries before treating this packet as reviewable assessment output.".to_string()
            }
        };
    }

    if brief.materially_closed() {
        return "The authored packet already materially closes the decision; preserve that closure and its supporting evidence instead of manufacturing extra balance.".to_string();
    }

    if !clarification_questions.is_empty() {
        if matches!(brief.mode, Mode::Architecture) {
            return "Answer the remaining decision-changing architecture questions, or let their documented defaults carry forward into `readiness-assessment.md`, before starting architecture mode.".to_string();
        }

        return format!(
            "Review the remaining authored {} questions before starting {} mode.",
            brief.family.label(),
            brief.mode.as_str()
        );
    }

    format!(
        "No critical clarification questions detected; the authored packet is bounded enough for {} mode.",
        brief.mode.as_str()
    )
}

// ── Clarification question helpers ────────────────────────────────────────────

pub(crate) fn push_clarification_question(
    questions: &mut Vec<ClarificationQuestionSummary>,
    id: &str,
    prompt: &str,
    rationale: &str,
    evidence: &str,
) {
    push_clarification_question_with_metadata(
        questions,
        id,
        prompt,
        rationale,
        evidence,
        ClarificationQuestionMetadata {
            affects: "packet readiness",
            default_if_skipped: "Keep the item visible as unresolved authored context before downstream reasoning.",
            status: "required",
        },
    );
}

fn push_architecture_clarification_question(
    questions: &mut Vec<ClarificationQuestionSummary>,
    id: &str,
    prompt: &str,
    rationale: &str,
    evidence: &str,
) {
    let metadata = architecture_question_metadata(id);
    push_clarification_question_with_metadata(questions, id, prompt, rationale, evidence, metadata);
}

struct ClarificationQuestionMetadata<'a> {
    affects: &'a str,
    default_if_skipped: &'a str,
    status: &'a str,
}

fn push_clarification_question_with_metadata(
    questions: &mut Vec<ClarificationQuestionSummary>,
    id: &str,
    prompt: &str,
    rationale: &str,
    evidence: &str,
    metadata: ClarificationQuestionMetadata<'_>,
) {
    if questions.iter().any(|question| question.prompt.eq_ignore_ascii_case(prompt)) {
        return;
    }

    questions.push(ClarificationQuestionSummary {
        id: id.to_string(),
        prompt: prompt.to_string(),
        rationale: rationale.to_string(),
        evidence: evidence.to_string(),
        affects: metadata.affects.to_string(),
        default_if_skipped: metadata.default_if_skipped.to_string(),
        status: metadata.status.to_string(),
    });
}

fn architecture_question_metadata(id: &str) -> ClarificationQuestionMetadata<'static> {
    match id {
        "clarify-authored-target" => ClarificationQuestionMetadata {
            affects: "recommended next mode",
            default_if_skipped: "Reroute to discovery until the problem and decision surface are bounded enough for architecture tradeoffs.",
            status: "required",
        },
        "clarify-authored-boundary" => ClarificationQuestionMetadata {
            affects: "architecture-decisions.md and context-map.md",
            default_if_skipped: "Reroute to requirements until the architecture boundary and scope limits are explicit.",
            status: "required",
        },
        "clarify-authored-support" => ClarificationQuestionMetadata {
            affects: "readiness-assessment.md",
            default_if_skipped: "Keep the packet conditional in readiness-assessment.md instead of treating the decision as fully grounded.",
            status: "required",
        },
        "clarify-authored-decision-posture" | "clarify-authored-tradeoffs" => {
            ClarificationQuestionMetadata {
                affects: "tradeoff-matrix.md",
                default_if_skipped: "Keep the structural options unresolved and record the missing tradeoff as a readiness blocker.",
                status: "required",
            }
        }
        _ if id.starts_with("authored-gap-question-") => ClarificationQuestionMetadata {
            affects: "readiness-assessment.md",
            default_if_skipped: "Carry the unanswered item forward into readiness-assessment.md as an unresolved question.",
            status: "required",
        },
        _ => ClarificationQuestionMetadata {
            affects: "architecture readiness",
            default_if_skipped: "Keep the gap visible as unresolved authored context before downstream reasoning.",
            status: "required",
        },
    }
}

fn architecture_reroute_guidance(brief: &AuthoredModeBrief) -> Option<String> {
    if !has_authored_value(&brief.primary_subject) {
        return Some(
            "Architecture mode is not ready yet; reroute to discovery until the problem and decision surface are bounded enough to compare structural options.".to_string(),
        );
    }

    if !has_authored_value(&brief.boundary) || !has_authored_value(&brief.support_evidence) {
        return Some(
            "Architecture mode is not ready yet; reroute to requirements until the scope, constraints, and supporting rationale are explicit.".to_string(),
        );
    }

    if count_captured_list_items(&brief.options) == 0
        && count_captured_list_items(&brief.tradeoffs) == 0
        && !has_authored_value(&brief.decision_state)
    {
        return Some(
            "Architecture mode is not ready yet; reroute to system-shaping until the candidate boundaries and structural options are explicit enough to compare.".to_string(),
        );
    }

    None
}

pub(crate) fn is_default_requirements_open_question(question: &str) -> bool {
    question.eq_ignore_ascii_case("Which downstream mode should consume this packet first?")
}

pub(crate) fn question_prompt(question: &str) -> String {
    let trimmed = question.trim().trim_end_matches('.');
    if trimmed.ends_with('?') { trimmed.to_string() } else { format!("{trimmed}?") }
}

// ── Requirements clarity ──────────────────────────────────────────────────────

pub(crate) fn requirements_missing_context(brief: &RequirementsBrief) -> Vec<String> {
    let mut missing = Vec::new();

    if brief.problem.contains("NOT CAPTURED") {
        missing.push(
            "Problem framing is missing explicit authored intent or operator goal.".to_string(),
        );
    }
    if brief.outcome.contains("NOT CAPTURED") {
        missing.push(
            "Outcome framing is missing an explicit success signal or bounded result.".to_string(),
        );
    }
    if list_contains_missing_markers(&brief.constraints) {
        missing.push(
            "Constraints are incomplete; downstream shaping would lack explicit non-negotiables."
                .to_string(),
        );
    }
    if list_contains_missing_markers(&brief.tradeoffs) {
        missing.push(
            "Tradeoffs are incomplete; option evaluation would drift toward generic guidance."
                .to_string(),
        );
    }
    if list_contains_missing_markers(&brief.out_of_scope) {
        missing.push(
            "Scope cuts are incomplete; the packet does not yet name explicit exclusions."
                .to_string(),
        );
    }

    missing
}

pub(crate) fn prioritized_requirements_clarification_questions(
    brief: &RequirementsBrief,
    context_summary: &str,
) -> Vec<ClarificationQuestionSummary> {
    let mut questions = Vec::new();
    let first_line = first_meaningful_line(context_summary);

    if brief.problem.contains("NOT CAPTURED") {
        push_clarification_question(
            &mut questions,
            "clarify-problem",
            "What bounded operator or engineering problem should this requirements packet frame?",
            "Without an explicit problem statement, later modes will optimize for the wrong boundary.",
            &format!("Current intake starts with: {first_line}"),
        );
    }
    if brief.outcome.contains("NOT CAPTURED") {
        push_clarification_question(
            &mut questions,
            "clarify-outcome",
            "What explicit outcome or success signal should this packet preserve?",
            "A requirements packet needs a bounded success condition before tradeoffs or exclusions make sense.",
            "No authored `## Outcome` or equivalent success section was detected in the supplied inputs.",
        );
    }
    if list_contains_missing_markers(&brief.constraints) {
        push_clarification_question(
            &mut questions,
            "clarify-constraints",
            "Which constraints are non-negotiable for this work?",
            "Constraints determine whether downstream shaping stays repo-specific instead of becoming generic planning advice.",
            "No authored `## Constraints`, `## Constraint`, or `## Non-Negotiables` section was detected in the supplied inputs.",
        );
    }
    if list_contains_missing_markers(&brief.tradeoffs) {
        push_clarification_question(
            &mut questions,
            "clarify-tradeoffs",
            "Which tradeoffs are acceptable, and which ones are explicitly rejected?",
            "Tradeoffs anchor option evaluation and keep the packet honest about what the team is willing to sacrifice.",
            "No authored `## Tradeoffs` section was detected in the supplied inputs.",
        );
    }
    if list_contains_missing_markers(&brief.out_of_scope) {
        push_clarification_question(
            &mut questions,
            "clarify-scope-cuts",
            "What is explicitly out of scope or deferred for this packet?",
            "Scope cuts keep the packet bounded and prevent later modes from inventing extra work.",
            "No authored `## Out of Scope`, `## Scope Cuts`, or equivalent exclusions section was detected in the supplied inputs.",
        );
    }

    for (index, question) in brief.open_questions.iter().enumerate() {
        if question.contains("NOT CAPTURED") || is_default_requirements_open_question(question) {
            continue;
        }

        let prompt = question_prompt(question);
        push_clarification_question(
            &mut questions,
            &format!("authored-open-question-{}", index + 1),
            &prompt,
            "This question is already explicit in the supplied brief and should be resolved before the packet is treated as stable downstream input.",
            "Captured from the authored open-questions or unknowns section.",
        );
    }

    questions.truncate(5);
    questions
}

pub(crate) fn requirements_reasoning_signals(
    source_inputs: &[String],
    brief: &RequirementsBrief,
) -> Vec<String> {
    vec![
        format!(
            "Detected {} authored input surface(s): {}.",
            source_inputs.len(),
            if source_inputs.is_empty() {
                "no-authored-source-inputs-recorded".to_string()
            } else {
                source_inputs.join(", ")
            }
        ),
        format!(
            "Captured {} constraint point(s), {} tradeoff point(s), {} scope cut(s), and {} open question(s).",
            count_captured_list_items(&brief.constraints),
            count_captured_list_items(&brief.tradeoffs),
            count_captured_list_items(&brief.out_of_scope),
            count_captured_list_items(&brief.open_questions)
        ),
        if brief.problem.contains("NOT CAPTURED") || brief.outcome.contains("NOT CAPTURED") {
            "The problem/outcome pair is still incomplete, so downstream design would rely on interpretation instead of authored intent.".to_string()
        } else {
            "The problem/outcome pair is explicit enough to bound a requirements packet before downstream mode selection.".to_string()
        },
    ]
}

// ── Discovery clarity ─────────────────────────────────────────────────────────

pub(crate) fn discovery_summary(brief: &DiscoveryBrief) -> String {
    format!(
        "Problem framing: {}\nConstraints: {}\nRepo focus: {}\nNext phase: {}",
        truncate_context_excerpt(&brief.problem, 180),
        truncate_context_excerpt(&brief.constraints, 180),
        truncate_context_excerpt(&brief.repo_focus, 180),
        truncate_context_excerpt(&brief.next_phase, 180),
    )
}

pub(crate) fn discovery_missing_context(brief: &DiscoveryBrief) -> Vec<String> {
    let mut missing = Vec::new();

    if brief.problem.contains("NOT CAPTURED") {
        missing.push(
            "Problem framing is missing; discovery still needs an explicit problem domain."
                .to_string(),
        );
    }
    if brief.constraints.contains("NOT CAPTURED") {
        missing.push(
            "Constraints are missing; discovery does not yet name the boundary it must preserve."
                .to_string(),
        );
    }
    if brief.repo_focus.contains("NOT CAPTURED") {
        missing.push(
            "Repository focus is missing; discovery is not yet anchored to concrete repo surfaces."
                .to_string(),
        );
    }
    if brief.unknowns.contains("NOT CAPTURED") {
        missing.push(
            "Unknowns are missing; discovery does not yet name the unresolved decision pressure points."
                .to_string(),
        );
    }
    if brief.next_phase.contains("NOT CAPTURED") {
        missing.push(
            "Next-phase handoff is missing; discovery does not yet say which downstream mode should consume the packet."
                .to_string(),
        );
    }

    missing
}

pub(crate) fn prioritized_discovery_clarification_questions(
    brief: &DiscoveryBrief,
) -> Vec<ClarificationQuestionSummary> {
    let mut questions = Vec::new();

    if brief.problem.contains("NOT CAPTURED") {
        push_clarification_question(
            &mut questions,
            "clarify-discovery-problem",
            "What exact problem domain should discovery bound before handoff?",
            "Discovery needs a named problem domain so later packets do not drift across unrelated repository surfaces.",
            "No authored `## Problem` or `## Problem Domain` section was detected in the supplied discovery brief.",
        );
    }
    if brief.constraints.contains("NOT CAPTURED") {
        push_clarification_question(
            &mut questions,
            "clarify-discovery-constraints",
            "Which explicit constraints or boundary rules must discovery preserve?",
            "Constraints keep the discovery packet honest about what later modes are allowed to change or assume.",
            "No authored `## Constraints` or equivalent boundary section was detected in the supplied discovery brief.",
        );
    }
    if brief.repo_focus.contains("NOT CAPTURED") {
        push_clarification_question(
            &mut questions,
            "clarify-discovery-repo-focus",
            "Which repository surfaces, modules, or files should discovery stay anchored to?",
            "Repo focus determines whether discovery remains grounded in the actual workspace instead of generic planning language.",
            "No authored `## Repo Focus`, `## Repository Focus`, or `## System Slice` section was detected in the supplied discovery brief.",
        );
    }
    if brief.unknowns.contains("NOT CAPTURED") {
        push_clarification_question(
            &mut questions,
            "clarify-discovery-unknowns",
            "Which unknowns or decision risks should discovery make explicit before handoff?",
            "Discovery becomes weaker when it names a boundary but not the unresolved questions that still matter.",
            "No authored `## Unknowns` or `## Open Questions` section was detected in the supplied discovery brief.",
        );
    }
    if brief.next_phase.contains("NOT CAPTURED") {
        push_clarification_question(
            &mut questions,
            "clarify-discovery-next-phase",
            "Which downstream governed mode should consume this packet next, and under what trigger?",
            "A discovery packet needs a concrete translation path so the result remains actionable.",
            "No authored `## Next Phase`, `## Handoff`, or `## Translation Trigger` section was detected in the supplied discovery brief.",
        );
    }

    if !brief.unknowns.contains("NOT CAPTURED") {
        for (index, question) in split_context_items(&brief.unknowns).into_iter().enumerate() {
            if question.contains("NOT CAPTURED") {
                continue;
            }

            let prompt = question_prompt(&question);
            push_clarification_question(
                &mut questions,
                &format!("authored-discovery-question-{}", index + 1),
                &prompt,
                "This unknown is already explicit in the discovery brief and should stay visible before downstream translation.",
                "Captured from the authored unknowns or open-questions section.",
            );
        }
    }

    questions.truncate(5);
    questions
}

// ── Supply-chain clarity ────────────────────────────────────────────────────

pub(crate) fn supply_chain_analysis_missing_context(
    brief: &SupplyChainAnalysisBrief,
) -> Vec<String> {
    let mut missing = Vec::new();

    if brief.declared_scope.contains("NOT CAPTURED") {
        missing.push(
            "Declared scope is missing; the packet is not yet bounded to a specific manifest or repository surface."
                .to_string(),
        );
    }
    if brief.licensing_posture.contains("MISSING AUTHORED DECISION") {
        missing.push(
            "Licensing posture is unresolved; Canon must not guess whether the project is commercial, permissive OSS, copyleft OSS, or mixed."
                .to_string(),
        );
    }
    if brief.distribution_model.contains("MISSING AUTHORED DECISION") {
        missing.push(
            "Distribution model is unresolved; Canon must not guess whether the analyzed dependencies ship externally or remain internal-only."
                .to_string(),
        );
    }
    if list_contains_missing_decision_markers(&brief.ecosystems_in_scope) {
        missing.push(
            "Ecosystem scope is unresolved; the packet needs an explicit keep-or-remove decision for the detected ecosystems."
                .to_string(),
        );
    }
    if list_contains_missing_decision_markers(&brief.out_of_scope_components) {
        missing.push(
            "Out-of-scope components are unresolved; vendored, generated, or third-party exclusions still need explicit confirmation."
                .to_string(),
        );
    }
    if list_contains_missing_decision_markers(&brief.scanner_decisions) {
        missing.push(
            "Scanner policy is unresolved; the packet still needs non-OSS tool-policy guidance and any install, skip, or replacement decisions."
                .to_string(),
        );
    }

    missing
}

pub(crate) fn prioritized_supply_chain_analysis_clarification_questions(
    brief: &SupplyChainAnalysisBrief,
) -> Vec<ClarificationQuestionSummary> {
    let mut questions = Vec::new();

    if brief.licensing_posture.contains("MISSING AUTHORED DECISION") {
        push_clarification_question(
            &mut questions,
            "clarify-licensing-posture",
            "What licensing posture governs this repository surface: commercial, oss-permissive, oss-copyleft, or mixed?",
            "License compatibility and obligations cannot be analyzed credibly until the governing posture is explicit.",
            "`## Licensing Posture` is missing from the authored supply-chain brief.",
        );
    }
    if brief.distribution_model.contains("MISSING AUTHORED DECISION") {
        push_clarification_question(
            &mut questions,
            "clarify-distribution-model",
            "Is the analyzed dependency surface distributed externally or used only internally?",
            "Distribution model changes which obligations and release-facing risks matter.",
            "`## Distribution Model` is missing from the authored supply-chain brief.",
        );
    }
    if list_contains_missing_decision_markers(&brief.ecosystems_in_scope) {
        push_clarification_question(
            &mut questions,
            "clarify-ecosystem-scope",
            "Which detected ecosystems remain in scope for this packet, and which should be removed from scope?",
            "Scanner selection and coverage-gap reporting depend on an explicit ecosystem boundary.",
            "`## Ecosystems In Scope` is missing from the authored supply-chain brief.",
        );
    }
    if list_contains_missing_decision_markers(&brief.out_of_scope_components) {
        push_clarification_question(
            &mut questions,
            "clarify-exclusions",
            "Which vendored, generated, or third-party components are explicitly out of scope for this packet?",
            "Without explicit exclusions, the packet may overstate or understate the bounded review surface.",
            "`## Out Of Scope Components` is missing from the authored supply-chain brief.",
        );
    }
    if list_contains_missing_decision_markers(&brief.scanner_decisions) {
        push_clarification_question(
            &mut questions,
            "clarify-tool-policy",
            "Are non-OSS scanner proposals allowed if OSS tooling cannot cover a required capability?",
            "Canon must keep missing-scanner guidance inside the user's stated tool-policy boundary.",
            "`## Scanner Decisions` is missing from the authored supply-chain brief.",
        );
    }

    questions
}

pub(crate) fn supply_chain_analysis_reasoning_signals(
    source_inputs: &[String],
    brief: &SupplyChainAnalysisBrief,
) -> Vec<String> {
    let mut signals = Vec::new();

    signals.push(format!("Source inputs inspected: {}", source_inputs.join(", ")));
    signals.push(format!(
        "Captured ecosystems in scope: {}",
        count_captured_list_items(&brief.ecosystems_in_scope)
    ));

    if brief.licensing_posture.contains("MISSING AUTHORED DECISION") {
        signals
            .push("Licensing posture still requires explicit maintainer confirmation.".to_string());
    }
    if brief.distribution_model.contains("MISSING AUTHORED DECISION") {
        signals.push(
            "Distribution model still requires explicit maintainer confirmation.".to_string(),
        );
    }
    if list_contains_missing_decision_markers(&brief.scanner_decisions) {
        signals.push("Scanner policy remains incomplete; missing-scanner guidance would otherwise drift outside the stated tool policy.".to_string());
    }

    signals
}

pub(crate) fn discovery_reasoning_signals(
    source_inputs: &[String],
    repo_surfaces: &[String],
    brief: &DiscoveryBrief,
) -> Vec<String> {
    vec![
        format!(
            "Detected {} authored input surface(s): {}.",
            source_inputs.len(),
            if source_inputs.is_empty() {
                "no-authored-source-inputs-recorded".to_string()
            } else {
                source_inputs.join(", ")
            }
        ),
        format!(
            "Mapped {} repository surface hint(s) for discovery anchoring.",
            repo_surfaces.len()
        ),
        format!(
            "Captured {} unknown or open-question item(s) and inferred next phase `{}`.",
            if brief.unknowns.contains("NOT CAPTURED") {
                0
            } else {
                count_markdown_entries(&brief.unknowns)
            },
            truncate_context_excerpt(&brief.next_phase, 96)
        ),
    ]
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn well_formed_requirements_brief() -> RequirementsBrief {
        RequirementsBrief {
            problem: "Reduce auth latency.".to_string(),
            outcome: "P99 auth latency under 50 ms.".to_string(),
            constraints: vec!["No breaking API changes.".to_string()],
            tradeoffs: vec!["Cache consistency vs latency.".to_string()],
            out_of_scope: vec!["UI changes.".to_string()],
            open_questions: vec!["Which cache backend?".to_string()],
            source_refs: vec!["canon-input/requirements.md".to_string()],
        }
    }

    fn empty_requirements_brief() -> RequirementsBrief {
        RequirementsBrief {
            problem: "NOT CAPTURED - missing.".to_string(),
            outcome: "NOT CAPTURED - missing.".to_string(),
            constraints: vec!["NOT CAPTURED - missing.".to_string()],
            tradeoffs: vec!["NOT CAPTURED - missing.".to_string()],
            out_of_scope: vec!["NOT CAPTURED - missing.".to_string()],
            open_questions: Vec::new(),
            source_refs: Vec::new(),
        }
    }

    #[test]
    fn requirements_missing_context_returns_empty_for_complete_brief() {
        let brief = well_formed_requirements_brief();
        assert!(requirements_missing_context(&brief).is_empty());
    }

    #[test]
    fn requirements_missing_context_lists_all_gaps() {
        let brief = empty_requirements_brief();
        let missing = requirements_missing_context(&brief);
        assert!(missing.len() >= 3, "should detect multiple missing items: {missing:?}");
        assert!(missing.iter().any(|m| m.contains("Problem")));
        assert!(missing.iter().any(|m| m.contains("Outcome")));
    }

    #[test]
    fn prioritized_requirements_clarification_questions_capped_at_five() {
        let brief = empty_requirements_brief();
        let questions = prioritized_requirements_clarification_questions(&brief, "");
        assert!(questions.len() <= 5);
    }

    #[test]
    fn question_prompt_adds_question_mark_when_missing() {
        assert_eq!(question_prompt("What is the problem"), "What is the problem?");
        assert_eq!(question_prompt("What is the problem?"), "What is the problem?");
    }

    #[test]
    fn push_clarification_question_deduplicates_by_prompt() {
        let mut questions = Vec::new();
        push_clarification_question(&mut questions, "id-1", "What is the problem?", "r", "e");
        push_clarification_question(&mut questions, "id-2", "What is the problem?", "r2", "e2");
        assert_eq!(questions.len(), 1);
        assert_eq!(questions[0].status, "required");
    }

    #[test]
    fn requirements_brief_summary_includes_problem_and_outcome() {
        let brief = well_formed_requirements_brief();
        let summary = brief.summary();
        assert!(summary.contains("Problem framing:"));
        assert!(summary.contains("Desired outcome:"));
        assert!(summary.contains("Source inputs:"));
    }

    #[test]
    fn discovery_missing_context_returns_empty_for_complete_brief() {
        let brief = DiscoveryBrief {
            context_summary: "context".to_string(),
            problem: "Real problem.".to_string(),
            constraints: "Must not break auth.".to_string(),
            repo_focus: "crates/canon-engine".to_string(),
            unknowns: "Unknown concurrency model.".to_string(),
            next_phase: "Move to architecture.".to_string(),
        };
        assert!(discovery_missing_context(&brief).is_empty());
    }

    #[test]
    fn discovery_missing_context_detects_not_captured_fields() {
        let brief = DiscoveryBrief {
            context_summary: "context".to_string(),
            problem: "NOT CAPTURED - missing".to_string(),
            constraints: "NOT CAPTURED - missing".to_string(),
            repo_focus: "focused".to_string(),
            unknowns: "some unknowns".to_string(),
            next_phase: "requirements".to_string(),
        };
        let missing = discovery_missing_context(&brief);
        assert_eq!(missing.len(), 2);
    }

    #[test]
    fn requirements_reasoning_signals_produces_three_items() {
        let brief = well_formed_requirements_brief();
        let signals = requirements_reasoning_signals(&["canon-input/reqs.md".to_string()], &brief);
        assert_eq!(signals.len(), 3);
        assert!(signals[0].contains("1 authored input surface"));
    }

    #[test]
    fn count_captured_list_items_counts_non_not_captured() {
        let items = vec![
            "real item".to_string(),
            "NOT CAPTURED - missing".to_string(),
            "another real".to_string(),
        ];
        assert_eq!(count_captured_list_items(&items), 2);
    }

    #[test]
    fn list_contains_missing_markers_detects_not_captured() {
        assert!(list_contains_missing_markers(&["NOT CAPTURED - x".to_string()]));
        assert!(!list_contains_missing_markers(&["valid item".to_string()]));
    }

    #[test]
    fn default_list_returns_fallback_for_empty_vec() {
        let result = default_list(Vec::new(), "fallback value");
        assert_eq!(result, vec!["fallback value".to_string()]);
    }

    #[test]
    fn default_list_returns_original_when_non_empty() {
        let result = default_list(vec!["item".to_string()], "fallback");
        assert_eq!(result, vec!["item".to_string()]);
    }

    #[test]
    fn authored_mode_reasoning_signals_detect_materially_closed_decisions() {
        let brief = AuthoredModeBrief {
            mode: Mode::Architecture,
            family: AuthoredClarityFamily::Planning,
            primary_subject: "Split artifact rendering from summary posture.".to_string(),
            boundary: "Keep existing .canon schema unchanged.".to_string(),
            support_evidence: "Shared posture avoids drift across modes.".to_string(),
            decision_state: "Use shared posture helpers in the runtime layer.".to_string(),
            preserved_boundary: "NOT APPLICABLE".to_string(),
            options: vec!["Shared helper".to_string()],
            tradeoffs: vec!["Less per-mode wording freedom.".to_string()],
            questions_or_gaps: Vec::new(),
            source_refs: vec!["canon-input/architecture.md".to_string()],
        };

        let signals = authored_mode_reasoning_signals(&brief.source_refs, &brief);
        assert!(signals.iter().any(|signal| signal.contains("materially closes the decision")));
    }

    #[test]
    fn clarity_output_quality_distinguishes_structural_and_publishable_posture() {
        let weak = clarity_output_quality(
            false,
            &["Planning support is missing.".to_string()],
            &[],
            &["Detected 1 authored input surface(s): canon-input/change.md.".to_string()],
        );
        assert_eq!(weak.posture, "structurally-complete");
        assert!(!weak.downgrade_reasons.is_empty());

        let publishable = clarity_output_quality(
            true,
            &[],
            &[],
            &["Detected 1 authored input surface(s): architecture.md.".to_string()],
        );
        assert_eq!(publishable.posture, "publishable");
        assert!(publishable.materially_closed);
        assert!(!publishable.evidence_signals.is_empty());
    }

    #[test]
    fn authored_mode_missing_context_flags_execution_preservation_gaps() {
        let brief = AuthoredModeBrief {
            mode: Mode::Implementation,
            family: AuthoredClarityFamily::Execution,
            primary_subject: "Implement shared clarity helpers.".to_string(),
            boundary: "crates/canon-engine/src/orchestrator/service".to_string(),
            support_evidence: "inspect_clarity targeted tests".to_string(),
            decision_state: "Use shared authored-mode parsing.".to_string(),
            preserved_boundary: authored_preserved_fallback().to_string(),
            options: Vec::new(),
            tradeoffs: Vec::new(),
            questions_or_gaps: Vec::new(),
            source_refs: vec!["canon-input/implementation.md".to_string()],
        };

        let missing = authored_mode_missing_context(&brief);
        assert!(missing.iter().any(|item| item.contains("Preserved behavior")));
    }

    #[test]
    fn architecture_reroute_guidance_prefers_discovery_for_unbounded_briefs() {
        let brief = AuthoredModeBrief {
            mode: Mode::Architecture,
            family: AuthoredClarityFamily::Planning,
            primary_subject: authored_primary_fallback(AuthoredClarityFamily::Planning).to_string(),
            boundary: authored_boundary_fallback(AuthoredClarityFamily::Planning).to_string(),
            support_evidence: authored_support_fallback(AuthoredClarityFamily::Planning)
                .to_string(),
            decision_state: authored_decision_fallback(AuthoredClarityFamily::Planning).to_string(),
            preserved_boundary: "NOT APPLICABLE".to_string(),
            options: Vec::new(),
            tradeoffs: Vec::new(),
            questions_or_gaps: Vec::new(),
            source_refs: vec!["architecture.md".to_string()],
        };

        let guidance = architecture_reroute_guidance(&brief).expect("reroute guidance");
        assert!(guidance.contains("discovery"));
    }

    #[test]
    fn supply_chain_analysis_brief_from_context_preserves_authored_sections() {
        let source_refs = vec!["canon-input/supply-chain-analysis.md".to_string()];
        let brief = SupplyChainAnalysisBrief::from_context(
            "# Supply Chain Analysis Brief\n\n## Declared Scope\nCargo manifests under crates/ and GitHub Actions workflows.\n\n## Licensing Posture\noss-permissive\n\n## Distribution Model\nexternal distribution\n\n## Ecosystems In Scope\n- cargo\n- github actions\n\n## Out Of Scope Components\n- vendored ui assets\n\n## Scanner Decisions\n- prefer OSS scanners first\n"
                .to_string(),
            &source_refs,
        );

        assert_eq!(
            brief.declared_scope,
            "Cargo manifests under crates/ and GitHub Actions workflows."
        );
        assert_eq!(brief.licensing_posture, "oss-permissive");
        assert_eq!(brief.distribution_model, "external distribution");
        assert_eq!(brief.ecosystems_in_scope.len(), 2);
        assert_eq!(brief.out_of_scope_components, vec!["vendored ui assets".to_string()]);
        assert_eq!(brief.scanner_decisions, vec!["prefer OSS scanners first".to_string()]);

        let summary = brief.summary();
        assert!(summary.contains("Declared scope:"));
        assert!(summary.contains("Source inputs: canon-input/supply-chain-analysis.md"));
        assert!(summary.contains("Ecosystems in scope: 2"));
    }

    #[test]
    fn supply_chain_analysis_questions_cover_unresolved_decisions() {
        let brief = SupplyChainAnalysisBrief::from_context(
            "# Supply Chain Analysis Brief\n\n## Declared Scope\nCargo manifests under crates/.\n"
                .to_string(),
            &[],
        );

        let missing = supply_chain_analysis_missing_context(&brief);
        assert_eq!(missing.len(), 5);
        assert!(missing.iter().any(|item| item.contains("Licensing posture is unresolved")));
        assert!(missing.iter().any(|item| item.contains("Distribution model is unresolved")));
        assert!(missing.iter().any(|item| item.contains("Ecosystem scope is unresolved")));
        assert!(missing.iter().any(|item| item.contains("Scanner policy is unresolved")));

        let questions = prioritized_supply_chain_analysis_clarification_questions(&brief);
        assert_eq!(questions.len(), 5);
        assert!(questions.iter().any(|question| {
            question.prompt.contains("What licensing posture governs this repository surface")
        }));
        assert!(
            questions.iter().any(|question| {
                question.prompt.contains("Are non-OSS scanner proposals allowed")
            })
        );
    }

    #[test]
    fn supply_chain_analysis_reasoning_signals_surface_incomplete_policy_markers() {
        let source_refs = vec!["supply-chain-analysis.md".to_string()];
        let brief = SupplyChainAnalysisBrief::from_context(
            "# Supply Chain Analysis Brief\n\n## Declared Scope\nCargo manifests only.\n"
                .to_string(),
            &source_refs,
        );

        let signals = supply_chain_analysis_reasoning_signals(&source_refs, &brief);
        assert!(
            signals
                .iter()
                .any(|signal| signal.contains("Source inputs inspected: supply-chain-analysis.md"))
        );
        assert!(signals.iter().any(|signal| signal.contains("Captured ecosystems in scope: 1")));
        assert!(signals.iter().any(|signal| {
            signal.contains("Licensing posture still requires explicit maintainer confirmation")
        }));
        assert!(signals.iter().any(|signal| {
            signal.contains("Distribution model still requires explicit maintainer confirmation")
        }));
        assert!(signals.iter().any(|signal| signal.contains("Scanner policy remains incomplete")));
    }

    #[test]
    fn requirements_brief_from_context_backfills_open_questions_when_none_are_authored() {
        let source_refs = vec!["idea.md".to_string()];
        let brief = RequirementsBrief::from_context(
            "# Requirements Brief\n\n## Problem\nReduce auth latency.\n\n## Outcome\nP99 auth latency under 50 ms.\n".to_string(),
            &source_refs,
        );

        assert_eq!(brief.source_refs, source_refs);
        assert_eq!(brief.problem, "Reduce auth latency.");
        assert_eq!(brief.outcome, "P99 auth latency under 50 ms.");
        assert!(brief.constraints[0].contains("Provide a `## Constraints` section"));
        assert!(brief.tradeoffs[0].contains("Provide a `## Tradeoffs` section"));
        assert!(!brief.open_questions.is_empty());
        assert!(brief.open_questions.len() <= 4);
    }

    #[test]
    fn discovery_brief_from_context_uses_repo_surface_fallbacks_and_prompt_helpers() {
        let repo_surfaces = vec!["crates/canon-engine/src/orchestrator/service".to_string()];
        let brief = DiscoveryBrief::from_context(
            "Need to bound orchestrator coverage before broad runtime work.\n\n## Constraints\nStay inside direct runtime tests first.\n\n## Unknowns\nWhich slices still need direct coverage?\n".to_string(),
            &repo_surfaces,
        );

        assert_eq!(brief.problem, "Need to bound orchestrator coverage before broad runtime work.");
        assert!(brief.repo_focus.contains("crates/canon-engine/src/orchestrator/service"));
        assert!(
            brief.next_phase.contains("Provide a `## Next Phase` section")
                || !brief.next_phase.is_empty()
        );

        let generation = brief.generation_prompt(&repo_surfaces);
        assert!(generation.contains("## Repo Surface"));
        assert!(generation.contains("Stay inside direct runtime tests first."));

        let critique = brief.critique_prompt("Generated discovery summary.", &repo_surfaces);
        assert!(critique.contains("## Generated Framing"));
        assert!(critique.contains("Check whether the generated framing stays anchored"));
    }

    #[test]
    fn authored_mode_brief_for_review_uses_assessment_family_and_detects_weak_reasoning() {
        let brief = AuthoredModeBrief::from_context(
            Mode::Review,
            "# Review Brief\n\n## Review Target\nAuth session rollback readiness.\n\n## Assessment Scope\nAuth session boundary only.\n\n## Evidence Basis\nContract tests and rollback rehearsal notes.\n\n## Evidence Gaps\n- Should the review reject the packet until rollback rehearsal is refreshed?\n"
                .to_string(),
            &["review.md".to_string()],
        );

        assert_eq!(brief.family, AuthoredClarityFamily::Assessment);
        assert!(brief.summary().contains("Assessment target: Auth session rollback readiness."));
        assert!(brief.summary().contains("Scope boundary: Auth session boundary only."));
        assert!(brief.weak_reasoning());
        assert!(!brief.materially_closed());
    }

    #[test]
    fn prioritized_authored_mode_clarification_questions_for_review_add_disposition_and_gap_items()
    {
        let brief = AuthoredModeBrief {
            mode: Mode::Review,
            family: AuthoredClarityFamily::Assessment,
            primary_subject: "Auth session rollback readiness.".to_string(),
            boundary: "Auth session boundary only.".to_string(),
            support_evidence: "Contract tests and rollback rehearsal notes.".to_string(),
            decision_state: authored_decision_fallback(AuthoredClarityFamily::Assessment)
                .to_string(),
            preserved_boundary: "NOT APPLICABLE".to_string(),
            options: Vec::new(),
            tradeoffs: Vec::new(),
            questions_or_gaps: vec![
                "Should the review reject the packet until rollback rehearsal is refreshed?"
                    .to_string(),
            ],
            source_refs: vec!["review.md".to_string()],
        };

        let questions = prioritized_authored_mode_clarification_questions(&brief);
        assert!(questions.iter().any(|question| question.id == "clarify-authored-disposition"));
        assert!(questions.iter().any(|question| {
            question.prompt.contains(
                "Should the review reject the packet until rollback rehearsal is refreshed",
            )
        }));
    }

    #[test]
    fn authored_mode_recommended_focus_handles_materially_closed_and_question_only_packets() {
        let materially_closed = AuthoredModeBrief {
            mode: Mode::Architecture,
            family: AuthoredClarityFamily::Planning,
            primary_subject: "Split artifact rendering from runtime posture.".to_string(),
            boundary: "Keep the runtime schema unchanged.".to_string(),
            support_evidence: "Existing packets already share the same runtime contract."
                .to_string(),
            decision_state: "Use shared posture helpers in the runtime layer.".to_string(),
            preserved_boundary: "NOT APPLICABLE".to_string(),
            options: vec!["Use shared posture helpers.".to_string()],
            tradeoffs: vec!["Less per-mode wording freedom.".to_string()],
            questions_or_gaps: Vec::new(),
            source_refs: vec!["architecture.md".to_string()],
        };
        assert!(
            authored_mode_recommended_focus(&materially_closed, &[], &[])
                .contains("materially closes the decision")
        );

        let question_only = AuthoredModeBrief {
            mode: Mode::Change,
            family: AuthoredClarityFamily::Planning,
            primary_subject: "Bound the auth repository slice.".to_string(),
            boundary: "Auth service and repository only.".to_string(),
            support_evidence: "Contract checks already protect audit ordering.".to_string(),
            decision_state: authored_decision_fallback(AuthoredClarityFamily::Planning).to_string(),
            preserved_boundary: "NOT APPLICABLE".to_string(),
            options: vec!["Keep repository helper local to auth.".to_string()],
            tradeoffs: vec!["Some duplication remains inside auth.".to_string()],
            questions_or_gaps: vec!["Should cleanup rollout stay in the same slice?".to_string()],
            source_refs: vec!["change.md".to_string()],
        };
        let questions = prioritized_authored_mode_clarification_questions(&question_only);
        assert!(authored_mode_missing_context(&question_only).is_empty());
        assert!(authored_mode_recommended_focus(&question_only, &[], &questions).contains(
            "Review the remaining authored planning questions before starting change mode"
        ));
    }
}
