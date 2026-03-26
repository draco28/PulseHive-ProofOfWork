use async_trait::async_trait;
use ignore::WalkBuilder;
use pulsehive::prelude::*;
use regex::Regex;
use serde_json::{json, Value};
use std::io::BufRead;
use std::path::PathBuf;

const MAX_RESULTS: usize = 50;

pub struct SearchTool {
    repo_root: PathBuf,
}

impl SearchTool {
    pub fn new(repo_root: impl Into<PathBuf>) -> Self {
        Self {
            repo_root: repo_root.into(),
        }
    }
}

#[async_trait]
impl Tool for SearchTool {
    fn name(&self) -> &str {
        "search"
    }

    fn description(&self) -> &str {
        "Search for a pattern in files (like grep)"
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "Search pattern (regex)"
                },
                "path": {
                    "type": "string",
                    "description": "Directory to search (default: repo root)"
                },
                "file_pattern": {
                    "type": "string",
                    "description": "File glob filter (e.g., '*.ts')"
                }
            },
            "required": ["pattern"]
        })
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> Result<ToolResult> {
        let pattern = params["pattern"].as_str().unwrap_or("");
        let subpath = params["path"].as_str().unwrap_or("");
        let file_pattern = params["file_pattern"].as_str();

        let re = Regex::new(pattern)
            .map_err(|e| PulseHiveError::tool(format!("Invalid regex '{}': {e}", pattern)))?;

        let base = if subpath.is_empty() {
            self.repo_root.clone()
        } else {
            self.repo_root.join(subpath)
        };

        let mut builder = WalkBuilder::new(&base);
        builder.hidden(false).git_ignore(true);

        let mut results = Vec::new();

        for entry in builder.build().flatten() {
            if results.len() >= MAX_RESULTS {
                break;
            }

            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            // Apply file pattern filter
            if let Some(fp) = file_pattern {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if let Ok(glob) = glob::Pattern::new(fp) {
                        if !glob.matches(name) {
                            continue;
                        }
                    }
                }
            }

            let file = match std::fs::File::open(path) {
                Ok(f) => f,
                Err(_) => continue,
            };
            let reader = std::io::BufReader::new(file);

            let rel_path = path
                .strip_prefix(&self.repo_root)
                .unwrap_or(path)
                .to_string_lossy();

            for (line_num, line) in reader.lines().enumerate() {
                if results.len() >= MAX_RESULTS {
                    break;
                }
                if let Ok(line) = line {
                    if re.is_match(&line) {
                        results.push(format!("{}:{}: {}", rel_path, line_num + 1, line.trim()));
                    }
                }
            }
        }

        if results.is_empty() {
            Ok(ToolResult::text(format!(
                "No matches found for '{}'",
                pattern
            )))
        } else {
            let count = results.len();
            let mut output = results.join("\n");
            if count == MAX_RESULTS {
                output.push_str(&format!("\n\n(showing first {} results)", MAX_RESULTS));
            }
            Ok(ToolResult::text(output))
        }
    }
}
