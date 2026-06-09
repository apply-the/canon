# Quickstart: Early Signal Pass Validation

**Feature**: 075-pr-review-early-signal-pass

## Prerequisites

- Canon CLI built (`cargo build --bin canon`)
- A Git repository with at least two branches to diff
- `CANON_BIN` environment variable pointing to the built binary, or use `cargo run --`

## 1. Basic early signal pass (default-on)

```bash
WORKDIR="$(mktemp -d)"
cd "$WORKDIR"
git init
git -c commit.gpgsign=false commit --allow-empty -m "initial"

# Create a file with a deliberate stale reference
echo 'fn main() { use removed_module; }' > src/main.rs
mkdir -p src
echo 'pub fn removed_fn() {}' > src/removed_module.rs
git add -A
git -c commit.gpgsign=false commit -m "add files"

# Remove the referenced module but don't update the import
git rm src/removed_module.rs
git -c commit.gpgsign=false commit -m "remove module, break reference"

# Run prepare
"$CANON_BIN" pr-review prepare --base HEAD~1 --head HEAD --output json
```

**Expected**: stdout contains `early_signal.finding_detected` with `rule_id: "reference.dangling_import"` for `src/main.rs` referencing `removed_module`.

## 2. Skip early signal with reason

```bash
"$CANON_BIN" pr-review prepare --base HEAD~1 --head HEAD --skip-early-signal --skip-reason "debugging accept flow" --output json
```

**Expected**: stdout contains `early_signal.skipped` with `reason: "debugging accept flow"`. No `early_signal.finding_detected` events.

## 3. Skip without reason (must fail)

```bash
"$CANON_BIN" pr-review prepare --base HEAD~1 --head HEAD --skip-early-signal
```

**Expected**: Exit code 1. Error message about missing skip reason.

## 4. Layer directories generated

```bash
"$CANON_BIN" pr-review prepare --base HEAD~1 --head HEAD --output json > /dev/null
RUN_ID=$(ls -t .canon/runs/ | head -1)
ls .canon/runs/$RUN_ID/pr-review/layers/
```

**Expected**: Directories `01-early-signal/` through `07-coverage-accounting/`, each containing `instructions.md`, `required-context.tsv`, and `output.md`.

## 5. Trace file persisted

```bash
ls .canon/runs/$RUN_ID/pr-review/traces/early-signal.jsonl
head -1 .canon/runs/$RUN_ID/pr-review/traces/early-signal.jsonl | python3 -m json.tool
```

**Expected**: Valid JSON, first event is `early_signal.started`.

## 6. Finding artifacts persisted

```bash
ls .canon/runs/$RUN_ID/pr-review/early-signal/
cat .canon/runs/$RUN_ID/pr-review/early-signal/findings.tsv
python3 -m json.tool .canon/runs/$RUN_ID/pr-review/early-signal/findings.json
cat .canon/runs/$RUN_ID/pr-review/early-signal/summary.md
```

**Expected**: `findings.tsv` is tab-separated, `findings.json` is valid JSON with matching finding IDs, `summary.md` is markdown with counts.

## 7. Review-plan.md exists

```bash
cat .canon/runs/$RUN_ID/pr-review/review-plan.md
```

**Expected**: Markdown document listing all 7 layers in order with their status.

## 8. Finalize rejects incomplete review

```bash
# Attempt finalize before accept — must fail
"$CANON_BIN" pr-review finalize --output json 2>&1 || true
```

**Expected**: Error message about run not in accepted state.

## 9. Text output (default)

```bash
"$CANON_BIN" pr-review prepare --base HEAD~1 --head HEAD
```

**Expected**: Human-readable markdown summary on stdout. No JSON. Trace is still persisted — verify with:

```bash
test -f .canon/runs/$(ls -t .canon/runs/ | head -1)/pr-review/traces/early-signal.jsonl \
  && echo "trace persisted" || echo "MISSING TRACE"
```
