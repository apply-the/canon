use std::fs;
use std::path::Path;

use time::OffsetDateTime;

use crate::{AdapterError, AdapterInvocation, AdapterKind, CapabilityKind, SideEffectClass};

/// A zero-cost adapter that wraps `std::fs` operations with Canon tracing.
///
/// Every method either returns an [`AdapterInvocation`] alongside the result
/// (traced variants) or returns a bare result (untraced variants). Callers
/// that need an audit trail should use the traced variants.
#[derive(Debug, Clone, Default)]
pub struct FilesystemAdapter;

impl FilesystemAdapter {
    /// Reads the file at `path` to a `String` and returns it with an invocation record.
    pub fn read_to_string_traced(
        &self,
        path: &Path,
        purpose: &str,
    ) -> Result<(String, AdapterInvocation), AdapterError> {
        let contents = fs::read_to_string(path)?;
        Ok((contents, self.read_invocation(path, purpose)))
    }

    /// Creates all directories in `path`, equivalent to `mkdir -p`.
    pub fn create_dir_all(&self, path: &Path) -> Result<(), AdapterError> {
        fs::create_dir_all(path)?;
        Ok(())
    }

    /// Creates all directories in `path` and returns an invocation record.
    pub fn create_dir_all_traced(
        &self,
        path: &Path,
        purpose: &str,
    ) -> Result<AdapterInvocation, AdapterError> {
        fs::create_dir_all(path)?;
        Ok(self.invocation(path, purpose))
    }

    /// Writes `contents` as UTF-8 text to `path`, creating or truncating the file.
    pub fn write_text(&self, path: &Path, contents: &str) -> Result<(), AdapterError> {
        fs::write(path, contents)?;
        Ok(())
    }

    /// Returns a write invocation record for tracing without actually writing anything.
    pub fn trace_write(&self, path: &Path, purpose: &str) -> AdapterInvocation {
        self.invocation(path, purpose)
    }

    fn read_invocation(&self, path: &Path, purpose: &str) -> AdapterInvocation {
        AdapterInvocation {
            adapter: AdapterKind::Filesystem,
            capability: CapabilityKind::ReadRepository,
            purpose: format!("{purpose}: {}", path.display()),
            side_effect: SideEffectClass::ReadOnly,
            allowed: true,
            occurred_at: OffsetDateTime::now_utc(),
        }
    }

    fn invocation(&self, path: &Path, purpose: &str) -> AdapterInvocation {
        AdapterInvocation {
            adapter: AdapterKind::Filesystem,
            capability: CapabilityKind::EmitArtifact,
            purpose: format!("{purpose}: {}", path.display()),
            side_effect: SideEffectClass::ArtifactWrite,
            allowed: true,
            occurred_at: OffsetDateTime::now_utc(),
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::FilesystemAdapter;

    #[test]
    fn write_text_creates_file_with_given_contents() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("output.txt");
        FilesystemAdapter.write_text(&path, "hello world").expect("write should succeed");
        assert_eq!(std::fs::read_to_string(&path).expect("read"), "hello world");
    }
}
