# Quickstart: Supply Chain And Legacy Analysis Mode

## 1. Author The Canonical Input

Create `canon-input/supply-chain-analysis.md` with the bounded repository scope,
licensing posture, distribution model, ecosystems in scope, excluded paths,
scanner policy, and the initial triage framing for vulnerabilities, licenses,
and legacy posture.

## 2. Run The Mode

Example bounded run:

```bash
canon run \
  --mode supply-chain-analysis \
  --system-context existing \
  --risk bounded-impact \
  --zone yellow \
  --owner "supply-chain-owner" \
  --input canon-input/supply-chain-analysis.md
```

Expected outcomes for a successful first-slice run:

- Canon persists the packet under `.canon/artifacts/<RUN_ID>/supply-chain-analysis/`
- The packet exposes recommendation-only posture
- Missing scanners or unresolved posture inputs surface explicit coverage or
  decision gaps rather than fabricated clean results

## 3. Review And Approve If Needed

If the run is high risk or red zone, review the packet and provide explicit
risk approval using the normal governed approval flow before publishing.

## 4. Publish The Packet

```bash
canon publish <RUN_ID>
```

Published packets land under `docs/supply-chain/<RUN_ID>/`.

## 5. Validate The Feature Closeout

Before feature completion, capture evidence for:

- focused supply-chain contract, renderer, runtime, and publish tests
- shared skill validation
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- high line coverage evidence for every Rust file added or modified by the
  feature