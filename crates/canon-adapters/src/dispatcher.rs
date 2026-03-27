use crate::{AdapterError, AdapterRequest, SideEffectClass};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispatchDisposition {
    Execute,
    RecommendationOnly,
}

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
