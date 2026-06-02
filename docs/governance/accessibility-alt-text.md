# Accessibility and Alt Text

Canon doesn't just govern system architecture and security; it also enforces strict user-experience quality gates. One of the most critical aspects of this is **Accessibility (a11y)**.

## Governance Rules for UI Artifacts

When AI agents generate frontend code, documentation, or design artifacts, Canon can apply specific compliance heuristics before allowing the session to transition.

If an agent proposes HTML, Markdown, or UI component changes, Canon checks for:
- **Semantic Structure**: Proper heading hierarchies and native HTML elements over `div` soups.
- **Alt Text**: Every image, diagram, or multimedia element *must* include descriptive alternative text.
- **ARIA Compliance**: Interactive elements must expose the correct roles and states.

## The Quality Gate

If an agent submits a UI plan that violates these rules (e.g., embedding an image without an `alt` attribute, or creating a button without a focus state), the Guardian checks will fail. 

The session will be blocked, and the agent will be forced into a refinement loop to correct the accessibility violations before the code is ever committed to the repository.

This ensures that accessibility is not an afterthought handled in code review, but a strict, non-negotiable runtime constraint.