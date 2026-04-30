# Contract: Winget Manifest Bundle

## Purpose

Define the publication artifact bundle Canon generates for Windows Package
Manager submission.

## Output Layout

The bundle MUST contain three YAML files for one Canon version:

```text
<bundle-root>/
├── ApplyThe.Canon.yaml
├── ApplyThe.Canon.installer.yaml
└── ApplyThe.Canon.locale.en-US.yaml
```

Alternative filenames that keep the same multi-file manifest semantics are
acceptable if they remain stable and documented.

## Required Version Manifest Fields

- `PackageIdentifier`
- `PackageVersion`
- `DefaultLocale`
- `ManifestType: version`
- `ManifestVersion: 1.12.0`

## Required Default Locale Manifest Fields

- `PackageIdentifier`
- `PackageVersion`
- `PackageLocale: en-US`
- `Publisher`
- `PackageName`
- `License`
- `ShortDescription`
- `ManifestType: defaultLocale`
- `ManifestVersion: 1.12.0`

## Required Installer Manifest Fields

- `PackageIdentifier`
- `PackageVersion`
- `Installers` with exactly one Windows x64 entry in this slice
- `InstallerType: zip`
- `NestedInstallerType: portable`
- `InstallerUrl` referencing the canonical GitHub Release zip
- `InstallerSha256` matching the release checksum manifest
- `NestedInstallerFiles` containing `canon.exe`
- `PortableCommandAlias: canon`
- `ManifestType: installer`
- `ManifestVersion: 1.12.0`

## Validation Rules

- The bundle MUST fail validation if the installer URL or checksum diverge from
  the release metadata.
- The installer bundle MUST not reference unsupported architectures or deferred
  Windows channels.
- The generated YAML MUST remain deterministic for the same release metadata
  input.