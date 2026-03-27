use std::fs;
use std::path::Path;

use time::OffsetDateTime;

use crate::{AdapterError, AdapterInvocation, AdapterKind, CapabilityKind, SideEffectClass};

#[derive(Debug, Clone, Default)]
pub struct FilesystemAdapter;

impl FilesystemAdapter {
    pub fn create_dir_all(&self, path: &Path) -> Result<(), AdapterError> {
        fs::create_dir_all(path)?;
        Ok(())
    }

    pub fn create_dir_all_traced(
        &self,
        path: &Path,
        purpose: &str,
    ) -> Result<AdapterInvocation, AdapterError> {
        fs::create_dir_all(path)?;
        Ok(self.invocation(path, purpose))
    }

    pub fn write_text(&self, path: &Path, contents: &str) -> Result<(), AdapterError> {
        fs::write(path, contents)?;
        Ok(())
    }

    pub fn trace_write(&self, path: &Path, purpose: &str) -> AdapterInvocation {
        self.invocation(path, purpose)
    }

    fn invocation(&self, path: &Path, purpose: &str) -> AdapterInvocation {
        AdapterInvocation {
            adapter: AdapterKind::Filesystem,
            capability: CapabilityKind::WriteArtifact,
            purpose: format!("{purpose}: {}", path.display()),
            side_effect: SideEffectClass::ArtifactWrite,
            allowed: true,
            occurred_at: OffsetDateTime::now_utc(),
        }
    }
}
