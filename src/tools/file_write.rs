use async_trait::async_trait;
use pulsehive::prelude::*;
use serde_json::{json, Value};
use std::path::PathBuf;

use super::validate_parent_path;

pub struct FileWriteTool {
    repo_root: PathBuf,
}

impl FileWriteTool {
    pub fn new(repo_root: impl Into<PathBuf>) -> Self {
        Self {
            repo_root: repo_root.into(),
        }
    }
}

#[async_trait]
impl Tool for FileWriteTool {
    fn name(&self) -> &str {
        "file_write"
    }

    fn description(&self) -> &str {
        "Write content to a file, creating it if it doesn't exist"
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "File path relative to repo root"
                },
                "content": {
                    "type": "string",
                    "description": "Complete file content to write"
                }
            },
            "required": ["path", "content"]
        })
    }

    fn requires_approval(&self) -> bool {
        true
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> Result<ToolResult> {
        let path = params["path"].as_str().unwrap_or("");
        let content = params["content"].as_str().unwrap_or("");
        let full_path = validate_parent_path(&self.repo_root, path)?;
        tokio::fs::write(&full_path, content).await.map_err(|e| {
            PulseHiveError::tool(format!("Cannot write '{}': {e}", path))
        })?;
        let line_count = content.lines().count();
        Ok(ToolResult::text(format!(
            "Written {} ({} lines)",
            path, line_count
        )))
    }
}
