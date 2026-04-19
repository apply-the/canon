# Quickstart: Install-First Canon Release Validation

This quickstart exercises the Phase 1 release UX defined for
`005-cli-release-ux`.

## Prerequisites

- a Canon release candidate version, such as `0.7.0`
- a GitHub Actions run or draft GitHub Release that produced the Phase 1
  artifact set
- Git available on the validation machine
- a clean test repository where `.canon/` is not yet initialized

## 1. Confirm the Release Surface Exists

The release candidate must expose these assets before any install walkthrough
starts:

- `canon-<VERSION>-macos-arm64.tar.gz`
- `canon-<VERSION>-macos-x86_64.tar.gz`
- `canon-<VERSION>-linux-arm64.tar.gz`
- `canon-<VERSION>-linux-x86_64.tar.gz`
- `canon-<VERSION>-windows-x86_64.zip`
- `canon-<VERSION>-SHA256SUMS.txt`

The release notes must also state:

- the Canon version being shipped
- the supported platform matrix
- the install verification step using `canon --version`

## 2. Validate a Unix Install Flow

Example for macOS or Linux:

```bash
VERSION=0.7.0
ARCHIVE="canon-${VERSION}-linux-x86_64.tar.gz"

curl -LO "https://github.com/apply-the/canon/releases/download/v${VERSION}/${ARCHIVE}"
curl -LO "https://github.com/apply-the/canon/releases/download/v${VERSION}/canon-${VERSION}-SHA256SUMS.txt"

grep "  ${ARCHIVE}$" "canon-${VERSION}-SHA256SUMS.txt" | shasum -a 256 -c -
tar -xzf "${ARCHIVE}"
install -m 0755 canon "$HOME/.local/bin/canon"

canon --version
command -v canon
```

Expected result:

- checksum verification passes for the selected archive
- `canon --version` reports the release version
- `command -v canon` resolves to the PATH-installed binary, not a Cargo build

## 3. Validate a Windows Install Flow

Example in PowerShell:

```powershell
$Version = "0.7.0"
$Archive = "canon-$Version-windows-x86_64.zip"

Invoke-WebRequest -Uri "https://github.com/apply-the/canon/releases/download/v$Version/$Archive" -OutFile $Archive
Invoke-WebRequest -Uri "https://github.com/apply-the/canon/releases/download/v$Version/canon-$Version-SHA256SUMS.txt" -OutFile "canon-$Version-SHA256SUMS.txt"

$ExpectedHash = ((Get-Content "canon-$Version-SHA256SUMS.txt") | Where-Object { $_ -match [regex]::Escape($Archive) } | Select-Object -First 1).Split(' ', [System.StringSplitOptions]::RemoveEmptyEntries)[0]
$ActualHash = (Get-FileHash $Archive -Algorithm SHA256).Hash.ToLower()
if ($ActualHash -ne $ExpectedHash.ToLower()) {
  throw "SHA256 mismatch for $Archive"
}
Expand-Archive -Path $Archive -DestinationPath "$env:USERPROFILE\bin\canon-$Version" -Force
Copy-Item "$env:USERPROFILE\bin\canon-$Version\canon.exe" "$env:USERPROFILE\bin\canon.exe" -Force

canon --version
Get-Command canon
```

Expected result:

- the downloaded zip checksum matches the published manifest entry
- `canon --version` reports the release version
- `Get-Command canon` resolves to the PATH-installed executable

## 4. Smoke-Test Canon in a Fresh Repository

```bash
mkdir -p ~/tmp/canon-install-smoke
cd ~/tmp/canon-install-smoke
git init

canon init --output json
```

Expected result:

- `.canon/` is created
- no Cargo command is required anywhere in the workflow

## 5. Validate Skill Recovery Behavior

Use the shared compatibility helper in both success and failure modes.

Success path:

```bash
/bin/bash .agents/skills/canon-shared/scripts/check-runtime.sh --command init --repo-root "$PWD"
```

Failure path after temporarily removing `canon` from PATH:

```bash
PATH="/usr/bin:/bin" /bin/bash .agents/skills/canon-shared/scripts/check-runtime.sh --command init --repo-root "$PWD"
```

Expected result:

- success path reports `STATUS=ready`
- failure path reports `STATUS=cli-missing`
- the failure guidance points users to release-based installation, not Cargo

## 6. Record Validation Evidence

Capture the following under `specs/005-cli-release-ux/validation-report.md`:

- release asset inventory
- checksum verification results
- `canon --version` output
- PATH resolution output
- `canon init` smoke-test output
- skill preflight recovery output for missing and incompatible Canon binaries