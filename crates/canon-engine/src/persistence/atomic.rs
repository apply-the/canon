use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// Writes `contents` to `path`, creating parent directories as needed.
pub fn write_text_file(path: &Path, contents: &str) -> std::io::Result<()> {
    write_bytes_durable(path, contents.as_bytes())
}

/// Atomically replaces one file after flushing data and its parent directory.
pub fn write_bytes_durable(path: &Path, contents: &[u8]) -> std::io::Result<()> {
    let parent = path.parent().ok_or_else(|| std::io::Error::other("missing parent directory"))?;
    fs::create_dir_all(parent)?;
    let temporary = parent.join(format!(
        ".{}.{}.tmp",
        path.file_name().and_then(|name| name.to_str()).unwrap_or("canon-state"),
        uuid::Uuid::now_v7()
    ));
    let mut file = File::create(&temporary)?;
    file.write_all(contents)?;
    file.sync_all()?;
    fs::rename(&temporary, path)?;
    File::open(parent)?.sync_all()
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::write_text_file;

    #[test]
    fn write_text_file_creates_parent_directories_as_needed()
    -> Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let path = dir.path().join("nested").join("sub").join("file.txt");
        write_text_file(&path, "hello")?;
        if std::fs::read_to_string(&path)? == "hello" {
            Ok(())
        } else {
            Err("durable text write changed content".into())
        }
    }
}
