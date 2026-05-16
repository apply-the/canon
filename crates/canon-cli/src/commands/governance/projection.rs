//! Run-state projection for the governance adapter.
//!
//! Converts raw `.canon/` runtime state (manifests, artifact contracts, approval
//! records, packet metadata) into the structured [`GovernanceResponse`] that
//! adapter clients consume.  All I/O errors are mapped to typed failure
//! responses rather than being propagated as generic Rust errors.

use super::status::{
    approval_state_value, authority_approval_state, authority_packet_readiness, default_headline,
    default_message, load_document_refs, missing_document_refs, normalized_status,
    packet_missing_sections, packet_readiness_value, rejected_document_refs, response_reason_code,
};
use super::*;

/// Builds a complete [`GovernanceResponse`] for an existing run.
///
/// Loads manifests, approval records, and artifact contracts from the store,
/// derives status/readiness/approval fields, and merges any persisted
/// [`RuntimePacketMetadata`] with the projected metadata before assembling
/// the final response.
pub(super) fn project_run_response(
    repo_root: &Path,
    run_ref: &str,
    headline_hint: Option<String>,
) -> GovernanceResponse {
    let projection = match load_run_projection(repo_root, run_ref) {
        Ok(projection) => projection,
        Err(response) => return *response,
    };

    let status = normalized_status(projection.run_state, projection.packet_readiness);
    let reason_code = response_reason_code(status, projection.packet_readiness);
    let headline = headline_hint.or_else(|| default_headline(status, projection.packet_readiness));
    let message = default_message(status, &projection.run_ref, projection.packet_readiness);

    GovernanceResponse {
        adapter_schema_version: ADAPTER_SCHEMA_VERSION,
        status,
        approval_state: projection.approval_state,
        message,
        run_ref: Some(projection.run_ref),
        packet_ref: projection.packet_ref,
        expected_document_refs: projection.expected_document_refs,
        document_refs: projection.document_refs,
        packet_readiness: projection.packet_readiness,
        missing_fields: Vec::new(),
        missing_sections: projection.missing_sections,
        headline,
        reason_code,
        authority_governance: projection.authority_governance,
        adaptive_governance: projection.adaptive_governance,
    }
}

/// Loads and assembles a [`RunProjection`] for the given `run_ref`.
///
/// All store reads are mapped to typed [`GovernanceResponse`] failures so the
/// caller can return them directly without additional error handling.
pub(super) fn load_run_projection(
    repo_root: &Path,
    run_ref: &str,
) -> Result<RunProjection, GovernanceFailure> {
    let store = WorkspaceStore::new(repo_root);
    let manifest = store.load_run_manifest(run_ref).map_err(|_| {
        Box::new(GovernanceResponse::failed(
            GovernanceReasonCode::RunNotFound,
            format!("run `{run_ref}` was not found in this workspace"),
            Some(run_ref.to_string()),
        ))
    })?;
    let state = store.load_run_state(run_ref).map_err(|error| {
        Box::new(GovernanceResponse::failed(
            GovernanceReasonCode::RuntimeError,
            format!("run `{run_ref}` state could not be loaded: {error}"),
            Some(run_ref.to_string()),
        ))
    })?;
    let approvals = store.load_approval_records(run_ref).map_err(|error| {
        Box::new(GovernanceResponse::failed(
            GovernanceReasonCode::RuntimeError,
            format!("run `{run_ref}` approvals could not be loaded: {error}"),
            Some(run_ref.to_string()),
        ))
    })?;

    let (expected_document_refs, artifact_contract_missing) =
        match store.load_artifact_contract(run_ref) {
            Ok(contract) => (
                contract
                    .artifact_requirements
                    .into_iter()
                    .map(|requirement| {
                        format!(
                            ".canon/artifacts/{run_ref}/{}/{}",
                            manifest.mode.as_str(),
                            requirement.file_name
                        )
                    })
                    .collect::<Vec<_>>(),
                false,
            ),
            Err(error) if error.kind() == ErrorKind::NotFound => (Vec::new(), true),
            Err(error) => {
                return Err(Box::new(GovernanceResponse::failed(
                    GovernanceReasonCode::ArtifactContractUnreadable,
                    format!("run `{run_ref}` artifact contract could not be loaded: {error}"),
                    Some(run_ref.to_string()),
                )));
            }
        };

    let document_refs = load_document_refs(&store, run_ref, manifest.mode, &expected_document_refs)
        .map_err(|error| {
            Box::new(GovernanceResponse::failed(
                GovernanceReasonCode::RuntimeError,
                format!("run `{run_ref}` artifacts could not be listed: {error}"),
                Some(run_ref.to_string()),
            ))
        })?;

    if artifact_contract_missing && !document_refs.is_empty() {
        return Err(Box::new(GovernanceResponse::failed(
            GovernanceReasonCode::ArtifactContractMissing,
            format!("run `{run_ref}` artifacts are present but artifact contract is missing"),
            Some(run_ref.to_string()),
        )));
    }

    let missing_refs = missing_document_refs(&expected_document_refs, &document_refs);
    let rejected_refs = rejected_document_refs(repo_root, &document_refs);
    let packet_readiness = packet_readiness_value(
        &expected_document_refs,
        &document_refs,
        &missing_refs,
        &rejected_refs,
    );
    let missing_sections = packet_missing_sections(&missing_refs, &rejected_refs);
    let approval_state = approval_state_value(
        state.state,
        approvals.iter().any(|record| record.is_approved()),
        approvals.iter().any(|record| !record.is_approved()),
    );
    let packet_ref = if expected_document_refs.is_empty() && document_refs.is_empty() {
        None
    } else {
        Some(format!(".canon/artifacts/{run_ref}/{}", manifest.mode.as_str()))
    };
    let projected_packet_metadata = project_runtime_packet_metadata(
        &manifest,
        approval_state,
        packet_readiness,
        &expected_document_refs,
        &document_refs,
    );
    let packet_metadata = load_runtime_packet_metadata(repo_root, run_ref, manifest.mode)
        .map_err(|error| {
            Box::new(GovernanceResponse::failed(
                GovernanceReasonCode::RuntimeError,
                format!("run `{run_ref}` packet metadata could not be loaded: {error}"),
                Some(run_ref.to_string()),
            ))
        })?
        .map(|metadata| merge_projected_governance_metadata(metadata, &projected_packet_metadata))
        .or(Some(projected_packet_metadata));

    Ok(RunProjection {
        run_ref: run_ref.to_string(),
        run_state: state.state,
        approval_state,
        packet_ref,
        expected_document_refs,
        document_refs,
        packet_readiness,
        missing_sections,
        authority_governance: packet_metadata
            .as_ref()
            .and_then(|metadata| metadata.authority_governance.clone()),
        adaptive_governance: packet_metadata
            .as_ref()
            .and_then(|metadata| metadata.adaptive_governance.clone()),
    })
}

/// Reads the persisted [`RuntimePacketMetadata`] for a run from disk, or
/// returns `Ok(None)` when the file does not exist yet.
///
/// A missing file is not an error; it means the engine has not emitted packet
/// metadata for this run yet and a projected snapshot will be used instead.
pub(super) fn load_runtime_packet_metadata(
    repo_root: &Path,
    run_ref: &str,
    mode: Mode,
) -> Result<Option<RuntimePacketMetadata>, std::io::Error> {
    let path = repo_root
        .join(".canon")
        .join("artifacts")
        .join(run_ref)
        .join(mode.as_str())
        .join(RUNTIME_PACKET_METADATA_FILE_NAME);

    match fs::read_to_string(path) {
        Ok(contents) => serde_json::from_str(&contents).map(Some).map_err(std::io::Error::other),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

/// Derives a [`RuntimePacketMetadata`] snapshot from the run manifest and
/// current readiness/approval state.
///
/// Used when no persisted packet metadata exists yet, or as a base that is
/// merged with any persisted metadata to fill in missing governance envelopes.
pub(super) fn project_runtime_packet_metadata(
    manifest: &canon_engine::persistence::manifests::RunManifest,
    approval_state: ApprovalState,
    packet_readiness: Option<PacketReadiness>,
    expected_document_refs: &[String],
    document_refs: &[String],
) -> RuntimePacketMetadata {
    let artifact_order = projected_artifact_order(expected_document_refs, document_refs);
    let primary_artifact = artifact_order.first().cloned().unwrap_or_default();
    let authority_approval_state = authority_approval_state(approval_state);
    let authority_packet_readiness = authority_packet_readiness(packet_readiness);

    RuntimePacketMetadata {
        primary_artifact: primary_artifact.clone(),
        artifact_order: artifact_order.clone(),
        publish_order: None,
        legacy_aliases: None,
        expertise_input: None,
        authority_governance: Some(AuthorityGovernanceV1Envelope::from_runtime_inputs(
            AuthorityGovernanceV1RuntimeInputs {
                mode: manifest.mode,
                risk: manifest.risk,
                zone: manifest.zone,
                approval_state: authority_approval_state,
                packet_readiness: authority_packet_readiness,
                primary_artifact: (!primary_artifact.is_empty()).then_some(primary_artifact),
                artifact_order,
                promotion_refs: Vec::new(),
            },
        )),
        adaptive_governance: Some(AdaptiveGovernanceV1Envelope::from_runtime_inputs(
            AdaptiveGovernanceV1RuntimeInputs {
                risk: manifest.risk,
                zone: manifest.zone,
                approval_state: authority_approval_state,
                packet_readiness: authority_packet_readiness,
            },
        )),
    }
}

/// Fills in any missing governance envelope fields in `metadata` from the
/// `projected` snapshot.
///
/// Only `authority_governance` and `adaptive_governance` are back-filled;
/// all other fields from the persisted metadata are kept as-is.
pub(super) fn merge_projected_governance_metadata(
    mut metadata: RuntimePacketMetadata,
    projected: &RuntimePacketMetadata,
) -> RuntimePacketMetadata {
    if metadata.authority_governance.is_none() {
        metadata.authority_governance = projected.authority_governance.clone();
    }
    if metadata.adaptive_governance.is_none() {
        metadata.adaptive_governance = projected.adaptive_governance.clone();
    }
    metadata
}

/// Returns the ordered list of artifact file-name leafs for the packet.
///
/// Prefers `document_refs` when available (actual artifacts on disk), falling
/// back to `expected_document_refs` when no artifacts have been written yet.
/// The [`RUNTIME_PACKET_METADATA_FILE_NAME`] entry is excluded so adapter
/// clients only see human-authored artifacts in the order list.
pub(super) fn projected_artifact_order(
    expected_document_refs: &[String],
    document_refs: &[String],
) -> Vec<String> {
    let source_refs = if document_refs.is_empty() { expected_document_refs } else { document_refs };

    source_refs
        .iter()
        .filter_map(|reference| {
            let file_name = Path::new(reference).file_name()?.to_str()?;
            (file_name != RUNTIME_PACKET_METADATA_FILE_NAME).then(|| file_name.to_string())
        })
        .collect()
}
