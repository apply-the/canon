mod app;
mod commands;
mod output;

fn main() {
    match app::run() {
        Ok(code) => std::process::exit(code),
        Err(error) => {
            let code = error
                .downcast_ref::<canon_engine::EngineError>()
                .map(|error| match error {
                    canon_engine::EngineError::Validation(_) => 5,
                    canon_engine::EngineError::Io(_) => 6,
                    _ => 1,
                })
                .unwrap_or(1);
            eprintln!("{error}");
            std::process::exit(code);
        }
    }
}
