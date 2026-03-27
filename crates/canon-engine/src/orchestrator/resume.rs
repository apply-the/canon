use serde::{Deserialize, Serialize};

use crate::domain::run::InputFingerprint;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumeDecision {
    pub action: String,
}

pub fn input_fingerprints_match(
    repo_root: &std::path::Path,
    stored_fingerprints: &[InputFingerprint],
) -> Result<bool, std::io::Error> {
    for fingerprint in stored_fingerprints {
        let path = std::path::PathBuf::from(&fingerprint.path);
        let resolved = if path.is_absolute() { path } else { repo_root.join(path) };
        if !resolved.is_file() {
            return Ok(false);
        }

        let metadata = std::fs::metadata(resolved)?;
        let modified = metadata
            .modified()
            .ok()
            .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs() as i64)
            .unwrap_or_default();

        if metadata.len() != fingerprint.size_bytes || modified != fingerprint.modified_unix_seconds
        {
            return Ok(false);
        }
    }

    Ok(true)
}
