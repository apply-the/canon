mod app;
mod commands;
mod error;
mod output;

use crate::error::CliError;

fn exit_code_for_error(error: &CliError) -> i32 {
    match error {
        CliError::Engine(canon_engine::EngineError::Validation(_)) => 5,
        CliError::Engine(canon_engine::EngineError::Io(_)) | CliError::Io(_) => 6,
        _ => 1,
    }
}

fn main() {
    match app::run() {
        Ok(code) => std::process::exit(code),
        Err(error) => {
            let code = exit_code_for_error(&error);
            eprintln!("{error}");
            std::process::exit(code);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use canon_engine::EngineError;

    use super::exit_code_for_error;
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
}
