# Canon Copilot Command And Prompt Pack

Copilot support is documented as a command and prompt pack. This repository does not claim a stable Copilot plugin manifest shape for Canon.

Use these prompts with the Canon CLI installed in the target repository:

| Task | Prompt | Canon Surface |
|------|--------|---------------|
| Clarify input | Clarify this Canon packet input before starting a run. | `canon inspect clarity --mode <MODE> --input <INPUT_PATH>` |
| Start governed packet | Start a governed Canon packet from this authored brief. | `canon run --mode <MODE> --risk <RISK> --zone <ZONE> --owner <OWNER> --input <INPUT_PATH>` |
| Inspect status | Inspect Canon packet status for this run. | `canon status --run <RUN_ID>` |
| Inspect evidence | Inspect Canon evidence for this run. | `canon inspect evidence --run <RUN_ID>` |
| Review packet | Review this packet for unsupported claims and missing evidence. | `canon run --mode review --risk <RISK> --zone <ZONE> --owner <OWNER> --input <INPUT_PATH>` |
| Verify claims | Verify these claims against recorded evidence. | `canon run --mode verification --risk <RISK> --zone <ZONE> --owner <OWNER> --input <INPUT_PATH>` |
| Publish packet | Publish this Canon packet after readiness is established. | `canon publish --run <RUN_ID>` |

Canon CLI and the governance adapter remain authoritative for packet behavior, run state, evidence, approvals, and provenance. Copilot prompts should reference Canon-owned skills and method guidance when those files are present in the repository; they should not invent run ids, approvals, or evidence.
