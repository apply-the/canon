# Data Model: Brainstorming Ideation Mode

## Entities

### `OptionMap`
Represents the primary output artifact of the brainstorming mode.
- **Fields**:
  - `problem_statement`: String
  - `options`: List of `ConceptualApproach`
  - `recommended_next_mode`: String
- **Relationships**: Contains `ConceptualApproach`

### `ConceptualApproach`
A distinct conceptual approach to solving the problem.
- **Fields**:
  - `title`: String
  - `description`: String
  - `trade_offs`: `TradeOffMatrix`

### `TradeOffMatrix`
Structured evaluation of an approach.
- **Fields**:
  - `pros`: List of String
  - `cons`: List of String
  - `unknowns`: List of String
- **Relationships**: Part of `ConceptualApproach`

### `SpikeProposal`
A minimal experiment scope to validate hypotheses when critical unknowns exist.
- **Fields**:
  - `related_option`: String (reference to a `ConceptualApproach`)
  - `hypothesis`: String
  - `experiment_scope`: String
  - `success_criteria`: String
