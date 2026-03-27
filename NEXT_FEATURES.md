# Next Features

## Distribution Without Cargo

There is a sensible path to distributing Canon without forcing end users to
install Cargo, and it should not start with `apt-get`.

### Recommendation

- Canonical distribution source: GitHub Releases with prebuilt binaries for
  macOS, Linux, and Windows.
- Release automation: `cargo-dist`, because it supports prebuilt binaries,
  installer scripts for macOS/Linux, PowerShell installers for Windows, and
  Homebrew publishing.
- macOS and Linux: Homebrew tap.
- Windows: `winget`, with optional Scoop support as a secondary channel.
- `apt-get`: defer to a later phase, once there is a real Debian repository or
  PPA worth maintaining.

### First Iteration

- Use `cargo-dist` to generate release assets and installer scripts.
- Use Homebrew as the first package-manager channel for macOS and Linux.
- Use `winget` as the primary Windows distribution channel.
- Use Scoop as a secondary Windows channel.
- Do not include `apt` in the first iteration.

### Why This Order

- GitHub Releases should be the canonical source of truth for downloadable
  binaries.
- `cargo-dist` reduces release plumbing and already fits the cross-platform
  packaging problem Canon has.
- Homebrew gives a good install story on macOS and is also viable on Linux.
- `winget` is the most credible first-class Windows package-manager target.
- Scoop is useful, but secondary.
- Debian packaging introduces repository maintenance overhead too early if
  there is not yet a stable release and publishing process.

### Next Step Options

- Prepare a concrete technical plan for distributing Canon without Cargo.
- Start wiring a release pipeline with GitHub Releases, `cargo-dist`,
  Homebrew, and `winget`.
