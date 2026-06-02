# Quickstart: Mode Clarification And Run Refinement

**Branch**: `062-clarify-run-refinement`

This walkthrough is the intended post-implementation validation path for the
feature.

## Scenario 1: Targeted Mode Refinement Reuses the Same Run Identity

1. Prepare a requirements brief with at least one material gap.

```text
canon-input/requirements/brief.md
```

2. Confirm the authored-input gap is visible before run start.

```bash
canon inspect clarity \
  --mode requirements \
  --input canon-input/requirements/brief.md \
  --output json
```

Expected result: the response reports a non-empty readiness delta or
clarification questions.

3. Start the run and capture the returned `run_id`.

```bash
canon run \
  --mode requirements \
  --risk bounded-impact \
  --zone green \
  --owner "Planner <planner@example.com>" \
  --input canon-input/requirements/brief.md \
  --output json
```

4. Provide an explicit continuation signal for the same work item, either via
assistant-host language such as `continue this run` or via the explicit CLI
path:

```bash
canon resume --run <RUN_ID>
```

5. Inspect the same run after clarification updates.

```bash
canon status --run <RUN_ID> --output json
canon inspect refinement --run <RUN_ID> --output json
```

Expected result:
- `run_id` remains unchanged through draft clarification and run start.
- `refinement_state.working_brief_path` points under `.canon/runs/<RUN_ID>/artifacts/`.
- `clarification_records` and `readiness_delta` reflect the newly captured
  answers or defaults.

## Scenario 2: Single Candidate Detection Is Advisory Only

1. Create one likely continuation candidate using Scenario 1.
2. Submit a fresh request without explicit continuation language.

Assistant-host example:

```text
Start a new requirements run for the mobile onboarding audit.
```

3. Check the old and new runs.

```bash
canon status --run <OLD_RUN_ID> --output json
canon status --run <NEW_RUN_ID> --output json
```

Expected result:
- Canon may surface `suggested_continuation` for the older run.
- Canon does not mutate the old run without explicit continuation intent.
- A fresh request results in a new `run_id`.

## Scenario 3: Pre-Start Mode Correction Stays In Place

1. Start a draft refinement in one targeted mode.
2. During clarification, redirect the work to a different targeted mode before
   execution begins.
3. Inspect the same run.

```bash
canon inspect refinement --run <RUN_ID> --output json
```

Expected result:
- The same `run_id` is preserved.
- `current_mode` changes in the refinement context.
- The working brief path updates to the corrected mode location.

## Scenario 4: Post-Start Mode Correction Creates a Successor

1. Start a run and let it enter a started governed state.
2. Surface clarification or evidence that shows the correct mode is different.
3. Inspect the original and successor runs.

```bash
canon inspect refinement --run <ORIGINAL_RUN_ID> --output json
canon inspect refinement --run <SUCCESSOR_RUN_ID> --output json
canon status --run <ORIGINAL_RUN_ID> --output json
canon status --run <SUCCESSOR_RUN_ID> --output json
```

Expected result:
- The original run remains inspectable under its original mode and state.
- The successor run contains `lineage.carried_from` and
  `lineage.supersedes` pointing to the original run.
- The successor reuses carried-forward working-brief and clarification state.

## Scenario 5: Non-Targeted Modes Preserve Identity Continuity Without Full Working-Brief Lifecycle

1. Start a non-targeted run such as `review` or `verification`.
2. Provide follow-up context with an explicit continuation signal.
3. Inspect the existing run.

```bash
canon status --run <RUN_ID> --output json
```

Expected result:
- The run preserves identity continuity for the same work.
- Canon does not claim a targeted-mode working brief exists where the feature
  does not provide one yet.
- Ambiguous continuation still requires disambiguation before mutation.

## Release Closeout Expectations

After runtime validation passes, release closeout must also record:

- repo-facing release notes and operator guidance updates in `CHANGELOG.md`,
  `README.md`, `tech-docs/guides/modes.md`, and
  `defaults/templates/canon-input/README.md`
- wiki alignment updates in `../canon.wiki/Home.md`,
  `../canon.wiki/Canon-Modes.md`, and
  `../canon.wiki/Lineage-And-Provenance.md`
- preservation review evidence showing that publish destinations, artifact
  families, and source-input honesty markers remain unchanged unless the final
  implementation explicitly adds a refinement-facing surface