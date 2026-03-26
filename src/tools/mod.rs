pub mod file_read;
pub mod file_write;
pub mod search;
pub mod shell_exec;
pub mod tree;

pub use file_read::FileReadTool;
pub use file_write::FileWriteTool;
pub use search::SearchTool;
pub use shell_exec::ShellExecTool;
pub use tree::TreeTool;

use pulsehive::prelude::PulseHiveError;
use std::path::{Path, PathBuf};

/// Validate that a relative path resolves within the repo root.
/// Prevents directory traversal attacks.
pub fn validate_path(repo_root: &Path, relative: &str) -> Result<PathBuf, PulseHiveError> {
    let full = repo_root.join(relative);
    let canonical = full
        .canonicalize()
        .map_err(|e| PulseHiveError::tool(format!("Invalid path '{}': {e}", relative)))?;
    let canonical_root = repo_root
        .canonicalize()
        .map_err(|e| PulseHiveError::tool(format!("Invalid repo root: {e}")))?;
    if !canonical.starts_with(&canonical_root) {
        return Err(PulseHiveError::tool(
            "Path traversal blocked — must stay within repo",
        ));
    }
    Ok(canonical)
}

/// Validate that the parent directory of a path is within repo root.
/// Used for file_write where the file itself may not exist yet.
pub fn validate_parent_path(repo_root: &Path, relative: &str) -> Result<PathBuf, PulseHiveError> {
    let full = repo_root.join(relative);
    let parent = full
        .parent()
        .ok_or_else(|| PulseHiveError::tool("Invalid path: no parent directory"))?;

    // Create parent dirs if they don't exist
    std::fs::create_dir_all(parent)
        .map_err(|e| PulseHiveError::tool(format!("Cannot create directories: {e}")))?;

    let canonical_parent = parent
        .canonicalize()
        .map_err(|e| PulseHiveError::tool(format!("Invalid parent path: {e}")))?;
    let canonical_root = repo_root
        .canonicalize()
        .map_err(|e| PulseHiveError::tool(format!("Invalid repo root: {e}")))?;
    if !canonical_parent.starts_with(&canonical_root) {
        return Err(PulseHiveError::tool(
            "Path traversal blocked — must stay within repo",
        ));
    }

    Ok(full)
}
