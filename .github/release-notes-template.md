# Canon {{VERSION}}

Tag: {{TAG}}

## Install Canon

### Homebrew (macOS / Linux)

```bash
brew tap apply-the/canon
brew install canon
canon --version
```

### Direct Download

Download the matching asset for your platform from this release:

- `canon-{{VERSION}}-macos-arm64.tar.gz`
- `canon-{{VERSION}}-macos-x86_64.tar.gz`
- `canon-{{VERSION}}-linux-arm64.tar.gz`
- `canon-{{VERSION}}-linux-x86_64.tar.gz`
- `canon-{{VERSION}}-windows-x86_64.zip`

## Verify Installation

### macOS and Linux

```bash
canon --version
command -v canon
```

### Windows PowerShell

```powershell
canon --version
Get-Command canon
```

## Release Artifact Matrix

| Platform | Artifact |
| --- | --- |
| macOS arm64 | `canon-{{VERSION}}-macos-arm64.tar.gz` |
| macOS x86_64 | `canon-{{VERSION}}-macos-x86_64.tar.gz` |
| Linux arm64 | `canon-{{VERSION}}-linux-arm64.tar.gz` |
| Linux x86_64 | `canon-{{VERSION}}-linux-x86_64.tar.gz` |
| Windows x86_64 | `canon-{{VERSION}}-windows-x86_64.zip` |

Integrity metadata is published as `canon-{{VERSION}}-SHA256SUMS.txt`.
Distribution metadata is published as `canon-{{VERSION}}-distribution-metadata.json`.
The Homebrew formula artifact is published as `canon-{{VERSION}}-homebrew-formula.rb`.

## Quickstart

After installation, initialize Canon in a repository:

```bash
canon init --output json
```

## Compatibility Notes

- Daily repository use does not require Cargo.
- Canon skills assume the installed `canon` binary is on PATH.
- If `canon` is missing or outdated, follow the repository install guide at
  `https://github.com/apply-the/canon#install`.