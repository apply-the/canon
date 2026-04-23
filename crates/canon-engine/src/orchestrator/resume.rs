use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

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

        if let Some(stored_digest) = &fingerprint.content_digest_sha256 {
            let current_digest = sha256_hex(&std::fs::read(&resolved)?);
            if &current_digest != stored_digest {
                return Ok(false);
            }
            continue;
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

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut encoded = String::with_capacity(digest.len() * 2);
    for byte in digest {
        use std::fmt::Write as _;
        let _ = write!(&mut encoded, "{byte:02x}");
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::run::{InputFingerprint, InputSourceKind};
    use std::fs;
    use tempfile::TempDir;

    fn fingerprint(path: &str, digest: Option<&str>, size: u64, mtime: i64) -> InputFingerprint {
        InputFingerprint {
            path: path.to_string(),
            source_kind: InputSourceKind::Path,
            size_bytes: size,
            modified_unix_seconds: mtime,
            content_digest_sha256: digest.map(str::to_string),
            snapshot_ref: None,
        }
    }

    #[test]
    fn empty_fingerprints_always_match() {
        let dir = TempDir::new().unwrap();
        assert!(input_fingerprints_match(dir.path(), &[]).unwrap());
    }

    #[test]
    fn missing_file_returns_false() {
        let dir = TempDir::new().unwrap();
        let fp = fingerprint("no-such-file.md", None, 0, 0);
        assert!(!input_fingerprints_match(dir.path(), &[fp]).unwrap());
    }

    #[test]
    fn matching_digest_returns_true() {
        let dir = TempDir::new().unwrap();
        let content = b"hello canon input";
        let path = dir.path().join("input.md");
        fs::write(&path, content).unwrap();
        let digest = sha256_hex(content);
        let fp = fingerprint("input.md", Some(&digest), content.len() as u64, 0);
        assert!(input_fingerprints_match(dir.path(), &[fp]).unwrap());
    }

    #[test]
    fn mismatched_digest_returns_false() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("input.md");
        fs::write(&path, b"hello canon input").unwrap();
        let fp = fingerprint(
            "input.md",
            Some("0000000000000000000000000000000000000000000000000000000000000000"),
            17,
            0,
        );
        assert!(!input_fingerprints_match(dir.path(), &[fp]).unwrap());
    }

    #[test]
    fn size_mismatch_without_digest_returns_false() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("input.md");
        fs::write(&path, b"hello canon input").unwrap();
        let fp = fingerprint("input.md", None, 9999, 0);
        assert!(!input_fingerprints_match(dir.path(), &[fp]).unwrap());
    }

    #[test]
    fn matching_size_and_mtime_without_digest_returns_true() {
        let dir = TempDir::new().unwrap();
        let content = b"hello canon input";
        let path = dir.path().join("input.md");
        fs::write(&path, content).unwrap();
        let metadata = std::fs::metadata(&path).unwrap();
        let mtime = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64)
            .unwrap_or_default();
        let fp = fingerprint("input.md", None, content.len() as u64, mtime);
        assert!(input_fingerprints_match(dir.path(), &[fp]).unwrap());
    }
}
