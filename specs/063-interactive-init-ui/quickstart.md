# Quickstart: Interactive Init Experience

## Prerequisites

- Build the CLI binary from the repository root:

```bash
cargo build -p canon-cli
```

- Use a temporary workspace for validation. Do not run `canon init` against the
  repository root.

## 1. Create an isolated workspace

```bash
WORKDIR="$(mktemp -d)"
CANON_BIN="$PWD/target/debug/canon"
printf 'Workspace: %s\n' "$WORKDIR"
```

## 2. Validate the default guided flow

```bash
cd "$WORKDIR"
"$CANON_BIN" init
```

Expected behavior:

- A full-screen guided init UI opens when the terminal supports the required
  interactive capabilities and the current layout fits.
- Arrow keys change the assistant selection.
- The first `Enter` confirms the current selection and opens the confirmation
  step.
- A second `Enter` on the confirmation step starts initialization.
- After success, the terminal returns to its normal state and `.canon/` exists
  inside the temp workspace.

## 3. Validate guided preselection

```bash
cd "$WORKDIR"
"$CANON_BIN" init --ai copilot
```

Expected behavior:

- The guided UI opens with `copilot` preselected.
- Confirming the flow still produces the normal `.canon/` scaffolding after
  the confirmation step.

## 4. Validate the non-interactive compatibility path

```bash
WORKDIR_NON_INTERACTIVE="$(mktemp -d)"
cd "$WORKDIR_NON_INTERACTIVE"
"$CANON_BIN" init --non-interactive --output json
```

Expected behavior:

- The command does not open the guided UI.
- The command prints the existing machine-readable summary.
- `.canon/` is created in the temp workspace.

## 5. Validate structured-output rejection in guided mode

```bash
WORKDIR_REJECT="$(mktemp -d)"
cd "$WORKDIR_REJECT"
"$CANON_BIN" init --output json
```

Expected behavior:

- The command rejects the request before opening the guided UI.
- The error tells the caller to use `--non-interactive` for structured output.
- `.canon/` is not created.

## 6. Validate interruption handling

```bash
WORKDIR_INTERRUPT="$(mktemp -d)"
cd "$WORKDIR_INTERRUPT"
"$CANON_BIN" init
```

Then press `Ctrl+C` before confirming initialization.

Expected behavior:

- The guided UI exits cleanly.
- The terminal is restored without requiring manual reset.
- `.canon/` is not created when interruption happens before initialization.

## 7. Validate too-small terminal handling

Run the default guided flow in a terminal resized smaller than the branded
layout requires.

Expected behavior:

- The command fails before opening the full-screen UI.
- The error explains that the current terminal layout does not fit.
- No `.canon/` side effects are created.

## 8. Validate parent Canon workspace binding

```bash
WORKDIR_SHARED="$(mktemp -d)"
mkdir -p "$WORKDIR_SHARED/repo-a"
cd "$WORKDIR_SHARED/repo-a"
git init
"$CANON_BIN" init --non-interactive --repo-root "$WORKDIR_SHARED/repo-a" --canon-root "$WORKDIR_SHARED"
```

Expected behavior:

- `.canon/` is created under `$WORKDIR_SHARED`, not inside `repo-a`.
- Follow-on Canon commands can target `repo-a` while reusing the shared parent runtime via `--repo-root` and `--canon-root`.
- Git-scoped behavior still resolves against `repo-a` rather than the parent workspace.