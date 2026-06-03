# Quickstart: Systematic Debugging Mode

## Triggering the workflow

To start a debugging session via Canon:

```bash
canon --mode debugging <path/to/bug-report.md>
```

This ensures the rigid packet gates (Reproduction Gate, TDD Gate, Root Cause Analysis) are activated.

## Workflow Overview

1. Supply a symptom or bug report. The mode will propose 2-3 isolated failure hypotheses.
2. Write/generate a minimal failing test to reproduce the bug. The system will record the `FAIL` evidence (Red State).
3. Apply the fix. The fix must be explicitly linked to the identified root cause.
4. The system will verify the fix by checking the `PASS` evidence (Green State) and verifying there are no regressions.
