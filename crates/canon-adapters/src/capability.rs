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
    WriteArtifact,
    ExecReadOnlyCommand,
    ExecMutatingCommand,
    InvokeAiGeneration,
    InvokeAiCritique,
    InvokeStructuredTool,
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
    pub side_effect: SideEffectClass,
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
