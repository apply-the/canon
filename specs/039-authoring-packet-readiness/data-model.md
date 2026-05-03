# Data Model: Authoring Experience And Packet Readiness

## AuthoringLifecycleSummary

- **Purpose**: Additive clarity-inspection summary that explains how Canon sees
  the supplied authored packet before a run starts.
- **Fields**:
  - `packet_shape`: human-readable class such as `single-file`,
    `directory-backed`, or `multi-input`
  - `authority_status`: concise posture such as `explicit-authoritative-brief`
    or `ambiguous-current-brief`
  - `authoritative_inputs`: ordered list of paths Canon treats as the current-
    mode readiness source of truth
  - `supporting_inputs`: ordered list of sibling or explicit inputs that add
    provenance, narrowed context, or auxiliary detail without replacing the
    current-mode brief
  - `readiness_delta`: explicit list of what still needs to be authored or
    tightened before the packet reads as strongly ready
  - `next_authoring_step`: one concise sentence telling the maintainer what to
    strengthen next

## AuthoritativeInputRole

- **Purpose**: Represents why an input is authoritative for readiness.
- **Values**:
  - `current-brief`: the explicit current-mode brief, usually `brief.md` in a
    folder-backed packet or the only supplied file in a single-file packet
  - `explicit-single-input`: a single explicit file when no safer authority
    split exists

## SupportingInputRole

- **Purpose**: Represents why an input is informative but not authoritative.
- **Values**:
  - `source-map`: carried-forward provenance and reused decisions
  - `selected-context`: narrowed excerpts from a broader upstream packet
  - `supporting-note`: extra authored material that can inform reasoning but
    must not silently satisfy current-mode readiness on its own

## ReadinessDeltaItem

- **Purpose**: A user-facing gap statement that ties the authored packet shape
  to the current missing-context posture.
- **Rules**:
  - Must distinguish missing current-mode brief content from merely present
    supporting files
  - Must remain honest when packet authority is ambiguous
  - Must not imply Canon will rewrite files or infer hidden sources

## Relationships

- `AuthoringLifecycleSummary.authoritative_inputs` is empty only when the
  supplied packet is ambiguous enough that Canon should keep authority
  unresolved.
- `supporting_inputs` may be non-empty for both strong and weak packets; their
  presence never upgrades readiness by itself.
- `readiness_delta` is derived from packet shape plus existing missing-context
  or clarification signals, not from speculative content Canon wishes existed.