# Quickstart: Logical Packet Ordering

## Scenario 1: Read a requirements packet in intended order

1. Run a requirements session that produces a publishable packet.
2. Open the emitted packet directory under `.canon/artifacts/<RUN_ID>/requirements/`.
3. Confirm that the first reader-facing artifact is `01-prd.md`.
4. Confirm that subsequent reader-facing artifacts use contiguous numeric ordering.
5. Confirm that sidecars such as packet metadata remain separate from the packet-body sequence.

## Scenario 2: Verify contiguous numbering when optional artifacts are omitted

1. Run an architecture session whose authored input does not support every optional view.
2. Open the emitted architecture packet.
3. Confirm that omitted optional artifacts do not leave numbering gaps.
4. Confirm that any paired Mermaid or support view artifacts still stay adjacent to the matching packet artifact.

## Scenario 3: Verify metadata and summaries

1. Read the packet metadata for a newly emitted packet.
2. Confirm that `primary_artifact` points at the `01-*` artifact.
3. Confirm that `artifact_order` matches the emitted reader-facing artifacts on disk.
4. Run Canon status or inspect flows for that packet.
5. Confirm that the primary artifact is surfaced first and the artifact summary is deterministic.

## Scenario 4: Verify publish preserves logical order

1. Publish a completed ordered packet with `canon publish <RUN_ID>`.
2. Inspect the published output or generated index.
3. Confirm that numeric prefixes are preserved.
4. Confirm that any generated artifact listing follows declared packet order instead of alphabetical sort.

## Scenario 5: Verify legacy compatibility

1. Inspect a historical packet that predates ordered naming.
2. Run the relevant status, inspect, or compatibility lookup flow.
3. Confirm that Canon can still resolve the packet without rewriting historical artifacts.