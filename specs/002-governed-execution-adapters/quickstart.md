# Quickstart: Governed Execution Adapters

## Scope

This quickstart covers the delivered governed-execution slices for this
increment: `requirements`, `brownfield-change`, and `pr-review`.

- `requirements` is governed end to end
- `brownfield-change` is governed end to end with bounded repository analysis
  and independent validation evidence
- `pr-review` is governed end to end with diff inspection, critique evidence,
  and retained review payload references

## 1. Initialize Canon

```bash
canon init
```

Expected result:

- `.canon/` exists
- methods and policies are materialized

## 2. Create a bounded requirements input

```bash
cat > idea.md <<'EOF'
# Idea

Define requirements for a bounded internal CLI without letting execution drift.
EOF
```

## 3. Run a bounded governed `requirements` flow

```bash
canon run \
  --mode requirements \
  --risk bounded-impact \
  --zone yellow \
  --owner product-lead \
  --input idea.md \
  --output json
```

Expected behavior:

- Canon resolves mode, risk, zone, and owner before any governed invocation is
  evaluated
- repository context capture, generation, critique, and denied workspace edit
  requests are all persisted as governed invocations
- run output contains invocation counts and an `evidence_bundle` reference

## 4. Inspect governed invocation evidence

```bash
canon inspect invocations --run <RUN_ID> --output json
canon inspect evidence --run <RUN_ID> --output json
```

Expected behavior:

- `inspect invocations` shows request id, adapter, capability, policy decision,
  approval state, latest outcome, and linked artifacts or decisions
- `inspect evidence` shows generation paths, validation paths, denied
  invocation refs, approval refs, decision refs, and artifact provenance links

## 5. Inspect the evidence-derived artifact bundle

```bash
canon inspect artifacts --run <RUN_ID> --output json
```

Expected behavior:

- requirements artifacts exist under `.canon/artifacts/<RUN_ID>/requirements/`
- artifact provenance links point back to governed invocation request ids and
  the run evidence bundle

## 6. Exercise the approval-gated path

Run the same flow with systemic impact:

```bash
canon run \
  --mode requirements \
  --risk systemic-impact \
  --zone yellow \
  --owner eng-lead \
  --input idea.md \
  --output json
```

Expected behavior:

- the run enters `AwaitingApproval`
- the pending generation request is persisted under
  `.canon/runs/<RUN_ID>/invocations/`
- the approval target is invocation-scoped

Approve and resume:

```bash
canon approve \
  --run <RUN_ID> \
  --target invocation:<REQUEST_ID> \
  --decision approve \
  --by eng-lead \
  --rationale "Allow bounded governed generation for this run."

canon resume --run <RUN_ID>
```

Expected behavior:

- approval attaches to the governed request, not only to a broad gate
- `resume` re-checks context freshness before dispatch
- the resumed run completes under the same `RUN_ID`

## 7. Run a governed `brownfield-change` flow

```bash
canon run \
  --mode brownfield-change \
  --risk bounded-impact \
  --zone yellow \
  --owner eng-lead \
  --input brownfield.md \
  --output json
```

Expected behavior:

- repository-context capture, bounded generation, validation, and
  recommendation-only mutation posture are all persisted as governed
  invocations
- the run emits `.canon/runs/<RUN_ID>/evidence.toml`
- `inspect evidence` shows a non-generative validation path challenging the
  generated framing

## 8. Run a governed `pr-review` flow

```bash
canon run \
  --mode pr-review \
  --risk bounded-impact \
  --zone yellow \
  --owner reviewer \
  --input refs/heads/main \
  --input HEAD \
  --output json
```

Expected behavior:

- diff inspection and critique are both persisted as governed invocations
- `.canon/runs/<RUN_ID>/invocations/<REQUEST_ID>/payload/` retains bounded diff
  payload files when policy allows it
- review artifacts under `.canon/artifacts/<RUN_ID>/pr-review/` link back to
  the evidence bundle and invocation request ids
