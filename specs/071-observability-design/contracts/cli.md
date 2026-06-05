# CLI Contract: Observability Design

This document specifies the CLI command interface for the observability-design mode.

## Command Signature

```bash
canon observability-design <input-file> [OPTIONS]
```

## Arguments

- `<input-file>`: Path to the target architecture, domain-model, or feature-spec document.

## Options

- `--dry-run`: Evaluate the document and detect boundaries, but do not write output artifacts to disk.
- `--verbose`: Output reasoning steps during the LLM boundary inference phase.
- `--interactive`: Force the mode to ask for manual boundary confirmation, bypassing automatic inference.

## Outputs

When run without `--dry-run`, the command writes the following files to the current working directory or the feature's workspace:

- `telemetry-plan.md`
- `03-slo-alerts.md`
- `04-runbook.md`
- `05-instrumentation-checklist.md`

## Exit Codes

- `0`: Success, artifacts generated.
- `1`: Validation error (e.g., input file missing, bad format).
- `2`: Evaluation error (LLM failure or prompt rejection).
- `3`: User aborted during interactive boundary clarification.
