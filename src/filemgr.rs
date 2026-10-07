use serde::Serialize;
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;
use zero_vfs::{
    detect_mime, is_text_editable, AtomicPathBuilder, JailSandbox, VfsEntry, VfsFileType,
};

/// Serializable DTO for VfsEntry for JSON API.
#[derive(Debug, Clone, Serialize)]
pub struct VfsEntryDto {
    pub name: String,
    pub rel_path: String,
    pub file_type: String,
    pub size_bytes: u64,
    pub modified_epoch_sec: u64,
    pub posix_mode: u32,
}

impl From<VfsEntry> for VfsEntryDto {
    fn from(e: VfsEntry) -> Self {
        Self {
            name: e.name,
            rel_path: e.rel_path,
            file_type: format!("{:?}", e.file_type),
            size_bytes: e.size_bytes,
            modified_epoch_sec: e.modified_epoch_sec,
            posix_mode: e.posix_mode,
        }
    }
}

pub struct FileManager;

impl FileManager {
    /// Lists entries in a directory confined within the jail sandbox.
    pub fn list_dir(jail_root: &str, rel_path: &str) -> Result<Vec<VfsEntryDto>, String> {
        let sandbox = JailSandbox::new(jail_root).map_err(|e| e.to_string())?;
        let abs_dir = sandbox.resolve(rel_path).map_err(|e| e.to_string())?;

        let dir_path = Path::new(&abs_dir);
        if !dir_path.exists() {
            return Err(format!("Directory does not exist: {rel_path}"));
        }
        if !dir_path.is_dir() {
            return Err(format!("Target is not a directory: {rel_path}"));
        }

        let read_dir = fs::read_dir(dir_path)
            .map_err(|e| format!("Failed to read directory {rel_path}: {e}"))?;

        let mut entries: Vec<VfsEntry> = Vec::new();

        for entry in read_dir.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let full_path = entry.path();
            let full_path_str = full_path.to_string_lossy();
            let rel = sandbox
                .relative_to_jail(&full_path_str)
                .unwrap_or(&name)
                .to_string();

            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            let file_type = if metadata.is_dir() {
                VfsFileType::Directory
            } else if metadata.is_symlink() {
                VfsFileType::Symlink
            } else {
                VfsFileType::File
            };

            let size_bytes = if metadata.is_dir() { 0 } else { metadata.len() };

            let modified_epoch_sec = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);

            #[cfg(unix)]
            let posix_mode = {
                use std::os::unix::fs::PermissionsExt;
                metadata.permissions().mode()
            };

            #[cfg(not(unix))]
            let posix_mode = if metadata.is_dir() { 0o755 } else { 0o644 };

            entries.push(VfsEntry::new(
                name,
                rel,
                file_type,
                size_bytes,
                modified_epoch_sec,
                posix_mode,
            ));
        }

        // Sort: directories first, then alphabetically
        entries.sort_by(|a, b| match (a.is_dir(), b.is_dir()) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });

        Ok(entries.into_iter().map(VfsEntryDto::from).collect())
    }

    /// Reads text file content safely within jail sandbox.
    ///
    /// Limits reads to 10 MB to prevent out-of-memory exhaustion in browser.
    pub fn read_file(jail_root: &str, rel_path: &str) -> Result<(String, &'static str), String> {
        let sandbox = JailSandbox::new(jail_root).map_err(|e| e.to_string())?;
        let abs_file = sandbox.resolve(rel_path).map_err(|e| e.to_string())?;

        let path = Path::new(&abs_file);
        if !path.exists() {
            return Err(format!("File does not exist: {rel_path}"));
        }
        if path.is_dir() {
            return Err(format!("Cannot view directory as file: {rel_path}"));
        }

        let mime = detect_mime(&abs_file);
        if !is_text_editable(&abs_file) {
            return Err(format!(
                "Binary file ({mime}) cannot be opened in text editor"
            ));
        }

        let metadata = fs::metadata(path).map_err(|e| e.to_string())?;
        if metadata.len() > 10 * 1024 * 1024 {
            return Err(format!(
                "File too large to open in browser editor ({} MB > 10 MB limit)",
                metadata.len() / (1024 * 1024)
            ));
        }

        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read file {rel_path}: {e}"))?;

        Ok((content, mime))
    }

    /// Atomically saves file content using temporary file + rename protocol.
    pub fn save_file(jail_root: &str, rel_path: &str, content: &str) -> Result<(), String> {
        let sandbox = JailSandbox::new(jail_root).map_err(|e| e.to_string())?;
        let abs_file = sandbox.resolve(rel_path).map_err(|e| e.to_string())?;

        let builder = AtomicPathBuilder::new(&abs_file).map_err(|e| e.to_string())?;
        let temp_path = builder.temp_path();

        // Ensure parent directory exists
        if let Some(parent) = Path::new(&abs_file).parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create parent directory: {e}"))?;
        }

        // 1. Write to sibling temporary file
        fs::write(&temp_path, content)
            .map_err(|e| format!("Failed to write to temporary file {temp_path}: {e}"))?;

        // 2. Atomic rename
        if let Err(e) = fs::rename(&temp_path, &abs_file) {
            // Clean up temporary file on failure
            let _ = fs::remove_file(&temp_path);
            return Err(format!("Atomic rename failed for {abs_file}: {e}"));
        }

        Ok(())
    }

    /// Saves binary content safely into a file within the jail sandbox using atomic rename.
    pub fn write_file_bytes(jail_root: &str, rel_path: &str, data: &[u8]) -> Result<(), String> {
        let sandbox = JailSandbox::new(jail_root).map_err(|e| e.to_string())?;
        let abs_file = sandbox.resolve(rel_path).map_err(|e| e.to_string())?;

        let builder = AtomicPathBuilder::new(&abs_file).map_err(|e| e.to_string())?;
        let temp_path = builder.temp_path();

        // Ensure parent directory exists
        if let Some(parent) = Path::new(&abs_file).parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create parent directory: {e}"))?;
        }

        // 1. Write to sibling temporary file
        fs::write(&temp_path, data)
            .map_err(|e| format!("Failed to write to temporary file: {e}"))?;

        // 2. Atomic rename
        if let Err(e) = fs::rename(&temp_path, &abs_file) {
            let _ = fs::remove_file(&temp_path);
            return Err(format!("Atomic rename failed for {abs_file}: {e}"));
        }

        Ok(())
    }

    /// Deletes a file or directory safely confined within jail sandbox.
    pub fn delete_entry(jail_root: &str, rel_path: &str) -> Result<(), String> {
        let sandbox = JailSandbox::new(jail_root).map_err(|e| e.to_string())?;
        let abs_path = sandbox.resolve(rel_path).map_err(|e| e.to_string())?;

        // Safety check: Never delete the jail root itself!
        if abs_path == sandbox.jail_root() {
            return Err("Forbidden: Cannot delete the web root directory itself".to_string());
        }

        let path = Path::new(&abs_path);
        if !path.exists() {
            return Err(format!("Target does not exist: {rel_path}"));
        }

        if path.is_dir() {
            fs::remove_dir_all(path)
                .map_err(|e| format!("Failed to remove directory {rel_path}: {e}"))?;
        } else {
            fs::remove_file(path).map_err(|e| format!("Failed to remove file {rel_path}: {e}"))?;
        }

        Ok(())
    }

    /// Creates a new empty file or directory.
    pub fn create_entry(jail_root: &str, rel_path: &str, is_dir: bool) -> Result<(), String> {
        let sandbox = JailSandbox::new(jail_root).map_err(|e| e.to_string())?;
        let abs_path = sandbox.resolve(rel_path).map_err(|e| e.to_string())?;

        let path = Path::new(&abs_path);
        if path.exists() {
            return Err(format!("Target already exists: {rel_path}"));
        }

        if is_dir {
            fs::create_dir_all(path)
                .map_err(|e| format!("Failed to create directory {rel_path}: {e}"))?;
        } else {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create parent directory: {e}"))?;
            }
            fs::write(path, b"")
                .map_err(|e| format!("Failed to create empty file {rel_path}: {e}"))?;
        }

        Ok(())
    }
}
