# Quickstart: Authoring Experience And Packet Readiness

## Scenario 1: Single-File Packet Reads As The Authoritative Brief

1. Create `canon-input/requirements.md` with a bounded requirements brief.
2. Run:

   ```bash
   canon inspect clarity --mode requirements --input canon-input/requirements.md
   ```

3. Confirm the clarity output reports a single-file packet shape, treats the
   input as authoritative, and keeps the next authoring step explicit.

## Scenario 2: Directory-Backed Carry-Forward Packet Keeps `brief.md` Authoritative

1. Create a packet such as:

   ```text
   canon-input/implementation/
     brief.md
     source-map.md
     selected-context.md
   ```

2. Run:

   ```bash
   canon inspect clarity --mode implementation --input canon-input/implementation
   ```

3. Confirm the clarity output treats `brief.md` as the readiness brief,
   surfaces `source-map.md` and `selected-context.md` as supporting inputs, and
   keeps any readiness delta explicit.

## Scenario 3: Ambiguous Folder Packet Stays Honest

1. Create a packet directory with multiple notes but no `brief.md`.
2. Run:

   ```bash
   canon inspect clarity --mode implementation --input canon-input/implementation
   ```

3. Confirm the clarity output keeps authority ambiguous and recommends
   tightening the packet shape rather than guessing the source of truth.

## Focused Validation Commands

```bash
cargo test --test inspect_clarity
cargo test -p canon-cli clarity_markdown_surfaces_questions_and_signals
cargo test --test release_036_release_provenance_integrity
cargo test --test skills_bootstrap skills_install_for_codex_carries_current_runtime_compatibility_reference
cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run
```