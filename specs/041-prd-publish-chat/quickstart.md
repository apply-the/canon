# Quickstart: Requirements PRD Publishing And Chat Publish Skill

## Scenario 1: Publish a completed requirements run to the default destination

1. Create a complete `canon-input/requirements.md` brief.
2. Run Canon in `requirements` mode until the run reaches `Completed`.
3. Publish the run:

```bash
canon publish <RUN_ID>
```

4. Confirm the destination under `specs/<date>-requirements/` contains:

```text
problem-statement.md
constraints.md
options.md
tradeoffs.md
scope-cuts.md
decision-checklist.md
prd.md
packet-metadata.json
```

## Scenario 2: Publish to a custom destination

```bash
canon publish <RUN_ID> --to docs/public/prd
```

Confirm `docs/public/prd/prd.md` exists alongside the sectional packet files and metadata.

## Scenario 3: Use chat-first publish guidance

1. Initialize the repo with Copilot or Codex skills.
2. Ask the assistant to publish the completed run.
3. Confirm the available publish skill guidance routes to the CLI command and explains:
   - the required run id,
   - the default publish destination,
   - the optional `--to` override,
   - and the fact that incomplete or gated runs still fail to publish.

## Validation Commands

```bash
cargo test --test requirements_authoring_renderer
cargo test -p canon-cli publish
/bin/bash scripts/validate-canon-skills.sh
```