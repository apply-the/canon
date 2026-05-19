use super::*;

const VERIFICATION_VERDICT_SUPPORTED: &str = "supported";
const VERIFICATION_VERDICT_UNSUPPORTED: &str = "unsupported";
const VERIFICATION_VERDICT_MIXED: &str = "mixed";
const VERIFICATION_OPEN_FINDINGS_OPEN: &str = "unresolved-findings-open";
const VERIFICATION_OPEN_FINDINGS_CLEAR: &str = "no-open-findings";

pub(super) fn generate_verification_summary(context: &str) -> String {
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
                "No explicit risk boundary was authored for this verification packet.".to_string()
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

    format!(
        "## Claims Under Test\n\n{}\n\n## Evidence Basis\n\n{}\n\n## Contract Assumptions\n\n{}\n\n## Risk Boundary\n\n{}\n\n## Challenge Focus\n\n{}\n\n## Out of Scope\n\n{}",
        claims_under_test,
        evidence_basis,
        contract_assumptions,
        risk_boundary,
        challenge_focus,
        out_of_scope,
    )
}

pub(super) fn critique_verification_summary(generated: &str) -> String {
    let claims_under_test = verification_section_items(generated, &["Claims Under Test"]);
    let evidence_basis = verification_section_items(generated, &["Evidence Basis"]);
    let contract_assumptions = verification_section_items(generated, &["Contract Assumptions"]);
    let challenge_focus = verification_section_items(generated, &["Challenge Focus"]);
    let risk_boundary =
        verification_section_block(generated, &["Risk Boundary"]).unwrap_or_else(|| {
            "No explicit risk boundary was authored for this verification packet.".to_string()
        });

    let (explicit_risk_claims, explicit_risk_evidence, strong_claims) =
        collect_verification_risk_signals(&claims_under_test, &evidence_basis);
    let challenge_findings = build_verification_challenge_findings(
        &challenge_focus,
        &strong_claims,
        &explicit_risk_claims,
        &explicit_risk_evidence,
    );
    let contradictions = build_verification_contradictions(
        &explicit_risk_claims,
        &explicit_risk_evidence,
        &challenge_focus,
    );
    let open_findings =
        build_verification_open_findings(&challenge_focus, &strong_claims, &contradictions);
    let has_open_findings = !open_findings.is_empty();
    let (verdict, open_findings_status) =
        determine_verification_verdict(&open_findings, &contradictions, &risk_boundary);
    let verified_claims =
        build_verified_claims(verdict, &claims_under_test, &evidence_basis, &contract_assumptions);
    let rejected_claims =
        build_rejected_claims(verdict, &explicit_risk_claims, &strong_claims, &challenge_focus);
    let required_follow_up = build_required_follow_up(
        has_open_findings,
        &challenge_focus,
        &contradictions,
        &strong_claims,
    );
    let rationale = verification_verdict_rationale(verdict, open_findings.len());

    format!(
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
    )
}

fn collect_verification_risk_signals(
    claims_under_test: &[String],
    evidence_basis: &[String],
) -> (Vec<String>, Vec<String>, Vec<String>) {
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

    (explicit_risk_claims, explicit_risk_evidence, strong_claims)
}

fn build_verification_challenge_findings(
    challenge_focus: &[String],
    strong_claims: &[String],
    explicit_risk_claims: &[String],
    explicit_risk_evidence: &[String],
) -> Vec<String> {
    let mut challenge_findings = Vec::new();
    for focus in challenge_focus {
        challenge_findings.push(format!(
            "Authored challenge focus remains open until explicit evidence answers it: {focus}"
        ));
    }
    for claim in strong_claims {
        challenge_findings.push(format!(
            "The packet makes a broad assurance claim that still needs direct evidence: {claim}"
        ));
    }
    for claim in explicit_risk_claims {
        challenge_findings.push(format!(
            "The authored claim already signals a contradiction or missing-evidence path: {claim}"
        ));
    }
    for entry in explicit_risk_evidence {
        challenge_findings.push(format!(
            "The authored evidence basis already records a contradiction or proof gap: {entry}"
        ));
    }
    challenge_findings
}

fn build_verification_contradictions(
    explicit_risk_claims: &[String],
    explicit_risk_evidence: &[String],
    challenge_focus: &[String],
) -> Vec<String> {
    let mut contradictions = Vec::new();
    for claim in explicit_risk_claims {
        contradictions.push(format!(
            "The authored claim under test already records a contradiction or unresolved support gap: {claim}"
        ));
    }
    for entry in explicit_risk_evidence {
        contradictions.push(format!(
            "The evidence basis still names a proof gap or unsupported path: {entry}"
        ));
    }
    for focus in challenge_focus {
        if contains_verification_failure_keyword(focus) {
            contradictions.push(format!(
                "The authored challenge focus already names an unresolved contradiction or evidence gap: {focus}"
            ));
        }
    }
    contradictions
}

fn build_verification_open_findings(
    challenge_focus: &[String],
    strong_claims: &[String],
    contradictions: &[String],
) -> Vec<String> {
    let mut open_findings = Vec::new();
    for focus in challenge_focus {
        open_findings.push(format!(
            "Answer this authored challenge focus with explicit evidence or narrow the affected claim: {focus}"
        ));
    }
    for claim in strong_claims {
        open_findings.push(format!(
            "This broad assurance claim still needs adversarial or contract-backed evidence: {claim}"
        ));
    }
    open_findings.extend(contradictions.iter().cloned());
    open_findings
}

fn determine_verification_verdict(
    open_findings: &[String],
    contradictions: &[String],
    risk_boundary: &str,
) -> (&'static str, &'static str) {
    let risk_boundary_blocks = contains_case_insensitive(risk_boundary, "block")
        || contains_case_insensitive(risk_boundary, "must")
        || contains_case_insensitive(risk_boundary, "cannot pass")
        || contains_case_insensitive(risk_boundary, "should fail");
    let verdict = if open_findings.is_empty() {
        VERIFICATION_VERDICT_SUPPORTED
    } else if !contradictions.is_empty() || risk_boundary_blocks {
        VERIFICATION_VERDICT_UNSUPPORTED
    } else {
        VERIFICATION_VERDICT_MIXED
    };
    let open_findings_status = if open_findings.is_empty() {
        VERIFICATION_OPEN_FINDINGS_CLEAR
    } else {
        VERIFICATION_OPEN_FINDINGS_OPEN
    };

    (verdict, open_findings_status)
}

fn build_verified_claims(
    verdict: &str,
    claims_under_test: &[String],
    evidence_basis: &[String],
    contract_assumptions: &[String],
) -> Vec<String> {
    if verdict == VERIFICATION_VERDICT_SUPPORTED {
        if claims_under_test.is_empty() {
            return vec![
                "The verification packet remained bounded to the authored evidence basis and contract surface."
                    .to_string(),
            ];
        }

        return claims_under_test.to_vec();
    }

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
}

fn build_rejected_claims(
    verdict: &str,
    explicit_risk_claims: &[String],
    strong_claims: &[String],
    challenge_focus: &[String],
) -> Vec<String> {
    if verdict == VERIFICATION_VERDICT_SUPPORTED {
        return Vec::new();
    }

    let mut rejected_claims = Vec::new();
    for claim in explicit_risk_claims {
        rejected_claims.push(format!("Still unsupported from the current packet: {claim}"));
    }
    for claim in strong_claims {
        rejected_claims.push(format!("Still unsupported from the current packet: {claim}"));
    }
    if rejected_claims.is_empty() {
        for focus in challenge_focus {
            rejected_claims.push(format!(
                "The packet does not yet close this authored challenge focus: {focus}"
            ));
        }
    }
    rejected_claims
}

fn build_required_follow_up(
    has_open_findings: bool,
    challenge_focus: &[String],
    contradictions: &[String],
    strong_claims: &[String],
) -> Vec<String> {
    if !has_open_findings {
        return vec![
            "Keep the verification packet attached to downstream release or approval discussion."
                .to_string(),
        ];
    }

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
}

fn verification_verdict_rationale(verdict: &str, open_findings_count: usize) -> String {
    match verdict {
        VERIFICATION_VERDICT_SUPPORTED => {
            "No explicit contradiction, missing-evidence marker, or open authored challenge remained in the normalized verification packet."
                .to_string()
        }
        VERIFICATION_VERDICT_UNSUPPORTED => format!(
            "The packet still carries {open_findings_count} unresolved finding(s) against the named claims and evidence basis, so readiness remains blocked."
        ),
        _ => {
            "Some verification concerns remain open and need follow-up before the packet can be treated as fully trusted."
                .to_string()
        }
    }
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
