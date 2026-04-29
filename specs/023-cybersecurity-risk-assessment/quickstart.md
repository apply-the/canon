# Quickstart: Cybersecurity Risk Assessment Mode

## Goal

Validate that `security-assessment` works as a first-class governed mode in the
`0.22.0` release, produces a readable recommendation-only security packet, and
lands with synchronized docs, skills, and regression coverage.

## Recommended Validation Flow

1. Confirm the new embedded and mirrored `canon-security-assessment` skill
   surfaces define the same canonical authored H2 sections, input rules, and
   recommendation-only posture.
2. Prepare a representative authored input at `canon-input/security-assessment.md`
   or `canon-input/security-assessment/brief.md` covering assets, trust
   boundaries, threats, risks, mitigations, assumptions, and evidence.
3. Run the focused security-assessment contract, renderer, docs, run, and
   publish tests.
4. Run `/bin/bash scripts/validate-canon-skills.sh` to confirm the embedded and
   mirrored skill surfaces remain synchronized.
5. Run `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and a full `cargo nextest run`.
6. Review `README.md`, `CHANGELOG.md`, `docs/guides/modes.md`, shared runtime
   compatibility references, and the published packet path to confirm the
   `0.22.0` release surface is coherent.

## Representative Packet Walkthroughs

- Run a green or bounded-impact assessment against a complete authored brief and
  verify the packet includes all seven expected artifacts.
- Run a negative-path example with one required authored section removed and
  verify the affected artifact surfaces `## Missing Authored Body`.
- Publish a completed assessment and verify files land under
  `docs/security-assessments/<RUN_ID>/`.
- Review the packet text to confirm findings remain recommendation-only and do
  not claim audit completion or autonomous remediation.
