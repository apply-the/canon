# Decision Log

- **DECISION 1**: We will generate both `github-comments.json` and `review-findings.json`.
  - **Rationale**: `github-comments.json` maps directly to GitHub's REST API payload, whereas `review-findings.json` is a broader Canon governance concept mapping to the internal security/audit tracking.
- **DECISION 2**: When >20 files or >500 lines change, the system shifts into explicit sampling mode.
  - **Rationale**: Enforcing a hard threshold prevents the agent from entering a long loop of hallucination or context-window overflow.
