# Quickstart: Stronger Architecture Outputs (C4 Model)

This quickstart shows how to validate the new C4 artifact behavior end-to-end against a temporary repo.

## 1. Prepare the brief

Use the example as the authored brief:

```bash
mkdir -p canon-input/architecture
cp docs/examples/canon-input/architecture/brief.md canon-input/architecture/brief.md
```

## 2. Run architecture

```bash
canon init
canon run \
  --mode architecture \
  --system-context existing \
  --risk bounded-impact \
  --zone yellow \
  --owner staff-architect \
  --input canon-input/architecture/brief.md
```

Take the `RUN_ID` from the output.

## 3. Verify the artifact set

```bash
canon inspect artifacts --run <RUN_ID>
```

Expect 8 artifacts:

- architecture-decisions.md
- invariants.md
- tradeoff-matrix.md
- boundary-map.md
- readiness-assessment.md
- system-context.md
- container-view.md
- component-view.md

## 4. Verify authored-body preservation

```bash
cat .canon/artifacts/<RUN_ID>/architecture/system-context.md
cat .canon/artifacts/<RUN_ID>/architecture/container-view.md
cat .canon/artifacts/<RUN_ID>/architecture/component-view.md
```

Each file body MUST contain the verbatim text of the matching H2 section in the example brief and MUST NOT contain `## Missing Authored Body`.

## 5. Verify the missing-body path

```bash
sed -i '' '/^## System Context$/,/^## /{ /^## System Context$/d; /^## /!d; }' canon-input/architecture/brief.md
canon run \
  --mode architecture \
  --system-context existing \
  --risk bounded-impact \
  --zone yellow \
  --owner staff-architect \
  --input canon-input/architecture/brief.md
```

Take the new `RUN_ID` and verify:

```bash
grep -c '^## Missing Authored Body$' .canon/artifacts/<NEW_RUN_ID>/architecture/system-context.md
```

Expect at least one match.

## 6. Publish and review

```bash
canon publish <RUN_ID>
```

Open the published architecture packet from the publish destination and confirm that all 8 artifacts are present and that the C4 artifacts contain the authored content verbatim.
