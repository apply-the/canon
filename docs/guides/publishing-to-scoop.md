# Publishing Canon to Scoop

This guide is for maintainers who need to publish Canon to Scoop after a release
is prepared.

GitHub Releases remains the canonical distribution surface. The Scoop manifest
is derived from the verified release bundle, validated locally, and then
submitted manually to the Scoop main bucket.

The preferred path is to let CI generate the manifest from a release tag. Local
script execution is mainly for dry runs, troubleshooting, or reproducing the CI
output before opening the Scoop bucket pull request.

## Preferred Flow: Tag-Driven CI

Canon ties Scoop artifact generation to the release workflow in CI.

- `.github/workflows/release.yml` runs automatically on `push` for tags that
  match `v*`.
- That workflow builds the platform archives, renders `release-notes.md`,
  generates checksums, writes `canon-<VERSION>-distribution-metadata.json`,
  renders `canon-<VERSION>-scoop-manifest.json`, and verifies the extended
  release surface.
- On a tag-triggered run, the workflow publishes the GitHub Release and uploads
  the generated Scoop manifest as a release asset.

That means the correct operational sequence is:

1. Bump the workspace version and merge the release-ready change.
2. Create and push the Git tag `v<VERSION>`.
3. Let CI build, verify, and publish the release bundle plus the generated
   Scoop manifest.
4. Use that generated manifest as the source for the manual Scoop submission.

If you want the same workflow without publishing a GitHub Release yet, run the
workflow manually with `workflow_dispatch` and leave `publish=false`.

## What This Guide Assumes

- You already have a release version such as `0.34.0`.
- The GitHub release tag will be `v0.34.0`.
- A local `dist/` directory exists with the release archives, checksum file,
  and release notes.
- `jq`, `tar`, `zip`, `unzip`, and `shasum` are available in your shell.

The scripts in this repository expect the raw semantic version like `0.34.0`,
not the Git tag form with the `v` prefix.

## Required Release Files

Before generating the Scoop manifest, make sure `dist/` contains:

- `canon-<VERSION>-macos-arm64.tar.gz`
- `canon-<VERSION>-macos-x86_64.tar.gz`
- `canon-<VERSION>-linux-arm64.tar.gz`
- `canon-<VERSION>-linux-x86_64.tar.gz`
- `canon-<VERSION>-windows-x86_64.zip`
- `canon-<VERSION>-SHA256SUMS.txt`
- `release-notes.md`
- `*.version.txt` evidence files for every archive

The Windows zip is the installation payload for both `winget` and Scoop. Scoop
uses the same canonical release URL and SHA256 checksum rather than a separate
installer build path.

## 1. Stage the Release Bundle

Prefer the tagged CI workflow above. For local reproduction, either download the
generated release bundle from CI or collect the generated release artifacts into
`dist/`.

If you want to check that the expected files are present before generating
metadata:

```bash
VERSION=0.34.0
DIST_DIR=dist

ls "$DIST_DIR"/canon-"$VERSION"-*.tar.gz
ls "$DIST_DIR"/canon-"$VERSION"-windows-x86_64.zip
ls "$DIST_DIR"/canon-"$VERSION"-SHA256SUMS.txt
ls "$DIST_DIR"/release-notes.md
```

## 2. Generate Distribution Metadata

Generate the machine-readable release description from the canonical release
bundle:

```bash
VERSION=0.34.0
DIST_DIR=dist

bash scripts/release/write-distribution-metadata.sh \
  --version "$VERSION" \
  --dist-dir "$DIST_DIR" \
  --output "$DIST_DIR/canon-$VERSION-distribution-metadata.json"
```

Expected output:

- `dist/canon-<VERSION>-distribution-metadata.json`

This file binds the Windows artifact URL and checksum directly from the release
bundle so the manifest does not need hand-copied values.

## 3. Render the Scoop Manifest

Render the repository-owned Scoop manifest from the distribution metadata:

```bash
bash scripts/release/render-scoop-manifest.sh \
  --metadata "$DIST_DIR/canon-$VERSION-distribution-metadata.json" \
  --output "$DIST_DIR/canon-$VERSION-scoop-manifest.json"
```

Expected output file:

- `dist/canon-<VERSION>-scoop-manifest.json`

The generated manifest should describe:

- the same Windows zip published in GitHub Releases
- the same SHA256 recorded in `canon-<VERSION>-SHA256SUMS.txt`
- `canon.exe` as the binary exposed by Scoop

## 4. Verify the Extended Release Surface

Run the release verifier against the bundle and generated manifest:

```bash
bash scripts/release/verify-release-surface.sh \
  --version "$VERSION" \
  --dist-dir "$DIST_DIR" \
  --release-notes "$DIST_DIR/release-notes.md" \
  --distribution-metadata "$DIST_DIR/canon-$VERSION-distribution-metadata.json" \
  --scoop-manifest "$DIST_DIR/canon-$VERSION-scoop-manifest.json"
```

This validation checks:

- the archive set is complete
- checksum entries exist and match
- release notes mention the published assets
- the Windows zip contains `canon.exe`
- the shared distribution metadata matches the canonical GitHub Release URLs
- the Scoop manifest points to the canonical Windows asset and checksum

Do not continue to submission if this command fails. Regenerate the artifacts
from the release bundle instead of editing URLs or hashes by hand.

## 5. Review the Generated Manifest

Before submission, inspect the rendered manifest and confirm it still describes
the current release:

```bash
sed -n '1,160p' "$DIST_DIR/canon-$VERSION-scoop-manifest.json"
```

Focus on these fields:

- `version` equals `<VERSION>`
- `architecture.64bit.url` points to the GitHub Release asset
- `architecture.64bit.hash` matches `canon-<VERSION>-SHA256SUMS.txt`
- `bin` is `canon.exe`

## 6. Submit the Manifest to Scoop

The first slice keeps final submission manual. After local verification:

1. Fork `ScoopInstaller/Main`.
2. Copy `dist/canon-<VERSION>-scoop-manifest.json` to `bucket/canon.json` in
   your fork.
3. Open a pull request to `ScoopInstaller/Main`.

Example:

```bash
VERSION=0.34.0
SCOOP_MAIN="$HOME/src/Main"

cp "$DIST_DIR/canon-$VERSION-scoop-manifest.json" "$SCOOP_MAIN/bucket/canon.json"
```

When you open the pull request, include:

- the GitHub Release URL for `v<VERSION>`
- confirmation that the manifest was generated from the repository scripts
- the Windows asset name `canon-<VERSION>-windows-x86_64.zip`

## 7. Confirm Installation After Merge

After the Scoop main bucket pull request merges and the bucket refreshes,
verify installation from a Windows machine:

```powershell
scoop install canon
scoop update canon
canon --version
Get-Command canon
```

If the app does not appear immediately, wait for bucket propagation before
reopening the release bundle.

## Common Failure Modes

- Passing `v0.34.0` to the scripts instead of `0.34.0`
- Missing `release-notes.md` or `canon-<VERSION>-SHA256SUMS.txt`
- A Windows zip that contains nested directories instead of a root `canon.exe`
- Hand-editing the generated JSON after verification instead of regenerating it
- Submitting the manifest before the GitHub Release asset is publicly available

## Related Files

- `scripts/release/write-distribution-metadata.sh`
- `scripts/release/render-scoop-manifest.sh`
- `scripts/release/verify-release-surface.sh`
- `.github/release-notes-template.md`
- `README.md`