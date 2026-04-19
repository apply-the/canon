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

pub fn render_system_shaping_artifact(
    file_name: &str,
    context_summary: &str,
    generation_summary: &str,
    critique_summary: &str,
) -> String {
    let normalized = context_summary.to_lowercase();
    let intent = extract_marker(context_summary, &normalized, "intent");
    let constraint = extract_marker(context_summary, &normalized, "constraint")
        .or_else(|| extract_marker(context_summary, &normalized, "constraints"));
    let system_shaping_gap = system_shaping_context_gap(intent.as_deref(), constraint.as_deref());
    let system_shape = if let Some(gap) = system_shaping_gap.as_deref() {
        gap.to_string()
    } else {
        format!(
            "{generation_summary}\n\nEvidence anchors:\n- Intent: {}\n- Constraint: {}",
            intent.as_deref().unwrap_or_default(),
            constraint.as_deref().unwrap_or_default()
        )
    };
    let structural_rationale = if let Some(gap) = system_shaping_gap.as_deref() {
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

fn preserve_markdown_block(value: &str) -> String {
    let mut lines = Vec::new();
    let mut previous_blank = false;

    for raw_line in value.lines() {
        let line = raw_line.split_whitespace().collect::<Vec<_>>().join(" ");
        if line.is_empty() {
            if !previous_blank && !lines.is_empty() {
                lines.push(String::new());
            }
            previous_blank = true;
        } else {
            lines.push(line);
            previous_blank = false;
        }
    }

    lines.join("\n").trim().to_string()
}

fn render_verification_summary(
    claims_under_test: &str,
    evidence_basis: Option<&str>,
    contract_assumptions: &str,
    challenge_focus: Option<&str>,
    validation_summary: &str,
    verdict: &str,
    has_open_findings: bool,
) -> String {
    let mut lines = Vec::new();

    if let Some(line) = verification_summary_field("Claims under test", Some(claims_under_test)) {
        lines.push(line);
    }
    if let Some(line) = verification_summary_field("Evidence basis", evidence_basis) {
        lines.push(line);
    }
    if let Some(line) = verification_summary_field("Contract surface", Some(contract_assumptions)) {
        lines.push(line);
    }
    if has_open_findings
        && let Some(line) = verification_summary_field("Challenge focus", challenge_focus)
    {
        lines.push(line);
    }

    lines.push(format!("- Verdict: {verdict}"));
    lines.push(if has_open_findings {
        "- Open findings: unresolved follow-up remains recorded in this packet.".to_string()
    } else {
        "- Open findings: none recorded.".to_string()
    });

    if let Some(line) = verification_summary_field("Validation evidence", Some(validation_summary))
    {
        lines.push(line);
    }

    lines.join("\n")
}

fn verification_summary_field(label: &str, value: Option<&str>) -> Option<String> {
    let summary = verification_summary_excerpt(value?)?;
    Some(format!("- {label}: {summary}"))
}

fn verification_summary_excerpt(value: &str) -> Option<String> {
    let items = value.lines().filter_map(verification_summary_line).collect::<Vec<_>>();

    if items.is_empty() {
        let compact = compact_summary_line(value);
        if compact.is_empty() { None } else { Some(compact) }
    } else {
        let mut summary = items.iter().take(2).cloned().collect::<Vec<_>>().join("; ");
        if items.len() > 2 {
            summary.push_str(&format!("; +{} more", items.len() - 2));
        }
        Some(summary)
    }
}

fn verification_summary_line(line: &str) -> Option<String> {
    let trimmed = trim_markdown_list_prefix(line.trim());
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }

    let lowered = trimmed.to_ascii_lowercase();
    if lowered.starts_with("status:") || lowered.starts_with("rationale:") {
        return None;
    }

    let compact = compact_summary_line(trimmed);
    if compact.is_empty() { None } else { Some(compact) }
}

fn trim_markdown_list_prefix(value: &str) -> &str {
    let trimmed = value.trim_start();

    for prefix in ["- ", "* ", "+ "] {
        if let Some(stripped) = trimmed.strip_prefix(prefix) {
            return stripped.trim_start();
        }
    }

    let digit_count = trimmed.bytes().take_while(|byte| byte.is_ascii_digit()).count();
    if digit_count > 0
        && trimmed.as_bytes().get(digit_count) == Some(&b'.')
        && trimmed.as_bytes().get(digit_count + 1) == Some(&b' ')
    {
        return trimmed[digit_count + 2..].trim_start();
    }

    trimmed
}

pub fn render_review_artifact(
    file_name: &str,
    context_summary: &str,
    generation_summary: &str,
    critique_summary: &str,
    validation_summary: &str,
) -> String {
    let status = review_disposition_status(
        context_summary,
        generation_summary,
        critique_summary,
        validation_summary,
    );
    let missing_evidence_open = review_missing_evidence_open(
        context_summary,
        generation_summary,
        critique_summary,
        validation_summary,
    );
    let evidence_basis = if context_summary.contains("## Input:") {
        "- Review grounded in explicit authored inputs captured for this run.\n- Validation evidence remains linked from the persisted run bundle.".to_string()
    } else {
        "- Review grounded in the bounded authored packet supplied to this run.\n- Validation evidence remains linked from the persisted run bundle.".to_string()
    };
    let boundary_findings = if contains_case_insensitive(critique_summary, "scope drift")
        || contains_case_insensitive(context_summary, "boundary")
        || contains_case_insensitive(generation_summary, "boundary")
    {
        preserve_markdown_block(critique_summary)
    } else {
        "- No boundary expansion beyond the authored review target was detected.".to_string()
    };
    let ownership_notes = if contains_case_insensitive(context_summary, "owner") {
        "- Ownership remains anchored to the named run owner and supplied review target."
            .to_string()
    } else {
        "- Confirm the accountable reviewer before downstream acceptance.".to_string()
    };
    let missing_evidence = if missing_evidence_open {
        format!(
            "Status: missing-evidence-open\n\n{}\n\n{}",
            preserve_markdown_block(critique_summary),
            preserve_markdown_block(validation_summary)
        )
    } else {
        "Status: evidence-bounded\n\n- No critical evidence gaps were detected from the authored package.".to_string()
    };
    let collection_priorities = if missing_evidence_open {
        "- Capture the missing evidence before accepting the packet as release-ready.\n- Keep any remaining accepted risk explicit in the disposition artifact.".to_string()
    } else {
        "- Preserve the current evidence bundle for any later approval or downstream implementation review.".to_string()
    };
    let decision_impact = if generation_summary.trim().is_empty() {
        "- No decision impact summary was generated for this packet.".to_string()
    } else {
        preserve_markdown_block(generation_summary)
    };
    let reversibility = if status == "awaiting-disposition" {
        "- Downstream work should stop until the remaining review concerns receive explicit disposition.".to_string()
    } else {
        "- The packet remains reversible because the current concerns are recorded as bounded review notes.".to_string()
    };
    let accepted_risks = if status == "awaiting-disposition" {
        "- No accepted risks recorded while disposition is still pending.".to_string()
    } else {
        "- Residual review notes remain bounded to the current package and can be inspected through the emitted artifacts.".to_string()
    };
    let rationale = if status == "awaiting-disposition" {
        "The review packet records unresolved concerns or missing evidence that require explicit human disposition before release-readiness can pass.".to_string()
    } else {
        "The review packet is bounded enough for downstream inspection and no unresolved must-fix concerns require disposition approval.".to_string()
    };

    match file_name {
        "review-brief.md" => format!(
            "# Review Brief\n\n## Summary\n\n{}\n\n## Review Target\n\n{}\n\n## Evidence Basis\n\n{}\n",
            compact_summary_line(context_summary),
            preserve_markdown_block(generation_summary),
            evidence_basis,
        ),
        "boundary-assessment.md" => format!(
            "# Boundary Assessment\n\n## Summary\n\n{}\n\n## Boundary Findings\n\n{}\n\n## Ownership Notes\n\n{}\n",
            compact_summary_line(context_summary),
            boundary_findings,
            ownership_notes,
        ),
        "missing-evidence.md" => format!(
            "# Missing Evidence\n\n## Summary\n\n{}\n\n## Missing Evidence\n\n{}\n\n## Collection Priorities\n\n{}\n",
            compact_summary_line(context_summary),
            missing_evidence,
            collection_priorities,
        ),
        "decision-impact.md" => format!(
            "# Decision Impact\n\n## Summary\n\n{}\n\n## Decision Impact\n\n{}\n\n## Reversibility Concerns\n\n{}\n",
            compact_summary_line(context_summary),
            decision_impact,
            reversibility,
        ),
        "review-disposition.md" => format!(
            "# Review Disposition\n\n## Summary\n\n{}\n\n## Final Disposition\n\nStatus: {}\n\nRationale: {}\n\n## Accepted Risks\n\n{}\n",
            compact_summary_line(context_summary),
            status,
            rationale,
            accepted_risks,
        ),
        other => render_markdown(other, context_summary),
    }
}

pub fn render_verification_artifact(
    file_name: &str,
    context_summary: &str,
    generation_summary: &str,
    critique_summary: &str,
    validation_summary: &str,
) -> String {
    let has_open_findings = verification_has_open_findings(
        context_summary,
        generation_summary,
        critique_summary,
        validation_summary,
    );
    let verdict = verification_verdict(
        context_summary,
        generation_summary,
        critique_summary,
        validation_summary,
    );
    let claims_under_test = verification_section_from_sources(
        generation_summary,
        Some(context_summary),
        &["claims under test"],
    )
    .unwrap_or_else(|| {
        "- No explicit claims under test were captured for this verification packet.".to_string()
    });
    let evidence_basis = verification_section_from_sources(
        generation_summary,
        Some(context_summary),
        &["evidence basis"],
    );
    let contract_assumptions = verification_section_from_sources(
        generation_summary,
        Some(context_summary),
        &["contract assumptions", "contract surface"],
    )
    .unwrap_or_else(|| {
        "- Contract assumptions were not explicitly authored for this verification packet."
            .to_string()
    });
    let challenge_findings = append_markdown_blocks(
        verification_section_from_sources(
            critique_summary,
            Some(context_summary),
            &["challenge findings", "challenge focus"],
        )
        .unwrap_or_else(|| {
            if has_open_findings {
                "- The verification packet still carries challenge findings that require explicit follow-up."
                    .to_string()
            } else {
                "- No additional challenge findings were recorded beyond the authored verification packet."
                    .to_string()
            }
        }),
        Some(preserve_markdown_block(validation_summary)),
    );
    let contradictions = verification_section_from_sources(
        critique_summary,
        Some(context_summary),
        &["contradictions"],
    )
    .unwrap_or_else(|| {
        if has_open_findings {
            "- The verification packet still contains contradictions or evidence gaps that keep the verdict open."
                .to_string()
        } else {
            "- No direct contradictions were identified from the current verification packet."
                .to_string()
        }
    });
    let verified_claims = verification_section_from_sources(
        critique_summary,
        Some(generation_summary),
        &["verified claims"],
    )
    .unwrap_or_else(|| {
        if has_open_findings {
            "- The packet captures explicit claims and evidence basis, but the strongest assurances remain under challenge."
                .to_string()
        } else {
            claims_under_test.clone()
        }
    });
    let rejected_claims =
        verification_section_from_sources(critique_summary, None, &["rejected claims"])
            .unwrap_or_else(|| {
                if has_open_findings {
                    contradictions.clone()
                } else {
                    "- No rejected claims were inferred from the current verification target."
                        .to_string()
                }
            });
    let open_findings = verification_section_from_sources(
        critique_summary,
        None,
        &["open findings"],
    )
    .unwrap_or_else(|| {
        if has_open_findings {
            format!("Status: unresolved-findings-open\n\n{contradictions}")
        } else {
            "Status: no-open-findings\n\n- No unresolved findings remain from the current verification target."
                .to_string()
        }
    });
    let required_follow_up = verification_section_from_sources(
        critique_summary,
        None,
        &["required follow-up"],
    )
    .unwrap_or_else(|| {
        if has_open_findings {
            "- Resolve or explicitly challenge the open contradictions before treating the packet as release-ready.\n- Preserve the unresolved findings for downstream inspection."
                .to_string()
        } else {
            "- Keep the verification packet attached to any downstream release or approval discussion."
                .to_string()
        }
    });
    let overall_verdict = verification_section_from_sources(
        critique_summary,
        None,
        &["overall verdict"],
    )
    .unwrap_or_else(|| {
        format!(
            "Status: {verdict}\nRationale: Verification verdict was derived from the current packet because no explicit overall verdict section was emitted."
        )
    });
    let challenge_focus = verification_section_from_sources(
        context_summary,
        Some(critique_summary),
        &["challenge focus", "challenge findings"],
    );
    let verification_summary = render_verification_summary(
        &claims_under_test,
        evidence_basis.as_deref(),
        &contract_assumptions,
        challenge_focus.as_deref(),
        validation_summary,
        verdict,
        has_open_findings,
    );

    match file_name {
        "invariants-checklist.md" => format!(
            "# Invariants Checklist\n\n## Summary\n\n{}\n\n## Claims Under Test\n\n{}\n\n## Invariant Checks\n\n{}\n",
            verification_summary,
            claims_under_test,
            if has_open_findings {
                "- Some invariant checks remain unresolved against the current evidence bundle."
            } else {
                "- The current invariants are bounded enough for recorded verification."
            },
        ),
        "contract-matrix.md" => format!(
            "# Contract Matrix\n\n## Summary\n\n{}\n\n## Contract Assumptions\n\n{}\n\n## Verification Outcome\n\nStatus: {}\n",
            verification_summary, contract_assumptions, verdict,
        ),
        "adversarial-review.md" => format!(
            "# Adversarial Review\n\n## Summary\n\n{}\n\n## Challenge Findings\n\n{}\n\n## Contradictions\n\n{}\n",
            verification_summary, challenge_findings, contradictions,
        ),
        "verification-report.md" => format!(
            "# Verification Report\n\n## Summary\n\n{}\n\n## Verified Claims\n\n{}\n\n## Rejected Claims\n\n{}\n\n## Overall Verdict\n\n{}\n",
            verification_summary, verified_claims, rejected_claims, overall_verdict,
        ),
        "unresolved-findings.md" => format!(
            "# Unresolved Findings\n\n## Summary\n\n{}\n\n## Open Findings\n\n{}\n\n## Required Follow-up\n\n{}\n",
            verification_summary, open_findings, required_follow_up,
        ),
        other => render_markdown(other, context_summary),
    }
}

fn review_disposition_status(
    context_summary: &str,
    generation_summary: &str,
    critique_summary: &str,
    validation_summary: &str,
) -> &'static str {
    if review_missing_evidence_open(
        context_summary,
        generation_summary,
        critique_summary,
        validation_summary,
    ) || contains_case_insensitive(critique_summary, "must-fix")
        || contains_case_insensitive(generation_summary, "must-fix")
        || contains_case_insensitive(critique_summary, "blocking")
    {
        "awaiting-disposition"
    } else {
        "ready-with-review-notes"
    }
}

fn review_missing_evidence_open(
    context_summary: &str,
    generation_summary: &str,
    critique_summary: &str,
    validation_summary: &str,
) -> bool {
    [context_summary, generation_summary, critique_summary, validation_summary].into_iter().any(
        |value| {
            value.contains("NOT CAPTURED")
                || contains_case_insensitive(value, "missing evidence")
                || contains_case_insensitive(value, "insufficient evidence")
        },
    )
}

fn verification_has_open_findings(
    context_summary: &str,
    generation_summary: &str,
    critique_summary: &str,
    validation_summary: &str,
) -> bool {
    let critique_normalized = critique_summary.to_lowercase();
    if let Some(open_findings) =
        extract_marker(critique_summary, &critique_normalized, "open findings")
        && let Some(status) = extract_labeled_block_value(&open_findings, "Status")
    {
        return status.eq_ignore_ascii_case("unresolved-findings-open");
    }

    [context_summary, generation_summary, critique_summary, validation_summary].into_iter().any(
        |value| {
            value.contains("NOT CAPTURED")
                || contains_case_insensitive(value, "unsupported")
                || contains_case_insensitive(value, "contradiction")
                || contains_case_insensitive(value, "unresolved")
                || contains_case_insensitive(value, "insufficient evidence")
        },
    )
}

fn verification_verdict(
    context_summary: &str,
    generation_summary: &str,
    critique_summary: &str,
    validation_summary: &str,
) -> &'static str {
    let critique_normalized = critique_summary.to_lowercase();
    if let Some(overall_verdict) =
        extract_marker(critique_summary, &critique_normalized, "overall verdict")
        && let Some(status) = extract_labeled_block_value(&overall_verdict, "Status")
    {
        return match status.to_ascii_lowercase().as_str() {
            "supported" => "supported",
            "unsupported" => "unsupported",
            "mixed" => "mixed",
            _ => "mixed",
        };
    }

    if !verification_has_open_findings(
        context_summary,
        generation_summary,
        critique_summary,
        validation_summary,
    ) {
        "supported"
    } else if context_summary.contains("NOT CAPTURED")
        || contains_case_insensitive(critique_summary, "contradiction")
        || contains_case_insensitive(validation_summary, "contradiction")
    {
        "unsupported"
    } else {
        "mixed"
    }
}

fn verification_section_from_sources(
    primary_source: &str,
    secondary_source: Option<&str>,
    markers: &[&str],
) -> Option<String> {
    extract_any_marker(primary_source, markers)
        .or_else(|| secondary_source.and_then(|source| extract_any_marker(source, markers)))
}

fn extract_any_marker(source: &str, markers: &[&str]) -> Option<String> {
    let normalized = source.to_lowercase();

    markers.iter().find_map(|marker| {
        extract_marker(source, &normalized, marker)
            .map(|value| preserve_markdown_block(&value))
            .filter(|value| !value.is_empty())
    })
}

fn append_markdown_blocks(primary: String, secondary: Option<String>) -> String {
    let Some(secondary) =
        secondary.map(|value| value.trim().to_string()).filter(|value| !value.is_empty())
    else {
        return primary;
    };

    if primary.trim().is_empty() {
        secondary
    } else if primary.contains(&secondary) {
        primary
    } else {
        format!("{primary}\n\n{secondary}")
    }
}

fn extract_labeled_block_value(block: &str, label: &str) -> Option<String> {
    let prefix = format!("{}:", label.to_ascii_lowercase());

    block.lines().find_map(|line| {
        let trimmed = line.trim();
        if !trimmed.to_ascii_lowercase().starts_with(&prefix) {
            return None;
        }

        let value = trimmed[trimmed.find(':')? + 1..].trim();
        if value.is_empty() { None } else { Some(value.to_string()) }
    })
}

fn contains_case_insensitive(value: &str, needle: &str) -> bool {
    value.to_lowercase().contains(&needle.to_lowercase())
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

fn system_shaping_context_gap(intent: Option<&str>, constraint: Option<&str>) -> Option<String> {
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
        render_discovery_artifact, render_pr_review_artifact, render_requirements_artifact,
        render_requirements_artifact_from_evidence, render_review_artifact,
        render_system_shaping_artifact, render_verification_artifact, trim_markdown_list_prefix,
        verification_summary_excerpt,
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
    fn render_review_artifacts_distinguish_ready_and_pending_disposition_packets() {
        let ready = render_review_artifact(
            "review-disposition.md",
            "# Review Brief\n\nReview target stays bounded to a named service boundary.",
            "Review packet preserved evidence basis, boundary findings, and decision impact.",
            "Challenge the proposed review packet for evidence coverage, hidden scope growth, ownership clarity, and acceptance rationale.",
            "Validation tool reviewed tracked repository surfaces: src/lib.rs, tests/lib_test.rs",
        );
        let pending = render_review_artifact(
            "review-disposition.md",
            "# Review Brief\n\nMissing evidence remains for rollback coverage on this package.",
            "Must-fix review note remains open for the release boundary.",
            "Scope drift introduced a must-fix follow-up before acceptance.",
            "Validation tool reviewed tracked repository surfaces: src/lib.rs, tests/lib_test.rs",
        );

        assert!(ready.contains("Status: ready-with-review-notes"));
        assert!(pending.contains("Status: awaiting-disposition"));
    }

    #[test]
    fn render_verification_artifacts_distinguish_supported_and_blocked_packets() {
        let supported = render_verification_artifact(
            "verification-report.md",
            "# Verification Brief\n\nClaims Under Test: rollback remains bounded and auditable.\nEvidence Basis: contract notes and repository checks.\nContract Surface: rollback metadata stays explicit.",
            "## Claims Under Test\n\n- rollback remains bounded and auditable\n\n## Evidence Basis\n\n- contract notes\n- repository checks\n\n## Contract Assumptions\n\n- rollback metadata stays explicit",
            "## Challenge Findings\n\n- No additional challenge findings were recorded beyond the authored verification packet.\n\n## Contradictions\n\n- No direct contradictions were identified from the current verification packet.\n\n## Verified Claims\n\n- rollback remains bounded and auditable\n\n## Rejected Claims\n\n- No rejected claims were inferred from the current verification target.\n\n## Open Findings\n\nStatus: no-open-findings\n\n- No unresolved findings remain from the current verification target.\n\n## Required Follow-up\n\n- Keep the verification packet attached to downstream release or approval discussion.\n\n## Overall Verdict\n\nStatus: supported\nRationale: No explicit contradiction or proof gap remained in the normalized packet.",
            "Validation tool reviewed tracked repository surfaces: src/lib.rs, tests/lib_test.rs",
        );
        let blocked = render_verification_artifact(
            "verification-report.md",
            "# Verification Brief\n\nClaims Under Test: an unresolved contradiction remains between the authored claim and the captured contract.\nEvidence Basis: unsupported rollback guarantee still lacks concrete proof.",
            "## Claims Under Test\n\n- an unresolved contradiction remains between the authored claim and the captured contract\n\n## Evidence Basis\n\n- unsupported rollback guarantee still lacks concrete proof\n\n## Contract Assumptions\n\n- rollback metadata stays explicit",
            "## Challenge Findings\n\n- The authored claim already signals a contradiction or missing-evidence path.\n\n## Contradictions\n\n- The authored claim under test already records a contradiction or unresolved support gap.\n\n## Verified Claims\n\n- The evidence basis is explicit enough for downstream inspection and follow-up.\n\n## Rejected Claims\n\n- Still unsupported from the current packet: an unresolved contradiction remains between the authored claim and the captured contract\n\n## Open Findings\n\nStatus: unresolved-findings-open\n\n- Resolve the contradiction before treating the packet as supported.\n\n## Required Follow-up\n\n- Resolve the contradictions or proof gaps before treating the packet as supported.\n\n## Overall Verdict\n\nStatus: unsupported\nRationale: The packet still carries unresolved findings against the named claim.",
            "Validation tool reviewed tracked repository surfaces: src/lib.rs, tests/lib_test.rs",
        );

        assert!(supported.contains("Status: supported"));
        assert!(blocked.contains("Status: unsupported"));
    }

    #[test]
    fn render_verification_artifacts_map_structured_sections_without_prompt_echo() {
        let context = "# Verification Brief\n\n## Claims Under Test\n- CSS sanitization blocks hostile style execution\n- CSS URL filtering stays bounded\n\n## Evidence Basis\n- README assertions\n- parser tests\n\n## Contract Surface\n- style attributes and CSS URLs stay conservative\n\n## Challenge Focus\n- look for unsupported jumps from parser coverage to full CSS-XSS coverage";
        let generation = "## Claims Under Test\n\n- CSS sanitization blocks hostile style execution\n- CSS URL filtering stays bounded\n\n## Evidence Basis\n\n- README assertions\n- parser tests\n\n## Contract Assumptions\n\n- style attributes and CSS URLs stay conservative";
        let critique = "## Challenge Findings\n\n- Authored challenge focus remains open until explicit evidence answers it: look for unsupported jumps from parser coverage to full CSS-XSS coverage\n\n## Contradictions\n\n- The packet still needs explicit adversarial proof for its broadest CSS-XSS assurances.\n\n## Verified Claims\n\n- The evidence basis is explicit enough for downstream inspection and follow-up.\n\n## Rejected Claims\n\n- Still unsupported from the current packet: CSS sanitization blocks hostile style execution\n\n## Open Findings\n\nStatus: unresolved-findings-open\n\n- Answer this authored challenge focus with explicit evidence or narrow the affected claim: look for unsupported jumps from parser coverage to full CSS-XSS coverage\n\n## Required Follow-up\n\n- Address each authored challenge-focus item with explicit evidence or narrow the affected claim.\n\n## Overall Verdict\n\nStatus: unsupported\nRationale: The packet still carries unresolved CSS evidence gaps.";

        let contract = render_verification_artifact(
            "contract-matrix.md",
            context,
            generation,
            critique,
            "Validation tool reviewed tracked repository surfaces: README.md, src/css.rs",
        );
        let report = render_verification_artifact(
            "verification-report.md",
            context,
            generation,
            critique,
            "Validation tool reviewed tracked repository surfaces: README.md, src/css.rs",
        );

        assert!(contract.contains("style attributes and CSS URLs stay conservative"));
        assert!(report.contains("Still unsupported from the current packet: CSS sanitization blocks hostile style execution"));
        assert!(
            report.contains("Rationale: The packet still carries unresolved CSS evidence gaps.")
        );
        assert!(!report.contains("Challenge the verification packet for claim support strength"));
        assert!(!report.contains("# Verification Brief Claims Under Test:"));
        assert!(report.contains("- Claims under test: CSS sanitization blocks hostile style execution; CSS URL filtering stays bounded"));
        assert!(report.contains("- Evidence basis: README assertions; parser tests"));
        assert!(report.contains("- Verdict: unsupported"));
    }

    #[test]
    fn verification_summary_excerpt_skips_headings_status_and_rationale() {
        let summary = verification_summary_excerpt(
            "# Verification Brief\n\n## Claims Under Test\n- claim one\n- claim two\nStatus: unsupported\nRationale: still open",
        )
        .expect("summary excerpt");

        assert_eq!(summary, "claim one; claim two");
    }

    #[test]
    fn verification_summary_excerpt_limits_extra_list_items() {
        let summary = verification_summary_excerpt(
            "1. first evidence anchor\n2. second evidence anchor\n3. third evidence anchor",
        )
        .expect("summary excerpt");

        assert_eq!(summary, "first evidence anchor; second evidence anchor; +1 more");
    }

    #[test]
    fn trim_markdown_list_prefix_handles_numbered_and_unordered_lists() {
        assert_eq!(trim_markdown_list_prefix("- first claim"), "first claim");
        assert_eq!(trim_markdown_list_prefix("* second claim"), "second claim");
        assert_eq!(trim_markdown_list_prefix("12. numbered claim"), "numbered claim");
        assert_eq!(trim_markdown_list_prefix("plain sentence"), "plain sentence");
    }

    #[test]
    fn render_verification_artifact_summary_omits_challenge_focus_when_supported() {
        let report = render_verification_artifact(
            "verification-report.md",
            "# Verification Brief\n\n## Claims Under Test\n- rollback remains bounded\n\n## Evidence Basis\n- audit log checks\n\n## Contract Surface\n- rollback metadata stays explicit\n\n## Challenge Focus\n- verify cross-module ownership stays explicit",
            "## Claims Under Test\n\n- rollback remains bounded\n\n## Evidence Basis\n\n- audit log checks\n\n## Contract Assumptions\n\n- rollback metadata stays explicit",
            "## Challenge Findings\n\n- No additional challenge findings were recorded beyond the authored verification packet.\n\n## Contradictions\n\n- No direct contradictions were identified from the current verification packet.\n\n## Verified Claims\n\n- rollback remains bounded\n\n## Rejected Claims\n\n- No rejected claims were inferred from the current verification target.\n\n## Open Findings\n\nStatus: no-open-findings\n\n- No unresolved findings remain from the current verification target.\n\n## Required Follow-up\n\n- Keep the verification packet attached to downstream review.\n\n## Overall Verdict\n\nStatus: supported\nRationale: The packet is sufficiently supported.",
            "Validation tool reviewed tracked repository surfaces: src/lib.rs",
        );

        assert!(report.contains("- Open findings: none recorded."));
        assert!(!report.contains("- Challenge focus:"));
    }

    #[test]
    fn render_verification_artifact_summary_includes_challenge_focus_when_unresolved() {
        let report = render_verification_artifact(
            "verification-report.md",
            "# Verification Brief\n\n## Claims Under Test\n- css sanitization blocks hostile style execution\n\n## Evidence Basis\n- parser tests\n\n## Contract Surface\n- css URLs stay conservative\n\n## Challenge Focus\n- prove broad css-xss coverage instead of parser-only confidence",
            "## Claims Under Test\n\n- css sanitization blocks hostile style execution\n\n## Evidence Basis\n\n- parser tests\n\n## Contract Assumptions\n\n- css URLs stay conservative",
            "## Challenge Findings\n\n- Broad css-xss coverage remains unsupported.\n\n## Contradictions\n\n- Parser coverage does not prove the broader claim.\n\n## Verified Claims\n\n- parser tests are present\n\n## Rejected Claims\n\n- css sanitization blocks hostile style execution\n\n## Open Findings\n\nStatus: unresolved-findings-open\n\n- Explicit contradiction remains open.\n\n## Required Follow-up\n\n- Narrow the claim or add adversarial proof.\n\n## Overall Verdict\n\nStatus: unsupported\nRationale: The broadest claim is still unsupported.",
            "Validation tool reviewed tracked repository surfaces: README.md, src/css.rs",
        );

        assert!(report.contains(
            "- Challenge focus: prove broad css-xss coverage instead of parser-only confidence"
        ));
        assert!(
            report
                .contains("- Open findings: unresolved follow-up remains recorded in this packet.")
        );
    }

    #[test]
    fn render_review_artifacts_preserve_multiline_bullets() {
        let artifact = render_review_artifact(
            "missing-evidence.md",
            "Review packet remains bounded.",
            "Decision impact remains bounded.",
            "Missing evidence:\n- Missing rollback rehearsal\n- Missing owner sign-off",
            "- Validation confirmed only partial repository coverage\n- Follow-up evidence still required",
        );

        assert!(
            artifact.contains(
                "Missing evidence:\n- Missing rollback rehearsal\n- Missing owner sign-off"
            )
        );
        assert!(artifact.contains(
            "- Validation confirmed only partial repository coverage\n- Follow-up evidence still required"
        ));
        assert!(!artifact.contains("- Missing rollback rehearsal - Missing owner sign-off"));
    }

    #[test]
    fn render_verification_artifacts_preserve_multiline_sections() {
        let invariants = render_verification_artifact(
            "invariants-checklist.md",
            "Verification remains bounded.",
            "## Claims Under Test\n- Claim one\n- Claim two",
            "## Contradictions\n- First contradiction\n- Second contradiction",
            "- Validation requires an independent follow-up",
        );
        let review = render_verification_artifact(
            "adversarial-review.md",
            "Verification remains bounded.",
            "## Claims Under Test\n- Claim one\n- Claim two",
            "## Contradictions\n- First contradiction\n- Second contradiction",
            "- Validation requires an independent follow-up",
        );

        assert!(
            review.contains("## Contradictions\n\n- First contradiction\n- Second contradiction")
        );
        assert!(invariants.contains("## Claims Under Test\n\n- Claim one\n- Claim two"));
        assert!(!review.contains("- First contradiction - Second contradiction"));
    }

    #[test]
    fn render_analysis_mode_artifacts_include_required_sections() {
        let discovery = render_discovery_artifact(
            "context-boundary.md",
            "# Discovery Brief\n\n## Problem\nExplore a bounded notification routing problem.\n\n## Constraints\nPreserve the current routing ownership boundaries.\n\n## Repo Surface\n- src/router.rs\n- tests/router_contract.rs\n\n## Unknowns\n- Which caller owns retry policy?\n\n## Next Phase\nTranslate this discovery packet into architecture mode with named boundaries.\n\nGenerated framing: Map the known constraints and unresolved actors.\n\nCritique evidence: Challenge scope drift around retry ownership.\n\nValidation evidence: Validation tool reviewed tracked repository surfaces: src/router.rs, tests/router_contract.rs",
        );
        let system_shaping = render_system_shaping_artifact(
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
        assert!(system_shaping.contains("## System Shape"));
        assert!(system_shaping.contains("## Boundary Decisions"));
        assert!(architecture.contains("## Evaluation Criteria"));
        assert!(architecture.contains("## Selected Option"));
    }
}
