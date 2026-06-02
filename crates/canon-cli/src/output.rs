//! CLI output rendering.
//!
//! `print_value`, `print_run_summary`, `print_status_summary`, and
//! `print_inspect` are the only public entry points.  All Markdown and
//! text-mode rendering is delegated to the private submodules below.

use canon_engine::{RunSummary, StatusSummary};
use serde::Serialize;

use crate::app::OutputFormat;
use crate::error::CliResult;

mod dispatch;
mod inspect;
mod primitives;
mod run;

use dispatch::render_markdown_from_json;
use inspect::{render_refinement_text, render_risk_zone_text};
use run::{
    render_run_summary_markdown, render_status_summary_markdown, render_status_summary_text,
};

/// Serialises `value` to the requested output format and prints it to stdout.
pub fn print_value<T: Serialize>(value: &T, format: OutputFormat) -> CliResult<()> {
    match format {
        OutputFormat::Text => {
            println!("{}", serde_json::to_string_pretty(value)?);
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(value)?);
        }
        OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(value)?);
        }
        OutputFormat::Markdown => {
            println!("{}", serde_json::to_string_pretty(value)?);
        }
    }

    Ok(())
}

/// Prints a [`RunSummary`], rendering Markdown when that format is requested.
pub fn print_run_summary(summary: &RunSummary, format: OutputFormat) -> CliResult<()> {
    match format {
        OutputFormat::Markdown => {
            println!("{}", render_run_summary_markdown(summary));
            Ok(())
        }
        other => print_value(summary, other),
    }
}

/// Prints a [`StatusSummary`], rendering Markdown when that format is requested.
pub fn print_status_summary(summary: &StatusSummary, format: OutputFormat) -> CliResult<()> {
    match format {
        OutputFormat::Text => {
            println!("{}", render_status_summary_text(summary));
            Ok(())
        }
        OutputFormat::Markdown => {
            println!("{}", render_status_summary_markdown(summary));
            Ok(())
        }
        other => print_value(summary, other),
    }
}

/// Prints an inspect payload, choosing the renderer based on `target_name` and
/// `format`.
///
/// The `risk-zone` target in `Text` mode produces machine-parsable KEY=VALUE
/// output; all other targets fall through to `print_value`.  In `Markdown`
/// mode, the payload is routed to a dedicated renderer via
/// [`dispatch::render_markdown_from_json`].
pub fn print_inspect<T: Serialize>(
    value: &T,
    target_name: &str,
    run_id: Option<&str>,
    format: OutputFormat,
) -> CliResult<()> {
    match format {
        OutputFormat::Text if target_name == "risk-zone" => {
            let json = serde_json::to_value(value)?;
            println!("{}", render_risk_zone_text(&json));
            Ok(())
        }
        OutputFormat::Text if target_name == "refinement" => {
            let json = serde_json::to_value(value)?;
            println!("{}", render_refinement_text(&json, run_id));
            Ok(())
        }
        OutputFormat::Markdown => {
            let json = serde_json::to_value(value)?;
            println!("{}", render_markdown_from_json(&json, target_name, run_id));
            Ok(())
        }
        other => print_value(value, other),
    }
}

#[cfg(test)]
mod tests {
    use canon_engine::orchestrator::service::{
        InspectResponse, RefinementCandidateSummary, RefinementStateSummary,
    };
    use canon_engine::{
        GateInspectSummary, ModeResultSummary, RecommendedActionSummary, ResultActionSummary,
        RunSummary, StatusSummary,
    };
    use serde_json::{Value, json};

    use super::dispatch::render_markdown_from_json;
    use super::inspect::{render_refinement_text, render_risk_zone_text};
    use super::primitives::{render_kv_field, render_scalar_field, scalar_value};
    use super::run::{
        render_run_summary_markdown, render_status_summary_markdown, render_status_summary_text,
    };
    use super::{print_inspect, print_status_summary};
    use crate::app::OutputFormat;

    #[test]
    fn clarity_markdown_surfaces_questions_and_signals() {
        let value = json!({
            "entries": [{
                "mode": "requirements",
                "summary": "Problem framing: Build a bounded USB flashing CLI.\nDesired outcome: Operators can flash firmware safely over USB with explicit logs.\nSource inputs: idea.md",
                "source_inputs": ["idea.md"],
                "requires_clarification": true,
                "missing_context": [
                    "Constraints are incomplete; downstream shaping would lack explicit non-negotiables."
                ],
                "clarification_questions": [{
                    "id": "clarify-constraints",
                    "prompt": "Which constraints are non-negotiable for this work?",
                    "rationale": "Constraints determine whether downstream shaping stays repo-specific instead of becoming generic planning advice.",
                    "evidence": "No authored `## Constraints`, `## Constraint`, or `## Non-Negotiables` section was detected in the supplied inputs.",
                    "affects": "options.md",
                    "default_if_skipped": "Keep the packet conditional until the non-negotiables are explicit.",
                    "status": "required"
                }],
                "reasoning_signals": [
                    "Detected 1 authored input surface(s): idea.md."
                ],
                "output_quality": {
                    "posture": "structurally-complete",
                    "materially_closed": false,
                    "evidence_signals": [
                        "Detected 1 authored input surface(s): idea.md."
                    ],
                    "downgrade_reasons": [
                        "Constraints are incomplete; downstream shaping would lack explicit non-negotiables."
                    ]
                },
                "authoring_lifecycle": {
                    "packet_shape": "single-file",
                    "authority_status": "single-input-authoritative-brief",
                    "authoritative_inputs": ["idea.md"],
                    "supporting_inputs": [],
                    "readiness_delta": [
                        "Constraints are incomplete; downstream shaping would lack explicit non-negotiables.",
                        "1 clarification question(s) still remain before this packet is unambiguously ready."
                    ],
                    "next_authoring_step": "Strengthen the authoritative brief by resolving the named missing-context items before starting the governed run."
                },
                "recommended_focus": "Resolve the missing context items before starting a requirements run or handing the packet to downstream design work."
            }]
        });

        let markdown = render_markdown_from_json(&value, "clarity", None);

        assert!(markdown.contains("# clarity"));
        assert!(markdown.contains("Mode: requirements"));
        assert!(markdown.contains("Requires Clarification: yes"));
        assert!(markdown.contains("## Authoring Lifecycle"));
        assert!(markdown.contains("Packet Shape: single-file"));
        assert!(markdown.contains("Authority Status: single-input-authoritative-brief"));
        assert!(markdown.contains("Authoritative Inputs:"));
        assert!(markdown.contains("Next Authoring Step:"));
        assert!(markdown.contains("## Clarification Questions"));
        assert!(markdown.contains("## Output Quality"));
        assert!(markdown.contains("Posture: structurally-complete"));
        assert!(markdown.contains("1. Which constraints are non-negotiable for this work?"));
        assert!(markdown.contains("Affects: options.md"));
        assert!(markdown.contains("Default if skipped: Keep the packet conditional until the non-negotiables are explicit."));
        assert!(markdown.contains("Status: required"));
        assert!(markdown.contains("## Recommended Focus"));
    }

    #[test]
    fn artifacts_markdown_humanizes_artifact_paths() {
        let value = json!({
            "entries": [
                "artifacts/run-123/requirements/problem-statement.md",
                ".canon/artifacts/run-123/requirements/options.md"
            ]
        });

        let markdown = render_markdown_from_json(&value, "artifacts", Some("run-123"));

        assert!(markdown.contains("# artifacts"));
        assert!(markdown.contains("Run ID: run-123"));
        assert!(markdown.contains("- .canon/artifacts/run-123/requirements/problem-statement.md"));
        assert!(markdown.contains("- .canon/artifacts/run-123/requirements/options.md"));
    }

    #[test]
    fn evidence_markdown_renders_sections_for_available_lineage() {
        let value = json!({
            "entries": [{
                "execution_posture": "recommendation-only",
                "upstream_feature_slice": "auth session revocation",
                "primary_upstream_mode": "change",
                "upstream_source_refs": [
                    "tech-docs/changes/R-20260422-AUTHREVOC/change-surface.md"
                ],
                "carried_forward_items": [
                    "Revocation output formatting stays stable."
                ],
                "excluded_upstream_scope": "login UI flow",
                "artifact_provenance_links": ["artifacts/run-123/pr-review/review-summary.md"],
                "generation_paths": ["generation:req-1"],
                "validation_paths": ["validation:req-2"],
                "denied_invocations": ["req-3"]
            }]
        });

        let markdown = render_markdown_from_json(&value, "evidence", Some("run-123"));

        assert!(markdown.contains("Execution Posture: recommendation-only"));
        assert!(markdown.contains("Feature Slice: auth session revocation"));
        assert!(markdown.contains("Primary Upstream Mode: change"));
        assert!(markdown.contains("Excluded Upstream Scope: login UI flow"));
        assert!(markdown.contains("## Upstream Sources"));
        assert!(markdown.contains("- tech-docs/changes/R-20260422-AUTHREVOC/change-surface.md"));
        assert!(markdown.contains("## Carried-Forward Context"));
        assert!(markdown.contains("- Revocation output formatting stays stable."));
        assert!(markdown.contains("## Readable Artifacts"));
        assert!(markdown.contains("- .canon/artifacts/run-123/pr-review/review-summary.md"));
        assert!(markdown.contains("## Generation Paths"));
        assert!(markdown.contains("- generation:req-1"));
        assert!(markdown.contains("## Validation Paths"));
        assert!(markdown.contains("- validation:req-2"));
        assert!(markdown.contains("## Denied Invocations"));
        assert!(markdown.contains("- req-3"));
    }

    #[test]
    fn invocations_markdown_renders_scalar_fields_and_linked_artifacts() {
        let value = json!({
            "entries": [{
                "request_id": "req-7",
                "adapter": "Shell",
                "capability": "ValidateWithTool",
                "orientation": "Validation",
                "policy_decision": "AllowConstrained",
                "recommendation_only": true,
                "approval_state": "NotRequired",
                "latest_outcome": "Succeeded",
                "linked_artifacts": ["artifacts/run-123/change/system-slice.md"]
            }]
        });

        let markdown = render_markdown_from_json(&value, "invocations", Some("run-123"));

        assert!(markdown.contains("# invocations"));
        assert!(markdown.contains("## req-7"));
        assert!(markdown.contains("Adapter: Shell"));
        assert!(markdown.contains("Capability: ValidateWithTool"));
        assert!(markdown.contains("Recommendation Only: true"));
        assert!(markdown.contains("Artifacts:"));
        assert!(markdown.contains("- .canon/artifacts/run-123/change/system-slice.md"));
    }

    #[test]
    fn refinement_markdown_renders_refinement_state_working_brief_and_guidance() {
        let value = json!({
            "system_context": "existing",
            "entries": [{
                "run_id": "run-123",
                "mode": "requirements",
                "state": "Draft",
                "working_brief_path": ".canon/runs/run-123/artifacts/requirements/working-brief.md",
                "authoritative_inputs": ["canon-input/requirements/brief.md"],
                "supporting_inputs": ["canon-input/requirements/context-links.md"],
                "clarification_records": [{
                    "id": "cq-001",
                    "prompt": "Which actor owns the problem?",
                    "answer": "platform operators",
                    "answer_kind": "explicit",
                    "affected_sections": ["Actors", "Problem Statement"],
                    "resolution_state": "resolved",
                    "recorded_at": "2026-05-29T12:10:00Z"
                }],
                "readiness_delta": [{
                    "id": "rd-001",
                    "section": "Validation Strategy",
                    "summary": "Independent validation owner is not yet named.",
                    "blocking": true,
                    "source_kind": "missing-context",
                    "default_available": false,
                    "resolved": false
                }],
                "suggested_continuation": {
                    "run_id": "run-prev-1",
                    "mode": "requirements",
                    "state": "Draft",
                    "match_reason": "same authoritative input fingerprint",
                    "advisory": true,
                    "mutation_allowed": false
                },
                "lineage": null
            }]
        });

        let markdown = render_markdown_from_json(&value, "refinement", Some("run-123"));

        assert!(markdown.contains("# refinement"));
        assert!(markdown.contains("Run ID: run-123"));
        assert!(markdown.contains("State: Draft"));
        assert!(markdown.contains("## Refinement State"));
        assert!(markdown.contains("## Working Brief"));
        assert!(markdown.contains("## Clarification Records"));
        assert!(markdown.contains("## Readiness Delta"));
        assert!(markdown.contains("## Continuation Guidance"));
        assert!(
            markdown.contains(
                "Candidate detection is advisory; continuation requires explicit intent."
            )
        );
    }

    #[test]
    fn refinement_text_renders_records_readiness_and_guidance() {
        let value = json!({
            "system_context": "existing",
            "entries": [{
                "run_id": "run-123",
                "mode": "requirements",
                "state": "Draft",
                "working_brief_path": ".canon/runs/run-123/artifacts/requirements/working-brief.md",
                "authoritative_inputs": ["canon-input/requirements/brief.md"],
                "supporting_inputs": ["canon-input/requirements/context-links.md"],
                "clarification_records": [{
                    "id": "cq-001",
                    "prompt": "Which actor owns the problem?",
                    "answer": "platform operators",
                    "answer_kind": "explicit",
                    "affected_sections": ["Actors", "Problem Statement"],
                    "resolution_state": "resolved",
                    "recorded_at": "2026-05-29T12:10:00Z"
                }],
                "readiness_delta": [{
                    "id": "rd-001",
                    "section": "Validation Strategy",
                    "summary": "Independent validation owner is not yet named.",
                    "blocking": true,
                    "source_kind": "missing-context",
                    "default_available": false,
                    "resolved": false
                }],
                "suggested_continuation": {
                    "run_id": "run-prev-1",
                    "mode": "requirements",
                    "state": "Draft",
                    "match_reason": "same authoritative input fingerprint",
                    "advisory": true,
                    "mutation_allowed": false
                },
                "lineage": null
            }]
        });

        let text = render_refinement_text(&value, Some("run-123"));

        assert!(text.contains("target: refinement"));
        assert!(text.contains("run id: run-123"));
        assert!(text.contains("clarification records:"));
        assert!(text.contains("Which actor owns the problem?"));
        assert!(text.contains("answer kind: explicit"));
        assert!(text.contains("readiness delta:"));
        assert!(text.contains("Independent validation owner is not yet named."));
        assert!(
            text.contains(
                "candidate detection is advisory; continuation requires explicit intent."
            )
        );
    }

    #[test]
    fn list_markdown_falls_back_for_unknown_targets() {
        let value = json!({
            "entries": ["one", {"two": 2}]
        });

        let markdown = render_markdown_from_json(&value, "methods", None);

        assert!(markdown.contains("# methods"));
        assert!(markdown.contains("- one"));
        assert!(markdown.contains("- {\"two\":2}"));
    }

    #[test]
    fn risk_zone_markdown_surfaces_provisional_classification() {
        let value = json!({
            "entries": [{
                "mode": "discovery",
                "risk": "bounded-impact",
                "zone": "yellow",
                "risk_was_supplied": false,
                "zone_was_supplied": true,
                "confidence": "moderate",
                "requires_confirmation": true,
                "headline": "Canon inferred the missing risk class as `bounded-impact` from the supplied intake.",
                "rationale": "Use the inferred pair as a provisional starting point.",
                "signals": ["Detected bounded-impact signal `boundary` in the intake."],
                "risk_signals": ["Detected bounded-impact signal `boundary` in the intake."],
                "zone_signals": ["User or caller already supplied the usage zone explicitly."]
            }]
        });

        let markdown = render_markdown_from_json(&value, "risk-zone", None);

        assert!(markdown.contains("# risk-zone"));
        assert!(markdown.contains("Risk: bounded-impact (inferred)"));
        assert!(markdown.contains("Zone: yellow (provided)"));
        assert!(markdown.contains("Needs Confirmation: yes"));
        assert!(markdown.contains("## Signals"));
    }

    #[test]
    fn risk_zone_text_is_machine_parsable() {
        let value = json!({
            "entries": [{
                "mode": "requirements",
                "risk": "low-impact",
                "zone": "green",
                "risk_was_supplied": false,
                "zone_was_supplied": false,
                "confidence": "low",
                "requires_confirmation": true,
                "headline": "Canon inferred `low-impact` risk and `green` zone from the supplied intake.",
                "rationale": "Use the inferred pair as a provisional starting point.",
                "risk_rationale": "The intake looks exploratory.",
                "zone_rationale": "The intake reads like isolated planning work.",
                "signals": ["Mode `requirements` stays read-only and exploratory at this stage."],
                "risk_signals": ["Mode `requirements` stays read-only and exploratory at this stage."],
                "zone_signals": ["Mode `requirements` can stay in green when the intake is still isolated to planning or analysis."]
            }]
        });

        let text = render_risk_zone_text(&value);

        assert!(text.contains("TARGET=risk-zone"));
        assert!(text.contains("INFERRED_RISK=low-impact"));
        assert!(text.contains("INFERRED_ZONE=green"));
        assert!(text.contains("NEEDS_CONFIRMATION=true"));
        assert!(text.contains(
            "RISK_SIGNAL_1=Mode `requirements` stays read-only and exploratory at this stage."
        ));
    }

    #[test]
    fn run_summary_markdown_surfaces_mode_result_without_mandatory_next_step() {
        let summary = RunSummary {
            run_id: "run-123".to_string(),
            uuid: None,
            owner: "Owner".to_string(),
            mode: "requirements".to_string(),
            risk: "bounded-impact".to_string(),
            zone: "yellow".to_string(),
            system_context: None,
            state: "Completed".to_string(),
            artifact_count: 6,
            invocations_total: 3,
            invocations_denied: 1,
            invocations_pending_approval: 0,
            blocking_classification: None,
            blocked_gates: vec![GateInspectSummary {
                gate: "release-readiness".to_string(),
                status: "Blocked".to_string(),
                blockers: vec!["missing approval".to_string()],
            }],
            approval_targets: Vec::new(),
            artifact_paths: vec![".canon/artifacts/run-123/requirements/problem-statement.md".to_string()],
            closure_status: None,
            decomposition_scope: None,
            closure_findings: Vec::new(),
            closure_notes: None,
            possible_actions: vec![canon_engine::PossibleActionSummary {
                action: "inspect-artifacts".to_string(),
                text: "Use $canon-inspect-artifacts for the full emitted packet on run run-123."
                    .to_string(),
                target: None,
            }],
            refinement_state: None,
            mode_result: Some(ModeResultSummary {
                headline: "Requirements packet ready for downstream review.".to_string(),
                artifact_packet_summary: "Primary artifact is ready.".to_string(),
                execution_posture: Some("recommendation-only".to_string()),
                primary_artifact_title: "Problem Statement".to_string(),
                primary_artifact_path: ".canon/artifacts/run-123/requirements/problem-statement.md".to_string(),
                primary_artifact_action: ResultActionSummary {
                    id: "open-primary-artifact".to_string(),
                    label: "Open primary artifact".to_string(),
                    host_action: "open-file".to_string(),
                    target: ".canon/artifacts/run-123/requirements/problem-statement.md"
                        .to_string(),
                    text_fallback:
                        "Open the primary artifact at .canon/artifacts/run-123/requirements/problem-statement.md."
                            .to_string(),
                },
                result_excerpt: "Build a bounded USB flashing CLI.".to_string(),
                action_chips: Vec::new(),
            }),
            recommended_next_action: None,
        };

        let markdown = render_run_summary_markdown(&summary);

        assert!(markdown.contains("## Result"));
        assert!(markdown.contains("Requirements packet ready for downstream review."));
        assert!(markdown.contains("Execution Posture: recommendation-only"));
        assert!(markdown.contains(
            "Primary Artifact: .canon/artifacts/run-123/requirements/problem-statement.md"
        ));
        assert!(markdown.contains("Primary Artifact Action: Open primary artifact (.canon/artifacts/run-123/requirements/problem-statement.md)"));
        assert!(!markdown.contains("## Recommended Next Step"));
        assert!(markdown.contains("## Possible Actions"));
        assert!(
            markdown.contains(
                "Use $canon-inspect-artifacts for the full emitted packet on run run-123."
            )
        );
        assert!(markdown.contains("## Blockers"));
    }

    #[test]
    fn run_summary_markdown_renders_operational_mode_action_chips() {
        let summary = RunSummary {
            run_id: "run-incident-123".to_string(),
            uuid: None,
            owner: "Owner".to_string(),
            mode: "incident".to_string(),
            risk: "systemic-impact".to_string(),
            zone: "red".to_string(),
            system_context: Some("existing".to_string()),
            state: "AwaitingApproval".to_string(),
            artifact_count: 6,
            invocations_total: 4,
            invocations_denied: 0,
            invocations_pending_approval: 1,
            blocking_classification: None,
            blocked_gates: Vec::new(),
            approval_targets: vec!["gate:risk".to_string()],
            artifact_paths: vec![".canon/artifacts/run-incident-123/incident/incident-frame.md".to_string()],
            closure_status: None,
            decomposition_scope: None,
            closure_findings: Vec::new(),
            closure_notes: None,
            possible_actions: Vec::new(),
            refinement_state: None,
            mode_result: Some(ModeResultSummary {
                headline: "Incident packet ready for governed containment review.".to_string(),
                artifact_packet_summary:
                    "Primary artifact bounds the active incident surface and preserves containment posture."
                        .to_string(),
                execution_posture: Some("recommendation-only".to_string()),
                primary_artifact_title: "Incident Frame".to_string(),
                primary_artifact_path:
                    ".canon/artifacts/run-incident-123/incident/incident-frame.md".to_string(),
                primary_artifact_action: ResultActionSummary {
                    id: "open-primary-artifact".to_string(),
                    label: "Open primary artifact".to_string(),
                    host_action: "open-file".to_string(),
                    target:
                        ".canon/artifacts/run-incident-123/incident/incident-frame.md".to_string(),
                    text_fallback:
                        "Open the primary artifact at .canon/artifacts/run-incident-123/incident/incident-frame.md."
                            .to_string(),
                },
                result_excerpt: "Containment stays bounded to payments-api and checkout flow."
                    .to_string(),
                action_chips: vec![canon_engine::ActionChip {
                    id: "inspect-evidence".to_string(),
                    label: "Inspect evidence".to_string(),
                    skill: "canon-inspect-evidence".to_string(),
                    intent: "Inspect".to_string(),
                    prefilled_args: std::collections::BTreeMap::new(),
                    required_user_inputs: Vec::new(),
                    visibility_condition:
                        "state is AwaitingApproval or Completed".to_string(),
                    recommended: true,
                    text_fallback:
                        "Inspect evidence for run run-incident-123: canon inspect evidence --run run-incident-123."
                            .to_string(),
                }],
            }),
            recommended_next_action: None,
        };

        let markdown = render_run_summary_markdown(&summary);

        assert!(markdown.contains("Mode: incident"));
        assert!(markdown.contains("Execution Posture: recommendation-only"));
        assert!(markdown.contains("Action Chips:"));
        assert!(markdown.contains(
            "Inspect evidence for run run-incident-123: canon inspect evidence --run run-incident-123. (recommended)"
        ));
    }

    #[test]
    fn run_summary_markdown_renders_system_assessment_primary_artifact() {
        let summary = RunSummary {
            run_id: "run-system-assessment-123".to_string(),
            uuid: None,
            owner: "Owner".to_string(),
            mode: "system-assessment".to_string(),
            risk: "bounded-impact".to_string(),
            zone: "yellow".to_string(),
            system_context: Some("existing".to_string()),
            state: "AwaitingApproval".to_string(),
            artifact_count: 10,
            invocations_total: 3,
            invocations_denied: 0,
            invocations_pending_approval: 1,
            blocking_classification: None,
            blocked_gates: Vec::new(),
            approval_targets: vec!["gate:risk".to_string()],
            artifact_paths: vec![
                ".canon/artifacts/run-system-assessment-123/system-assessment/assessment-overview.md"
                    .to_string(),
            ],
            closure_status: None,
            decomposition_scope: None,
            closure_findings: Vec::new(),
            closure_notes: None,
            possible_actions: Vec::new(),
            refinement_state: None,
            mode_result: Some(ModeResultSummary {
                headline: "System assessment packet ready for governed architecture review."
                    .to_string(),
                artifact_packet_summary:
                    "Primary artifact bounds the as-is system surface and keeps coverage gaps explicit."
                        .to_string(),
                execution_posture: Some("recommendation-only".to_string()),
                primary_artifact_title: "Assessment Overview".to_string(),
                primary_artifact_path:
                    ".canon/artifacts/run-system-assessment-123/system-assessment/assessment-overview.md"
                        .to_string(),
                primary_artifact_action: ResultActionSummary {
                    id: "open-primary-artifact".to_string(),
                    label: "Open primary artifact".to_string(),
                    host_action: "open-file".to_string(),
                    target:
                        ".canon/artifacts/run-system-assessment-123/system-assessment/assessment-overview.md"
                            .to_string(),
                    text_fallback:
                        "Open the primary artifact at .canon/artifacts/run-system-assessment-123/system-assessment/assessment-overview.md."
                            .to_string(),
                },
                result_excerpt:
                    "Coverage remains strongest for component and integration views; deployment evidence is partial."
                        .to_string(),
                action_chips: Vec::new(),
            }),
            recommended_next_action: None,
        };

        let markdown = render_run_summary_markdown(&summary);

        assert!(markdown.contains("Mode: system-assessment"));
        assert!(markdown.contains("Execution Posture: recommendation-only"));
        assert!(
            markdown.contains("System assessment packet ready for governed architecture review.")
        );
        assert!(markdown.contains(
            ".canon/artifacts/run-system-assessment-123/system-assessment/assessment-overview.md"
        ));
        assert!(markdown.contains("coverage gaps explicit"));
    }

    #[test]
    fn run_summary_markdown_keeps_mandatory_next_step_for_gated_runs() {
        let summary = RunSummary {
            run_id: "run-456".to_string(),
            uuid: None,
            owner: "Owner".to_string(),
            mode: "change".to_string(),
            risk: "systemic-impact".to_string(),
            zone: "yellow".to_string(),
            system_context: Some("existing".to_string()),
            state: "AwaitingApproval".to_string(),
            artifact_count: 0,
            invocations_total: 2,
            invocations_denied: 0,
            invocations_pending_approval: 1,
            blocking_classification: Some("approval-gated".to_string()),
            blocked_gates: Vec::new(),
            approval_targets: vec!["invocation:req-1".to_string()],
            artifact_paths: Vec::new(),
            closure_status: None,
            decomposition_scope: None,
            closure_findings: Vec::new(),
            closure_notes: None,
            possible_actions: vec![canon_engine::PossibleActionSummary {
                action: "approve".to_string(),
                text: "Use $canon-approve for target invocation:req-1 on run run-456 after review."
                    .to_string(),
                target: Some("invocation:req-1".to_string()),
            }],
            refinement_state: None,
            mode_result: None,
            recommended_next_action: Some(RecommendedActionSummary {
                action: "inspect-evidence".to_string(),
                rationale: "Approval is required; inspect the evidence lineage before deciding."
                    .to_string(),
                target: None,
            }),
        };

        let markdown = render_run_summary_markdown(&summary);

        assert!(markdown.contains("## Recommended Next Step"));
        assert!(markdown.contains("Action: inspect-evidence"));
        assert!(markdown.contains("## Possible Actions"));
        assert!(markdown.contains(
            "Use $canon-approve for target invocation:req-1 on run run-456 after review."
        ));
    }

    #[test]
    fn render_list_markdown_empty_entries_shows_placeholder() {
        let value = serde_json::json!({ "entries": [] });
        let markdown = render_markdown_from_json(&value, "modes", None);
        assert!(markdown.contains("# modes"));
        assert!(markdown.contains("- No entries recorded."));
    }

    #[test]
    fn render_list_markdown_serializes_non_string_entries() {
        let value = serde_json::json!({ "entries": [{"key": "val"}] });
        let markdown = render_markdown_from_json(&value, "policies", None);
        assert!(markdown.contains("# policies"));
        assert!(markdown.contains("- {\"key\":\"val\"}"));
    }

    #[test]
    fn render_artifacts_markdown_empty_entries_shows_placeholder() {
        let value = serde_json::json!({ "entries": [] });
        let markdown = render_markdown_from_json(&value, "artifacts", Some("run-abc"));
        assert!(markdown.contains("# artifacts"));
        assert!(markdown.contains("Run ID: run-abc"));
        assert!(markdown.contains("- No artifacts recorded."));
    }

    #[test]
    fn render_artifacts_markdown_with_system_context_and_run_id() {
        let value = serde_json::json!({ "entries": ["artifacts/run-1/req/ps.md"], "system_context": "new" });
        let markdown = render_markdown_from_json(&value, "artifacts", Some("run-1"));
        assert!(markdown.contains("Run ID: run-1"));
        assert!(markdown.contains("System Context: new"));
        assert!(markdown.contains("- .canon/artifacts/run-1/req/ps.md"));
    }

    #[test]
    fn render_evidence_markdown_empty_entries_shows_placeholder() {
        let value = serde_json::json!({ "entries": [] });
        let markdown = render_markdown_from_json(&value, "evidence", None);
        assert!(markdown.contains("# evidence"));
        assert!(markdown.contains("- No evidence recorded."));
    }

    #[test]
    fn render_evidence_markdown_non_object_first_entry_shows_placeholder() {
        let value = serde_json::json!({ "entries": ["not-an-object"] });
        let markdown = render_markdown_from_json(&value, "evidence", None);
        assert!(markdown.contains("- No evidence recorded."));
    }

    #[test]
    fn render_invocations_markdown_empty_entries_shows_placeholder() {
        let value = serde_json::json!({ "entries": [] });
        let markdown = render_markdown_from_json(&value, "invocations", Some("run-x"));
        assert!(markdown.contains("# invocations"));
        assert!(markdown.contains("Run ID: run-x"));
        assert!(markdown.contains("- No invocations recorded."));
    }

    #[test]
    fn render_invocations_markdown_skips_non_object_entries() {
        let value = serde_json::json!({ "entries": ["not-an-object", {"request_id": "req-1", "adapter": "Shell"}] });
        let markdown = render_markdown_from_json(&value, "invocations", None);
        // The non-object entry should be skipped (no crash), the object entry should render
        assert!(markdown.contains("## req-1"));
        assert!(markdown.contains("Adapter: Shell"));
    }

    #[test]
    fn render_risk_zone_markdown_empty_entries_shows_placeholder() {
        let value = serde_json::json!({ "entries": [] });
        let markdown = render_markdown_from_json(&value, "risk-zone", None);
        assert!(markdown.contains("# risk-zone"));
        assert!(markdown.contains("- No classification suggestion recorded."));
    }

    #[test]
    fn render_markdown_from_json_without_entries_key_treats_as_empty() {
        // value has no "entries" key at all
        let value = serde_json::json!({ "something_else": "data" });
        let markdown = render_markdown_from_json(&value, "invocations", None);
        assert!(markdown.contains("- No invocations recorded."));
    }

    // --- coverage gap tests ---

    #[test]
    fn primitives_scalar_value_handles_null_number_and_complex_types() {
        // Null → None
        assert_eq!(scalar_value(Some(&Value::Null)), None);
        // Number → string representation
        assert_eq!(scalar_value(Some(&serde_json::json!(42))), Some("42".to_string()));
        // Object (other branch) → JSON string
        assert_eq!(scalar_value(Some(&serde_json::json!({"a": 1}))), Some("{\"a\":1}".to_string()));
    }

    #[test]
    fn primitives_render_scalar_field_skips_null_value() {
        let mut lines: Vec<String> = Vec::new();
        render_scalar_field(&mut lines, "Label", Some(&Value::Null));
        assert!(lines.is_empty());
    }

    #[test]
    fn primitives_render_kv_field_skips_when_scalar_is_null() {
        let mut lines: Vec<String> = Vec::new();
        render_kv_field(&mut lines, "KEY", Some(&Value::Null));
        assert!(lines.is_empty());
    }

    #[test]
    fn status_summary_markdown_renders_run_id_state_and_system_context() {
        let summary = StatusSummary {
            run: "run-status-1".to_string(),
            owner: "owner".to_string(),
            state: "Completed".to_string(),
            system_context: Some("existing".to_string()),
            invocations_total: 2,
            pending_invocation_approvals: 0,
            validation_independence_satisfied: true,
            blocking_classification: None,
            blocked_gates: Vec::new(),
            approval_targets: Vec::new(),
            artifact_paths: Vec::new(),
            closure_status: None,
            decomposition_scope: None,
            closure_findings: Vec::new(),
            closure_notes: None,
            possible_actions: Vec::new(),
            refinement_state: None,
            mode_result: None,
            recommended_next_action: None,
        };
        let markdown = render_status_summary_markdown(&summary);
        assert!(markdown.contains("# status"));
        assert!(markdown.contains("Run ID: run-status-1"));
        assert!(markdown.contains("State: Completed"));
        assert!(markdown.contains("System Context: existing"));
    }

    #[test]
    fn status_summary_markdown_renders_refinement_state_and_continuation_guidance() {
        let summary = StatusSummary {
            run: "run-status-2".to_string(),
            owner: "owner".to_string(),
            state: "Draft".to_string(),
            system_context: Some("existing".to_string()),
            invocations_total: 0,
            pending_invocation_approvals: 0,
            validation_independence_satisfied: true,
            blocking_classification: None,
            blocked_gates: Vec::new(),
            approval_targets: Vec::new(),
            artifact_paths: Vec::new(),
            closure_status: None,
            decomposition_scope: None,
            closure_findings: Vec::new(),
            closure_notes: None,
            possible_actions: Vec::new(),
            refinement_state: Some(RefinementStateSummary {
                workflow_family: "planning".to_string(),
                current_mode: "requirements".to_string(),
                working_brief_path:
                    ".canon/runs/run-status-2/artifacts/requirements/working-brief.md".to_string(),
                template_ref: "defaults/templates/canon-input/requirements.md".to_string(),
                status: "active".to_string(),
                explicit_continuation_required: true,
                authoritative_input_refs: vec!["idea.md".to_string()],
                supporting_input_refs: Vec::new(),
                records_total: 2,
                unresolved_records: 1,
                readiness_delta: vec![
                    "Explicit continuation is still required before governed execution."
                        .to_string(),
                ],
                readiness_items: Vec::new(),
                suggested_candidate: Some(RefinementCandidateSummary {
                    run_id: "run-prev-1".to_string(),
                    mode: "requirements".to_string(),
                    state: "Draft".to_string(),
                    match_reason: "same authoritative input fingerprint".to_string(),
                    advisory: true,
                }),
            }),
            mode_result: None,
            recommended_next_action: None,
        };

        let markdown = render_status_summary_markdown(&summary);

        assert!(markdown.contains("## Refinement State"));
        assert!(markdown.contains("Current Mode: requirements"));
        assert!(markdown.contains("Unresolved Clarification Records: 1"));
        assert!(markdown.contains("## Continuation Guidance"));
        assert!(markdown.contains("Suggested Continuation: run-prev-1 (requirements, Draft)"));
        assert!(
            markdown.contains(
                "Candidate detection is advisory; continuation requires explicit intent."
            )
        );
    }

    #[test]
    fn status_summary_text_renders_refinement_counts_and_guidance() {
        let summary = StatusSummary {
            run: "run-status-2".to_string(),
            owner: "owner".to_string(),
            state: "Draft".to_string(),
            system_context: Some("existing".to_string()),
            invocations_total: 0,
            pending_invocation_approvals: 0,
            validation_independence_satisfied: true,
            blocking_classification: None,
            blocked_gates: Vec::new(),
            approval_targets: Vec::new(),
            artifact_paths: Vec::new(),
            closure_status: None,
            decomposition_scope: None,
            closure_findings: Vec::new(),
            closure_notes: None,
            possible_actions: Vec::new(),
            refinement_state: Some(RefinementStateSummary {
                workflow_family: "planning".to_string(),
                current_mode: "requirements".to_string(),
                working_brief_path:
                    ".canon/runs/run-status-2/artifacts/requirements/working-brief.md".to_string(),
                template_ref: "defaults/templates/canon-input/requirements.md".to_string(),
                status: "active".to_string(),
                explicit_continuation_required: true,
                authoritative_input_refs: vec!["idea.md".to_string()],
                supporting_input_refs: vec!["context.md".to_string()],
                records_total: 2,
                unresolved_records: 1,
                readiness_delta: vec!["Independent validation owner is not yet named.".to_string()],
                readiness_items: Vec::new(),
                suggested_candidate: Some(RefinementCandidateSummary {
                    run_id: "run-prev-1".to_string(),
                    mode: "requirements".to_string(),
                    state: "Draft".to_string(),
                    match_reason: "same authoritative input fingerprint".to_string(),
                    advisory: true,
                }),
            }),
            mode_result: None,
            recommended_next_action: None,
        };

        let text = render_status_summary_text(&summary);

        assert!(text.contains("run id: run-status-2"));
        assert!(text.contains("refinement state:"));
        assert!(text.contains("clarification records: 2 total, 1 unresolved"));
        assert!(text.contains("supporting inputs:"));
        assert!(
            text.contains(
                "candidate detection is advisory; continuation requires explicit intent."
            )
        );
    }

    #[test]
    fn print_status_summary_supports_text_format() {
        let summary = StatusSummary {
            run: "run-status-print".to_string(),
            owner: "owner".to_string(),
            state: "Draft".to_string(),
            system_context: Some("existing".to_string()),
            invocations_total: 0,
            pending_invocation_approvals: 0,
            validation_independence_satisfied: true,
            blocking_classification: None,
            blocked_gates: Vec::new(),
            approval_targets: Vec::new(),
            artifact_paths: Vec::new(),
            closure_status: None,
            decomposition_scope: None,
            closure_findings: Vec::new(),
            closure_notes: None,
            possible_actions: Vec::new(),
            refinement_state: None,
            mode_result: None,
            recommended_next_action: None,
        };

        assert!(print_status_summary(&summary, OutputFormat::Text).is_ok());
    }

    #[test]
    fn print_inspect_supports_refinement_text_format() {
        let response = InspectResponse {
            target: "refinement".to_string(),
            system_context: Some("existing".to_string()),
            entries: Vec::new(),
        };

        assert!(
            print_inspect(
                &response,
                "refinement",
                Some("run-print-refinement"),
                OutputFormat::Text
            )
            .is_ok()
        );
    }

    #[test]
    fn run_summary_markdown_renders_uuid_when_present() {
        let summary = RunSummary {
            run_id: "run-uuid-1".to_string(),
            uuid: Some("550e8400-e29b-41d4-a716-446655440000".to_string()),
            owner: "owner".to_string(),
            mode: "requirements".to_string(),
            risk: "low-impact".to_string(),
            zone: "green".to_string(),
            system_context: None,
            state: "Completed".to_string(),
            artifact_count: 0,
            invocations_total: 0,
            invocations_denied: 0,
            invocations_pending_approval: 0,
            blocking_classification: None,
            blocked_gates: Vec::new(),
            approval_targets: Vec::new(),
            artifact_paths: Vec::new(),
            closure_status: None,
            decomposition_scope: None,
            closure_findings: Vec::new(),
            closure_notes: None,
            possible_actions: Vec::new(),
            refinement_state: None,
            mode_result: None,
            recommended_next_action: None,
        };
        let markdown = render_run_summary_markdown(&summary);
        assert!(markdown.contains("UUID: 550e8400-e29b-41d4-a716-446655440000"));
    }

    #[test]
    fn recommended_next_step_with_target_renders_target_line() {
        let summary = RunSummary {
            run_id: "run-target-1".to_string(),
            uuid: None,
            owner: "owner".to_string(),
            mode: "change".to_string(),
            risk: "low-impact".to_string(),
            zone: "green".to_string(),
            system_context: None,
            state: "AwaitingApproval".to_string(),
            artifact_count: 0,
            invocations_total: 1,
            invocations_denied: 0,
            invocations_pending_approval: 1,
            blocking_classification: None,
            blocked_gates: Vec::new(),
            approval_targets: Vec::new(),
            artifact_paths: Vec::new(),
            closure_status: None,
            decomposition_scope: None,
            closure_findings: Vec::new(),
            closure_notes: None,
            possible_actions: Vec::new(),
            refinement_state: None,
            mode_result: None,
            recommended_next_action: Some(RecommendedActionSummary {
                action: "approve".to_string(),
                rationale: "Explicit approval needed.".to_string(),
                target: Some("invocation:req-1".to_string()),
            }),
        };
        let markdown = render_run_summary_markdown(&summary);
        assert!(markdown.contains("Target: invocation:req-1"));
    }

    #[test]
    fn risk_zone_text_with_empty_entries_returns_target_marker_only() {
        let value = json!({ "entries": [] });
        let text = render_risk_zone_text(&value);
        assert_eq!(text, "TARGET=risk-zone");
    }

    #[test]
    fn evidence_markdown_with_system_context_renders_context_line() {
        let value = json!({
            "entries": [{"execution_posture": "recommendation-only"}],
            "system_context": "existing"
        });
        let markdown = render_markdown_from_json(&value, "evidence", Some("run-sc-1"));
        assert!(markdown.contains("System Context: existing"));
        assert!(markdown.contains("Run ID: run-sc-1"));
    }

    #[test]
    fn invocations_markdown_with_system_context_renders_context_line() {
        let value = json!({
            "entries": [],
            "system_context": "new"
        });
        let markdown = render_markdown_from_json(&value, "invocations", Some("run-sc-2"));
        assert!(markdown.contains("System Context: new"));
        assert!(markdown.contains("Run ID: run-sc-2"));
    }

    #[test]
    fn risk_zone_markdown_with_rationale_renders_why_section() {
        let value = json!({
            "entries": [{
                "mode": "implementation",
                "risk": "systemic-impact",
                "zone": "red",
                "risk_was_supplied": true,
                "zone_was_supplied": true,
                "confidence": "high",
                "requires_confirmation": false,
                "headline": "Systemic impact detected.",
                "rationale": "Changes touch cross-cutting infrastructure.",
                "signals": []
            }]
        });
        let markdown = render_markdown_from_json(&value, "risk-zone", None);
        assert!(markdown.contains("## Why"));
        assert!(markdown.contains("Changes touch cross-cutting infrastructure."));
    }

    #[test]
    fn clarity_markdown_with_supporting_inputs_readiness_delta_and_quality_signals() {
        let value = json!({
            "entries": [{
                "mode": "architecture",
                "requires_clarification": false,
                "authoring_lifecycle": {
                    "packet_shape": "directory-backed",
                    "authority_status": "explicit-authoritative-brief",
                    "authoritative_inputs": ["canon-input/architecture.md"],
                    "supporting_inputs": ["canon-input/context.md"],
                    "readiness_delta": [
                        "Add deployment view.",
                        "Clarify scaling strategy."
                    ],
                    "next_authoring_step": "Proceed to governed run."
                },
                "output_quality": {
                    "posture": "publishable",
                    "materially_closed": true,
                    "evidence_signals": ["Full section coverage detected."],
                    "downgrade_reasons": ["Minor gap in deployment view."]
                }
            }]
        });
        let markdown = render_markdown_from_json(&value, "clarity", None);
        assert!(markdown.contains("Supporting Inputs:"));
        assert!(markdown.contains("- canon-input/context.md"));
        assert!(markdown.contains("Readiness Delta:"));
        assert!(markdown.contains("- Add deployment view."));
        assert!(markdown.contains("Evidence Signals:"));
        assert!(markdown.contains("- Full section coverage detected."));
        assert!(markdown.contains("Downgrade Reasons:"));
        assert!(markdown.contains("- Minor gap in deployment view."));
        assert!(markdown.contains("Next Authoring Step:"));
        assert!(markdown.contains("Proceed to governed run."));
    }
}
