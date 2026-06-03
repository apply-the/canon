# Canon

> [!TIP]
> This wiki is aligned with **Canon 0.65.0**. For older versions, refer to the repository tags.

![Canon - Semantic Governance Runtime](https://github.com/apply-the/canon/blob/0.65.0/tech-docs/images/canon-banner.jpg?raw=true)


**The governance runtime for AI-assisted engineering.** Keep AI agents bounded, inspectable, and safely restricted to approved work zones.

## <i class="fa-solid fa-rocket"></i> Why Canon?

- <i class="fa-solid fa-ban"></i> **No Opaque Loops:** You control exactly when agents plan, run, and publish.
- <i class="fa-solid fa-shield-halved"></i> **Bounded Execution:** Agents operate strictly within approved risk and zone limits.
- <i class="fa-solid fa-magnifying-glass"></i> **Inspectable State:** Every decision, approval, and output is captured as durable evidence.
- <i class="fa-solid fa-book-open"></i> **Governed Packets:** Turn unstructured chat into canonical, versioned markdown artifacts.

## <i class="fa-solid fa-brain"></i> How it Works

Canon operates on a simple, predictable four-step mental model:
1. `init` → Prepare the workspace.
2. `run` → Start a governed session with explicit boundaries.
3. `approve` → Review and unblock the agent when human judgment is needed.
4. `publish` → Promote the final artifacts into your repository's permanent memory.

## <i class="fa-solid fa-bolt"></i> Quick Start

Get your first governed session running in seconds:

```bash
brew tap apply-the/canon && brew install canon
cd my-project
canon init
canon run --mode requirements --risk bounded-impact
```

## <i class="fa-solid fa-hammer"></i> Key Commands

These are the commands you'll actually use every day:

| Command | What it does |
|---|---|
| `canon run` | Start a new governed session with explicit boundaries. |
| `canon status` | See exactly what the agent is doing right now. |
| `canon inspect` | Review generated evidence and artifacts. |
| `canon approve` | Unblock a session that hit a governance gate. |
| `canon publish` | Commit the final work into your repository. |

## <i class="fa-solid fa-book"></i> Deep Dive Documentation

Explore the wiki sidebar for advanced integrations, semantics, and architecture:
- [Getting Started Guide](Getting-Started)
- [Governance Modes](Canon-Modes)
- [Evidence and Approvals](Evidence-And-Approvals)
- [Machine-Facing Governance Adapter](Boundline-Integration)
