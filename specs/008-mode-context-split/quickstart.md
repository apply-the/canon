# Quickstart: Mode Context Split

## 1. Prepare canonical change input

Create the bounded-change packet at the new canonical path:

```text
canon-input/change.md
```

or the directory form:

```text
canon-input/change/
```

## 2. Run bounded change planning for an existing system

```bash
canon run \
  --mode change \
  --system-context existing \
  --risk bounded-impact \
  --zone yellow \
  --input canon-input/change.md
```

Expected result:

- Canon starts a real run
- emitted artifacts live under `.canon/artifacts/<RUN_ID>/change/`
- persisted run context includes `system_context = "existing"`

## 3. Confirm invalid combinations fail early

Missing required context:

```bash
canon run \
  --mode change \
  --risk bounded-impact \
  --zone yellow \
  --input canon-input/change.md
```

Unsupported combination:

```bash
canon run \
  --mode change \
  --system-context new \
  --risk bounded-impact \
  --zone green \
  --input canon-input/change.md
```

Both commands must fail before run creation with corrective guidance.

## 4. Verify optional-context behavior

```bash
canon run \
  --mode requirements \
  --risk low-impact \
  --zone green \
  --input canon-input/requirements.md
```

Expected result:

- the run may proceed without `--system-context`
- `context.toml` omits the field instead of inventing a default

## 5. Inspect persisted context and renamed artifacts

```bash
canon status --run <RUN_ID>
canon inspect artifacts --run <RUN_ID>
canon inspect evidence --run <RUN_ID>
```

Expected result:

- `change` appears as the mode name
- artifact paths use `/change/`
- persisted context is visible when present

## 6. Run validation and coverage

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run
cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info
/bin/bash scripts/validate-canon-skills.sh
pwsh -File scripts/validate-canon-skills.ps1
```

The feature is ready for review only after targeted context-validation tests and the workspace coverage run confirm the touched patch reaches the agreed threshold.