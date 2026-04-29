# Quickstart: Distribution Channels Beyond GitHub Releases

## 1. Prepare The Canonical Release Bundle

Assemble the normal release bundle in `dist/` using the existing release
workflow outputs for version `0.25.0`.

Expected bundle inputs:

- the canonical platform archives
- `canon-0.25.0-SHA256SUMS.txt`
- `release-notes.md`

## 2. Verify The Release Surface And Emit Distribution Metadata

```bash
VERSION=0.25.0

bash scripts/release/verify-release-surface.sh \
  --version "$VERSION" \
  --dist-dir dist \
  --release-notes dist/release-notes.md \
  --write-checksums

bash scripts/release/write-distribution-metadata.sh \
  --version "$VERSION" \
  --dist-dir dist \
  --output "dist/canon-${VERSION}-distribution-metadata.json"
```

Expected outcome: `dist/canon-0.25.0-distribution-metadata.json` exists and
describes the verified archives and checksums.

## 3. Render The Homebrew Formula Artifact

```bash
bash scripts/release/render-homebrew-formula.sh \
  --metadata "dist/canon-${VERSION}-distribution-metadata.json" \
  --output "dist/canon-${VERSION}-homebrew-formula.rb"
```

Expected outcome: `dist/canon-0.25.0-homebrew-formula.rb` contains the
platform-conditional URLs and checksums for the supported Homebrew targets.

## 4. Publish Or Export The Tap Update

If tap publication is configured:

```bash
bash scripts/release/sync-homebrew-tap.sh \
  --formula "dist/canon-${VERSION}-homebrew-formula.rb" \
  --repo "$HOMEBREW_TAP_REPO"
```

If tap publication is not configured, retain the rendered formula artifact as
the ready-to-apply tap update.

## 5. Smoke-Test The Install Path

```bash
brew install --formula "./dist/canon-${VERSION}-homebrew-formula.rb"
canon init
```

Expected outcome:

- Homebrew installs Canon from the canonical release archive.
- `canon init` succeeds in the temporary test location.
- Manual archive-based installation remains documented as a fallback path.