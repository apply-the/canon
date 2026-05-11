# Quickstart: Mode Publish Alignment

## 1. Validate the runtime publish mismatch before implementation

Run the focused security-assessment publish regression once the new test exists:

```bash
cargo test --test security_assessment_direct_runtime
```

Confirm the slice specifically covers readable `AwaitingApproval` and `Blocked` security-assessment packets without widening publishability for unrelated modes.

## 2. Validate assistant publish command surfaces

Run the assistant package validation binary:

```bash
cargo test --test assistant_plugin_packages
```

Confirm all assistant-facing publish examples use positional `canon publish <RUN_ID>` syntax.

## 3. Run structural validation

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## 4. Run final regression and coverage closeout

```bash
cargo nextest run --workspace --all-features
cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info
```

Record touched-file coverage results in `validation-report.md` and justify any accepted exception before merge.