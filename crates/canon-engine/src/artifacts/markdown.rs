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
    match file_name {
        "problem-statement.md" => format!(
            "# Problem Statement\n\n## Summary\n\n{idea_summary}\n\n## Problem\n\n{generation_summary}\n\n## Boundary\n\nThe governed execution path stays in requirements mode and does not mutate the workspace.\n\n## Success Signal\n\nThe next step can proceed using bounded problem framing backed by recorded invocation evidence.\n"
        ),
        "constraints.md" => format!(
            "# Constraints\n\n## Summary\n\n{idea_summary}\n\n## Constraints\n\n- Govern execution before generation.\n- Preserve durable evidence under `.canon/`.\n- Keep validation challenge separate from generation.\n\n## Non-Negotiables\n\n- {critique_summary}\n"
        ),
        "options.md" => format!(
            "# Options\n\n## Summary\n\n{idea_summary}\n\n## Options\n\n1. Continue with a bounded governed change plan.\n2. Reframe the problem if critique evidence shows scope drift.\n\n## Recommended Path\n\nUse the generated framing plus critique evidence to choose the smallest governed next move.\n"
        ),
        "tradeoffs.md" => format!(
            "# Tradeoffs\n\n## Summary\n\n{idea_summary}\n\n## Tradeoffs\n\n- Governed execution adds structure before speed.\n- Evidence-first runs are heavier than freeform prompting.\n- Denied mutation requests keep requirements mode bounded.\n\n## Consequences\n\n- {denied_summary}\n"
        ),
        "scope-cuts.md" => format!(
            "# Scope Cuts\n\n## Summary\n\n{idea_summary}\n\n## Scope Cuts\n\n- No workspace mutation in requirements mode.\n- No runtime MCP execution in this tranche.\n- No code edits are authorized from this run.\n\n## Deferred Work\n\nMove from framing to execution only after the next mode is chosen explicitly.\n"
        ),
        "decision-checklist.md" => format!(
            "# Decision Checklist\n\n## Summary\n\n{idea_summary}\n\n## Decision Checklist\n\n- [x] Governed context capture ran before generation.\n- [x] Generation and critique were recorded as separate invocations.\n- [x] Denied mutation attempts remain visible as evidence.\n- [x] The artifact bundle links back to execution evidence.\n\n## Open Questions\n\n- Which downstream mode should consume this evidence bundle?\n- Does the current critique require a narrower scope before planning continues?\n"
        ),
        other => render_requirements_artifact(other, idea_summary),
    }
}

pub fn render_brownfield_artifact(file_name: &str, brief_summary: &str) -> String {
    let normalized = brief_summary.to_lowercase();
    let system_slice = extract_marker(brief_summary, &normalized, "system slice")
        .unwrap_or("Map the bounded subsystem before change planning.".to_string());
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

    match file_name {
        "system-slice.md" => format!(
            "# System Slice\n\n## Summary\n\n{brief_summary}\n\n## System Slice\n\n{system_slice}\n\n## Excluded Areas\n\n- Do not expand beyond the named bounded subsystem in this run.\n"
        ),
        "legacy-invariants.md" => match legacy_invariants {
            Some(value) => format!(
                "# Legacy Invariants\n\n## Summary\n\n{brief_summary}\n\n## Legacy Invariants\n\n- {value}\n\n## Forbidden Normalization\n\n- Do not simplify away existing behavior that operators or downstream systems still depend on.\n"
            ),
            None => format!(
                "# Legacy Invariants\n\n## Summary\n\n{brief_summary}\n\n## Missing Context\n\nCapture preserved behavior before this run can pass brownfield preservation.\n"
            ),
        },
        "change-surface.md" => match change_surface {
            Some(value) => format!(
                "# Change Surface\n\n## Summary\n\n{brief_summary}\n\n## Change Surface\n\n- {value}\n\n## Ownership\n\n- Primary owner: bounded-system-maintainer\n"
            ),
            None => format!(
                "# Change Surface\n\n## Summary\n\n{brief_summary}\n\n## Missing Context\n\nName the affected modules, interfaces, and owners before change planning can proceed.\n"
            ),
        },
        "implementation-plan.md" => format!(
            "# Implementation Plan\n\n## Summary\n\n{brief_summary}\n\n## Plan\n\n- {implementation_plan}\n\n## Sequencing\n\n- Preserve externally meaningful behavior before any internal cleanup.\n"
        ),
        "validation-strategy.md" => format!(
            "# Validation Strategy\n\n## Summary\n\n{brief_summary}\n\n## Validation Strategy\n\n- {validation_strategy}\n\n## Independent Checks\n\n- Re-run invariant checks separately from generated implementation notes.\n"
        ),
        "decision-record.md" => format!(
            "# Decision Record\n\n## Summary\n\n{brief_summary}\n\n## Decision\n\n{decision_record}\n\n## Consequences\n\n- The preserved surface remains explicit and reviewable.\n\n## Unresolved Questions\n\n- Which adjacent slices should stay out of scope until this change stabilizes?\n"
        ),
        other => render_markdown(other, brief_summary),
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
    let marker_with_colon = format!("{marker}:");
    let start = normalized.find(&marker_with_colon)?;
    let remainder = &source[start + marker_with_colon.len()..];
    let line = remainder.lines().next()?.trim();
    if line.is_empty() { None } else { Some(line.to_string()) }
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
