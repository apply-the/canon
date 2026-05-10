# Quickstart: Standard ADR Publish Artifacts

## Scenario 1: Architecture publishes a standard ADR by default

1. Create and complete an `architecture` run with publishable decision artifacts.
2. Publish the run with the normal command:

   ```bash
   canon publish <RUN_ID>
   ```

3. Verify the packet publish destination still exists under the normal architecture folder.
4. Verify one new ADR file exists under `docs/adr/` with a numbered `ADR-XXXX-<slug>.md` name.
5. Open the ADR and confirm it includes `Date`, `Status`, `Context`, `Decision`, and `Consequences`, plus traceability back to the source packet.

## Scenario 2: Change publishes an ADR only when explicitly requested

1. Create and complete a publishable `change` run.
2. Publish without ADR export:

   ```bash
   canon publish <RUN_ID>
   ```

3. Verify the normal `docs/changes/...` packet is published and no new `docs/adr/ADR-*.md` file appears.
4. Publish the same run with ADR export enabled:

   ```bash
   canon publish <RUN_ID> --adr
   ```

5. Verify the packet publish output is still present and one new ADR file is added to `docs/adr/`.

## Scenario 3: Migration can opt in, unsupported modes cannot

1. Create and complete a publishable `migration` run.
2. Publish with ADR export:

   ```bash
   canon publish <RUN_ID> --adr
   ```

3. Verify the migration packet publishes normally and one ADR file is added to `docs/adr/`.
4. Create or reuse a publishable unsupported mode run such as `incident`.
5. Attempt to publish with ADR export:

   ```bash
   canon publish <RUN_ID> --adr
   ```

6. Verify publish returns a validation error and no ADR file is created.

## Scenario 4: Packet destination override does not move the ADR registry

1. Publish a supported run with a packet override destination:

   ```bash
   canon publish <RUN_ID> --to docs/custom/output --adr
   ```

2. Verify the normal packet files land in `docs/custom/output`.
3. Verify the ADR still lands in `docs/adr/` and is listed in the publish summary output.