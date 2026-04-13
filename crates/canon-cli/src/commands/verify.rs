use crate::error::{CliError, CliResult};

pub fn execute() -> CliResult<i32> {
    Err(CliError::Unimplemented("verify is not implemented yet"))
}

#[cfg(test)]
mod tests {
    use super::execute;

    #[test]
    fn execute_reports_unimplemented_command() {
        let error = execute().expect_err("verify should remain unimplemented for now");

        assert_eq!(error.to_string(), "verify is not implemented yet");
    }
}
