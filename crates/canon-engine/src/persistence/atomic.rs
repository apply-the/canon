use std::fs;
use std::path::Path;

/// Writes `contents` to `path`, creating parent directories as needed.
pub fn write_text_file(path: &Path, contents: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, contents)
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::write_text_file;

    #[test]
    fn write_text_file_creates_parent_directories_as_needed() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("nested").join("sub").join("file.txt");
        write_text_file(&path, "hello").expect("write should succeed");
        assert_eq!(std::fs::read_to_string(&path).expect("read"), "hello");
    }
}
