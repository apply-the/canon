# Canon Rust Language Rules

These rules are normative for Rust code changes in this repository. They are
part of the repository's AI-visible governance surface and apply to both human
and AI-authored changes.

## No Panic Outside Entrypoints And Tests

- In Rust code outside `main.rs`, `#[cfg(test)]` modules, and files under
  `tests/`, do not introduce panic-prone control flow.
- Treat `unwrap()`, `expect()`, `panic!()`, `todo!()`, `unimplemented!()`,
  `unreachable!()`, and assert-family macros used as runtime guards as banned
  outside `main.rs` and test code.
- When a failure can arise from repository state, user input, IO, parsing,
  serialization, validation, or mode dispatch, surface it with explicit error
  propagation or a typed blocked/unsupported state.
- `main.rs` may still panic when immediate process termination is the intended
  behavior of the executable entrypoint, but explicit exit handling remains
  preferred when practical.
- `#[cfg(test)]` code and files under `tests/` may use panicking helpers
  freely.

## No Magic Literals In Owned Logic

- In Rust code outside `main.rs`, `#[cfg(test)]` modules, and files under
  `tests/`, do not introduce magic strings or magic numbers in repository
  logic, mode dispatch, protocol handling, persistence, configuration, CLI
  contracts, or serialization paths.
- Promote reusable literals, wire-format keys, status strings, exit codes,
  schema versions, filenames, and sentinel numeric values into named `const`
  items or typed enums/newtypes scoped to the owning module or type.
- Prefer typed wrappers or enums when the literal represents a constrained
  domain value rather than a generic scalar.

## Typed Serialization For Stable Shapes

- When a serialized or deserialized shape is stable, model it with typed
  `struct` or `enum` definitions plus `serde` derives rather than assembling
  `serde_json::Map`, relying on repeated raw key strings, or constructing
  stable payloads with ad hoc `json!` objects.
- Use map- or value-oriented assembly only when the payload shape is genuinely
  dynamic or partially open-ended, and keep the dynamic boundary explicit in
  the owning type or function.

## Allowed Non-Panicking Helpers

- This rule does not ban non-panicking combinators such as
  `unwrap_or_default`, `unwrap_or_else`, `unwrap_or`, or `Option`/`Result`
  handling that returns explicit errors instead of panicking.

## Review Expectation

- Reviewers and implementers should treat newly introduced panic-prone calls
  outside `main.rs` and test code as policy violations.
- Reviewers and implementers should treat newly introduced magic literals or
  stable-shape ad hoc map/json serialization outside `main.rs` and test code
  as policy violations.

## Clean Code & Modularity

- **NO GIGANTIC FILES**: Do not dump all logic into a single massive file. If a module grows complex, extract helpers, algorithms, and state transitions into private submodules (`pub(crate)`).
- **APPLY DESIGN PATTERNS**: Do not use monolithic match statements or procedural god-functions. Extract responsibilities using appropriate design patterns (e.g. Builder, Strategy, Dependency Injection). Keep business logic strictly isolated from I/O and HTTP/CLI transport boundaries.
- **ZERO MAGIC STRINGS/NUMBERS**: You MUST NOT use magic strings or magic numbers in domain logic, protocol handling, persistence, configuration, CLI contracts, timeouts, retry limits, or serialization paths. Extract them into named `const` items or typed `enum`s/newtypes owned by the relevant module or type.
- **EXTRACT HELPERS PROACTIVELY**: Aim for <50 lines per function. If you need a comment to explain the middle of a function, extract that block into a well-named helper function.
- **NO DEAD CODE**: Remove all commented-out code, unused variables, and unreachable branches immediately. `git` remembers.
- **WHY NOT WHAT**: Documentation and comments must explain the *why*, business constraints, and invariants, not narrate the *what*.
- **COMPREHENSIVE DOCUMENTATION**: Every folder/module MUST have a module-level doc comment (e.g. `//!` in `mod.rs` or `<module_name>.rs`) explaining its purpose, and these docs must be kept up to date. Furthermore, all structs, public functions, enums, and constants MUST have clear and up-to-date doc comments (`///`).
- **LOGGING & OUTPUT BOUNDARIES**: Log at major state-transition decision points using structured `tracing` spans/events. Always include reproducible context (IDs) but NEVER log secrets, tokens, or PII. Maintain strict separation between presentation and core logic: use `println!` or `eprintln!` ONLY in presentation layers (e.g., `cli.rs`, `init.rs`). For orchestrator, core logic, and adapters, NEVER use `println!`. User-facing messages must be propagated up to the CLI layer via return values (e.g., `Result<T, Error>`).
- **CONCURRENCY**: Avoid `Arc<Mutex<T>>` lock-contention. Prefer message-passing (channels) or immutable data snapshots to share state across async boundaries.