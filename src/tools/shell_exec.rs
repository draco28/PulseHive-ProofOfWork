use async_trait::async_trait;
use pulsehive::prelude::*;
use serde_json::{json, Value};
use std::path::PathBuf;
use tokio::process::Command;
use tokio::time::{timeout, Duration};

pub struct ShellExecTool {
    repo_root: PathBuf,
}

impl ShellExecTool {
    pub fn new(repo_root: impl Into<PathBuf>) -> Self {
        Self {
            repo_root: repo_root.into(),
        }
    }
}

#[async_trait]
impl Tool for ShellExecTool {
    fn name(&self) -> &str {
        "shell_exec"
    }

    fn description(&self) -> &str {
        "Execute a shell command in the repository directory"
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "Shell command to execute"
                },
                "cwd": {
                    "type": "string",
                    "description": "Working directory (defaults to repo root)"
                }
            },
            "required": ["command"]
        })
    }

    fn requires_approval(&self) -> bool {
        true
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> Result<ToolResult> {
        let command = params["command"].as_str().unwrap_or("");
        let cwd = params["cwd"]
            .as_str()
            .map(|s| self.repo_root.join(s))
            .unwrap_or_else(|| self.repo_root.clone());

        let result = timeout(
            Duration::from_secs(60),
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .current_dir(&cwd)
                .output(),
        )
        .await;

        match result {
            Ok(Ok(output)) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                let mut combined = String::new();
                if !stdout.is_empty() {
                    combined.push_str(&stdout);
                }
                if !stderr.is_empty() {
                    if !combined.is_empty() {
                        combined.push('\n');
                    }
                    combined.push_str(&stderr);
                }
                if !output.status.success() {
                    combined.push_str(&format!("\n[exit code: {}]", output.status));
                }
                Ok(ToolResult::text(combined))
            }
            Ok(Err(e)) => Ok(ToolResult::Error(format!(
                "Failed to execute command: {e}"
            ))),
            Err(_) => Ok(ToolResult::Error(
                "Command timed out after 60 seconds".to_string(),
            )),
        }
    }
}
