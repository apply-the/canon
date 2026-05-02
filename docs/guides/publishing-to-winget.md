# Publishing Canon to winget

This guide is for maintainers who need to publish Canon to Windows Package
Manager after a release is prepared.

GitHub Releases remains the canonical distribution surface. The `winget`
manifests are derived from the release bundle, validated locally, and then
submitted manually to `winget-pkgs`.

The generated distribution metadata now carries explicit `source_of_truth` and
`channels` contracts so the renderer and verifier fail closed before
publication.

The preferred path is to let CI generate the `winget` bundle from a release
tag. Local script execution is mainly for dry runs, troubleshooting, or
reproducing the CI output before opening the `winget-pkgs` pull request.

If you need the parallel Scoop maintainer flow, use
`docs/guides/publishing-to-scoop.md`.

## Preferred Flow: Tag-Driven CI

Canon already ties the `winget` artifact generation to the release workflow in
CI.

- `.github/workflows/release.yml` runs automatically on `push` for tags that
  match `v*`.
- That workflow builds the platform archives, renders `release-notes.md`,
  generates checksums, writes `canon-<VERSION>-distribution-metadata.json`,
  renders the `winget` YAML bundle, and verifies the extended release surface.
- On a tag-triggered run, the workflow publishes the GitHub Release and uploads
  the generated `winget` files as release assets.

That means the correct operational sequence is:

1. Bump the workspace version and merge the release-ready change.
2. Create and push the Git tag `v<VERSION>`.
3. Let CI build, verify, and publish the release bundle plus the generated
   `winget` manifests.
4. Use those generated CI artifacts as the source for the manual
   `winget-pkgs` submission.

If you want the same workflow without publishing a GitHub Release yet, run the
workflow manually with `workflow_dispatch` and leave `publish=false`.

## What This Guide Assumes

- You already have a release version such as `0.37.0`.
- The GitHub release tag will be `v0.37.0`.
- A local `dist/` directory exists with the release archives, checksum file,
  and release notes.
- `jq`, `tar`, `zip`, `unzip`, and `shasum` are available in your shell.

The scripts in this repository expect the raw semantic version like `0.37.0`,
not the Git tag form with the `v` prefix.

## Required Release Files

Before generating `winget` manifests, make sure `dist/` contains:

- `canon-<VERSION>-macos-arm64.tar.gz`
- `canon-<VERSION>-macos-x86_64.tar.gz`
- `canon-<VERSION>-linux-arm64.tar.gz`
- `canon-<VERSION>-linux-x86_64.tar.gz`
- `canon-<VERSION>-windows-x86_64.zip`
- `canon-<VERSION>-SHA256SUMS.txt`
- `release-notes.md`
- `*.version.txt` evidence files for every archive

The Windows archive is the installer input for `winget`. Canon publishes it as
an archive installer with a nested portable `canon.exe` entry.

## 1. Stage the Release Bundle

Prefer the tagged CI workflow above. For local reproduction, either download the
generated release bundle from CI or collect the generated release artifacts into
`dist/`.

If you want to check that the expected files are present before generating
metadata:

```bash
VERSION=0.37.0
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
VERSION=0.37.0
DIST_DIR=dist

bash scripts/release/write-distribution-metadata.sh \
  --version "$VERSION" \
  --dist-dir "$DIST_DIR" \
  --output "$DIST_DIR/canon-$VERSION-distribution-metadata.json"
```

Expected output:

- `dist/canon-<VERSION>-distribution-metadata.json`

This file binds the Windows artifact URL and checksum directly from the release
bundle and records the canonical `source_of_truth` plus per-channel contracts,
so they do not need to be hand-copied into the manifest files.

## 3. Render the winget Manifest Bundle

Render the repository-owned `winget` manifests from the distribution metadata:

```bash
bash scripts/release/render-winget-manifests.sh \
  --metadata "$DIST_DIR/canon-$VERSION-distribution-metadata.json" \
  --output-dir "$DIST_DIR/winget"
```

Expected output files:

- `dist/winget/ApplyThe.Canon.yaml`
- `dist/winget/ApplyThe.Canon.locale.en-US.yaml`
- `dist/winget/ApplyThe.Canon.installer.yaml`

The installer manifest should describe:

- `InstallerType: zip`
- `NestedInstallerType: portable`
- `RelativeFilePath: canon.exe`
- `PortableCommandAlias: canon`

## 4. Verify the Release Surface

Run the release verifier against the bundle and generated manifests:

```bash
bash scripts/release/verify-release-surface.sh \
  --version "$VERSION" \
  --dist-dir "$DIST_DIR" \
  --release-notes "$DIST_DIR/release-notes.md" \
  --distribution-metadata "$DIST_DIR/canon-$VERSION-distribution-metadata.json" \
  --winget-manifest-dir "$DIST_DIR/winget"
```

This validation checks:

- the archive set is complete
- checksum entries exist and match
- release notes mention the published assets
- the Windows zip contains `canon.exe`
- the distribution metadata matches the canonical GitHub release URLs
- the `winget` manifests match the Windows artifact and checksum

Do not continue to submission if this command fails. Regenerate the artifacts
from the release bundle instead of editing URLs or checksums by hand.

## 5. Review the Generated Manifest Bundle

Before submission, inspect the rendered files and confirm they still describe
the current release:

```bash
sed -n '1,120p' "$DIST_DIR/winget/ApplyThe.Canon.yaml"
sed -n '1,200p' "$DIST_DIR/winget/ApplyThe.Canon.locale.en-US.yaml"
sed -n '1,200p' "$DIST_DIR/winget/ApplyThe.Canon.installer.yaml"
```

Focus on these fields:

- `PackageIdentifier: ApplyThe.Canon`
- `PackageVersion: <VERSION>`
- `InstallerUrl` points to the GitHub Release asset
- `InstallerSha256` matches `canon-<VERSION>-SHA256SUMS.txt`
- `ReleaseNotesUrl` points to the tagged GitHub release

## 6. Submit the Manifests to winget-pkgs

The first slice keeps final submission manual. After local verification:

1. Fork `microsoft/winget-pkgs`.
2. Create the manifest directory for the release version.
3. Copy the three generated YAML files into that directory.
4. Open a pull request to `winget-pkgs`.

For Canon, the expected path shape is:

```text
manifests/a/ApplyThe/Canon/<VERSION>/
```

Example:

```bash
VERSION=0.37.0
WINGET_PKGS="$HOME/src/winget-pkgs"

mkdir -p "$WINGET_PKGS/manifests/a/ApplyThe/Canon/$VERSION"
cp "$DIST_DIR"/winget/ApplyThe.Canon*.yaml \
  "$WINGET_PKGS/manifests/a/ApplyThe/Canon/$VERSION/"
```

When you open the pull request, include:

- the GitHub Release URL for `v<VERSION>`
- confirmation that the manifests were generated from the repository scripts
- the Windows asset name `canon-<VERSION>-windows-x86_64.zip`

## 7. Confirm Installation After Merge

After the `winget-pkgs` pull request merges and Windows Package Manager picks
up the new package metadata, verify installation from a Windows machine:

```powershell
winget show ApplyThe.Canon
winget install ApplyThe.Canon
canon --version
Get-Command canon
```

If the package does not appear immediately, wait for index propagation before
reopening the release bundle.

## Common Failure Modes

- Passing `v0.37.0` to the scripts instead of `0.37.0`
- Missing `release-notes.md` or `canon-<VERSION>-SHA256SUMS.txt`
- A Windows zip that contains nested directories instead of a root `canon.exe`
- Hand-editing the generated YAML after verification instead of regenerating it
- Submitting manifests before the GitHub Release asset is publicly available

## Related Files

- `scripts/release/write-distribution-metadata.sh`
- `scripts/release/render-winget-manifests.sh`
- `scripts/release/verify-release-surface.sh`
- `.github/release-notes-template.md`
- `README.md`