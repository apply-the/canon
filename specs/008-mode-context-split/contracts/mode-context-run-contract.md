# CLI Contract: Mode And System Context Runs

## Public Mode Catalog

- Supported public mode values remain work types only.
- `change` replaces `brownfield-change`.
- `brownfield-change`, `brownfield`, and `greenfield` are rejected as public run-start vocabulary.

## Run Command Shape

```bash
canon run --mode <MODE> --risk <RISK> --zone <ZONE> [--system-context <new|existing>] [--owner <OWNER>] (--input <INPUT_PATH> | --input-text <INPUT_TEXT>)
```

## `--system-context` Matrix

| Mode | `--system-context` | Notes |
|------|--------------------|-------|
| `system-shaping` | required | applies to both `new` and `existing` systems |
| `architecture` | required | applies to both `new` and `existing` systems |
| `change` | required | `existing` only in the first release |
| `implementation` | required | modeled mode must enforce context even before full runtime depth |
| `refactor` | required | modeled mode must enforce context even before full runtime depth |
| `migration` | required | modeled mode must enforce context even before full runtime depth |
| `incident` | required | modeled mode must enforce context even before full runtime depth |
| `discovery` | optional | omission persists no context value |
| `requirements` | optional | omission persists no context value |
| `review` | optional | omission persists no context value |
| `verification` | optional | omission persists no context value |
| `pr-review` | optional | omission persists no context value |

## Change Mode Contract

- Canonical authored input is `canon-input/change.md` or `canon-input/change/`.
- `canon run --mode change --system-context existing ...` preserves the previous bounded-change workflow.
- `canon run --mode change --system-context new ...` fails before run creation with guidance explaining that `change` is currently defined only for existing systems.
- Emitted artifacts live under `.canon/artifacts/<RUN_ID>/change/`.

## Persistence And Inspection Contract

- `.canon/runs/<RUN_ID>/context.toml` includes `system_context` whenever the caller supplied one.
- Optional-context runs may omit `system_context`; Canon must not write a placeholder or implied default.
- `canon status --run <RUN_ID>`, `canon inspect artifacts --run <RUN_ID>`, and `canon inspect evidence --run <RUN_ID>` expose the renamed `change` namespace and the explicit context value when present.

## Failure Contract

- Missing required `--system-context` is rejected before run creation.
- Legacy public mode names are rejected with corrective guidance to use `change` and, where appropriate, `--system-context existing`.
- Legacy canonical input hints no longer point to `canon-input/brownfield-change.md` or `canon-input/brownfield-change/`.