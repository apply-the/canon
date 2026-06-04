# Canon Modes

Modes dictate the structural requirements, required metadata, and expected evidence for packets. Canon supports a wide variety of modes out-of-the-box, ensuring you can govern virtually any type of engineering knowledge.

## Available Modes

Below is a list of the primary modes available in Canon, based on the default templates:

### Architecture & Domain
- **`brainstorming`**: Explore lateral thinking, evaluate multiple conceptual approaches, and map trade-offs.
- **`architecture`**: Capture structural decisions, architectural boundaries, and system-level trade-offs.
- **`domain-language`**: Define the ubiquitous language and terms used across the project to maintain semantic consistency.
- **`domain-model`**: Document entities, relationships, invariants, and boundaries for a specific bounded context.

### Delivery & Engineering
- **`requirements`**: Structure product or technical requirements before execution begins.
- **`backlog`**: Formulate backlog items with explicit acceptance criteria and lineage to requirements.
- **`discovery`**: Log investigative work, spikes, or feasibility studies.
- **`implementation`**: Record the implementation plan, logic design, or completed code structure.
- **`verification`**: Track test plans, coverage goals, and verification outcomes.

### Evolution & Maintenance
- **`change`**: Govern standard operational changes or configuration updates.
- **`refactor`**: Propose and document internal structural improvements without altering external behavior.
- **`migration`**: Track data, system, or library migrations, including rollback strategies.

### Quality & Review
- **`pr-review`**: Structure peer review or AI-assisted review findings for Pull Requests.
- **`review`**: Generic review artifacts for designs, documents, or processes.
- **`security-assessment`**: Document threat models, risk surfaces, and security review outcomes.
- **`system-assessment`**: Evaluate system health, performance, or compliance.
- **`supply-chain-analysis`**: Assess dependencies, licensing, and third-party risks.

### Operations
- **`incident`**: Capture incident reports, root cause analyses, and remediation actions.
- **`debugging`**: Systematic troubleshooting and root cause isolation with red-to-green verification.
- **`system-shaping`**: Govern broad, cross-cutting structural adjustments or organizational engineering alignments.

## Choosing a Mode

Pick a mode based on the *intent* of your work, not just the file you happen to have open. If you are defining terms, use `domain-language`. If you are planning how a feature will be built, use `implementation`.

Each mode defines its own required `evidence` and `readiness` criteria. You can run `canon init --mode <mode-name>` to generate a skeleton packet that contains the specific requirements for that mode.
