use serde::{Deserialize, Serialize};

/// Represents the primary output artifact of the brainstorming mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionMap {
    /// The problem statement being explored.
    pub problem_statement: String,
    /// Distinct conceptual approaches to the problem.
    pub options: Vec<ConceptualApproach>,
    /// The recommended next mode based on the exploration.
    pub recommended_next_mode: String,
}

/// A distinct conceptual approach to solving the problem.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConceptualApproach {
    /// Title of the approach.
    pub title: String,
    /// Detailed description of the approach.
    pub description: String,
    /// Structured evaluation of the approach.
    pub trade_offs: TradeOffMatrix,
}

/// Structured evaluation of an approach.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TradeOffMatrix {
    /// List of pros.
    pub pros: Vec<String>,
    /// List of cons.
    pub cons: Vec<String>,
    /// List of unknowns or open questions.
    pub unknowns: Vec<String>,
}

/// A minimal experiment scope to validate hypotheses when critical unknowns exist.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpikeProposal {
    /// Reference to the related conceptual approach.
    pub related_option: String,
    /// The hypothesis to be tested.
    pub hypothesis: String,
    /// The scope of the experiment.
    pub experiment_scope: String,
    /// The criteria that define a successful experiment.
    pub success_criteria: String,
}
