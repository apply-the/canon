//! Canon CLI binary.
mod app;
mod commands;
mod error;
mod output;
mod workspace;

use crate::error::CliError;

fn exit_code_for_error(error: &CliError) -> i32 {
    match error {
        CliError::Engine(canon_engine::EngineError::Validation(_)) => 5,
        CliError::Engine(canon_engine::EngineError::Io(_)) | CliError::Io(_) => 6,
        _ => 1,
    }
}

fn exit_code_for_result_with(
    result: Result<i32, CliError>,
    mut report_error: impl FnMut(&CliError),
) -> i32 {
    match result {
        Ok(code) => code,
        Err(error) => {
            report_error(&error);
            exit_code_for_error(&error)
        }
    }
}

fn exit_code_for_result(result: Result<i32, CliError>) -> i32 {
    exit_code_for_result_with(result, |error| eprintln!("{error}"))
}

fn main() {
    std::process::exit(exit_code_for_result(app::run()));
}

#[cfg(test)]
mod tests {
    use std::io;

    use canon_engine::EngineError;

    use super::{exit_code_for_error, exit_code_for_result, exit_code_for_result_with};
    use crate::error::CliError;

    #[test]
    fn exit_code_for_error_maps_engine_errors_and_falls_back_to_one() {
        assert_eq!(
            exit_code_for_error(&CliError::Engine(EngineError::Validation(
                "bad input".to_string()
            ))),
            5
        );
        assert_eq!(
            exit_code_for_error(&CliError::Engine(EngineError::Io(io::Error::other(
                "disk failure"
            )))),
            6
        );
        assert_eq!(
            exit_code_for_error(&CliError::Engine(EngineError::UnsupportedInspectTarget(
                "weird".to_string()
            ))),
            1
        );
        assert_eq!(exit_code_for_error(&CliError::Io(io::Error::other("plain io"))), 6);
        assert_eq!(exit_code_for_error(&CliError::InvalidInput("bad flag".to_string())), 1);
    }

    #[test]
    fn exit_code_for_result_with_returns_success_code_without_reporting_error() {
        let mut reported = Vec::new();

        let code = exit_code_for_result_with(Ok(9), |error| reported.push(error.to_string()));

        assert_eq!(code, 9);
        assert!(reported.is_empty());
    }

    #[test]
    fn exit_code_for_result_with_reports_error_and_maps_cli_exit_code() {
        let mut reported = Vec::new();

        let code = exit_code_for_result_with(
            Err(CliError::Engine(EngineError::Validation("bad input".to_string()))),
            |error| reported.push(error.to_string()),
        );

        assert_eq!(code, 5);
        assert_eq!(reported, vec!["validation failed: bad input".to_string()]);
    }

    #[test]
    fn exit_code_for_result_uses_default_error_reporter_and_returns_io_code() {
        let code = exit_code_for_result(Err(CliError::Io(io::Error::other("plain io"))));

        assert_eq!(code, 6);
    }
}
