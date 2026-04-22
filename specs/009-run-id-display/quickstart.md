# Quickstart: Run Identity, Display Id, and Authored-Input Refactor

**Feature**: 009-run-id-display  
**Audience**: Independent reviewer validating the change against
[contracts/run-identity-contract.md](contracts/run-identity-contract.md).

This quickstart is install-first / binary-first. It assumes a built
`canon` binary on the operator's `PATH` (cargo-installed or release
package). For contributor work, substitute `cargo run --bin canon --`.

## 0. Setup

```sh
cd $(mktemp -d)
canon init
```

Verify `.canon/` exists and `canon-input/` is the editable authoring
surface.

## 1. Author an input and create a run (US1, US2, C-1, C-4, C-5)

```sh
mkdir -p canon-input
cat > canon-input/requirements.md <<'MD'
# Auth hardening scope
We need to bound the OAuth refresh-token rotation work for Q3.
MD

canon run requirements --system-context existing
```

**Expect** the CLI to print both an internal `uuid` and a display
`run_id` of the form `R-YYYYMMDD-XXXXXXXX`. Note the printed values.

**Verify on disk**:

```sh
ls -d .canon/runs/$(date -u +%Y)/$(date -u +%m)/R-*
```

The run directory MUST be named `R-YYYYMMDD-SHORTID` or
`R-YYYYMMDD-SHORTID--auth-hardening-scope` (slug derived from the H1).

```sh
cat .canon/runs/*/*/R-*/manifest.toml | head -20
```

The manifest MUST contain `uuid`, `run_id`, `short_id`, `created_at`,
`mode`, `owner`, `risk`, `zone`, and (when slug derived) `slug`.

## 2. Authored input is not mutated (US2, C-5)

```sh
sha256sum canon-input/requirements.md > /tmp/before.sha
canon status R-YYYYMMDD-XXXXXXXX                    # use the printed run_id
sha256sum canon-input/requirements.md > /tmp/after.sha
diff /tmp/before.sha /tmp/after.sha
```

**Expect** no diff. The authored file MUST be byte-identical.

```sh
ls .canon/runs/*/*/R-*/inputs/
```

**Expect** an immutable snapshot of `requirements.md` with a digest
recorded in the run's evidence.

## 3. Resolve by display id, short id, and UUID (US1, C-6)

```sh
canon status R-20260422-0190f4cf       # full run_id from step 1
canon status 0190f4cf                  # short id (must be unique)
canon status 0190f4cf-3a91-7a1c-9e8b-fa9203b1f0d4   # full uuid
```

All three MUST resolve to the same run.

## 4. Ambiguous short-id resolution (C-6)

Create a second run with a deliberately colliding short id (or simulate by
copying the manifest under a second directory in a fixture). Then:

```sh
canon status 0190f4cf
```

**Expect** a clear error that enumerates the matching `run_id`s and
exits non-zero. The CLI MUST NOT pick one silently.

## 5. List runs (US3, C-7)

```sh
canon list runs
```

**Expect** rows containing at least `run_id`, `mode`, `slug`/`title`,
`created_at`, and `state`, sorted by `created_at` descending. Both
new-layout and any legacy UUID-keyed runs MUST appear.

## 6. `@last` shortcut (US4, C-6)

```sh
canon status @last
```

**Expect** resolution to the most recent run by `created_at`.

```sh
( cd $(mktemp -d) && canon init && canon status @last )
```

**Expect** a clear, dedicated error in an empty repository.

## 7. Legacy compatibility (US1, C-8)

```sh
mkdir -p .canon/runs/0190f4cf-aaaa-7000-8000-000000000001
cp .canon/runs/$(date -u +%Y)/$(date -u +%m)/R-*/manifest.toml \
   .canon/runs/0190f4cf-aaaa-7000-8000-000000000001/manifest.toml
canon status 0190f4cf-aaaa-7000-8000-000000000001
canon list runs
```

**Expect** the legacy directory to be discoverable, listable, and
resolvable by UUID without any rewrite or relocation. After the
commands, the legacy directory MUST still exist at its original path.

## 8. Inspect, approve, resume across both layouts (FR-016)

For both a new-layout run and the legacy run from step 7:

```sh
canon inspect evidence  <run_id_or_uuid>
canon inspect artifacts <run_id_or_uuid>
canon approve           <run_id_or_uuid> --gate <pending-gate>
canon resume            <run_id_or_uuid>
```

**Expect** each command to function identically against both layouts.

## Sign-off

Reviewer confirms:

- [ ] All steps above executed successfully.
- [ ] Authored files under `canon-input/` were never modified.
- [ ] Snapshot files under `.canon/runs/<…>/inputs/` are byte-stable.
- [ ] Ambiguous short ids fail clearly.
- [ ] Legacy UUID-keyed runs are still readable.
- [ ] CLI help and `MODE_GUIDE.md` speak in `run_id` / `uuid` /
  `slug` terms and use the installed `canon` binary in daily-use
  examples.
