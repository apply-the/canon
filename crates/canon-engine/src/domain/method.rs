use serde::{Deserialize, Serialize};

/// A condition that causes a method step to halt before its exit criteria are reached.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StopCondition {
    /// Human-readable description of the condition that triggers an early stop.
    pub description: String,
}

/// The observable conditions that must be met for a method step to be considered complete.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExitCriteria {
    /// Human-readable description of what "done" means for this step.
    pub description: String,
}

/// A single step in a governed method, carrying its inputs and expected outputs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StepDefinition {
    /// Unique identifier for this step within its parent method.
    pub id: String,
    /// Human-readable label for the step.
    pub label: String,
    /// Input artifact kinds or context sources required before this step begins.
    pub required_inputs: Vec<String>,
    /// Artifact kinds or evidence this step is expected to produce.
    pub outputs: Vec<String>,
}
