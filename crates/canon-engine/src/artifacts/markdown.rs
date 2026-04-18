use crate::review::findings::{FindingCategory, ReviewFinding, ReviewPacket};
use crate::review::summary::{ReviewSummary, summary_severity_label};

pub fn render_markdown(title: &str, summary: &str) -> String {
    format!("# {title}\n\n## Summary\n\n{summary}\n")
}

pub fn render_requirements_artifact(file_name: &str, idea_summary: &str) -> String {
    match file_name {
        "problem-statement.md" => format!(
            "# Problem Statement\n\n## Summary\n\n{idea_summary}\n\n## Problem\n\nThe team needs a bounded statement of work before AI-assisted generation expands the solution space.\n\n## Boundary\n\nThis run is limited to framing the problem, constraints, and decision surface for the proposed change.\n\n## Success Signal\n\nStakeholders can decide whether to proceed using explicit constraints, exclusions, and recorded tradeoffs.\n"
        ),
        "constraints.md" => format!(
            "# Constraints\n\n## Summary\n\n{idea_summary}\n\n## Constraints\n\n- Keep the implementation local-first and auditable.\n- Preserve explicit human ownership and approval checkpoints.\n- Prefer filesystem persistence over transient chat memory.\n\n## Non-Negotiables\n\n- Risk and zone classification happen before generation.\n- Artifacts must remain inspectable and reusable across later steps.\n"
        ),
        "options.md" => format!(
            "# Options\n\n## Summary\n\n{idea_summary}\n\n## Options\n\n1. Deliver a minimal governed CLI focused on requirements artifacts first.\n2. Expand into a broader mode surface only after the governance spine is stable.\n\n## Recommended Path\n\nStart with the narrow CLI slice so the method, artifacts, and gates are trustworthy before deeper execution modes are added.\n"
        ),
        "tradeoffs.md" => format!(
            "# Tradeoffs\n\n## Summary\n\n{idea_summary}\n\n## Tradeoffs\n\n- Favoring governability reduces raw generation speed.\n- Durable artifacts add upfront structure but improve reviewability.\n- Typed modes constrain flexibility in exchange for safer execution.\n\n## Consequences\n\nThe product will feel opinionated by design, and that opinionation is what keeps acceleration reviewable.\n"
        ),
        "scope-cuts.md" => format!(
            "# Scope Cuts\n\n## Summary\n\n{idea_summary}\n\n## Scope Cuts\n\n- No autonomous multi-agent orchestration in v0.1.\n- No IDE-first experience or plugin marketplace in v0.1.\n- No distributed execution or hosted control plane in v0.1.\n\n## Deferred Work\n\nFuture slices may add broader adapter support only after the local governance path remains stable.\n"
        ),
        "decision-checklist.md" => format!(
            "# Decision Checklist\n\n## Summary\n\n{idea_summary}\n\n## Decision Checklist\n\n- [x] The mode and scope are explicit.\n- [x] Risk classification is recorded before execution.\n- [x] The artifact bundle captures constraints, options, tradeoffs, and cuts.\n- [x] The next stage can review the bundle without relying on chat history.\n\n## Open Questions\n\n- Which downstream mode should consume this bundle first?\n- Does the current artifact contract need stricter organization-specific policy overrides?\n"
        ),
        other => render_markdown(other, idea_summary),
    }
}

pub fn render_requirements_artifact_from_evidence(
    file_name: &str,
    idea_summary: &str,
    generation_summary: &str,
    critique_summary: &str,
    denied_summary: &str,
) -> String {
    let generation_normalized = generation_summary.to_lowercase();
    let critique_normalized = critique_summary.to_lowercase();
    let problem = extract_marker(generation_summary, &generation_normalized, "problem")
        .unwrap_or_else(|| generation_summary.to_string());
    let outcome = extract_marker(generation_summary, &generation_normalized, "outcome")
        .unwrap_or_else(|| {
            "NOT CAPTURED - Provide a `## Outcome` section in the requirements input.".to_string()
        });
    let constraints = extract_marker(generation_summary, &generation_normalized, "constraints")
        .unwrap_or_else(|| {
            "NOT CAPTURED - Provide a `## Constraints` section in the requirements input."
                .to_string()
        });
    let tradeoffs = extract_marker(generation_summary, &generation_normalized, "tradeoffs")
        .unwrap_or_else(|| {
            "NOT CAPTURED - Provide a `## Tradeoffs` section in the requirements input.".to_string()
        });
    let scope_cuts = extract_marker(generation_summary, &generation_normalized, "scope cuts")
        .unwrap_or_else(|| {
            "NOT CAPTURED - Provide a `## Out of Scope` or `## Scope Cuts` section in the requirements input."
                .to_string()
        });
    let options = extract_marker(generation_summary, &generation_normalized, "options")
        .unwrap_or_else(|| {
            "1. Review the packet with the named owner before downstream planning.".to_string()
        });
    let recommended_path =
        extract_marker(generation_summary, &generation_normalized, "recommended path")
            .unwrap_or_else(|| {
                "Review the packet and choose the smallest downstream mode that preserves the current boundary."
                    .to_string()
            });
    let open_questions =
        extract_marker(generation_summary, &generation_normalized, "open questions")
            .unwrap_or_else(|| {
                "NOT CAPTURED - No open questions were recorded for this requirements packet."
                    .to_string()
            });
    let coverage = extract_marker(critique_summary, &critique_normalized, "coverage")
        .unwrap_or_else(|| "- Requirements critique coverage was not recorded.".to_string());
    let missing_context = extract_marker(critique_summary, &critique_normalized, "missing context")
        .unwrap_or_else(|| "- Missing-context critique was not recorded.".to_string());
    let risk_notes = extract_marker(critique_summary, &critique_normalized, "risk notes")
        .unwrap_or_else(|| "- Risk notes were not recorded.".to_string());
    let recommended_focus =
        extract_marker(critique_summary, &critique_normalized, "recommended focus").unwrap_or_else(
            || "Review the packet before choosing the next governed mode.".to_string(),
        );

    match file_name {
        "problem-statement.md" => format!(
            "# Problem Statement\n\n## Summary\n\n{idea_summary}\n\n## Problem\n\n{problem}\n\n## Boundary\n\n{}\n\n## Success Signal\n\n{outcome}\n",
            render_requirements_boundary(&scope_cuts)
        ),
        "constraints.md" => format!(
            "# Constraints\n\n## Summary\n\n{idea_summary}\n\n## Constraints\n\n{constraints}\n\n## Non-Negotiables\n\n{coverage}\n{risk_notes}\n"
        ),
        "options.md" => format!(
            "# Options\n\n## Summary\n\n{idea_summary}\n\n## Options\n\n{options}\n\n## Recommended Path\n\n{recommended_path}\n"
        ),
        "tradeoffs.md" => format!(
            "# Tradeoffs\n\n## Summary\n\n{idea_summary}\n\n## Tradeoffs\n\n{tradeoffs}\n- Governed execution adds structure before speed.\n- Denied mutation requests keep requirements mode bounded.\n\n## Consequences\n\n- {denied_summary}\n{risk_notes}\n"
        ),
        "scope-cuts.md" => format!(
            "# Scope Cuts\n\n## Summary\n\n{idea_summary}\n\n## Scope Cuts\n\n{scope_cuts}\n\n## Deferred Work\n\n{recommended_focus}\n"
        ),
        "decision-checklist.md" => format!(
            "# Decision Checklist\n\n## Summary\n\n{idea_summary}\n\n## Decision Checklist\n\n{}\n\n## Open Questions\n\n{}\n",
            render_requirements_checklist(&problem, &outcome, &constraints, &scope_cuts),
            render_open_questions(&open_questions, &missing_context)
        ),
        other => render_requirements_artifact(other, idea_summary),
    }
}

pub fn render_discovery_artifact(file_name: &str, brief_summary: &str) -> String {
    let normalized = brief_summary.to_lowercase();
    let problem = extract_marker(brief_summary, &normalized, "problem")
        .or_else(|| extract_marker(brief_summary, &normalized, "problem domain"))
        .unwrap_or_else(|| {
            "NOT CAPTURED - Provide a `## Problem` section in the discovery brief.".to_string()
        });
    let constraints = extract_marker(brief_summary, &normalized, "constraints")
        .or_else(|| extract_marker(brief_summary, &normalized, "constraint"))
        .unwrap_or_else(|| {
            "NOT CAPTURED - Provide a `## Constraints` section in the discovery brief.".to_string()
        });
    let repo_focus = extract_marker(brief_summary, &normalized, "repo focus")
        .or_else(|| extract_marker(brief_summary, &normalized, "boundary"))
        .unwrap_or_else(|| {
            "NOT CAPTURED - Provide a `## Repo Focus` section in the discovery brief.".to_string()
        });
    let repo_surface = extract_marker(brief_summary, &normalized, "repo surface")
        .unwrap_or_else(|| "NOT CAPTURED - No repository surfaces were mapped.".to_string());
    let unknowns = extract_marker(brief_summary, &normalized, "unknowns").unwrap_or_else(|| {
        "NOT CAPTURED - Provide an `## Unknowns` section in the discovery brief.".to_string()
    });
    let next_phase =
        extract_marker(brief_summary, &normalized, "next phase").unwrap_or_else(|| {
            "NOT CAPTURED - Provide a `## Next Phase` section in the discovery brief.".to_string()
        });
    let generation_summary = extract_marker(brief_summary, &normalized, "generated framing")
        .unwrap_or_else(|| "NOT CAPTURED - Generated framing was absent.".to_string());
    let critique_summary = extract_marker(brief_summary, &normalized, "critique evidence")
        .unwrap_or_else(|| "NOT CAPTURED - Critique evidence is missing.".to_string());
    let validation_summary = extract_marker(brief_summary, &normalized, "validation evidence")
        .unwrap_or_else(|| "NOT CAPTURED - Repository validation evidence is missing.".to_string());
    let summary = render_discovery_bundle_summary(file_name, &problem, &constraints, &next_phase);

    match file_name {
        "problem-map.md" => format!(
            "# Problem Map\n\n## Summary\n\n{summary}\n\n## Repo Signals\n\n{repo_surface}\n\n## Problem Domain\n\n{generation_summary}\n\n## Immediate Tensions\n\n{critique_summary}\n\n## Downstream Handoff\n\n{next_phase}\n"
        ),
        "unknowns-and-assumptions.md" => format!(
            "# Unknowns And Assumptions\n\n## Summary\n\n{summary}\n\n## Unknowns\n\n{unknowns}\n\n## Assumptions\n\n- {constraints}\n- Discovery should stay tied to the named repository surface instead of generic product framing.\n\n## Validation Targets\n\n{validation_summary}\n\n## Confidence Levels\n\n{critique_summary}\n"
        ),
        "context-boundary.md" => format!(
            "# Context Boundary\n\n## Summary\n\n{summary}\n\n## In-Scope Context\n\n{problem}\n\n{repo_focus}\n\n## Repo Surface\n\n{repo_surface}\n\n## Out-of-Scope Context\n\n- Workspace mutation and implementation edits remain out of scope for discovery.\n- Do not expand beyond the named repository signals until the next governed mode is chosen.\n\n## Translation Trigger\n\n{next_phase}\n"
        ),
        "exploration-options.md" => format!(
            "# Exploration Options\n\n## Summary\n\n{summary}\n\n## Options\n\n1. Stay in discovery only to close the highest-risk unknowns tied to the current repository surface.\n2. Translate this packet into requirements mode if the main need is bounded problem framing and scope cuts.\n3. Translate this packet into architecture or brownfield planning if the repository signals already point to concrete boundaries or preserved behavior.\n\n## Constraints\n\n- {constraints}\n- Preserve explicit repository anchoring in the next phase.\n\n## Recommended Direction\n\n{generation_summary}\n\n## Next-Phase Shape\n\n{next_phase}\n"
        ),
        "decision-pressure-points.md" => format!(
            "# Decision Pressure Points\n\n## Summary\n\n{summary}\n\n## Pressure Points\n\n{critique_summary}\n\n## Blocking Decisions\n\n{validation_summary}\n\n## Open Questions\n\n{unknowns}\n\n## Recommended Owner\n\n- The named discovery owner should route this packet into the next governed mode with explicit boundary ownership.\n"
        ),
        other => render_markdown(other, brief_summary),
    }
}

pub fn render_greenfield_artifact(
    file_name: &str,
    context_summary: &str,
    generation_summary: &str,
    critique_summary: &str,
) -> String {
    let normalized = context_summary.to_lowercase();
    let intent = extract_marker(context_summary, &normalized, "intent");
    let constraint = extract_marker(context_summary, &normalized, "constraint")
        .or_else(|| extract_marker(context_summary, &normalized, "constraints"));
    let greenfield_gap = greenfield_context_gap(intent.as_deref(), constraint.as_deref());
    let system_shape = if let Some(gap) = greenfield_gap.as_deref() {
        gap.to_string()
    } else {
        format!(
            "{generation_summary}\n\nEvidence anchors:\n- Intent: {}\n- Constraint: {}",
            intent.as_deref().unwrap_or_default(),
            constraint.as_deref().unwrap_or_default()
        )
    };
    let structural_rationale = if let Some(gap) = greenfield_gap.as_deref() {
        gap.to_string()
    } else {
        format!(
            "{critique_summary}\n\nConstraint anchor: {}",
            constraint.as_deref().unwrap_or_default()
        )
    };

    match file_name {
        "system-shape.md" => format!(
            "# System Shape\n\n## Summary\n\n{context_summary}\n\n## System Shape\n\n{system_shape}\n\n## Boundary Decisions\n\n{structural_rationale}\n\n## Domain Responsibilities\n\n- Responsibilities remain bounded to the capability described in the supplied context.\n"
        ),
        "architecture-outline.md" => format!(
            "# Architecture Outline\n\n## Summary\n\n{context_summary}\n\n## Structural Options\n\n{system_shape}\n\n## Selected Boundaries\n\n{structural_rationale}\n\n## Rationale\n\nThe selected boundaries favor explicit ownership and staged delivery over unbounded system growth.\n"
        ),
        "capability-map.md" => format!(
            "# Capability Map\n\n## Summary\n\n{context_summary}\n\n## Capabilities\n\n{system_shape}\n\n## Dependencies\n\n- Dependencies remain limited to the bounded capability described in the run context.\n\n## Gaps\n\n{structural_rationale}\n"
        ),
        "delivery-options.md" => format!(
            "# Delivery Options\n\n## Summary\n\n{context_summary}\n\n## Delivery Phases\n\n{generation_summary}\n\n## Sequencing Rationale\n\n{critique_summary}\n\n## Risk per Phase\n\n- Each phase should preserve bounded delivery slices and visible rollback points.\n"
        ),
        "risk-hotspots.md" => format!(
            "# Risk Hotspots\n\n## Summary\n\n{context_summary}\n\n## Hotspots\n\n{critique_summary}\n\n## Mitigation Status\n\n- Mitigations remain proposed until downstream modes or reviewers accept them.\n\n## Unresolved Risks\n\n{generation_summary}\n"
        ),
        other => render_markdown(other, context_summary),
    }
}

pub fn render_architecture_artifact(
    file_name: &str,
    context_summary: &str,
    generation_summary: &str,
    critique_summary: &str,
) -> String {
    match file_name {
        "architecture-decisions.md" => format!(
            "# Architecture Decisions\n\n## Summary\n\n{context_summary}\n\n## Decisions\n\n{generation_summary}\n\n## Tradeoffs\n\n{critique_summary}\n\n## Consequences\n\nThe recorded decisions constrain later implementation and review work.\n\n## Unresolved Questions\n\n- Which structural assumptions still need explicit acceptance?\n"
        ),
        "invariants.md" => format!(
            "# Invariants\n\n## Summary\n\n{context_summary}\n\n## Invariants\n\n{generation_summary}\n\n## Rationale\n\n{critique_summary}\n\n## Verification Hooks\n\n- Downstream modes must be able to validate these invariants against emitted evidence.\n"
        ),
        "tradeoff-matrix.md" => format!(
            "# Tradeoff Matrix\n\n## Summary\n\n{context_summary}\n\n## Options\n\n{generation_summary}\n\n## Evaluation Criteria\n\n- Boundary preservation\n- Invariant clarity\n- Reversibility\n\n## Scores\n\n{critique_summary}\n\n## Selected Option\n\nThe preferred option is the one that best preserves explicit boundaries and reviewable tradeoffs.\n"
        ),
        "boundary-map.md" => format!(
            "# Boundary Map\n\n## Summary\n\n{context_summary}\n\n## Boundaries\n\n{generation_summary}\n\n## Ownership\n\n- Ownership must remain explicit for each named boundary before implementation begins.\n\n## Crossing Rules\n\n{critique_summary}\n"
        ),
        "readiness-assessment.md" => format!(
            "# Readiness Assessment\n\n## Summary\n\n{context_summary}\n\n## Readiness Status\n\nArchitecture analysis is ready for downstream consumption once approvals and unresolved questions are addressed.\n\n## Blockers\n\n{critique_summary}\n\n## Accepted Risks\n\n{generation_summary}\n"
        ),
        other => render_markdown(other, context_summary),
    }
}

pub fn render_brownfield_artifact(file_name: &str, brief_summary: &str) -> String {
    let normalized = brief_summary.to_lowercase();
    let system_slice = extract_marker(brief_summary, &normalized, "system slice")
        .unwrap_or("Map the bounded subsystem before change planning.".to_string());
    let intended_change = extract_marker(brief_summary, &normalized, "intended change").unwrap_or(
        "Bound the intended change explicitly before implementation expands the surface area."
            .to_string(),
    );
    let legacy_invariants = extract_marker(brief_summary, &normalized, "legacy invariants");
    let change_surface = extract_marker(brief_summary, &normalized, "change surface");
    let implementation_plan = extract_marker(brief_summary, &normalized, "implementation plan")
        .unwrap_or(
            "Sequence the change so preserved behavior remains observable at every step."
                .to_string(),
        );
    let validation_strategy = extract_marker(brief_summary, &normalized, "validation strategy")
        .unwrap_or(
            "Challenge the change with invariant checks, contract tests, and rollback evidence."
                .to_string(),
        );
    let decision_record = extract_marker(brief_summary, &normalized, "decision record").unwrap_or(
        "Prefer additive change over normalization when the legacy surface still matters."
            .to_string(),
    );
    let owner = extract_marker(brief_summary, &normalized, "owner")
        .unwrap_or("bounded-system-maintainer".to_string());
    let risk_level = extract_marker(brief_summary, &normalized, "risk level")
        .unwrap_or("unspecified-risk".to_string());
    let zone = extract_marker(brief_summary, &normalized, "zone")
        .unwrap_or("unspecified-zone".to_string());
    let summary = render_brownfield_bundle_summary(
        file_name,
        &system_slice,
        &intended_change,
        &owner,
        &risk_level,
        &zone,
    );

    match file_name {
        "system-slice.md" => format!(
            "# System Slice\n\n## Summary\n\n{summary}\n\n## System Slice\n\n{system_slice}\n\n## Excluded Areas\n\n- Do not expand beyond the named bounded subsystem in this run.\n"
        ),
        "legacy-invariants.md" => match legacy_invariants {
            Some(value) => format!(
                "# Legacy Invariants\n\n## Summary\n\n{summary}\n\n## Legacy Invariants\n\n{value}\n\n## Forbidden Normalization\n\n- Do not simplify away existing behavior that operators or downstream systems still depend on.\n"
            ),
            None => format!(
                "# Legacy Invariants\n\n## Summary\n\n{summary}\n\n## Missing Context\n\nCapture preserved behavior before this run can pass brownfield preservation.\n"
            ),
        },
        "change-surface.md" => match change_surface {
            Some(value) => format!(
                "# Change Surface\n\n## Summary\n\n{summary}\n\n## Change Surface\n\n{value}\n\n## Ownership\n\n- Primary owner: bounded-system-maintainer\n"
            ),
            None => format!(
                "# Change Surface\n\n## Summary\n\n{summary}\n\n## Missing Context\n\nName the affected modules, interfaces, and owners before change planning can proceed.\n"
            ),
        },
        "implementation-plan.md" => format!(
            "# Implementation Plan\n\n## Summary\n\n{summary}\n\n## Plan\n\n{implementation_plan}\n\n## Sequencing\n\n- Preserve externally meaningful behavior before any internal cleanup.\n"
        ),
        "validation-strategy.md" => format!(
            "# Validation Strategy\n\n## Summary\n\n{summary}\n\n## Validation Strategy\n\n{validation_strategy}\n\n## Independent Checks\n\n- Re-run invariant checks separately from generated implementation notes.\n"
        ),
        "decision-record.md" => format!(
            "# Decision Record\n\n## Summary\n\n{summary}\n\n## Decision\n\n{decision_record}\n\n## Consequences\n\n- The preserved surface remains explicit and reviewable.\n\n## Unresolved Questions\n\n- Which adjacent slices should stay out of scope until this change stabilizes?\n"
        ),
        other => render_markdown(other, brief_summary),
    }
}

fn render_brownfield_bundle_summary(
    current_file: &str,
    system_slice: &str,
    intended_change: &str,
    owner: &str,
    risk_level: &str,
    zone: &str,
) -> String {
    let detail_links = [
        "system-slice.md",
        "legacy-invariants.md",
        "change-surface.md",
        "implementation-plan.md",
        "validation-strategy.md",
        "decision-record.md",
    ]
    .into_iter()
    .filter(|file_name| *file_name != current_file)
    .map(|file_name| format!("[{file_name}]({file_name})"))
    .collect::<Vec<_>>()
    .join(", ");

    format!(
        "- Bounded slice: `{}`\n- Intended change: {}\n- Owner / risk / zone: `{}` / `{}` / `{}`\n- Details: {}",
        compact_summary_line(system_slice),
        compact_summary_line(intended_change),
        compact_summary_line(owner),
        compact_summary_line(risk_level),
        compact_summary_line(zone),
        detail_links,
    )
}

fn compact_summary_line(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn render_discovery_bundle_summary(
    current_file: &str,
    problem: &str,
    constraints: &str,
    next_phase: &str,
) -> String {
    let detail_links = [
        "problem-map.md",
        "unknowns-and-assumptions.md",
        "context-boundary.md",
        "exploration-options.md",
        "decision-pressure-points.md",
    ]
    .into_iter()
    .filter(|file_name| *file_name != current_file)
    .map(|file_name| format!("[{file_name}]({file_name})"))
    .collect::<Vec<_>>()
    .join(", ");

    let format_field = |label: &str, content: &str| {
        let trimmed = content.trim();
        if trimmed.is_empty() {
            return String::new();
        }
        if trimmed.contains('\n') || trimmed.starts_with('-') || trimmed.len() > 100 {
            format!("- **{label}:**\n\n  {}", trimmed.replace("\n", "\n  "))
        } else {
            format!("- **{label}:** {trimmed}")
        }
    };

    let mut parts = Vec::new();
    let prob = format_field("Problem", problem);
    if !prob.is_empty() {
        parts.push(prob);
    }

    let con = format_field("Constraints", constraints);
    if !con.is_empty() {
        parts.push(con);
    }

    let phase = format_field("Next phase", next_phase);
    if !phase.is_empty() {
        parts.push(phase);
    }

    parts.push(format!("- **Details:** {detail_links}"));
    parts.join("\n")
}

fn greenfield_context_gap(intent: Option<&str>, constraint: Option<&str>) -> Option<String> {
    if intent.is_some() && constraint.is_some() {
        None
    } else {
        Some(
            "Insufficient evidence: supply explicit `Intent:` and `Constraint:` markers in the system-shaping brief before system shaping can proceed."
                .to_string(),
        )
    }
}

pub fn render_pr_review_artifact(
    file_name: &str,
    packet: &ReviewPacket,
    summary: &ReviewSummary,
) -> String {
    match file_name {
        "pr-analysis.md" => format!(
            "# PR Analysis\n\n## Summary\n\nReviewing `{}` against `{}` across {} changed surface(s).\n\n## Evidence Posture\n\n- Review packet derived from governed diff inspection and critique evidence.\n- Artifact provenance remains linked from the run evidence bundle.\n\n## Scope Summary\n\n- Base ref: `{}`\n- Head ref: `{}`\n- Changed surface count: {}\n\n## Changed Modules\n\n{}\n\n## Inferred Intent\n\n{}\n\n## Surprising Surface Area\n\n{}\n",
            packet.head_ref,
            packet.base_ref,
            packet.changed_surfaces.len(),
            packet.base_ref,
            packet.head_ref,
            packet.changed_surfaces.len(),
            render_surface_list(&packet.changed_surfaces, "- No changed surfaces detected."),
            packet.inferred_intent,
            render_surface_list(
                &packet.surprising_surface_area,
                "- No surprising surface area inferred from the diff."
            ),
        ),
        "boundary-check.md" => format!(
            "# Boundary Check\n\n## Summary\n\nBoundary review for changed surfaces between `{}` and `{}`.\n\n## Boundary Findings\n\n{}\n\n## Ownership Breaks\n\n{}\n\n## Unauthorized Structural Impact\n\nStatus: {}\n\n{}\n",
            packet.base_ref,
            packet.head_ref,
            render_findings(
                &packet.findings_for(FindingCategory::BoundaryCheck),
                "- No boundary findings detected."
            ),
            ownership_breaks(packet),
            if packet.findings_for(FindingCategory::BoundaryCheck).is_empty() {
                "no-structural-impact-detected"
            } else {
                "structural-impact-reviewed"
            },
            if packet.findings_for(FindingCategory::BoundaryCheck).is_empty() {
                "- No unauthorized structural impact inferred."
            } else {
                "- Boundary-marked files changed and remain explicit in the review packet."
            },
        ),
        "duplication-check.md" => format!(
            "# Duplication Check\n\n## Summary\n\nDuplication review for the bounded diff.\n\n## Duplicate Behavior\n\n{}\n\n## Canonical Owner Conflicts\n\n{}\n",
            render_findings(
                &packet.findings_for(FindingCategory::DuplicationCheck),
                "- No duplicate behavior concerns inferred."
            ),
            if packet.findings_for(FindingCategory::DuplicationCheck).is_empty() {
                "- No canonical owner conflicts inferred."
            } else {
                "- Canonical ownership needs review where duplicate logic surfaced."
            },
        ),
        "contract-drift.md" => format!(
            "# Contract Drift\n\n## Summary\n\nContract review for the changed surfaces.\n\n## Interface Drift\n\n{}\n\n## Compatibility Concerns\n\nStatus: {}\n\n{}\n",
            render_findings(
                &packet.findings_for(FindingCategory::ContractDrift),
                "- No contract drift inferred."
            ),
            if packet.findings_for(FindingCategory::ContractDrift).is_empty() {
                "no-contract-drift-detected"
            } else {
                "explicit-contract-drift"
            },
            if packet.findings_for(FindingCategory::ContractDrift).is_empty() {
                "- No compatibility concerns inferred from the reviewed diff."
            } else {
                "- Compatibility risk remains explicit until reviewer disposition is recorded."
            },
        ),
        "missing-tests.md" => format!(
            "# Missing Tests\n\n## Summary\n\nVerification coverage review for the diff.\n\n## Missing Invariant Checks\n\n{}\n\n## Missing Contract Checks\n\n{}\n\n## Weak or Mirrored Tests\n\n{}\n",
            render_findings(
                &packet.findings_for(FindingCategory::MissingTests),
                "- No missing invariant checks inferred."
            ),
            if packet.findings_for(FindingCategory::MissingTests).is_empty() {
                "- No missing contract checks inferred."
            } else {
                "- Contract-facing changes should carry explicit test evidence."
            },
            if packet.findings_for(FindingCategory::MissingTests).is_empty() {
                "- Updated tests moved with the changed surface."
            } else {
                "- The current diff changes source files without parallel verification updates."
            },
        ),
        "decision-impact.md" => format!(
            "# Decision Impact\n\n## Summary\n\nDecision-impact review for the bounded diff.\n\n## Implied Decisions\n\n{}\n\n## Absent Decision Records\n\n{}\n\n## Reversibility Concerns\n\n{}\n",
            render_findings(
                &packet.findings_for(FindingCategory::DecisionImpact),
                "- No hidden decision impact inferred."
            ),
            if packet.findings_for(FindingCategory::DecisionImpact).is_empty() {
                "- No absent decision records were inferred from the changed surfaces."
            } else {
                "- Structural or contract-facing changes imply decisions that are not yet explicitly accepted."
            },
            if packet.findings_for(FindingCategory::DecisionImpact).is_empty() {
                "- Reversibility concerns remain bounded."
            } else {
                "- Reverting the current change may require downstream coordination because high-impact surfaces changed."
            },
        ),
        "review-summary.md" => format!(
            "# Review Summary\n\n## Summary\n\nStructured review completed for `{}` against `{}`.\n\n## Evidence\n\n- Governed diff inspection and critique were both recorded before disposition.\n- Review artifacts remain derived from the persisted evidence bundle.\n\n## Severity\n\n- Overall severity: {}\n- Must-fix findings: {}\n- Review notes: {}\n\n## Must-Fix Findings\n\n{}\n\n## Accepted Risks\n\n{}\n\n## Final Disposition\n\nStatus: {}\n\nRationale: {}\n",
            packet.head_ref,
            packet.base_ref,
            summary_severity_label(packet),
            summary.must_fix_findings.len(),
            packet.note_findings().len(),
            render_findings(&packet.must_fix_findings(), "- No must-fix findings remain."),
            render_string_list(&summary.accepted_risks, "- No accepted risks recorded."),
            summary.disposition.as_str(),
            summary.rationale,
        ),
        other => render_markdown(other, &packet.inferred_intent),
    }
}

fn extract_marker(source: &str, normalized: &str, marker: &str) -> Option<String> {
    extract_markdown_section(source, marker)
        .or_else(|| extract_inline_marker(source, normalized, marker))
}

fn extract_inline_marker(source: &str, normalized: &str, marker: &str) -> Option<String> {
    let marker_with_colon = format!("{marker}:");
    let start = normalized.find(&marker_with_colon)?;
    let remainder = &source[start + marker_with_colon.len()..];
    let line = remainder.lines().next()?.trim();
    if line.is_empty() { None } else { Some(line.to_string()) }
}

fn extract_markdown_section(source: &str, marker: &str) -> Option<String> {
    let mut lines = source.lines().peekable();

    while let Some(line) = lines.next() {
        if !is_matching_heading(line, marker) {
            continue;
        }

        let mut section_lines = Vec::new();
        while let Some(next_line) = lines.peek() {
            if is_section_boundary(next_line) {
                break;
            }

            section_lines.push(lines.next().unwrap_or_default());
        }

        let section = trim_multiline_block(&section_lines.join("\n"));
        if !section.is_empty() {
            return Some(section);
        }
    }

    None
}

fn is_matching_heading(line: &str, marker: &str) -> bool {
    let trimmed = line.trim();
    if !trimmed.starts_with('#') {
        return false;
    }

    trimmed.trim_start_matches('#').trim().eq_ignore_ascii_case(marker)
}

fn is_section_boundary(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('#')
        || trimmed.starts_with("Generated framing:")
        || trimmed.starts_with("Critique evidence:")
        || trimmed.starts_with("Validation evidence:")
        || trimmed.starts_with("Mutation posture:")
}

fn trim_multiline_block(value: &str) -> String {
    let lines = value.lines().collect::<Vec<_>>();
    let start = lines.iter().position(|line| !line.trim().is_empty());
    let end = lines.iter().rposition(|line| !line.trim().is_empty());

    match (start, end) {
        (Some(start), Some(end)) => lines[start..=end].join("\n"),
        _ => String::new(),
    }
}

fn render_surface_list(values: &[String], empty_message: &str) -> String {
    render_string_list(values, empty_message)
}

fn render_string_list(values: &[String], empty_message: &str) -> String {
    if values.is_empty() {
        empty_message.to_string()
    } else {
        values.iter().map(|value| format!("- {value}")).collect::<Vec<_>>().join("\n")
    }
}

fn render_requirements_boundary(scope_cuts: &str) -> String {
    if scope_cuts.contains("NOT CAPTURED") {
        "The packet remains bounded to explicit requirements framing and does not authorize implementation changes."
            .to_string()
    } else {
        format!("The current packet is bounded by these explicit scope cuts:\n\n{scope_cuts}")
    }
}

fn render_requirements_checklist(
    problem: &str,
    outcome: &str,
    constraints: &str,
    scope_cuts: &str,
) -> String {
    let items = [
        ("The problem statement is explicit.", !problem.contains("NOT CAPTURED")),
        ("The desired outcome is explicit.", !outcome.contains("NOT CAPTURED")),
        ("The packet names concrete constraints.", !constraints.contains("NOT CAPTURED")),
        ("The packet names explicit scope cuts.", !scope_cuts.contains("NOT CAPTURED")),
    ];

    items
        .into_iter()
        .map(|(label, complete)| format!("- [{}] {label}", if complete { "x" } else { " " }))
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_open_questions(open_questions: &str, missing_context: &str) -> String {
    if missing_context.contains("No additional missing context")
        || missing_context.contains("Missing-context critique was not recorded")
    {
        open_questions.to_string()
    } else {
        format!("{open_questions}\n{missing_context}")
    }
}

fn render_findings(findings: &[&ReviewFinding], empty_message: &str) -> String {
    if findings.is_empty() {
        return empty_message.to_string();
    }

    findings
        .iter()
        .map(|finding| {
            format!(
                "- [{}] {}: {}\n  - Surfaces: {}",
                finding.severity.as_str(),
                finding.title,
                finding.details,
                if finding.changed_surfaces.is_empty() {
                    "none".to_string()
                } else {
                    finding.changed_surfaces.join(", ")
                }
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn ownership_breaks(packet: &ReviewPacket) -> String {
    let boundary_findings = packet.findings_for(FindingCategory::BoundaryCheck);
    if boundary_findings.is_empty() {
        "- No ownership breaks inferred from changed surfaces.".to_string()
    } else {
        "- Reviewer ownership is required before boundary-marked changes can be treated as acceptable output.".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        extract_marker, render_architecture_artifact, render_brownfield_artifact,
        render_discovery_artifact, render_greenfield_artifact, render_pr_review_artifact,
        render_requirements_artifact, render_requirements_artifact_from_evidence,
    };
    use crate::review::findings::{FindingCategory, FindingSeverity, ReviewFinding, ReviewPacket};
    use crate::review::summary::{ReviewDisposition, ReviewSummary};

    #[test]
    fn extract_marker_prefers_markdown_section_over_inline_mentions() {
        let source = "# Brownfield Brief\n\n## Change Surface\n- bounded module\n- stable interface\n\nMutation posture: propose bounded legacy transformation within declared change surface: workspace, adjacent modules";
        let normalized = source.to_lowercase();

        let marker = extract_marker(source, &normalized, "change surface").expect("change surface");

        assert_eq!(marker, "- bounded module\n- stable interface");
    }

    #[test]
    fn render_brownfield_change_surface_preserves_markdown_bullets() {
        let source = "# Brownfield Brief\n\n## System Slice\nSchema validation\n\n## Intended Change\nAdd debug logging for null arguments.\n\n## Change Surface\n- Public API entrypoints\n- Debug logging only\n\n## Owner\nLead Eng\n\n## Risk Level\nlow-impact\n\n## Zone\ngreen\n";

        let rendered = render_brownfield_artifact("change-surface.md", source);

        assert!(
            rendered
                .contains("## Change Surface\n\n- Public API entrypoints\n- Debug logging only")
        );
        assert!(rendered.contains("- Owner / risk / zone: `Lead Eng` / `low-impact` / `green`"));
    }

    #[test]
    fn render_brownfield_validation_strategy_preserves_markdown_bullets() {
        let source = "# Brownfield Brief\n\n## System Slice\nSchema validation\n\n## Intended Change\nAdd debug logging for null arguments.\n\n## Validation Strategy\n- Unit tests\n- Log assertion checks\n";

        let rendered = render_brownfield_artifact("validation-strategy.md", source);

        assert!(
            rendered.contains("## Validation Strategy\n\n- Unit tests\n- Log assertion checks")
        );
    }

    #[test]
    fn render_requirements_artifacts_cover_named_templates_and_fallback() {
        let summary = "Bound the requirements work before planning";

        let constraints = render_requirements_artifact("constraints.md", summary);
        let fallback = render_requirements_artifact("custom-note.md", summary);
        let evidence = render_requirements_artifact_from_evidence(
            "tradeoffs.md",
            summary,
            "generated framing",
            "critique note",
            "denied mutation request remained visible",
        );

        assert!(constraints.contains("## Non-Negotiables"));
        assert!(constraints.contains("Risk and zone classification happen before generation."));
        assert!(fallback.starts_with(
            "# custom-note.md\n\n## Summary\n\nBound the requirements work before planning"
        ));
        assert!(evidence.contains("Denied mutation requests keep requirements mode bounded."));
        assert!(evidence.contains("- denied mutation request remained visible"));
    }

    #[test]
    fn render_brownfield_artifact_reports_missing_context_and_default_metadata() {
        let source = "# Brownfield Brief\n\n## System Slice\nSession repository\n\n## Intended Change\nStabilize resumable execution\n";

        let invariants = render_brownfield_artifact("legacy-invariants.md", source);
        let decision = render_brownfield_artifact("decision-record.md", source);

        assert!(invariants.contains("## Missing Context\n\nCapture preserved behavior before this run can pass brownfield preservation."));
        assert!(decision.contains(
            "Prefer additive change over normalization when the legacy surface still matters."
        ));
        assert!(decision.contains("- Owner / risk / zone: `bounded-system-maintainer` / `unspecified-risk` / `unspecified-zone`"));
    }

    #[test]
    fn render_pr_review_artifacts_handle_empty_and_populated_findings() {
        let review_notes_packet = ReviewPacket::from_diff(
            "origin/main",
            "HEAD",
            vec!["src/lib.rs".to_string(), "tests/lib_test.rs".to_string()],
            "@@ -1 +1 @@\n-old\n+new\n",
        );
        let review_notes_summary = ReviewSummary::from_packet(&review_notes_packet, false);
        let boundary = render_pr_review_artifact(
            "boundary-check.md",
            &review_notes_packet,
            &review_notes_summary,
        );

        assert!(boundary.contains("- No boundary findings detected."));
        assert!(boundary.contains("Status: no-structural-impact-detected"));

        let must_fix_packet = ReviewPacket {
            base_ref: "origin/main".to_string(),
            head_ref: "feature".to_string(),
            changed_surfaces: vec!["contracts/public-api.json".to_string()],
            inferred_intent: "Review contract drift on a public API change.".to_string(),
            surprising_surface_area: vec!["contracts/public-api.json".to_string()],
            findings: vec![
                ReviewFinding {
                    category: FindingCategory::ContractDrift,
                    severity: FindingSeverity::MustFix,
                    title: "Contract-facing files changed".to_string(),
                    details: "Compatibility drift needs explicit reviewer acceptance.".to_string(),
                    changed_surfaces: vec!["contracts/public-api.json".to_string()],
                },
                ReviewFinding {
                    category: FindingCategory::DecisionImpact,
                    severity: FindingSeverity::Note,
                    title: "Decision note".to_string(),
                    details: "A broader acceptance note should be recorded.".to_string(),
                    changed_surfaces: vec!["contracts/public-api.json".to_string()],
                },
            ],
        };
        let must_fix_summary = ReviewSummary {
            disposition: ReviewDisposition::AcceptedWithApproval,
            rationale: "Explicit reviewer approval accepted the remaining must-fix findings with named ownership.".to_string(),
            must_fix_findings: vec!["Contract-facing files changed".to_string()],
            accepted_risks: vec!["Decision note".to_string()],
        };

        let contract =
            render_pr_review_artifact("contract-drift.md", &must_fix_packet, &must_fix_summary);
        let summary =
            render_pr_review_artifact("review-summary.md", &must_fix_packet, &must_fix_summary);

        assert!(contract.contains("Status: explicit-contract-drift"));
        assert!(contract.contains(
            "Compatibility risk remains explicit until reviewer disposition is recorded."
        ));
        assert!(summary.contains("Overall severity: must-fix"));
        assert!(summary.contains("Status: accepted-with-approval"));
        assert!(summary.contains("- Decision note"));
    }

    #[test]
    fn render_analysis_mode_artifacts_include_required_sections() {
        let discovery = render_discovery_artifact(
            "context-boundary.md",
            "# Discovery Brief\n\n## Problem\nExplore a bounded notification routing problem.\n\n## Constraints\nPreserve the current routing ownership boundaries.\n\n## Repo Surface\n- src/router.rs\n- tests/router_contract.rs\n\n## Unknowns\n- Which caller owns retry policy?\n\n## Next Phase\nTranslate this discovery packet into architecture mode with named boundaries.\n\nGenerated framing: Map the known constraints and unresolved actors.\n\nCritique evidence: Challenge scope drift around retry ownership.\n\nValidation evidence: Validation tool reviewed tracked repository surfaces: src/router.rs, tests/router_contract.rs",
        );
        let greenfield = render_greenfield_artifact(
            "system-shape.md",
            "Shape a new notification delivery capability.",
            "Separate ingest, routing, and delivery responsibilities.",
            "Keep delivery phase boundaries explicit and reversible.",
        );
        let architecture = render_architecture_artifact(
            "tradeoff-matrix.md",
            "Evaluate architectural boundaries for routing state.",
            "Compare centralized and partitioned routing designs.",
            "Partitioned routing better preserves ownership boundaries.",
        );

        assert!(discovery.contains("## In-Scope Context"));
        assert!(discovery.contains("## Repo Surface"));
        assert!(discovery.contains("## Translation Trigger"));
        assert!(greenfield.contains("## System Shape"));
        assert!(greenfield.contains("## Boundary Decisions"));
        assert!(architecture.contains("## Evaluation Criteria"));
        assert!(architecture.contains("## Selected Option"));
    }
}
