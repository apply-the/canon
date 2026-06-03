# Data Model: Systematic Debugging Mode

The system relies on file-based payloads within the `.canon/` execution workspace rather than traditional relational schemas.

## Entities

### Debugging Packet

A debugging packet consists of the following structure:
- `01-context.md`: Contains symptom, affected surface, suspected blast radius.
- `02-reproduction.md`: Exact reproduction steps, failing commands, observed evidence.
- `03-root-cause.md`: Traced source of failure and rejected hypotheses.
- `04-fix-decision.md`: Bounded fix, tradeoffs, and why adjacent changes were not taken.
- `05-verification.md`: Red/green proof plus any remaining uncertainty.

### Execution Evidence
- `debugging-trace.md`: Documentation of hypotheses, reproduction steps, and precise fix rationale to be included in the final change packet.

## State Transitions
1. **Hypothesis Generation**: Evaluates bug report and proposes 2-3 isolated failure hypotheses.
2. **Reproduction Harness (Red State)**: Minimal failing test generated, `FAIL` evidence recorded.
3. **Isolation & Fix (Green State)**: Fix is applied exclusively to code matching root cause.
4. **Verification**: `PASS` evidence recorded alongside regression suite confirmation.
