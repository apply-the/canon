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

    pub fn critique(&self, generated: &str) -> CopilotCliOutput {
        let normalized = normalize_multiline_context(generated);
        let summary = format!(
            "Challenge the generated frame for scope drift, weak invariants, circular validation, and missing exclusions. Review target: {}",
            normalized
        );
        self.output(CapabilityKind::CritiqueContent, "copilot-cli critique", summary)
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

    use super::CopilotCliAdapter;

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
}
