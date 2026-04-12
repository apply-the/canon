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
