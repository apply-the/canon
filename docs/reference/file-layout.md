# File Layout

Canon governs knowledge by maintaining explicit, versioned artifacts within your repository. Unlike Boundline, which has a `.boundline` configuration folder, Canon interacts directly with your tracked repository structure to manage domain concepts and governance metadata.

## Core Directories

- `specs/`: Contains feature specifications, checklists, and plan artifacts generated and validated through Canon's governance loops.
- `tech-docs/`: Holds architectural decisions, diagrams, and semantic models forming the Project Memory.
- `assistant/`: Contains instructions, prompts, and context rules that guide AI assistants in adhering to project conventions.
- `distribution/`: Holds packaged assets or artifacts ready for deployment or external consumption.

## Configuration & Metadata

Canon relies heavily on explicit metadata and inline Frontmatter to determine the readiness of documents, evidence traces, and approval chains.

While Canon does not require a rigid `.canon/` dot-folder for execution state (as Boundline does), it requires strict adherence to semantic file structures within the repository to properly read and validate domain constraints.