use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdapterKind {
    Filesystem,
    Shell,
    CopilotCli,
    McpStdio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapabilityKind {
    ReadRepository,
    InspectDiff,
    ReadArtifact,
    EmitArtifact,
    RunCommand,
    GenerateContent,
    ProposeWorkspaceEdit,
    CritiqueContent,
    ValidateWithTool,
    InvokeStructuredTool,
    ExecuteBoundedTransformation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvocationOrientation {
    Context,
    Generation,
    Validation,
    ArtifactDerivation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MutabilityClass {
    ReadOnly,
    ArtifactWrite,
    BoundedWorkspaceWrite,
    BroadWorkspaceWrite,
    ExternalStateChange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustBoundaryKind {
    LocalDeterministic,
    LocalProcess,
    AiReasoning,
    RemoteStructuredTool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineageClass {
    NonGenerative,
    AiVendorFamily,
    HumanReview,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterCapability {
    pub kind: CapabilityKind,
    pub orientation: InvocationOrientation,
    pub mutability: MutabilityClass,
    pub trust_boundary: TrustBoundaryKind,
    pub lineage: LineageClass,
}

pub fn classify_capability(adapter: AdapterKind, capability: CapabilityKind) -> AdapterCapability {
    let trust_boundary = match adapter {
        AdapterKind::Filesystem => TrustBoundaryKind::LocalDeterministic,
        AdapterKind::Shell => TrustBoundaryKind::LocalProcess,
        AdapterKind::CopilotCli => TrustBoundaryKind::AiReasoning,
        AdapterKind::McpStdio => TrustBoundaryKind::RemoteStructuredTool,
    };

    let orientation = match capability {
        CapabilityKind::ReadRepository
        | CapabilityKind::InspectDiff
        | CapabilityKind::ReadArtifact
        | CapabilityKind::RunCommand => InvocationOrientation::Context,
        CapabilityKind::GenerateContent
        | CapabilityKind::ProposeWorkspaceEdit
        | CapabilityKind::ExecuteBoundedTransformation => InvocationOrientation::Generation,
        CapabilityKind::CritiqueContent | CapabilityKind::ValidateWithTool => {
            InvocationOrientation::Validation
        }
        CapabilityKind::EmitArtifact => InvocationOrientation::ArtifactDerivation,
        CapabilityKind::InvokeStructuredTool => InvocationOrientation::Context,
    };

    let mutability = match capability {
        CapabilityKind::ReadRepository
        | CapabilityKind::InspectDiff
        | CapabilityKind::ReadArtifact
        | CapabilityKind::RunCommand
        | CapabilityKind::GenerateContent
        | CapabilityKind::CritiqueContent
        | CapabilityKind::ValidateWithTool
        | CapabilityKind::InvokeStructuredTool => MutabilityClass::ReadOnly,
        CapabilityKind::EmitArtifact => MutabilityClass::ArtifactWrite,
        CapabilityKind::ExecuteBoundedTransformation => MutabilityClass::BoundedWorkspaceWrite,
        CapabilityKind::ProposeWorkspaceEdit => MutabilityClass::BroadWorkspaceWrite,
    };

    let lineage = match capability {
        CapabilityKind::GenerateContent
        | CapabilityKind::ProposeWorkspaceEdit
        | CapabilityKind::CritiqueContent => LineageClass::AiVendorFamily,
        _ => LineageClass::NonGenerative,
    };

    AdapterCapability { kind: capability, orientation, mutability, trust_boundary, lineage }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SideEffectClass {
    ReadOnly,
    ArtifactWrite,
    WorkspaceMutation,
    ExternalStateChange,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterRequest {
    pub adapter: AdapterKind,
    pub capability: CapabilityKind,
    pub purpose: String,
    pub orientation: Option<InvocationOrientation>,
    pub trust_boundary: Option<TrustBoundaryKind>,
    pub lineage: Option<LineageClass>,
    pub side_effect: SideEffectClass,
}

#[cfg(test)]
mod tests {
    use super::{
        AdapterKind, CapabilityKind, InvocationOrientation, LineageClass, MutabilityClass,
        TrustBoundaryKind, classify_capability,
    };

    #[test]
    fn copilot_generation_is_classified_as_ai_reasoning_generation() {
        let capability =
            classify_capability(AdapterKind::CopilotCli, CapabilityKind::GenerateContent);
        assert_eq!(capability.orientation, InvocationOrientation::Generation);
        assert_eq!(capability.trust_boundary, TrustBoundaryKind::AiReasoning);
        assert_eq!(capability.lineage, LineageClass::AiVendorFamily);
    }

    #[test]
    fn validation_tools_are_classified_as_validation_with_local_process_boundary() {
        let capability = classify_capability(AdapterKind::Shell, CapabilityKind::ValidateWithTool);
        assert_eq!(capability.orientation, InvocationOrientation::Validation);
        assert_eq!(capability.trust_boundary, TrustBoundaryKind::LocalProcess);
        assert_eq!(capability.mutability, MutabilityClass::ReadOnly);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterInvocation {
    pub adapter: AdapterKind,
    pub capability: CapabilityKind,
    pub purpose: String,
    pub side_effect: SideEffectClass,
    pub allowed: bool,
    pub occurred_at: OffsetDateTime,
}
