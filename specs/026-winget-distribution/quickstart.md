# Quickstart: Winget Distribution And Roadmap Refocus

## Goal

Exercise the Windows distribution slice from release metadata generation through
artifact validation and documentation verification.

## Steps

1. Build or stage the standard release bundle so the Windows archive,
   release notes, and checksum manifest exist.
2. Generate distribution metadata from the release bundle.
3. Render the `winget` manifest bundle from that metadata.
4. Verify the release surface, including the Windows package-manager bundle.
5. Run the focused release and documentation tests for this feature.
6. Review the updated install docs, changelog, and roadmap to confirm Windows
   distribution guidance and MCP removal are consistent.

## Expected Result

- The release bundle includes a Windows package-manager publication artifact.
- Validation rejects missing or mismatched Windows metadata.
- Documentation names `winget` as the primary Windows package-manager path and
  keeps the archive fallback visible.
- `ROADMAP.md` no longer presents Protocol Interoperability / MCP as active
  next work.