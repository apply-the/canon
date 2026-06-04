use super::*;

pub(super) fn render_backlog_artifact(
    file_name: &str,
    brief_summary: &str,
    planning_context: &BacklogPlanningContext,
) -> String {
    let file_name = artifact_slug(file_name);
    let normalized = brief_summary.to_lowercase();
    let delivery_intent = extract_marker(brief_summary, &normalized, "delivery intent")
        .unwrap_or_else(|| planning_context.delivery_intent.clone());
    let planning_horizon = extract_marker(brief_summary, &normalized, "planning horizon")
        .or_else(|| planning_context.planning_horizon.clone())
        .unwrap_or_else(|| "No explicit planning horizon was authored.".to_string());
    let source_refs = extract_marker(brief_summary, &normalized, "source references")
        .or_else(|| extract_marker(brief_summary, &normalized, "source inputs"))
        .unwrap_or_else(|| {
            render_string_list(
                &planning_context.source_refs,
                "- No explicit source references were recorded.",
            )
        });
    let priorities =
        extract_marker(brief_summary, &normalized, "priorities").unwrap_or_else(|| {
            render_string_list(
                &planning_context.priority_inputs,
                "- No explicit planning priorities were recorded.",
            )
        });
    let constraints =
        extract_marker(brief_summary, &normalized, "constraints").unwrap_or_else(|| {
            render_string_list(
                &planning_context.constraints,
                "- No explicit planning constraints were recorded.",
            )
        });
    let out_of_scope =
        extract_marker(brief_summary, &normalized, "out of scope").unwrap_or_else(|| {
            render_string_list(
                &planning_context.out_of_scope,
                "- No explicit exclusions were recorded.",
            )
        });
    let closure_findings = extract_marker(brief_summary, &normalized, "closure findings")
        .unwrap_or_else(|| {
            if planning_context.closure_assessment.findings.is_empty() {
                "- No closure findings remain open.".to_string()
            } else {
                planning_context
                    .closure_assessment
                    .findings
                    .iter()
                    .map(|finding| {
                        format!(
                            "- [{}] {} on {}. Follow-up: {}",
                            finding.severity.as_str(),
                            finding.category,
                            finding.affected_scope,
                            finding.recommended_followup
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        });
    let generated_framing = extract_marker(brief_summary, &normalized, "generated framing")
        .unwrap_or_else(|| "Generated backlog framing was not recorded.".to_string());
    let critique_evidence = extract_marker(brief_summary, &normalized, "critique evidence")
        .unwrap_or_else(|| "Critique evidence was not recorded.".to_string());
    let validation_evidence = extract_marker(brief_summary, &normalized, "validation evidence")
        .unwrap_or_else(|| "Validation evidence was not recorded.".to_string());
    let decomposition_posture = planning_context.closure_assessment.decomposition_scope.as_str();

    let authored_epic_tree = extract_marker(brief_summary, &normalized, "epic tree");
    let authored_capability_map =
        extract_marker(brief_summary, &normalized, "capability to epic map")
            .or_else(|| extract_marker(brief_summary, &normalized, "capability map"))
            .or_else(|| extract_marker(brief_summary, &normalized, "capability mapping"));
    let authored_dependency_map = extract_marker(brief_summary, &normalized, "dependency map")
        .or_else(|| extract_marker(brief_summary, &normalized, "dependencies"));
    let authored_delivery_slices = extract_marker(brief_summary, &normalized, "delivery slices")
        .or_else(|| extract_marker(brief_summary, &normalized, "slices"));
    let authored_sequencing = extract_marker(brief_summary, &normalized, "sequencing plan")
        .or_else(|| extract_marker(brief_summary, &normalized, "sequencing"));
    let authored_acceptance = extract_marker(brief_summary, &normalized, "acceptance anchors")
        .or_else(|| extract_marker(brief_summary, &normalized, "acceptance criteria"));
    let authored_planning_risks = extract_marker(brief_summary, &normalized, "planning risks")
        .or_else(|| extract_marker(brief_summary, &normalized, "risks"));
    let handoff_availability = render_handoff_availability(planning_context);
    let stable_slice_ids = render_slice_ids(planning_context);

    match file_name {
        "backlog-overview.md" => render_backlog_overview_artifact(
            &delivery_intent,
            &generated_framing,
            &planning_horizon,
            &source_refs,
            decomposition_posture,
            &handoff_availability,
        ),
        "epic-tree.md" => render_epic_tree_artifact(
            &delivery_intent,
            authored_epic_tree.as_deref(),
            planning_context.desired_granularity.as_str(),
            &out_of_scope,
            &source_refs,
        ),
        "capability-to-epic-map.md" => render_capability_to_epic_map_artifact(
            &delivery_intent,
            authored_capability_map.as_deref(),
            &source_refs,
            &closure_findings,
        ),
        "dependency-map.md" => render_dependency_map_artifact(
            &delivery_intent,
            authored_dependency_map.as_deref(),
            &stable_slice_ids,
            &constraints,
            &closure_findings,
        ),
        "delivery-slices.md" => render_delivery_slices_artifact(
            &delivery_intent,
            authored_delivery_slices.as_deref(),
            &stable_slice_ids,
            &out_of_scope,
            &source_refs,
        ),
        "sequencing-plan.md" => render_sequencing_plan_artifact(
            &delivery_intent,
            authored_sequencing.as_deref(),
            &stable_slice_ids,
            &priorities,
        ),
        "acceptance-anchors.md" => render_acceptance_anchors_artifact(
            &delivery_intent,
            authored_acceptance.as_deref(),
            &stable_slice_ids,
            &source_refs,
        ),
        "planning-risks.md" => render_planning_risks_artifact(
            &delivery_intent,
            authored_planning_risks.as_deref(),
            &closure_findings,
            &critique_evidence,
            &validation_evidence,
        ),
        "execution-handoff.md" => render_execution_handoff_artifact(planning_context),
        other => render_markdown(other, brief_summary),
    }
}

fn render_backlog_overview_artifact(
    delivery_intent: &str,
    generated_framing: &str,
    planning_horizon: &str,
    source_refs: &str,
    decomposition_posture: &str,
    handoff_availability: &str,
) -> String {
    format!(
        "# Backlog Overview\n\n## Summary\n\n{}\n\n## Scope\n\n{}\n\n## Planning Horizon\n\n{}\n\n## Source Inputs\n\n{}\n\n## Delivery Intent\n\n{}\n\n## Decomposition Posture\n\n{}\n\n## Execution Handoff\n\n{}\n",
        delivery_intent,
        truncate_context_excerpt(generated_framing, 260),
        planning_horizon,
        source_refs,
        delivery_intent,
        decomposition_posture,
        handoff_availability,
    )
}

fn render_epic_tree_artifact(
    delivery_intent: &str,
    authored_body: Option<&str>,
    desired_granularity: &str,
    out_of_scope: &str,
    source_refs: &str,
) -> String {
    match authored_body {
        Some(body) => format!(
            "# Epic Tree\n\n## Summary\n\n{}\n\n## Epic Tree\n\n{}\n\n## Scope Boundaries\n\n- Preserve planning-only granularity at {}.\n- Keep excluded work explicit: {}\n\n## Source Trace Links\n\n{}\n",
            delivery_intent,
            body,
            desired_granularity,
            truncate_context_excerpt(out_of_scope, 200),
            source_refs,
        ),
        None => format!(
            "# Epic Tree\n\n## Summary\n\n{}\n\n## Epic Tree\n\nNo authored epic tree was recorded for this packet.\n\n## Scope Boundaries\n\n- Preserve planning-only granularity at {}.\n- Keep excluded work explicit: {}\n\n## Source Trace Links\n\n{}\n\n## Missing Authored Body\n\nNo `## Epic Tree` section was authored in the backlog input. Canon did not synthesize placeholder epics because that would look like approved decomposition. Author a real epic tree in the backlog input and rerun.\n",
            delivery_intent,
            desired_granularity,
            truncate_context_excerpt(out_of_scope, 200),
            source_refs,
        ),
    }
}

fn render_capability_to_epic_map_artifact(
    delivery_intent: &str,
    authored_body: Option<&str>,
    source_refs: &str,
    closure_findings: &str,
) -> String {
    match authored_body {
        Some(body) => format!(
            "# Capability To Epic Map\n\n## Summary\n\n{}\n\n## Capability Mapping\n\n{}\n\n## Source Trace Links\n\n{}\n\n## Planning Gaps\n\n{}\n",
            delivery_intent, body, source_refs, closure_findings,
        ),
        None => format!(
            "# Capability To Epic Map\n\n## Summary\n\n{}\n\n## Capability Mapping\n\nNo authored capability-to-epic map was recorded for this packet.\n\n## Source Trace Links\n\n{}\n\n## Planning Gaps\n\n{}\n\n## Missing Authored Body\n\nNo `## Capability To Epic Map` section was authored in the backlog input. Canon did not infer capability-to-epic mappings from priorities alone because that would look like approved reasoning.\n",
            delivery_intent, source_refs, closure_findings,
        ),
    }
}

fn render_dependency_map_artifact(
    delivery_intent: &str,
    authored_body: Option<&str>,
    stable_slice_ids: &str,
    constraints: &str,
    closure_findings: &str,
) -> String {
    match authored_body {
        Some(body) => format!(
            "# Dependency Map\n\n## Summary\n\n{}\n\n## Dependencies\n\n{}\n\n## Slice IDs\n\n{}\n\n## Blocking Edges\n\n{}\n\n## External Dependencies\n\n- Any external blockers must remain visible in planning risks before downstream implementation work starts.\n",
            delivery_intent, body, stable_slice_ids, closure_findings,
        ),
        None => format!(
            "# Dependency Map\n\n## Summary\n\n{}\n\n## Dependencies\n\nNo authored dependency map was recorded for this packet. Shared planning constraints remain: {}\n\n## Slice IDs\n\n{}\n\n## Blocking Edges\n\n{}\n\n## External Dependencies\n\n- Source references remain the upstream dependency basis.\n- Any external blockers must remain visible in planning risks before downstream implementation work starts.\n\n## Missing Authored Body\n\nNo `## Dependency Map` section was authored in the backlog input. Canon kept only explicit constraints and closure findings instead of inventing dependency edges.\n",
            delivery_intent,
            truncate_context_excerpt(constraints, 220),
            stable_slice_ids,
            closure_findings,
        ),
    }
}

fn render_delivery_slices_artifact(
    delivery_intent: &str,
    authored_body: Option<&str>,
    stable_slice_ids: &str,
    out_of_scope: &str,
    source_refs: &str,
) -> String {
    match authored_body {
        Some(body) => format!(
            "# Delivery Slices\n\n## Summary\n\n{}\n\n## Delivery Slices\n\n{}\n\n## Slice IDs\n\n{}\n\n## Slice Boundaries\n\n- Slices stay above task level and stop at implementation-ready decomposition.\n- Excluded work stays explicit: {}\n\n## Dependency Links\n\n{}\n",
            delivery_intent,
            body,
            stable_slice_ids,
            truncate_context_excerpt(out_of_scope, 220),
            source_refs,
        ),
        None => format!(
            "# Delivery Slices\n\n## Summary\n\n{}\n\n## Delivery Slices\n\nNo authored delivery slices were recorded for this packet.\n\n## Slice IDs\n\n{}\n\n## Slice Boundaries\n\n- Slices stay above task level and stop at implementation-ready decomposition.\n- Excluded work stays explicit: {}\n\n## Dependency Links\n\n{}\n\n## Missing Authored Body\n\nNo `## Delivery Slices` section was authored in the backlog input. Canon did not synthesize example slices because that would look like approved decomposition.\n",
            delivery_intent,
            stable_slice_ids,
            truncate_context_excerpt(out_of_scope, 220),
            source_refs,
        ),
    }
}

fn render_sequencing_plan_artifact(
    delivery_intent: &str,
    authored_body: Option<&str>,
    stable_slice_ids: &str,
    priorities: &str,
) -> String {
    match authored_body {
        Some(body) => format!(
            "# Sequencing Plan\n\n## Summary\n\n{}\n\n## Sequencing\n\n{}\n\n## Slice IDs\n\n{}\n\n## Ordering Rationale\n\n{}\n\n## Readiness Signals\n\n- A downstream implementation reader can identify one bounded slice, its dependencies, and its acceptance anchor without hidden context.\n- Closure findings remain explicit if they still weaken confidence.\n",
            delivery_intent,
            body,
            stable_slice_ids,
            truncate_context_excerpt(priorities, 220),
        ),
        None => format!(
            "# Sequencing Plan\n\n## Summary\n\n{}\n\n## Sequencing\n\nNo authored sequencing plan was recorded for this packet.\n\n## Slice IDs\n\n{}\n\n## Ordering Rationale\n\n{}\n\n## Readiness Signals\n\n- A downstream implementation reader can identify one bounded slice, its dependencies, and its acceptance anchor without hidden context.\n- Closure findings remain explicit if they still weaken confidence.\n\n## Missing Authored Body\n\nNo `## Sequencing` or `## Sequencing Plan` section was authored in the backlog input. Canon did not invent sequencing steps from priorities alone.\n",
            delivery_intent,
            stable_slice_ids,
            truncate_context_excerpt(priorities, 220),
        ),
    }
}

fn render_acceptance_anchors_artifact(
    delivery_intent: &str,
    authored_body: Option<&str>,
    stable_slice_ids: &str,
    source_refs: &str,
) -> String {
    match authored_body {
        Some(body) => format!(
            "# Acceptance Anchors\n\n## Summary\n\n{}\n\n## Acceptance Anchors\n\n{}\n\n## Slice IDs\n\n{}\n\n## Source Trace Links\n\n{}\n\n## Deferred Detail\n\n- Task breakdown, tracker-specific work items, and executable test plans remain downstream work.\n",
            delivery_intent, body, stable_slice_ids, source_refs,
        ),
        None => format!(
            "# Acceptance Anchors\n\n## Summary\n\n{}\n\n## Acceptance Anchors\n\nNo authored acceptance anchors were recorded for this packet.\n\n## Slice IDs\n\n{}\n\n## Source Trace Links\n\n{}\n\n## Deferred Detail\n\n- Task breakdown, tracker-specific work items, and executable test plans remain downstream work.\n\n## Missing Authored Body\n\nNo `## Acceptance Anchors` section was authored in the backlog input. Canon did not fabricate acceptance anchors from traceability alone.\n",
            delivery_intent, stable_slice_ids, source_refs,
        ),
    }
}

fn render_planning_risks_artifact(
    delivery_intent: &str,
    authored_body: Option<&str>,
    closure_findings: &str,
    critique_evidence: &str,
    validation_evidence: &str,
) -> String {
    match authored_body {
        Some(body) => format!(
            "# Planning Risks\n\n## Summary\n\n{}\n\n## Closure Findings\n\n{}\n\n## Planning Risks\n\n{}\n\n## Follow-Up Triggers\n\n- Return to architecture or change when closure findings stay blocking.\n- Strengthen the authored backlog brief when exclusions or priorities remain vague.\n- Re-run backlog only after the bounded upstream packet becomes more credible.\n",
            delivery_intent, closure_findings, body,
        ),
        None => format!(
            "# Planning Risks\n\n## Summary\n\n{}\n\n## Closure Findings\n\n{}\n\n## Planning Risks\n\nNo authored planning risks were recorded for this packet.\n\n## Evidence Boundaries\n\n- Critique evidence remains: {}\n- Validation evidence remains: {}\n- Granularity drift risk remains explicit: backlog output must stay above task level.\n\n## Missing Authored Body\n\nNo `## Planning Risks` section was authored in the backlog input. Canon preserved closure findings and cited evidence boundaries instead of inventing risk bullets.\n\n## Follow-Up Triggers\n\n- Return to architecture or change when closure findings stay blocking.\n- Strengthen the authored backlog brief when exclusions or priorities remain vague.\n- Re-run backlog only after the bounded upstream packet becomes more credible.\n",
            delivery_intent,
            closure_findings,
            truncate_context_excerpt(critique_evidence, 220),
            truncate_context_excerpt(validation_evidence, 220),
        ),
    }
}

fn render_execution_handoff_artifact(planning_context: &BacklogPlanningContext) -> String {
    match planning_context.execution_handoff.as_ref() {
        Some(handoff) => format!(
            "# Execution Handoff\n\n## Summary\n\n{}\n\n## Selected Slice\n\n{}\n\n## Implementation Artifact References\n\n{}\n\n## Dependency Prerequisites\n\n{}\n\n## Independent Verification Anchors\n\n{}\n\n## Execution Boundary\n\n- Canon emits governed handoff signals but does not grant execution authority.\n- Task breakdown and runtime-specific admission remain downstream work.\n- Blocked assumptions remain explicit: {}\n",
            truncate_context_excerpt(&handoff.selection_rationale, 220),
            handoff.selected_slice_id,
            render_string_list(
                &handoff.implementation_artifact_refs,
                "- No implementation artifact references were recorded.",
            ),
            render_string_list(
                &handoff.dependency_prerequisites,
                "- No dependency prerequisites were recorded.",
            ),
            render_string_list(
                &handoff.independent_verification_anchors,
                "- No independent verification anchors were recorded.",
            ),
            render_string_list(
                &handoff.blocked_assumptions,
                "- No blocked assumptions remain explicit.",
            ),
        ),
        None => render_markdown(
            "Execution Handoff",
            "No governed execution handoff is available for this backlog packet.",
        ),
    }
}

fn render_handoff_availability(planning_context: &BacklogPlanningContext) -> String {
    let summary = match planning_context.handoff_availability {
        crate::domain::run::BacklogHandoffAvailability::Available => {
            "governed execution handoff is available"
        }
        crate::domain::run::BacklogHandoffAvailability::Unavailable => "handoff unavailable",
        crate::domain::run::BacklogHandoffAvailability::WithheldForClosure => {
            "handoff withheld for closure reasons"
        }
    };
    let findings = render_string_list(
        &planning_context.handoff_findings,
        "- No explicit handoff findings were recorded.",
    );
    format!("{summary}\n\n{findings}")
}

fn render_slice_ids(planning_context: &BacklogPlanningContext) -> String {
    render_string_list(&planning_context.slice_ids, "- No stable slice identifiers were recorded.")
}
