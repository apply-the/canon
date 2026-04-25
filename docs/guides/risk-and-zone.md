# Governance Guardrails: Risk and Zone

In Canon, `risk` and `zone` function as governance guardrails to constrain AI autonomy and ensure sensitive operations are properly supervised. They transform the AI from an autonomous executor capable of causing damage into a supervised tool.

## What They Mean

### Risk
Defines the potential "blast radius" or impact of a task.
- `low-impact`: The operation has a minimal footprint. Failsafes and rollbacks are trivial.
- `bounded-impact`: The operation affects a known, limited area of the system.
- `systemic-impact`: The operation touches core systems, widespread dependencies, or sensitive data.

### Zone
Indicates the environment or the operational confidence level of the action.
- `green`: Safe environment (e.g., local development, isolated sandbox).
- `yellow`: Cautious environment (e.g., staging, shared development).
- `red`: Critical environment (e.g., production, live databases).

## Practical Effects on AI Behavior

1. **Blocking Autonomous Execution (Human-In-The-Loop)**
   If an AI initiates an operation with `risk=systemic-impact` or `zone=red`, the Canon engine's background gates will trigger. The task transitions to an `AwaitingApproval` state. This physically prevents the AI from executing changes autonomously in a single step. The AI is forced to halt, generate descriptive artifacts, and request explicit validation and approval from a human (e.g., using the `canon-approve` skill).

2. **Requirements Gathering Constraint**
   The AI has strict instructions (e.g., within its `canon-incident` skill) never to invent or guess these parameters. If you do not declare them and they aren't obvious, the AI must ask you for them using predefined options to establish the restrictions upfront.

3. **Recommendation-Only Posture**
   For high-risk operational modes (such as `incident` or `architecture`), specific risk/zone combinations force the AI into a "recommendation-only" posture. Even if the AI discovers a command to remediate an incident, the policy bound to the risk level dictates that it can only produce markdown documentation and must refrain from mutating code or executing scripts.

By explicitly declaring the risk and zone, you establish the boundaries within which the AI is permitted to operate, ensuring all high-impact work is strictly governed and recorded.
