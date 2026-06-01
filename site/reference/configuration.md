# Configuration Reference

Canon operates via governed packets and standard repository conventions rather than a centralized, monolithic configuration file. Its "configuration" is defined through its modes and the structures it creates within the repository.

## Governing Context

Canon uses semantic governance to maintain architectural integrity. To configure Canon's behavior, you define the constraints and rules directly in the relevant domain models or instructions:

- **Mode Configuration:** Canon's CLI allows you to execute different modes (e.g., `pr-review`, `init`). The configuration for these modes is often derived from the target context and explicitly provided arguments.
- **Project Memory:** Instead of static JSON configurations, Canon treats your `docs/` and `specs/` directories as a living configuration base. AI systems read these structured documents to align with your project's Domain Language.
- **Evidence Requirements:** The rules for what constitutes acceptable evidence are embedded in the mode templates, meaning configuration is inherently tied to the workflow you select.

## Integration Adapters

Canon supports Governance Adapters to integrate with other tools (like Boundline or GitHub). Configuration for these adapters typically involves:
- Ensuring the adapter binary is in your PATH.
- Setting up environment variables for remote endpoints (if using hosted solutions).
- Defining the target repository and branches using standard Git workflows.

For detailed configuration of specific commands, see the [CLI Reference](/reference/cli).
