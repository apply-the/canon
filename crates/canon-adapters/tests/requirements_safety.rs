//! Safety-oriented coverage for the public requirements generation and critique boundary.

use canon_adapters::copilot_cli::{CopilotCliAdapter, RequirementsGenerationInput};

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn require(condition: bool, message: &str) -> TestResult {
    if condition { Ok(()) } else { Err(message.into()) }
}

#[test]
fn requirements_generation_keeps_incomplete_context_out_of_downstream_design() -> TestResult {
    let adapter = CopilotCliAdapter;

    let output = adapter.generate_requirements(RequirementsGenerationInput {
        problem: "NOT CAPTURED - the operator problem is missing.",
        outcome: "Operators receive a bounded requirements packet.",
        constraints: &[],
        tradeoffs: &[],
        out_of_scope: &[],
        open_questions: &[],
        source_refs: &[],
    });

    require(
        output.summary.contains(
            "Tighten the authored brief before moving into system-shaping or architecture.",
        ),
        "missing problem did not keep the packet in requirements",
    )?;
    require(
        output
            .summary
            .contains("Stay in requirements long enough to replace the missing-context markers"),
        "missing context did not produce corrective downstream guidance",
    )?;
    require(
        output.summary.contains("NOT CAPTURED - No explicit constraints were supplied."),
        "empty constraints did not retain an explicit marker",
    )?;
    require(
        output.summary.contains("- Which downstream mode should consume this packet first?"),
        "empty open questions did not receive the documented default",
    )?;
    require(
        output.summary.contains("- no-authored-source-inputs-recorded"),
        "empty source references did not receive the documented default",
    )
}

#[test]
fn requirements_critique_blocks_missing_context_and_routes_complete_packets() -> TestResult {
    let adapter = CopilotCliAdapter;
    let missing_marker = vec!["NOT CAPTURED - owner decision required.".to_string()];

    let incomplete = adapter.critique_requirements(
        "NOT CAPTURED - problem statement missing.",
        "NOT CAPTURED - outcome missing.",
        &missing_marker,
        &missing_marker,
        &[],
        "Generated packet with unresolved markers.",
    );
    require(
        incomplete.summary.contains("Problem framing is still missing explicit authored input."),
        "critique accepted a missing problem",
    )?;
    require(
        incomplete.summary.contains("Outcome framing is still missing explicit authored input."),
        "critique accepted a missing outcome",
    )?;
    require(
        incomplete.summary.contains("Constraints are incomplete"),
        "critique accepted missing constraints",
    )?;
    require(
        incomplete.summary.contains("Scope cuts are incomplete"),
        "critique accepted missing scope cuts",
    )?;
    require(
        incomplete.summary.contains("lacks a complete problem/outcome pair"),
        "critique did not explain the incomplete framing",
    )?;
    require(
        incomplete.summary.contains(
            "Resolve the missing context markers before moving into system-shaping or architecture."
        ),
        "critique did not block downstream design",
    )?;

    let no_open_questions = adapter.critique_requirements(
        "Bound the release contract.",
        "Produce a reviewable packet.",
        &["Preserve public artifacts.".to_string()],
        &["No runtime implementation.".to_string()],
        &[],
        "Complete generated packet.",
    );
    require(
        no_open_questions
            .summary
            .contains("No additional missing context was detected in the authored brief."),
        "complete packet was reported as missing context",
    )?;
    require(
        no_open_questions
            .summary
            .contains("Review the completed packet and choose the smallest downstream mode"),
        "complete packet without open questions received the wrong routing guidance",
    )?;

    let open_questions = adapter.critique_requirements(
        "Bound the release contract.",
        "Produce a reviewable packet.",
        &["Preserve public artifacts.".to_string()],
        &["No runtime implementation.".to_string()],
        &["Which owner accepts the residual risk?".to_string()],
        "Complete generated packet with one open question.",
    );
    require(
        open_questions
            .summary
            .contains("Review the open questions, then choose the smallest downstream mode"),
        "complete packet with open questions received the wrong routing guidance",
    )
}
