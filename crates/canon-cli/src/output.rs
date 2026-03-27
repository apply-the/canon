use serde::Serialize;

use crate::app::OutputFormat;

pub fn print_value<T: Serialize>(
    value: &T,
    format: OutputFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    match format {
        OutputFormat::Text => {
            println!("{}", serde_json::to_string_pretty(value)?);
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(value)?);
        }
        OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(value)?);
        }
    }

    Ok(())
}
