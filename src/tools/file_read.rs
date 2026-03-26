use async_trait::async_trait;
use pulsehive::prelude::*;
use serde_json::{json, Value};
use std::path::PathBuf;

use super::validate_path;

pub struct FileReadTool {
    repo_root: PathBuf,
}

impl FileReadTool {
    pub fn new(repo_root: impl Into<PathBuf>) -> Self {
        Self {
            repo_root: repo_root.into(),
        }
    }
}

#[async_trait]
impl Tool for FileReadTool {
    fn name(&self) -> &str {
        "file_read"
    }

    fn description(&self) -> &str {
        "Read the contents of a file at the given path"
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "File path relative to repo root"
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> Result<ToolResult> {
        let path = params["path"].as_str().unwrap_or("");
        let full_path = validate_path(&self.repo_root, path)?;
        let content = tokio::fs::read_to_string(&full_path)
            .await
            .map_err(|e| PulseHiveError::tool(format!("Cannot read '{}': {e}", path)))?;
        Ok(ToolResult::text(content))
    }
}
