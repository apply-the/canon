use time::OffsetDateTime;

use crate::{
    AdapterInvocation, AdapterKind, AdapterRequest, CapabilityKind, InvocationOrientation,
    LineageClass, SideEffectClass, TrustBoundaryKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopilotCliOutput {
    pub summary: String,
    pub invocation: AdapterInvocation,
    pub executor: String,
}

pub struct RequirementsGenerationInput<'a> {
    pub problem: &'a str,
    pub outcome: &'a str,
    pub constraints: &'a [String],
    pub tradeoffs: &'a [String],
    pub out_of_scope: &'a [String],
    pub open_questions: &'a [String],
    pub source_refs: &'a [String],
}

#[derive(Debug, Default)]
pub struct CopilotCliAdapter;

impl CopilotCliAdapter {
    pub fn generation_request(&self, purpose: &str) -> AdapterRequest {
        AdapterRequest {
            adapter: AdapterKind::CopilotCli,
            capability: CapabilityKind::GenerateContent,
            purpose: purpose.to_string(),
            orientation: Some(InvocationOrientation::Generation),
            trust_boundary: Some(TrustBoundaryKind::AiReasoning),
            lineage: Some(LineageClass::AiVendorFamily),
            side_effect: SideEffectClass::ReadOnly,
        }
    }

    pub fn critique_request(&self, purpose: &str) -> AdapterRequest {
        AdapterRequest {
            adapter: AdapterKind::CopilotCli,
            capability: CapabilityKind::CritiqueContent,
            purpose: purpose.to_string(),
            orientation: Some(InvocationOrientation::Validation),
            trust_boundary: Some(TrustBoundaryKind::AiReasoning),
            lineage: Some(LineageClass::AiVendorFamily),
            side_effect: SideEffectClass::ReadOnly,
        }
    }

    pub fn workspace_edit_request(&self, purpose: &str) -> AdapterRequest {
        AdapterRequest {
            adapter: AdapterKind::CopilotCli,
            capability: CapabilityKind::ProposeWorkspaceEdit,
            purpose: purpose.to_string(),
            orientation: Some(InvocationOrientation::Generation),
            trust_boundary: Some(TrustBoundaryKind::AiReasoning),
            lineage: Some(LineageClass::AiVendorFamily),
            side_effect: SideEffectClass::WorkspaceMutation,
        }
    }

    pub fn generate(&self, context: &str) -> CopilotCliOutput {
        let normalized = normalize_multiline_context(context);
        let summary = format!(
            "Bound the problem before implementation. Preserve explicit ownership, constraints, options, tradeoffs, and scope cuts for: {}",
            normalized
        );
        self.output(CapabilityKind::GenerateContent, "copilot-cli generation", summary)
    }

    pub fn generate_requirements(
        &self,
        input: RequirementsGenerationInput<'_>,
    ) -> CopilotCliOutput {
        let missing_context = input.problem.contains("NOT CAPTURED")
            || input.outcome.contains("NOT CAPTURED")
            || contains_missing_markers(input.constraints)
            || contains_missing_markers(input.tradeoffs)
            || contains_missing_markers(input.out_of_scope);
        let options = if missing_context {
            vec![
                "1. Tighten the authored brief before moving into system-shaping or architecture."
                    .to_string(),
                "2. Review the current packet with the named owner so missing context is resolved explicitly."
                    .to_string(),
            ]
        } else {
            vec![
                "1. Review the packet with the named owner and then move into the next bounded analysis or design mode."
                    .to_string(),
                "2. Hold the packet in requirements mode only if new constraints or exclusions appear during review."
                    .to_string(),
            ]
        };
        let recommended_path = if missing_context {
            "Stay in requirements long enough to replace the missing-context markers with authored decisions before downstream design work."
        } else {
            "Use this packet as the bounded handoff into the smallest downstream mode that preserves the current scope."
        };

        let summary = format!(
            "## Problem\n\n{}\n\n## Outcome\n\n{}\n\n## Constraints\n\n{}\n\n## Tradeoffs\n\n{}\n\n## Scope Cuts\n\n{}\n\n## Options\n\n{}\n\n## Recommended Path\n\n{}\n\n## Open Questions\n\n{}\n\n## Source References\n\n{}",
            normalize_multiline_context(input.problem),
            normalize_multiline_context(input.outcome),
            render_markdown_list(
                input.constraints,
                "NOT CAPTURED - No explicit constraints were supplied."
            ),
            render_markdown_list(
                input.tradeoffs,
                "NOT CAPTURED - No explicit tradeoffs were supplied."
            ),
            render_markdown_list(
                input.out_of_scope,
                "NOT CAPTURED - No explicit scope cuts were supplied."
            ),
            options.join("\n"),
            recommended_path,
            render_markdown_list(
                input.open_questions,
                "- Which downstream mode should consume this packet first?"
            ),
            render_markdown_list(input.source_refs, "- no-authored-source-inputs-recorded"),
        );

        self.output(CapabilityKind::GenerateContent, "copilot-cli requirements generation", summary)
    }

    pub fn critique(&self, generated: &str) -> CopilotCliOutput {
        let normalized = normalize_multiline_context(generated);
        let summary = format!(
            "Challenge the generated frame for scope drift, weak invariants, circular validation, and missing exclusions. Review target: {}",
            normalized
        );
        self.output(CapabilityKind::CritiqueContent, "copilot-cli critique", summary)
    }

    pub fn critique_requirements(
        &self,
        problem: &str,
        outcome: &str,
        constraints: &[String],
        out_of_scope: &[String],
        open_questions: &[String],
        generated: &str,
    ) -> CopilotCliOutput {
        let mut coverage = Vec::new();
        coverage.push(if problem.contains("NOT CAPTURED") {
            "Problem framing is still missing explicit authored input.".to_string()
        } else {
            "Problem framing is explicit enough to review downstream.".to_string()
        });
        coverage.push(if outcome.contains("NOT CAPTURED") {
            "Outcome framing is still missing explicit authored input.".to_string()
        } else {
            "Outcome framing is explicit enough to review downstream.".to_string()
        });

        let mut missing_context = Vec::new();
        if contains_missing_markers(constraints) {
            missing_context.push(
                "Constraints are incomplete; downstream shaping should stop until they are authored explicitly."
                    .to_string(),
            );
        }
        if contains_missing_markers(out_of_scope) {
            missing_context.push(
                "Scope cuts are incomplete; the packet still needs explicit exclusions or deferred work."
                    .to_string(),
            );
        }
        if problem.contains("NOT CAPTURED") || outcome.contains("NOT CAPTURED") {
            missing_context.push(
                "The packet still lacks a complete problem/outcome pair; implementation planning would be premature."
                    .to_string(),
            );
        }
        if missing_context.is_empty() {
            missing_context.push(
                "No additional missing context was detected in the authored brief.".to_string(),
            );
        }

        let risk_notes = vec![
            "Review the packet against the source documentation before treating the scope as implementation-ready."
                .to_string(),
            format!(
                "The generated framing stays read-only and critique-backed: {}",
                normalize_multiline_context(generated)
            ),
        ];
        let recommended_focus = if missing_context
            .iter()
            .any(|item| !item.starts_with("No additional missing context"))
        {
            "Resolve the missing context markers before moving into system-shaping or architecture."
        } else if open_questions.is_empty() {
            "Review the completed packet and choose the smallest downstream mode that preserves the current boundary."
        } else {
            "Review the open questions, then choose the smallest downstream mode that preserves the current boundary."
        };

        let summary = format!(
            "## Coverage\n\n{}\n\n## Missing Context\n\n{}\n\n## Risk Notes\n\n{}\n\n## Recommended Focus\n\n{}",
            render_markdown_list(&coverage, "- Requirements critique coverage was not recorded."),
            render_markdown_list(&missing_context, "- Missing-context critique was not recorded."),
            render_markdown_list(&risk_notes, "- Risk notes were not recorded."),
            recommended_focus,
        );

        self.output(CapabilityKind::CritiqueContent, "copilot-cli requirements critique", summary)
    }

    fn output(
        &self,
        capability: CapabilityKind,
        purpose: &str,
        summary: String,
    ) -> CopilotCliOutput {
        CopilotCliOutput {
            summary,
            invocation: AdapterInvocation {
                adapter: AdapterKind::CopilotCli,
                capability,
                purpose: purpose.to_string(),
                side_effect: SideEffectClass::ReadOnly,
                allowed: true,
                occurred_at: OffsetDateTime::now_utc(),
            },
            executor: "copilot-cli(synthetic-summary)".to_string(),
        }
    }
}

fn render_markdown_list(values: &[String], empty_message: &str) -> String {
    if values.is_empty() {
        empty_message.to_string()
    } else {
        values
            .iter()
            .map(|value| format!("- {}", normalize_multiline_context(value)))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn contains_missing_markers(values: &[String]) -> bool {
    values.iter().any(|value| value.contains("NOT CAPTURED"))
}

fn normalize_multiline_context(value: &str) -> String {
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

#[cfg(test)]
mod tests {
    use crate::{
        AdapterKind, CapabilityKind, InvocationOrientation, LineageClass, SideEffectClass,
        TrustBoundaryKind,
    };

    use super::{CopilotCliAdapter, RequirementsGenerationInput};

    #[test]
    fn request_builders_emit_expected_metadata() {
        let adapter = CopilotCliAdapter;

        let generation = adapter.generation_request("summarize scope");
        assert_eq!(generation.adapter, AdapterKind::CopilotCli);
        assert_eq!(generation.capability, CapabilityKind::GenerateContent);
        assert_eq!(generation.orientation, Some(InvocationOrientation::Generation));
        assert_eq!(generation.trust_boundary, Some(TrustBoundaryKind::AiReasoning));
        assert_eq!(generation.lineage, Some(LineageClass::AiVendorFamily));
        assert_eq!(generation.side_effect, SideEffectClass::ReadOnly);

        let critique = adapter.critique_request("challenge output");
        assert_eq!(critique.capability, CapabilityKind::CritiqueContent);
        assert_eq!(critique.orientation, Some(InvocationOrientation::Validation));
        assert_eq!(critique.side_effect, SideEffectClass::ReadOnly);

        let edit = adapter.workspace_edit_request("propose patch");
        assert_eq!(edit.capability, CapabilityKind::ProposeWorkspaceEdit);
        assert_eq!(edit.orientation, Some(InvocationOrientation::Generation));
        assert_eq!(edit.side_effect, SideEffectClass::WorkspaceMutation);
    }

    #[test]
    fn generate_normalizes_multiline_context_and_marks_output_allowed() {
        let adapter = CopilotCliAdapter;

        let output = adapter.generate("  First   line\n\n\n second   line  ");

        assert!(output.summary.contains("First line\n\nsecond line"));
        assert_eq!(output.invocation.adapter, AdapterKind::CopilotCli);
        assert_eq!(output.invocation.capability, CapabilityKind::GenerateContent);
        assert!(output.invocation.allowed);
        assert_eq!(output.executor, "copilot-cli(synthetic-summary)");
    }

    #[test]
    fn critique_uses_validation_summary_language() {
        let adapter = CopilotCliAdapter;

        let output = adapter.critique("Generated   plan\n\n needs tightening ");

        assert!(output.summary.contains("Challenge the generated frame"));
        assert!(output.summary.contains("Generated plan\n\nneeds tightening"));
        assert_eq!(output.invocation.capability, CapabilityKind::CritiqueContent);
    }

    #[test]
    fn requirements_generation_emits_structured_sections() {
        let adapter = CopilotCliAdapter;

        let output = adapter.generate_requirements(RequirementsGenerationInput {
            problem: "Build a bounded USB flashing CLI.",
            outcome: "Operators can flash a device safely over USB.",
            constraints: &["USB transport only".to_string()],
            tradeoffs: &["Safety over throughput".to_string()],
            out_of_scope: &["No Bluetooth support in v1".to_string()],
            open_questions: &["How is the bootloader entered?".to_string()],
            source_refs: &["canon-input/requirements/project-brief.md".to_string()],
        });

        assert!(output.summary.contains("## Problem"));
        assert!(output.summary.contains("## Constraints"));
        assert!(output.summary.contains("## Scope Cuts"));
        assert!(output.summary.contains("## Open Questions"));
        assert!(output.summary.contains("canon-input/requirements/project-brief.md"));
    }
}
