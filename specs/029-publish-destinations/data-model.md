# Data Model: Structured External Publish Destinations

## Entity: Publish Family Root

- **Purpose**: Represents the stable external repository root where a packet
  family is published.
- **Fields**:
  - `mode`: Canon mode being published
  - `root_path`: repository-relative external family root
  - `publishable_states`: run states allowed to publish under current policy

## Entity: Publish Descriptor

- **Purpose**: Represents the human-readable label used in the default
  structured publish directory name.
- **Fields**:
  - `source_kind`: explicit slug, title-derived slug, or fallback descriptor
  - `value`: sanitized descriptor string used in the path
  - `publish_date`: date prefix paired with the descriptor
  - `canonical_run_id`: originating run id preserved separately from the path

## Entity: Published Packet Metadata

- **Purpose**: Carries the traceability and lineage information that must stay
  recoverable from the published output itself.
- **Fields**:
  - `run_id`: canonical run identity
  - `mode`: originating mode
  - `risk`: declared risk classification
  - `zone`: declared usage zone
  - `publish_timestamp`: when the publish operation materialized the packet
  - `source_artifacts`: list of materialized artifact file names or paths
  - `descriptor`: resolved publish descriptor
  - `destination`: final publish directory path

## Entity: Publish Request

- **Purpose**: Represents one publish invocation against a run.
- **Fields**:
  - `run_reference`: canonical run id, short id, or `@last` resolved before publish
  - `destination_override`: optional explicit publish destination
  - `default_destination`: computed structured path used when no override exists

## Relationships

- Each **Publish Request** resolves to exactly one **Publish Family Root** and
  one resulting publish destination.
- Each resolved default destination uses exactly one **Publish Descriptor**.
- Each successful publish operation emits exactly one **Published Packet
  Metadata** record and one or more published artifacts.