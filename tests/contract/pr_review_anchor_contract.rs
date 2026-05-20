use std::fs;
use std::path::Path;

use canon_engine::artifacts::markdown::render_pr_review_artifact;
use canon_engine::review::findings::{ConventionalCommentScope, ReviewAnchor, ReviewPacket};
use canon_engine::review::summary::ReviewSummary;

fn fixture_text(name: &str) -> String {
    fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name))
        .expect("fixture text")
}

fn render_comments(packet: &ReviewPacket) -> String {
    let summary = ReviewSummary::from_packet(packet, false);
    render_pr_review_artifact("conventional-comments.md", packet, &summary)
}

#[test]
fn pr_review_anchor_contract_renders_line_anchor_with_explicit_scope() {
    let patch = fixture_text("pr_review_anchor_line.diff");
    let packet =
        ReviewPacket::from_diff("main", "HEAD", vec!["tests/reviewer.md".to_string()], &patch);

    let note = packet.note_findings().into_iter().next().expect("note finding");
    assert_eq!(note.scope, ConventionalCommentScope::Surface);
    assert_eq!(
        note.anchor,
        Some(ReviewAnchor {
            surface: "tests/reviewer.md".to_string(),
            line_start: 3,
            line_end: None,
        })
    );

    let output = render_comments(&packet);
    assert!(output.contains("praise(scope:surface):"));
    assert!(output.contains("Anchor: tests/reviewer.md:3"));
}

#[test]
fn pr_review_anchor_contract_renders_span_anchor_with_explicit_scope() {
    let patch = fixture_text("pr_review_anchor_span.diff");
    let packet =
        ReviewPacket::from_diff("main", "HEAD", vec!["tests/reviewer.md".to_string()], &patch);

    let note = packet.note_findings().into_iter().next().expect("note finding");
    assert_eq!(note.scope, ConventionalCommentScope::Surface);
    assert_eq!(
        note.anchor,
        Some(ReviewAnchor {
            surface: "tests/reviewer.md".to_string(),
            line_start: 3,
            line_end: Some(4),
        })
    );

    let output = render_comments(&packet);
    assert!(output.contains("praise(scope:surface):"));
    assert!(output.contains("Anchor: tests/reviewer.md:3-4"));
}

#[test]
fn pr_review_anchor_contract_omits_anchor_for_cross_surface_packets() {
    let patch = fixture_text("pr_review_anchor_cross_surface.diff");
    let packet = ReviewPacket::from_diff(
        "main",
        "HEAD",
        vec!["src/reviewer.rs".to_string(), "tests/reviewer.md".to_string()],
        &patch,
    );

    assert!(packet.findings.iter().all(|finding| finding.anchor.is_none()));

    let output = render_comments(&packet);
    assert!(output.contains("praise(scope:file):") || output.contains("praise(scope:surface):"));
    assert!(!output.contains("Anchor:"));
}

#[test]
fn pr_review_anchor_contract_omits_anchor_for_stale_and_evidence_free_packets() {
    let stale_patch = fixture_text("pr_review_anchor_stale.diff");
    let stale_packet = ReviewPacket::from_diff(
        "main",
        "HEAD",
        vec!["tests/reviewer.md".to_string()],
        &stale_patch,
    );
    assert!(stale_packet.findings.iter().all(|finding| finding.anchor.is_none()));
    assert!(!render_comments(&stale_packet).contains("Anchor:"));

    let imported_packet = ReviewPacket::from_evidence(
        "main",
        "HEAD",
        vec!["tests/reviewer.md".to_string()],
        "",
        "Imported historical packet without persisted diff evidence.",
    );
    assert!(imported_packet.findings.iter().all(|finding| finding.anchor.is_none()));
    assert!(!render_comments(&imported_packet).contains("Anchor:"));
}
