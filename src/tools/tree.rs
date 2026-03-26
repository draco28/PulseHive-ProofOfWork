use async_trait::async_trait;
use ignore::WalkBuilder;
use pulsehive::prelude::*;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::PathBuf;

pub struct TreeTool {
    repo_root: PathBuf,
}

impl TreeTool {
    pub fn new(repo_root: impl Into<PathBuf>) -> Self {
        Self {
            repo_root: repo_root.into(),
        }
    }
}

const EXCLUDED_DIRS: &[&str] = &[
    ".git",
    "node_modules",
    "target",
    "__pycache__",
    ".devstudio",
    ".venv",
    "dist",
    "build",
];

#[async_trait]
impl Tool for TreeTool {
    fn name(&self) -> &str {
        "tree"
    }

    fn description(&self) -> &str {
        "List directory structure"
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Directory path (defaults to repo root)"
                },
                "max_depth": {
                    "type": "integer",
                    "description": "Maximum depth (default: 3)"
                }
            }
        })
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> Result<ToolResult> {
        let subpath = params["path"].as_str().unwrap_or("");
        let max_depth = params["max_depth"].as_u64().unwrap_or(3) as usize;

        let base = if subpath.is_empty() {
            self.repo_root.clone()
        } else {
            self.repo_root.join(subpath)
        };

        if !base.is_dir() {
            return Ok(ToolResult::Error(format!(
                "Not a directory: {}",
                base.display()
            )));
        }

        let mut entries: BTreeMap<String, bool> = BTreeMap::new();

        let walker = WalkBuilder::new(&base)
            .max_depth(Some(max_depth))
            .hidden(false)
            .git_ignore(true)
            .build();

        for entry in walker.flatten() {
            let path = entry.path();
            if path == base {
                continue;
            }

            // Skip excluded directories
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if EXCLUDED_DIRS.contains(&name) && path.is_dir() {
                    continue;
                }
            }

            if let Ok(rel) = path.strip_prefix(&base) {
                let excluded = rel.ancestors().any(|a| {
                    a.file_name()
                        .and_then(|n| n.to_str())
                        .map(|n| EXCLUDED_DIRS.contains(&n))
                        .unwrap_or(false)
                });
                if !excluded {
                    entries.insert(rel.to_string_lossy().to_string(), path.is_dir());
                }
            }
        }

        let mut output = String::new();
        let base_name = base.file_name().and_then(|n| n.to_str()).unwrap_or(".");
        output.push_str(&format!("{}/\n", base_name));

        let sorted: Vec<_> = entries.into_iter().collect();
        for (i, (path, is_dir)) in sorted.iter().enumerate() {
            let depth = path.matches('/').count();
            let is_last = sorted
                .get(i + 1)
                .map(|(next, _)| next.matches('/').count() <= depth)
                .unwrap_or(true);

            let indent = "  ".repeat(depth);
            let connector = if is_last { "└── " } else { "├── " };
            let name = std::path::Path::new(path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(path);
            let suffix = if *is_dir { "/" } else { "" };
            output.push_str(&format!("{}{}{}{}\n", indent, connector, name, suffix));
        }

        Ok(ToolResult::text(output))
    }
}
