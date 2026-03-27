use std::io::{Error, ErrorKind};

use canon_engine::{
    EngineService, RunRequest,
    domain::mode::Mode,
    domain::policy::{RiskClass, UsageZone},
};

use crate::app::OutputFormat;
use crate::commands::exit_code_for_state;
use crate::output;

#[allow(clippy::too_many_arguments)]
pub fn execute(
    service: &EngineService,
    mode: String,
    risk: String,
    zone: String,
    owner: String,
    inputs: Vec<String>,
    excluded_paths: Vec<String>,
    policy_root: Option<String>,
    method_root: Option<String>,
    format: OutputFormat,
) -> Result<i32, Box<dyn std::error::Error>> {
    let request = RunRequest {
        mode: mode.parse::<Mode>().map_err(|error| Error::new(ErrorKind::InvalidInput, error))?,
        risk: risk
            .parse::<RiskClass>()
            .map_err(|error| Error::new(ErrorKind::InvalidInput, error))?,
        zone: zone
            .parse::<UsageZone>()
            .map_err(|error| Error::new(ErrorKind::InvalidInput, error))?,
        owner,
        inputs,
        excluded_paths,
        policy_root,
        method_root,
    };
    let summary = service.run(request)?;
    output::print_value(&summary, format)?;
    Ok(exit_code_for_state(&summary.state))
}
