//! Bounded exactly-one-request JSON RPC transport for deterministic governance.
//!
//! The transport owns framing and public projection only. It never creates a
//! server loop, invokes an external capability, or turns a rejected request
//! into a successful governance result.

use std::io::{Read, Write};

use canon_contracts::{
    CanonContractVersion, OneShotOperation, OneShotResponse, OutcomeNextAction,
    RecordOutcomeDisposition, RecordOutcomeRejectionReason, RecordOutcomeRequest,
    RecordOutcomeResponse,
};
use canon_engine::EngineService;
use canon_engine::decision_memory::GovernanceBundleDraft;
use canon_engine::modes::stable_profile_registry;
use serde::{Deserialize, Serialize};

use super::stable_governance::{self, StableGovernanceResult};
use crate::error::{CliError, CliResult};

/// Frozen maximum size of one Canon RPC request, including JSON framing.
pub const MAX_REQUEST_BYTES: usize = 1_048_576;

const REJECTED_STATUS: &str = "rejected";
const INVALID_INPUT_REASON: &str = "invalid_input";
const UNSUPPORTED_OPERATION_REASON: &str = "unsupported_operation";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RequestEnvelope {
    contract_version: CanonContractVersion,
    request_id: String,
    operation: String,
    payload: OperationPayload,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct OperationPayload {
    #[serde(default)]
    bundle: Option<GovernanceBundleDraft>,
    #[serde(default)]
    outcome: Option<RecordOutcomeRequest>,
}

#[derive(Debug, Serialize)]
struct CapabilitiesResult {
    operations: [OneShotOperation; 7],
    operation_capabilities: [OperationCapability; 7],
    profiles: Vec<&'static str>,
    exactly_one_request: bool,
    max_request_bytes: usize,
    semantic_review_execution: bool,
    background_work: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct OperationCapability {
    operation: OneShotOperation,
    available: bool,
    reason_code: Option<RecordOutcomeRejectionReason>,
}

#[derive(Debug, Serialize)]
struct RejectionResult {
    status: &'static str,
    reason_code: &'static str,
    message: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum RpcResult {
    Capabilities(CapabilitiesResult),
    Governance(StableGovernanceResult),
    Outcome(RecordOutcomeResponse),
    Rejection(RejectionResult),
}

impl RpcResult {
    const fn exit_code(&self) -> i32 {
        match self {
            Self::Capabilities(_) => 0,
            Self::Governance(result) => stable_governance::exit_code(result.terminal_status),
            Self::Outcome(result) => match result.disposition {
                RecordOutcomeDisposition::Recorded | RecordOutcomeDisposition::Replayed => 0,
                RecordOutcomeDisposition::Rejected => 8,
            },
            Self::Rejection(_) => 1,
        }
    }
}

#[derive(Debug)]
enum RequestError {
    Empty,
    TooLarge,
    Invalid(String),
    Unsupported(String),
    Governance(CliError),
}

/// Processes one request from stdin, emits one response, flushes, and returns.
pub fn execute(service: &EngineService) -> CliResult<i32> {
    execute_with(service, std::io::stdin().lock(), std::io::stdout().lock())
}

fn execute_with(
    service: &EngineService,
    mut input: impl Read,
    mut output: impl Write,
) -> CliResult<i32> {
    let bytes = match read_bounded(&mut input) {
        Ok(bytes) => bytes,
        Err(error) => return reject(&mut output, "", error),
    };
    let request = match serde_json::from_slice::<RequestEnvelope>(&bytes) {
        Ok(request) => request,
        Err(error) => return reject(&mut output, "", RequestError::Invalid(error.to_string())),
    };
    let request_id = request.request_id.clone();
    match dispatch(service, request) {
        Ok(result) => {
            let exit_code = result.exit_code();
            write_response(
                &mut output,
                &OneShotResponse { contract_version: CanonContractVersion::V1, request_id, result },
            )?;
            Ok(exit_code)
        }
        Err(error) => reject(&mut output, &request_id, error),
    }
}

fn dispatch(service: &EngineService, request: RequestEnvelope) -> Result<RpcResult, RequestError> {
    if request.contract_version != CanonContractVersion::V1 {
        return Err(RequestError::Invalid("unsupported contract version".to_string()));
    }
    let empty = request.payload.bundle.is_none() && request.payload.outcome.is_none();
    match request.operation.as_str() {
        "capabilities" if empty => Ok(RpcResult::Capabilities(capabilities())),
        "start" | "approve" if request.payload.outcome.is_none() => {
            let bundle = request.payload.bundle.ok_or_else(|| {
                RequestError::Invalid("mutation payload requires `bundle`".to_string())
            })?;
            stable_governance::mutate(service, &request.request_id, bundle)
                .map(RpcResult::Governance)
                .map_err(RequestError::Governance)
        }
        "refresh" | "inspect" | "publish" if empty => stable_governance::inspect(service)
            .map(RpcResult::Governance)
            .map_err(RequestError::Governance),
        "record_outcome" if request.payload.bundle.is_none() => {
            let outcome = request.payload.outcome.ok_or_else(|| {
                RequestError::Invalid("record_outcome payload requires `outcome`".to_string())
            })?;
            Ok(RpcResult::Outcome(unavailable_outcome(&request.request_id, outcome)))
        }
        "capabilities" | "refresh" | "inspect" | "publish" | "record_outcome" => {
            Err(RequestError::Invalid("operation payload must be an empty object".to_string()))
        }
        "start" | "approve" => {
            Err(RequestError::Invalid("mutation payload accepts only `bundle`".to_string()))
        }
        operation => Err(RequestError::Unsupported(operation.to_string())),
    }
}

fn read_bounded(input: &mut impl Read) -> Result<Vec<u8>, RequestError> {
    let limit = u64::try_from(MAX_REQUEST_BYTES).map_err(|_| RequestError::TooLarge)?;
    let mut bytes = Vec::new();
    input
        .take(limit.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|error| RequestError::Invalid(error.to_string()))?;
    if bytes.len() > MAX_REQUEST_BYTES {
        return Err(RequestError::TooLarge);
    }
    if bytes.iter().all(u8::is_ascii_whitespace) {
        return Err(RequestError::Empty);
    }
    Ok(bytes)
}

fn capabilities() -> CapabilitiesResult {
    let operations = [
        OneShotOperation::Capabilities,
        OneShotOperation::Start,
        OneShotOperation::Refresh,
        OneShotOperation::Approve,
        OneShotOperation::Inspect,
        OneShotOperation::Publish,
        OneShotOperation::RecordOutcome,
    ];
    CapabilitiesResult {
        operations,
        operation_capabilities: operations.map(operation_capability),
        profiles: stable_profile_registry().iter().map(|entry| entry.id()).collect(),
        exactly_one_request: true,
        max_request_bytes: MAX_REQUEST_BYTES,
        semantic_review_execution: false,
        background_work: false,
    }
}

const fn operation_capability(operation: OneShotOperation) -> OperationCapability {
    match operation {
        OneShotOperation::RecordOutcome => OperationCapability {
            operation,
            available: false,
            reason_code: Some(RecordOutcomeRejectionReason::UnsupportedOperation),
        },
        OneShotOperation::Capabilities
        | OneShotOperation::Start
        | OneShotOperation::Refresh
        | OneShotOperation::Approve
        | OneShotOperation::Inspect
        | OneShotOperation::Publish => {
            OperationCapability { operation, available: true, reason_code: None }
        }
    }
}

fn unavailable_outcome(request_id: &str, outcome: RecordOutcomeRequest) -> RecordOutcomeResponse {
    let reason_code = if request_id == outcome.event_id.as_str() {
        RecordOutcomeRejectionReason::UnsupportedOperation
    } else {
        RecordOutcomeRejectionReason::IdentityDigestConflict
    };
    RecordOutcomeResponse {
        event_id: outcome.event_id,
        event_digest: outcome.event_digest,
        disposition: RecordOutcomeDisposition::Rejected,
        decision_memory_revision: None,
        decision_memory_digest: None,
        reason_code: Some(reason_code),
        next_actions: vec![OutcomeNextAction::Retry],
    }
}

fn reject(output: &mut impl Write, request_id: &str, error: RequestError) -> CliResult<i32> {
    let (exit_code, reason_code, message) = rejection_details(error);
    write_response(
        output,
        &OneShotResponse {
            contract_version: CanonContractVersion::V1,
            request_id: request_id.to_string(),
            result: RpcResult::Rejection(RejectionResult {
                status: REJECTED_STATUS,
                reason_code,
                message,
            }),
        },
    )?;
    Ok(exit_code)
}

fn rejection_details(error: RequestError) -> (i32, &'static str, String) {
    match error {
        RequestError::Empty => {
            (1, INVALID_INPUT_REASON, "stdin must contain exactly one JSON request".to_string())
        }
        RequestError::TooLarge => (
            1,
            INVALID_INPUT_REASON,
            format!("request exceeds the {MAX_REQUEST_BYTES}-byte framing limit"),
        ),
        RequestError::Invalid(message) => {
            (1, INVALID_INPUT_REASON, format!("request framing is invalid: {message}"))
        }
        RequestError::Unsupported(operation) => {
            (8, UNSUPPORTED_OPERATION_REASON, format!("operation `{operation}` is not supported"))
        }
        RequestError::Governance(CliError::IdentityDigestConflict(message)) => {
            (7, "identity_digest_conflict", message)
        }
        RequestError::Governance(CliError::Io(error)) => {
            (6, "persistence_failure", error.to_string())
        }
        RequestError::Governance(error) => (1, INVALID_INPUT_REASON, error.to_string()),
    }
}

fn write_response(output: &mut impl Write, response: &impl Serialize) -> CliResult<()> {
    serde_json::to_writer(&mut *output, response)?;
    output.write_all(b"\n")?;
    output.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use canon_engine::EngineService;
    use serde_json::Value;

    use std::io::{Error, Read};

    use super::{MAX_REQUEST_BYTES, execute_with};
    use crate::commands::stable_governance::tests::draft;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    fn require(condition: bool, message: &str) -> TestResult {
        if condition { Ok(()) } else { Err(message.to_string().into()) }
    }

    fn invoke(input: &[u8]) -> Result<(i32, Value), Box<dyn std::error::Error>> {
        let workspace = tempfile::tempdir()?;
        let service = EngineService::new(workspace.path());
        let mut output = Vec::new();
        let code = execute_with(&service, input, &mut output)?;
        Ok((code, serde_json::from_slice(&output)?))
    }

    struct FailingReader;

    impl Read for FailingReader {
        fn read(&mut self, _buffer: &mut [u8]) -> std::io::Result<usize> {
            Err(Error::other("injected read failure"))
        }
    }

    #[test]
    fn direct_dispatch_covers_capabilities_and_strict_transport_rejections() -> TestResult {
        let (code, capabilities) = invoke(
            br#"{"contract_version":"1.0","request_id":"cap","operation":"capabilities","payload":{}}"#,
        )?;
        require(code == 0, "capabilities exit code drifted")?;
        require(
            capabilities["result"]["operations"].as_array().is_some_and(|items| items.len() == 7),
            "capabilities registry drifted",
        )?;

        for input in [
            b"".as_slice(),
            b" \n".as_slice(),
            br#"{"contract_version":"1.0""#.as_slice(),
            br#"{"contract_version":"1.0","request_id":"x","operation":"execute","payload":{}}"#
                .as_slice(),
            br#"{"contract_version":"1.0","request_id":"x","operation":"inspect","payload":{"bundle":null}}"#
                .as_slice(),
            br#"{"contract_version":"future","request_id":"x","operation":"capabilities","payload":{}}"#
                .as_slice(),
        ] {
            let (code, response) = invoke(input)?;
            require(code != 0, "invalid transport returned success")?;
            require(
                response["result"]["status"] == "rejected",
                "invalid transport lacked a typed rejection",
            )?;
        }
        let workspace = tempfile::tempdir()?;
        let mut output = Vec::new();
        let code = execute_with(&EngineService::new(workspace.path()), FailingReader, &mut output)?;
        require(code == 1, "reader failure exit code drifted")?;
        Ok(())
    }

    #[test]
    fn direct_reader_enforces_the_frozen_size_limit() -> TestResult {
        let oversized = vec![b'x'; MAX_REQUEST_BYTES.saturating_add(1)];
        let (code, response) = invoke(&oversized)?;
        require(code == 1, "oversized request exit code drifted")?;
        require(
            response["result"]["reason_code"] == "invalid_input",
            "oversized request reason drifted",
        )
    }

    #[test]
    fn direct_dispatch_covers_all_governance_operation_arms() -> TestResult {
        let workspace = tempfile::tempdir()?;
        let service = EngineService::new(workspace.path());
        let bundle = draft("bundle-rpc-direct", 1);
        let start = serde_json::to_vec(&serde_json::json!({
            "contract_version": "1.0",
            "request_id": "bundle-rpc-direct",
            "operation": "start",
            "payload": {"bundle": bundle}
        }))?;
        let mut output = Vec::new();
        require(
            execute_with(&service, start.as_slice(), &mut output)? == 0,
            "direct start failed",
        )?;
        for operation in ["refresh", "inspect", "publish"] {
            let request = serde_json::to_vec(&serde_json::json!({
                "contract_version": "1.0",
                "request_id": format!("request-{operation}"),
                "operation": operation,
                "payload": {}
            }))?;
            let mut operation_output = Vec::new();
            require(
                execute_with(&service, request.as_slice(), &mut operation_output)? == 0,
                "read-only operation failed",
            )?;
        }
        let missing_bundle = br#"{"contract_version":"1.0","request_id":"missing","operation":"approve","payload":{}}"#;
        let mut rejected = Vec::new();
        require(
            execute_with(&service, missing_bundle.as_slice(), &mut rejected)? == 1,
            "missing mutation bundle was accepted",
        )?;

        let identity_mismatch = serde_json::to_vec(&serde_json::json!({
            "contract_version": "1.0",
            "request_id": "different-request",
            "operation": "approve",
            "payload": {"bundle": draft("bundle-rpc-mismatch", 1)}
        }))?;
        let mut conflict = Vec::new();
        require(
            execute_with(&service, identity_mismatch.as_slice(), &mut conflict)? == 7,
            "identity mismatch did not use the conflict exit code",
        )?;

        let non_empty_capabilities = serde_json::to_vec(&serde_json::json!({
            "contract_version": "1.0",
            "request_id": "cap-with-payload",
            "operation": "capabilities",
            "payload": {"bundle": draft("bundle-rpc-capabilities", 1)}
        }))?;
        let mut invalid_payload = Vec::new();
        require(
            execute_with(&service, non_empty_capabilities.as_slice(), &mut invalid_payload)? == 1,
            "non-empty capabilities payload was accepted",
        )?;

        let stale = serde_json::to_vec(&serde_json::json!({
            "contract_version": "1.0",
            "request_id": "bundle-rpc-stale",
            "operation": "start",
            "payload": {"bundle": draft("bundle-rpc-stale", 1)}
        }))?;
        let mut stale_output = Vec::new();
        require(
            execute_with(&service, stale.as_slice(), &mut stale_output)? == 1,
            "second terminal identity did not fail stale",
        )?;

        let obstructed = tempfile::tempdir()?;
        std::fs::write(obstructed.path().join(".canon"), b"not-a-directory")?;
        let persistence = serde_json::to_vec(&serde_json::json!({
            "contract_version": "1.0",
            "request_id": "bundle-rpc-persistence",
            "operation": "start",
            "payload": {"bundle": draft("bundle-rpc-persistence", 1)}
        }))?;
        let mut persistence_output = Vec::new();
        require(
            execute_with(
                &EngineService::new(obstructed.path()),
                persistence.as_slice(),
                &mut persistence_output,
            )? == 6,
            "persistence failure exit code drifted",
        )
    }
}
