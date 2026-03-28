use std::fs;
use std::path::Path;

use time::OffsetDateTime;

use crate::{AdapterError, AdapterInvocation, AdapterKind, CapabilityKind, SideEffectClass};

#[derive(Debug, Clone, Default)]
pub struct FilesystemAdapter;

impl FilesystemAdapter {
    pub fn read_to_string_traced(
        &self,
        path: &Path,
        purpose: &str,
    ) -> Result<(String, AdapterInvocation), AdapterError> {
        let contents = fs::read_to_string(path)?;
        Ok((contents, self.read_invocation(path, purpose)))
    }

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
