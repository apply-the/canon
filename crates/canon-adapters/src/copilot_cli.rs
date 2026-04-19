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

    pub fn generate_review(&self, context: &str) -> CopilotCliOutput {
        let normalized = normalize_multiline_context(context);
        let summary = format!(
            "Review the bounded non-PR change package or artifact bundle. Preserve explicit evidence basis, boundary findings, decision impact, and disposition cues for: {}",
            normalized
        );
        self.output(CapabilityKind::GenerateContent, "copilot-cli review generation", summary)
    }

    pub fn critique_review(&self, generated: &str) -> CopilotCliOutput {
        let normalized = normalize_multiline_context(generated);
        let summary = format!(
            "Challenge the proposed review packet for evidence coverage, hidden scope growth, ownership clarity, and acceptance rationale. Review target: {}",
            normalized
        );
        self.output(CapabilityKind::CritiqueContent, "copilot-cli review critique", summary)
    }

    pub fn generate_verification(&self, context: &str) -> CopilotCliOutput {
        let claims_under_test = verification_section_list_or_fallback(
            context,
            &["Claims Under Test"],
            "- No explicit claims under test were authored for this verification packet.",
        );
        let evidence_basis = verification_section_list_or_fallback(
            context,
            &["Evidence Basis", "In Scope Evidence"],
            "- No explicit evidence basis was authored for this verification packet.",
        );
        let contract_assumptions = verification_contract_assumptions(context);
        let risk_boundary =
            verification_section_block(context, &["Risk Boundary", "Critical Boundary"])
                .unwrap_or_else(|| {
                    "No explicit risk boundary was authored for this verification packet."
                        .to_string()
                });
        let challenge_focus = verification_section_list_or_fallback(
            context,
            &["Challenge Focus"],
            "- No additional challenge focus was authored for this verification packet.",
        );
        let out_of_scope = verification_section_list_or_fallback(
            context,
            &["Out of Scope"],
            "- No explicit out-of-scope constraints were authored for this verification packet.",
        );
        let summary = format!(
            "## Claims Under Test\n\n{}\n\n## Evidence Basis\n\n{}\n\n## Contract Assumptions\n\n{}\n\n## Risk Boundary\n\n{}\n\n## Challenge Focus\n\n{}\n\n## Out of Scope\n\n{}",
            claims_under_test,
            evidence_basis,
            contract_assumptions,
            risk_boundary,
            challenge_focus,
            out_of_scope,
        );
        self.output(CapabilityKind::GenerateContent, "copilot-cli verification generation", summary)
    }

    pub fn critique_verification(&self, generated: &str) -> CopilotCliOutput {
        let claims_under_test = verification_section_items(generated, &["Claims Under Test"]);
        let evidence_basis = verification_section_items(generated, &["Evidence Basis"]);
        let contract_assumptions = verification_section_items(generated, &["Contract Assumptions"]);
        let challenge_focus = verification_section_items(generated, &["Challenge Focus"]);
        let risk_boundary = verification_section_block(generated, &["Risk Boundary"])
            .unwrap_or_else(|| {
                "No explicit risk boundary was authored for this verification packet.".to_string()
            });

        let explicit_risk_claims = claims_under_test
            .iter()
            .filter(|claim| contains_verification_failure_keyword(claim))
            .cloned()
            .collect::<Vec<_>>();
        let explicit_risk_evidence = evidence_basis
            .iter()
            .filter(|entry| contains_verification_failure_keyword(entry))
            .cloned()
            .collect::<Vec<_>>();
        let strong_claims = claims_under_test
            .iter()
            .filter(|claim| has_strong_verification_claim_language(claim))
            .cloned()
            .collect::<Vec<_>>();

        let mut challenge_findings = Vec::new();
        for focus in &challenge_focus {
            challenge_findings.push(format!(
                "Authored challenge focus remains open until explicit evidence answers it: {focus}"
            ));
        }
        for claim in &strong_claims {
            challenge_findings.push(format!(
                "The packet makes a broad assurance claim that still needs direct evidence: {claim}"
            ));
        }
        for claim in &explicit_risk_claims {
            challenge_findings.push(format!(
                "The authored claim already signals a contradiction or missing-evidence path: {claim}"
            ));
        }
        for entry in &explicit_risk_evidence {
            challenge_findings.push(format!(
                "The authored evidence basis already records a contradiction or proof gap: {entry}"
            ));
        }

        let mut contradictions = Vec::new();
        for claim in &explicit_risk_claims {
            contradictions.push(format!(
                "The authored claim under test already records a contradiction or unresolved support gap: {claim}"
            ));
        }
        for entry in &explicit_risk_evidence {
            contradictions.push(format!(
                "The evidence basis still names a proof gap or unsupported path: {entry}"
            ));
        }
        for focus in &challenge_focus {
            if contains_verification_failure_keyword(focus) {
                contradictions.push(format!(
                    "The authored challenge focus already names an unresolved contradiction or evidence gap: {focus}"
                ));
            }
        }

        let mut open_findings = Vec::new();
        for focus in &challenge_focus {
            open_findings.push(format!(
                "Answer this authored challenge focus with explicit evidence or narrow the affected claim: {focus}"
            ));
        }
        for claim in &strong_claims {
            open_findings.push(format!(
                "This broad assurance claim still needs adversarial or contract-backed evidence: {claim}"
            ));
        }
        for contradiction in &contradictions {
            open_findings.push(contradiction.clone());
        }

        let has_open_findings = !open_findings.is_empty();
        let risk_boundary_blocks = contains_case_insensitive(&risk_boundary, "block")
            || contains_case_insensitive(&risk_boundary, "must")
            || contains_case_insensitive(&risk_boundary, "cannot pass")
            || contains_case_insensitive(&risk_boundary, "should fail");
        let verdict = if !has_open_findings {
            "supported"
        } else if !contradictions.is_empty() || risk_boundary_blocks {
            "unsupported"
        } else {
            "mixed"
        };
        let open_findings_status =
            if has_open_findings { "unresolved-findings-open" } else { "no-open-findings" };

        let verified_claims = if verdict == "supported" {
            if claims_under_test.is_empty() {
                vec![
                    "The verification packet remained bounded to the authored evidence basis and contract surface."
                        .to_string(),
                ]
            } else {
                claims_under_test.clone()
            }
        } else {
            let mut supported = Vec::new();
            if !evidence_basis.is_empty() {
                supported.push(
                    "The evidence basis is explicit enough for downstream inspection and follow-up."
                        .to_string(),
                );
            }
            if !contract_assumptions.is_empty() {
                supported.push(
                    "The contract surface or assumptions remain explicit in the verification packet."
                        .to_string(),
                );
            }
            if supported.is_empty() {
                supported.push(
                    "Only the packet boundaries were captured; the claims themselves remain under challenge."
                        .to_string(),
                );
            }
            supported
        };

        let mut rejected_claims = Vec::new();
        if verdict != "supported" {
            for claim in &explicit_risk_claims {
                rejected_claims.push(format!("Still unsupported from the current packet: {claim}"));
            }
            for claim in &strong_claims {
                rejected_claims.push(format!("Still unsupported from the current packet: {claim}"));
            }
            if rejected_claims.is_empty() {
                for focus in &challenge_focus {
                    rejected_claims.push(format!(
                        "The packet does not yet close this authored challenge focus: {focus}"
                    ));
                }
            }
        }

        let required_follow_up = if has_open_findings {
            let mut follow_up = Vec::new();
            if !challenge_focus.is_empty() {
                follow_up.push(
                    "Address each authored challenge-focus item with explicit evidence or narrow the affected claim."
                        .to_string(),
                );
            }
            if !contradictions.is_empty() {
                follow_up.push(
                    "Resolve the contradictions or proof gaps before treating the packet as supported."
                        .to_string(),
                );
            }
            if !strong_claims.is_empty() {
                follow_up.push(
                    "Reduce absolute claim language or add adversarial evidence for the strongest assurances."
                        .to_string(),
                );
            }
            follow_up.push(
                "Keep the unresolved findings visible in the packet until the next verification or review pass."
                    .to_string(),
            );
            follow_up
        } else {
            vec![
                "Keep the verification packet attached to downstream release or approval discussion."
                    .to_string(),
            ]
        };

        let rationale = match verdict {
            "supported" => {
                "No explicit contradiction, missing-evidence marker, or open authored challenge remained in the normalized verification packet."
                    .to_string()
            }
            "unsupported" => format!(
                "The packet still carries {} unresolved finding(s) against the named claims and evidence basis, so readiness remains blocked.",
                open_findings.len()
            ),
            _ => {
                "Some verification concerns remain open and need follow-up before the packet can be treated as fully trusted."
                    .to_string()
            }
        };

        let summary = format!(
            "## Challenge Findings\n\n{}\n\n## Contradictions\n\n{}\n\n## Verified Claims\n\n{}\n\n## Rejected Claims\n\n{}\n\n## Open Findings\n\nStatus: {}\n\n{}\n\n## Required Follow-up\n\n{}\n\n## Overall Verdict\n\nStatus: {}\nRationale: {}",
            render_markdown_list(
                &dedupe_preserving_order(challenge_findings),
                "- No additional challenge findings were recorded beyond the authored verification packet."
            ),
            render_markdown_list(
                &dedupe_preserving_order(contradictions),
                "- No direct contradictions were identified from the current verification packet."
            ),
            render_markdown_list(
                &dedupe_preserving_order(verified_claims),
                "- The authored claims remain bounded and supported by the current evidence bundle."
            ),
            render_markdown_list(
                &dedupe_preserving_order(rejected_claims),
                "- No rejected claims were inferred from the current verification target."
            ),
            open_findings_status,
            render_markdown_list(
                &dedupe_preserving_order(open_findings),
                "- No unresolved findings remain from the current verification target."
            ),
            render_markdown_list(
                &required_follow_up,
                "- Keep the verification packet attached to any downstream release or approval discussion."
            ),
            verdict,
            rationale,
        );
        self.output(CapabilityKind::CritiqueContent, "copilot-cli verification critique", summary)
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

fn verification_contract_assumptions(context: &str) -> String {
    if let Some(contract_surface) =
        verification_section_block(context, &["Contract Assumptions", "Contract Surface"])
    {
        let items = section_list_items(&contract_surface);
        if !items.is_empty() {
            return render_markdown_list(
                &items,
                "- Contract assumptions were not explicitly authored for this verification packet.",
            );
        }

        return format!("- {}", normalize_multiline_context(&contract_surface));
    }

    "- The verification packet relies on the authored claims staying bounded to the named evidence basis."
        .to_string()
}

fn verification_section_list_or_fallback(
    source: &str,
    markers: &[&str],
    empty_message: &str,
) -> String {
    let items = verification_section_items(source, markers);
    if items.is_empty() {
        empty_message.to_string()
    } else {
        render_markdown_list(&items, empty_message)
    }
}

fn verification_section_items(source: &str, markers: &[&str]) -> Vec<String> {
    verification_section_block(source, markers)
        .map(|block| section_list_items(&block))
        .unwrap_or_default()
        .into_iter()
        .filter(|item| !is_verification_placeholder_item(item))
        .collect()
}

fn verification_section_block(source: &str, markers: &[&str]) -> Option<String> {
    let normalized = source.to_ascii_lowercase();

    markers.iter().find_map(|marker| {
        extract_verification_markdown_section(source, marker)
            .or_else(|| extract_verification_inline_marker(source, &normalized, marker))
            .map(|value| normalize_multiline_context(&value))
            .filter(|value| !value.is_empty())
    })
}

fn extract_verification_inline_marker(
    source: &str,
    normalized: &str,
    marker: &str,
) -> Option<String> {
    let marker_with_colon = format!("{}:", marker.to_ascii_lowercase());
    let start = normalized.find(&marker_with_colon)?;
    let remainder = &source[start + marker_with_colon.len()..];
    let line = remainder.lines().next()?.trim();
    if line.is_empty() { None } else { Some(line.to_string()) }
}

fn extract_verification_markdown_section(source: &str, marker: &str) -> Option<String> {
    let mut lines = source.lines().peekable();

    while let Some(line) = lines.next() {
        if !is_matching_section_heading(line, marker) {
            continue;
        }

        let mut section_lines = Vec::new();
        while let Some(next_line) = lines.peek() {
            if next_line.trim_start().starts_with('#') {
                break;
            }
            section_lines.push(lines.next().unwrap_or_default());
        }

        let section = trim_multiline_block(&section_lines.join("\n"));
        if !section.is_empty() {
            return Some(section);
        }
    }

    None
}

fn is_matching_section_heading(line: &str, marker: &str) -> bool {
    let trimmed = line.trim();
    if !trimmed.starts_with('#') {
        return false;
    }

    trimmed.trim_start_matches('#').trim().eq_ignore_ascii_case(marker)
}

fn section_list_items(block: &str) -> Vec<String> {
    let items = block
        .lines()
        .filter_map(|line| trim_list_item(line.trim()))
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    if !items.is_empty() {
        return items;
    }

    let normalized = normalize_multiline_context(block);
    if normalized.is_empty() { Vec::new() } else { vec![normalized] }
}

fn trim_list_item(line: &str) -> Option<String> {
    if line.is_empty() {
        return None;
    }

    if let Some(item) = line.strip_prefix("- ").or_else(|| line.strip_prefix("* ")) {
        return Some(item.trim().to_string());
    }

    let mut digits = 0usize;
    for character in line.chars() {
        if character.is_ascii_digit() {
            digits += 1;
            continue;
        }

        if digits > 0 && (character == '.' || character == ')') {
            return Some(line[digits + 1..].trim().to_string());
        }

        break;
    }

    None
}

fn is_verification_placeholder_item(value: &str) -> bool {
    let trimmed = value.trim();

    contains_case_insensitive(trimmed, "for this verification packet.")
        && (trimmed.starts_with("No explicit ")
            || trimmed.starts_with("No additional ")
            || trimmed.starts_with("Contract assumptions were not explicitly authored"))
}

fn trim_multiline_block(value: &str) -> String {
    let lines = value.lines().collect::<Vec<_>>();
    let start = lines.iter().position(|line| !line.trim().is_empty());
    let end = lines.iter().rposition(|line| !line.trim().is_empty());

    match (start, end) {
        (Some(start), Some(end)) => lines[start..=end].join("\n"),
        _ => String::new(),
    }
}

fn contains_case_insensitive(value: &str, needle: &str) -> bool {
    value.to_ascii_lowercase().contains(&needle.to_ascii_lowercase())
}

fn contains_verification_failure_keyword(value: &str) -> bool {
    [
        "unsupported",
        "contradiction",
        "unresolved",
        "missing evidence",
        "insufficient evidence",
        "lacks concrete proof",
        "lacks proof",
    ]
    .into_iter()
    .any(|needle| contains_case_insensitive(value, needle))
}

fn has_strong_verification_claim_language(value: &str) -> bool {
    [" all ", "fully", "in practice", "sufficient to", "no additional", "guarantee"]
        .into_iter()
        .any(|needle| contains_case_insensitive(value, needle))
}

fn dedupe_preserving_order(values: Vec<String>) -> Vec<String> {
    let mut deduped = Vec::new();

    for value in values {
        if !deduped.iter().any(|existing: &String| existing.eq_ignore_ascii_case(&value)) {
            deduped.push(value);
        }
    }

    deduped
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

    #[test]
    fn verification_generation_preserves_authored_sections() {
        let adapter = CopilotCliAdapter;

        let output = adapter.generate_verification(
            "# Verification Brief\n\n## Claims Under Test\n- rollback remains bounded and auditable\n\n## Evidence Basis\n- repository checks\n- operator logs\n\n## Contract Surface\n- rollback metadata must remain explicit\n\n## Challenge Focus\n- look for unsupported rollback jumps\n",
        );

        assert!(output.summary.contains("## Claims Under Test"));
        assert!(output.summary.contains("rollback remains bounded and auditable"));
        assert!(output.summary.contains("## Evidence Basis"));
        assert!(output.summary.contains("## Contract Assumptions"));
        assert!(output.summary.contains("rollback metadata must remain explicit"));
        assert!(output.summary.contains("## Challenge Focus"));
    }

    #[test]
    fn verification_critique_emits_structured_findings_and_verdict() {
        let adapter = CopilotCliAdapter;

        let output = adapter.critique_verification(
            "## Claims Under Test\n\n- the rollback guarantee is fully proven without any additional evidence\n\n## Evidence Basis\n\n- an unsupported rollback guarantee still lacks concrete proof\n\n## Contract Assumptions\n\n- rollback metadata must remain explicit\n\n## Risk Boundary\n\n- contradiction or missing evidence should block readiness\n\n## Challenge Focus\n\n- look for contradictions between the rollback claim and the runtime contract",
        );

        assert!(output.summary.contains("## Challenge Findings"));
        assert!(output.summary.contains("## Open Findings"));
        assert!(output.summary.contains("Status: unresolved-findings-open"));
        assert!(output.summary.contains("## Overall Verdict"));
        assert!(output.summary.contains("Status: unsupported"));
        assert!(output.summary.contains("Still unsupported from the current packet"));
    }

    #[test]
    fn verification_critique_ignores_placeholder_challenge_focus_when_none_authored() {
        let adapter = CopilotCliAdapter;

        let generated = adapter.generate_verification(
            "# Verification Brief\n\n## Claims Under Test\n- rollback remains bounded and auditable\n- operator evidence remains tied to the rollback boundary\n\n## Evidence Basis\n- current contract notes\n- repository checks\n- operator logs\n\n## Contract Surface\n- rollback metadata must remain explicit\n\n## Risk Boundary\n- contradictions or missing evidence on rollback scope should block readiness\n",
        );
        let critique = adapter.critique_verification(&generated.summary);

        assert!(critique.summary.contains("Status: no-open-findings"));
        assert!(critique.summary.contains("Status: supported"));
        assert!(!critique.summary.contains("Answer this authored challenge focus"));
    }
}
