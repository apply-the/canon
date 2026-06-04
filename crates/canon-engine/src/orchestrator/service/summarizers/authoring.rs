use crate::orchestrator::service::ModeResultSummary;
use crate::orchestrator::service::context_parse::{
    count_markdown_entries, count_missing_context_markers, extract_context_section,
    truncate_context_excerpt,
};
use crate::persistence::store::PersistedArtifact;

use super::{
    packet_output_quality_artifact_prefix, packet_output_quality_headline,
    primary_artifact_action_for,
};

pub(super) fn summarize_requirements_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.slug() == "problem-statement.md")?;
    let constraints_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "constraints.md");
    let scope_cuts_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "scope-cuts.md");
    let decision_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "decision-checklist.md");

    let problem = extract_context_section(&primary.contents, "Problem")
        .or_else(|| extract_context_section(&primary.contents, "Summary"))
        .unwrap_or_else(|| "NOT CAPTURED - Problem statement summary is missing.".to_string());
    let constraints = constraints_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Constraints"))
        .unwrap_or_else(|| "NOT CAPTURED - Constraints section is missing.".to_string());
    let scope_cuts = scope_cuts_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Scope Cuts"))
        .unwrap_or_else(|| "NOT CAPTURED - Scope cuts section is missing.".to_string());
    let open_questions = decision_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Open Questions"))
        .unwrap_or_else(|| "NOT CAPTURED - Open questions section is missing.".to_string());

    let missing_context_markers = [&problem, &constraints, &scope_cuts, &open_questions]
        .into_iter()
        .filter(|section| section.contains("NOT CAPTURED"))
        .count();
    let constraint_count = count_markdown_entries(&constraints);
    let scope_cut_count = count_markdown_entries(&scope_cuts);
    let open_question_count = count_markdown_entries(&open_questions);

    let headline = packet_output_quality_headline(
        "Requirements",
        missing_context_markers,
        open_question_count,
        "open question set(s)",
        "downstream review",
    );
    let artifact_packet_summary = format!(
        "{} Packet captures {constraint_count} constraint point(s), {scope_cut_count} scope cut(s), and {open_question_count} open question(s).",
        packet_output_quality_artifact_prefix(
            missing_context_markers,
            open_question_count,
            "open question set(s)"
        )
    );

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Problem Statement".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&problem, 320),
        action_chips: Vec::new(),
    })
}

pub(super) fn summarize_discovery_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary = artifacts.iter().find(|artifact| artifact.record.slug() == "problem-map.md")?;
    let unknowns_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "unknowns-and-assumptions.md");
    let boundary_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "context-boundary.md");

    let problem_domain = extract_context_section(&primary.contents, "Problem Domain")
        .or_else(|| extract_context_section(&primary.contents, "Summary"))
        .unwrap_or_else(|| "NOT CAPTURED - Problem domain summary is missing.".to_string());
    let repo_signals = extract_context_section(&primary.contents, "Repo Surface")
        .unwrap_or_else(|| "NOT CAPTURED - Repository signals are missing.".to_string());
    let next_phase = extract_context_section(&primary.contents, "Downstream Handoff")
        .or_else(|| {
            boundary_artifact.and_then(|artifact| {
                extract_context_section(&artifact.contents, "Translation Trigger")
            })
        })
        .unwrap_or_else(|| "NOT CAPTURED - Next-phase handoff is missing.".to_string());
    let unknowns = unknowns_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Unknowns"))
        .unwrap_or_else(|| "NOT CAPTURED - Unknowns section is missing.".to_string());

    let missing_context_markers =
        count_missing_context_markers([&problem_domain, &repo_signals, &next_phase, &unknowns]);
    let repo_signal_count = count_markdown_entries(&repo_signals);
    let unknown_count = count_markdown_entries(&unknowns);

    let headline = packet_output_quality_headline(
        "Discovery",
        missing_context_markers,
        unknown_count,
        "unknown or assumption set(s)",
        "downstream translation",
    );
    let artifact_packet_summary = format!(
        "{} Packet maps {repo_signal_count} repository signal(s) and {unknown_count} unknown or assumption set(s). Next phase: {}.",
        packet_output_quality_artifact_prefix(
            missing_context_markers,
            unknown_count,
            "unknown or assumption set(s)"
        ),
        truncate_context_excerpt(&next_phase, 120)
    );

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Problem Map".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&problem_domain, 320),
        action_chips: Vec::new(),
    })
}

pub(super) fn summarize_system_shaping_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary = artifacts.iter().find(|artifact| artifact.record.slug() == "system-shape.md")?;
    let domain_model =
        artifacts.iter().find(|artifact| artifact.record.slug() == "domain-model.md");
    let capability_map =
        artifacts.iter().find(|artifact| artifact.record.slug() == "capability-map.md");
    let delivery_options =
        artifacts.iter().find(|artifact| artifact.record.slug() == "delivery-options.md");
    let risk_hotspots =
        artifacts.iter().find(|artifact| artifact.record.slug() == "risk-hotspots.md");

    let system_shape = extract_context_section(&primary.contents, "System Shape")
        .or_else(|| extract_context_section(&primary.contents, "Summary"))
        .unwrap_or_else(|| "NOT CAPTURED - System shape summary is missing.".to_string());
    let boundary_decisions = extract_context_section(&primary.contents, "Boundary Decisions")
        .unwrap_or_else(|| "NOT CAPTURED - Boundary decisions are missing.".to_string());
    let capabilities = capability_map
        .and_then(|artifact| extract_context_section(&artifact.contents, "Capabilities"))
        .unwrap_or_else(|| "NOT CAPTURED - Capability map is missing.".to_string());
    let bounded_contexts = domain_model
        .and_then(|artifact| {
            extract_context_section(&artifact.contents, "Candidate Bounded Contexts")
        })
        .unwrap_or_else(|| "NOT CAPTURED - Candidate bounded contexts are missing.".to_string());
    let domain_invariants = domain_model
        .and_then(|artifact| extract_context_section(&artifact.contents, "Domain Invariants"))
        .unwrap_or_else(|| "NOT CAPTURED - Domain invariants are missing.".to_string());
    let delivery_phases = delivery_options
        .and_then(|artifact| extract_context_section(&artifact.contents, "Delivery Phases"))
        .unwrap_or_else(|| "NOT CAPTURED - Delivery phases are missing.".to_string());
    let hotspots = risk_hotspots
        .and_then(|artifact| extract_context_section(&artifact.contents, "Hotspots"))
        .unwrap_or_else(|| "NOT CAPTURED - Risk hotspots are missing.".to_string());

    let missing_context_markers = count_missing_context_markers([
        &system_shape,
        &boundary_decisions,
        &capabilities,
        &bounded_contexts,
        &domain_invariants,
        &delivery_phases,
        &hotspots,
    ]);
    let capability_count = count_markdown_entries(&capabilities);
    let bounded_context_count = count_markdown_entries(&bounded_contexts);
    let domain_invariant_count = count_markdown_entries(&domain_invariants);
    let delivery_count = count_markdown_entries(&delivery_phases);
    let hotspot_count = count_markdown_entries(&hotspots);

    let headline = packet_output_quality_headline(
        "System-shaping",
        missing_context_markers,
        0,
        "",
        "downstream architecture or delivery planning",
    );
    let artifact_packet_summary = format!(
        "{} Packet names {capability_count} capability slice(s), {bounded_context_count} bounded context candidate(s), {domain_invariant_count} domain invariant set(s), {delivery_count} delivery phase set(s), and {hotspot_count} risk hotspot set(s).",
        packet_output_quality_artifact_prefix(missing_context_markers, 0, "")
    );

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "System Shape".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&system_shape, 320),
        action_chips: Vec::new(),
    })
}

pub(super) fn summarize_brainstorming_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary = artifacts.iter().find(|artifact| {
        artifact.record.slug() == crate::modes::brainstorming::ARTIFACT_CONTEXT_SLUG
    })?;
    let options_artifact = artifacts.iter().find(|artifact| {
        artifact.record.slug() == crate::modes::brainstorming::ARTIFACT_OPTIONS_SLUG
    });
    let tradeoffs_artifact = artifacts.iter().find(|artifact| {
        artifact.record.slug() == crate::modes::brainstorming::ARTIFACT_TRADEOFFS_SLUG
    });
    let spikes_artifact = artifacts.iter().find(|artifact| {
        artifact.record.slug() == crate::modes::brainstorming::ARTIFACT_SPIKES_SLUG
    });

    let context_summary =
        extract_context_section(&primary.contents, crate::modes::brainstorming::HEADING_CONTEXT)
            .or_else(|| {
                extract_context_section(
                    &primary.contents,
                    crate::modes::brainstorming::HEADING_SUMMARY,
                )
            })
            .unwrap_or_else(|| "NOT CAPTURED - Context summary is missing.".to_string());
    let options = options_artifact
        .and_then(|artifact| {
            extract_context_section(
                &artifact.contents,
                crate::modes::brainstorming::HEADING_OPTIONS,
            )
        })
        .unwrap_or_else(|| "NOT CAPTURED - Options section is missing.".to_string());
    let tradeoffs = tradeoffs_artifact
        .and_then(|artifact| {
            extract_context_section(
                &artifact.contents,
                crate::modes::brainstorming::HEADING_TRADEOFFS,
            )
        })
        .unwrap_or_else(|| "NOT CAPTURED - Tradeoffs section is missing.".to_string());
    let spikes = spikes_artifact
        .and_then(|artifact| {
            extract_context_section(&artifact.contents, crate::modes::brainstorming::HEADING_SPIKES)
        })
        .unwrap_or_else(|| "NOT CAPTURED - Spikes section is missing.".to_string());

    let missing_context_markers =
        count_missing_context_markers([&context_summary, &options, &tradeoffs, &spikes]);
    let option_count = count_markdown_entries(&options);
    let tradeoff_count = count_markdown_entries(&tradeoffs);
    let spike_count = count_markdown_entries(&spikes);

    let headline = packet_output_quality_headline(
        "Brainstorming",
        missing_context_markers,
        0,
        "",
        "downstream decision making or planning",
    );
    let artifact_packet_summary = format!(
        "{} Packet surfaces {option_count} option(s), {tradeoff_count} tradeoff(s), and {spike_count} spike(s).",
        packet_output_quality_artifact_prefix(missing_context_markers, 0, "")
    );

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: crate::modes::brainstorming::HEADING_CONTEXT.to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&context_summary, 320),
        action_chips: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::artifact::ArtifactRecord;
    use crate::persistence::store::PersistedArtifact;

    #[test]
    fn summarize_brainstorming_mode_result_extracts_all_sections() {
        let artifacts = vec![
            PersistedArtifact {
                record: ArtifactRecord {
                    file_name: crate::modes::brainstorming::ARTIFACT_CONTEXT_SLUG.to_string(),
                    relative_path: format!(
                        "artifacts/abc/brainstorming/{}",
                        crate::modes::brainstorming::ARTIFACT_CONTEXT_SLUG
                    ),
                    format: crate::domain::artifact::ArtifactFormat::Markdown,
                    provenance: None,
                },
                contents: format!(
                    "# {}\n\nThis is the context.\n",
                    crate::modes::brainstorming::HEADING_CONTEXT
                ),
            },
            PersistedArtifact {
                record: ArtifactRecord {
                    file_name: crate::modes::brainstorming::ARTIFACT_OPTIONS_SLUG.to_string(),
                    relative_path: format!(
                        "artifacts/abc/brainstorming/{}",
                        crate::modes::brainstorming::ARTIFACT_OPTIONS_SLUG
                    ),
                    format: crate::domain::artifact::ArtifactFormat::Markdown,
                    provenance: None,
                },
                contents: format!(
                    "# {}\n\n- Option A\n- Option B\n",
                    crate::modes::brainstorming::HEADING_OPTIONS
                ),
            },
            PersistedArtifact {
                record: ArtifactRecord {
                    file_name: crate::modes::brainstorming::ARTIFACT_TRADEOFFS_SLUG.to_string(),
                    relative_path: format!(
                        "artifacts/abc/brainstorming/{}",
                        crate::modes::brainstorming::ARTIFACT_TRADEOFFS_SLUG
                    ),
                    format: crate::domain::artifact::ArtifactFormat::Markdown,
                    provenance: None,
                },
                contents: format!(
                    "# {}\n\n- Tradeoff 1\n",
                    crate::modes::brainstorming::HEADING_TRADEOFFS
                ),
            },
            PersistedArtifact {
                record: ArtifactRecord {
                    file_name: crate::modes::brainstorming::ARTIFACT_SPIKES_SLUG.to_string(),
                    relative_path: format!(
                        "artifacts/abc/brainstorming/{}",
                        crate::modes::brainstorming::ARTIFACT_SPIKES_SLUG
                    ),
                    format: crate::domain::artifact::ArtifactFormat::Markdown,
                    provenance: None,
                },
                contents: format!(
                    "# {}\n\n- Spike Alpha\n",
                    crate::modes::brainstorming::HEADING_SPIKES
                ),
            },
        ];

        let summary = summarize_brainstorming_mode_result(&artifacts).expect("summary");
        assert_eq!(summary.primary_artifact_title, crate::modes::brainstorming::HEADING_CONTEXT);
        assert!(
            summary
                .artifact_packet_summary
                .contains("surfaces 2 option(s), 1 tradeoff(s), and 1 spike(s)")
        );
    }

    #[test]
    fn summarize_requirements_mode_result_extracts_all_sections() {
        let artifacts = vec![
            PersistedArtifact {
                record: ArtifactRecord {
                    file_name: "problem-statement.md".to_string(),
                    relative_path: "artifacts/req/problem-statement.md".to_string(),
                    format: crate::domain::artifact::ArtifactFormat::Markdown,
                    provenance: None,
                },
                contents: "# Problem\n\nReq problem.\n".to_string(),
            },
            PersistedArtifact {
                record: ArtifactRecord {
                    file_name: "constraints.md".to_string(),
                    relative_path: "artifacts/req/constraints.md".to_string(),
                    format: crate::domain::artifact::ArtifactFormat::Markdown,
                    provenance: None,
                },
                contents: "# Constraints\n\n- Const 1\n".to_string(),
            },
            PersistedArtifact {
                record: ArtifactRecord {
                    file_name: "scope-cuts.md".to_string(),
                    relative_path: "artifacts/req/scope-cuts.md".to_string(),
                    format: crate::domain::artifact::ArtifactFormat::Markdown,
                    provenance: None,
                },
                contents: "# Scope Cuts\n\n- Cut 1\n".to_string(),
            },
            PersistedArtifact {
                record: ArtifactRecord {
                    file_name: "decision-checklist.md".to_string(),
                    relative_path: "artifacts/req/decision-checklist.md".to_string(),
                    format: crate::domain::artifact::ArtifactFormat::Markdown,
                    provenance: None,
                },
                contents: "# Open Questions\n\n- Q1\n".to_string(),
            },
        ];

        let summary = summarize_requirements_mode_result(&artifacts).expect("summary");
        assert_eq!(summary.primary_artifact_title, "Problem Statement");
        assert!(
            summary
                .artifact_packet_summary
                .contains("1 constraint point(s), 1 scope cut(s), and 1 open question(s)")
        );
    }

    #[test]
    fn summarize_discovery_mode_result_extracts_all_sections() {
        let artifacts = vec![
            PersistedArtifact {
                record: ArtifactRecord {
                    file_name: "problem-map.md".to_string(),
                    relative_path: "artifacts/disc/problem-map.md".to_string(),
                    format: crate::domain::artifact::ArtifactFormat::Markdown,
                    provenance: None,
                },
                contents: "# Problem Domain\n\nDomain stuff.\n# Repo Surface\n\n- Sig 1\n# Downstream Handoff\n\nHandoff.\n".to_string(),
            },
            PersistedArtifact {
                record: ArtifactRecord {
                    file_name: "unknowns-and-assumptions.md".to_string(),
                    relative_path: "artifacts/disc/unknowns-and-assumptions.md".to_string(),
                    format: crate::domain::artifact::ArtifactFormat::Markdown,
                    provenance: None,
                },
                contents: "# Unknowns\n\n- U1\n".to_string(),
            },
        ];

        let summary = summarize_discovery_mode_result(&artifacts).expect("summary");
        assert_eq!(summary.primary_artifact_title, "Problem Map");
        assert!(
            summary
                .artifact_packet_summary
                .contains("1 repository signal(s) and 1 unknown or assumption set(s)")
        );
    }

    #[test]
    fn summarize_system_shaping_mode_result_extracts_all_sections() {
        let artifacts = vec![
            PersistedArtifact {
                record: ArtifactRecord {
                    file_name: "system-shape.md".to_string(),
                    relative_path: "artifacts/shape/system-shape.md".to_string(),
                    format: crate::domain::artifact::ArtifactFormat::Markdown,
                    provenance: None,
                },
                contents: "# System Shape\n\nShape.\n# Boundary Decisions\n\n- B1\n".to_string(),
            },
            PersistedArtifact {
                record: ArtifactRecord {
                    file_name: "domain-model.md".to_string(),
                    relative_path: "artifacts/shape/domain-model.md".to_string(),
                    format: crate::domain::artifact::ArtifactFormat::Markdown,
                    provenance: None,
                },
                contents: "# Candidate Bounded Contexts\n\n- C1\n# Domain Invariants\n\n- I1\n"
                    .to_string(),
            },
            PersistedArtifact {
                record: ArtifactRecord {
                    file_name: "capability-map.md".to_string(),
                    relative_path: "artifacts/shape/capability-map.md".to_string(),
                    format: crate::domain::artifact::ArtifactFormat::Markdown,
                    provenance: None,
                },
                contents: "# Capabilities\n\n- Cap1\n".to_string(),
            },
            PersistedArtifact {
                record: ArtifactRecord {
                    file_name: "delivery-options.md".to_string(),
                    relative_path: "artifacts/shape/delivery-options.md".to_string(),
                    format: crate::domain::artifact::ArtifactFormat::Markdown,
                    provenance: None,
                },
                contents: "# Delivery Phases\n\n- Ph1\n".to_string(),
            },
            PersistedArtifact {
                record: ArtifactRecord {
                    file_name: "risk-hotspots.md".to_string(),
                    relative_path: "artifacts/shape/risk-hotspots.md".to_string(),
                    format: crate::domain::artifact::ArtifactFormat::Markdown,
                    provenance: None,
                },
                contents: "# Hotspots\n\n- H1\n".to_string(),
            },
        ];

        let summary = summarize_system_shaping_mode_result(&artifacts).expect("summary");
        assert_eq!(summary.primary_artifact_title, "System Shape");
        assert!(summary.artifact_packet_summary.contains("1 capability slice(s), 1 bounded context candidate(s), 1 domain invariant set(s), 1 delivery phase set(s), and 1 risk hotspot set(s)"));
    }

    #[test]
    fn summarize_brainstorming_mode_result_handles_missing_sections() {
        let artifacts = vec![
            PersistedArtifact {
                record: ArtifactRecord {
                    file_name: crate::modes::brainstorming::ARTIFACT_CONTEXT_SLUG.to_string(),
                    relative_path: format!(
                        "artifacts/abc/brainstorming/{}",
                        crate::modes::brainstorming::ARTIFACT_CONTEXT_SLUG
                    ),
                    format: crate::domain::artifact::ArtifactFormat::Markdown,
                    provenance: None,
                },
                contents: "# Other Heading\n\nNo context here.\n".to_string(),
            },
            PersistedArtifact {
                record: ArtifactRecord {
                    file_name: crate::modes::brainstorming::ARTIFACT_OPTIONS_SLUG.to_string(),
                    relative_path: format!(
                        "artifacts/abc/brainstorming/{}",
                        crate::modes::brainstorming::ARTIFACT_OPTIONS_SLUG
                    ),
                    format: crate::domain::artifact::ArtifactFormat::Markdown,
                    provenance: None,
                },
                contents: "# Other Heading\n\nNo options here.\n".to_string(),
            },
        ];

        let summary = summarize_brainstorming_mode_result(&artifacts).expect("summary");
        assert!(summary.result_excerpt.contains("NOT CAPTURED - Context summary is missing."));
        assert!(
            summary
                .artifact_packet_summary
                .contains("surfaces 0 option(s), 0 tradeoff(s), and 0 spike(s)")
        );
    }

    #[test]
    fn summarize_requirements_mode_result_handles_missing_sections() {
        let artifacts = vec![PersistedArtifact {
            record: ArtifactRecord {
                file_name: "problem-statement.md".to_string(),
                relative_path: "artifacts/req/problem-statement.md".to_string(),
                format: crate::domain::artifact::ArtifactFormat::Markdown,
                provenance: None,
            },
            contents: "# Other Heading\n\nNo prob.\n".to_string(),
        }];

        let summary = summarize_requirements_mode_result(&artifacts).expect("summary");
        assert!(
            summary.result_excerpt.contains("NOT CAPTURED - Problem statement summary is missing.")
        );
        assert!(summary.artifact_packet_summary.contains("0 constraint point(s)"));
    }

    #[test]
    fn summarize_discovery_mode_result_handles_missing_sections() {
        let artifacts = vec![PersistedArtifact {
            record: ArtifactRecord {
                file_name: "problem-map.md".to_string(),
                relative_path: "artifacts/disc/problem-map.md".to_string(),
                format: crate::domain::artifact::ArtifactFormat::Markdown,
                provenance: None,
            },
            contents: "# Other Heading\n\nNo prob.\n".to_string(),
        }];

        let summary = summarize_discovery_mode_result(&artifacts).expect("summary");
        assert!(
            summary.result_excerpt.contains("NOT CAPTURED - Problem domain summary is missing.")
        );
        assert!(summary.artifact_packet_summary.contains("0 repository signal(s)"));
    }

    #[test]
    fn summarize_system_shaping_mode_result_handles_missing_sections() {
        let artifacts = vec![PersistedArtifact {
            record: ArtifactRecord {
                file_name: "system-shape.md".to_string(),
                relative_path: "artifacts/shape/system-shape.md".to_string(),
                format: crate::domain::artifact::ArtifactFormat::Markdown,
                provenance: None,
            },
            contents: "# Other Heading\n\nNo shape.\n".to_string(),
        }];

        let summary = summarize_system_shaping_mode_result(&artifacts).expect("summary");
        assert!(summary.result_excerpt.contains("NOT CAPTURED - System shape summary is missing."));
        assert!(summary.artifact_packet_summary.contains("0 capability slice(s)"));
    }
}
