use crate::{AdapterError, AdapterRequest, SideEffectClass};

/// Whether an adapter request should be executed or downgraded to recommendation-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispatchDisposition {
    /// The request is permitted to execute and produce real side effects.
    Execute,
    /// The request is not permitted to mutate; only a recommendation is produced.
    RecommendationOnly,
}

/// Determines the dispatch disposition for a request given the current mutation policy.
///
/// Returns [`DispatchDisposition::RecommendationOnly`] when `request.side_effect`
/// is `WorkspaceMutation` or `ExternalStateChange` and `allow_mutation` is `false`.
pub fn dispatch_disposition(request: &AdapterRequest, allow_mutation: bool) -> DispatchDisposition {
    if matches!(
        request.side_effect,
        SideEffectClass::WorkspaceMutation | SideEffectClass::ExternalStateChange
    ) && !allow_mutation
    {
        DispatchDisposition::RecommendationOnly
    } else {
        DispatchDisposition::Execute
    }
}

/// Enforces the mutation policy for a request, returning an error if mutation is blocked.
///
/// This is the boundary check that adapter call sites must use before executing
/// any mutating capability.
pub fn enforce_mutation_policy(
    request: &AdapterRequest,
    allow_mutation: bool,
) -> Result<(), AdapterError> {
    if matches!(
        dispatch_disposition(request, allow_mutation),
        DispatchDisposition::RecommendationOnly
    ) {
        return Err(AdapterError::MutationBlocked);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        AdapterError, AdapterKind, AdapterRequest, CapabilityKind, InvocationOrientation,
        LineageClass, SideEffectClass, TrustBoundaryKind,
    };

    use super::{DispatchDisposition, dispatch_disposition, enforce_mutation_policy};

    fn request(side_effect: SideEffectClass) -> AdapterRequest {
        AdapterRequest {
            adapter: AdapterKind::Shell,
            capability: CapabilityKind::RunCommand,
            purpose: "test request".to_string(),
            orientation: Some(InvocationOrientation::Context),
            trust_boundary: Some(TrustBoundaryKind::LocalProcess),
            lineage: Some(LineageClass::NonGenerative),
            side_effect,
        }
    }

    #[test]
    fn dispatch_disposition_blocks_mutation_when_not_allowed() {
        assert_eq!(
            dispatch_disposition(&request(SideEffectClass::WorkspaceMutation), false),
            DispatchDisposition::RecommendationOnly
        );
        assert_eq!(
            dispatch_disposition(&request(SideEffectClass::ExternalStateChange), false),
            DispatchDisposition::RecommendationOnly
        );
    }

    #[test]
    fn dispatch_disposition_executes_read_only_or_allowed_mutation() {
        assert_eq!(
            dispatch_disposition(&request(SideEffectClass::ReadOnly), false),
            DispatchDisposition::Execute
        );
        assert_eq!(
            dispatch_disposition(&request(SideEffectClass::WorkspaceMutation), true),
            DispatchDisposition::Execute
        );
    }

    #[test]
    fn enforce_mutation_policy_returns_error_for_blocked_requests() {
        let error = enforce_mutation_policy(&request(SideEffectClass::WorkspaceMutation), false)
            .expect_err("blocked mutation should fail");

        assert!(matches!(error, AdapterError::MutationBlocked));
        assert!(enforce_mutation_policy(&request(SideEffectClass::ReadOnly), false).is_ok());
    }
}
