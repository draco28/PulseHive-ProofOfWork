use pulsehive::prelude::*;
use std::path::Path;
use std::sync::Arc;

use crate::tools::{FileReadTool, SearchTool, TreeTool};

pub fn build_explorer(
    task: &str,
    repo_path: &Path,
    provider_name: &str,
    model: &str,
) -> AgentDefinition {
    let system_prompt = format!(
        r#"You are a codebase exploration agent. Your job is to thoroughly understand the
repository structure, technology stack, dependencies, and existing code patterns
before any changes are made.

Given the task: "{task}"
Repository: {repo}

Use the `tree` tool to see the directory structure.
Use the `search` tool to find code related to the task.
Use the `file_read` tool to read key files (package.json, Cargo.toml, README, main entry points).

Provide a comprehensive summary of:
1. Technology stack and framework
2. Project structure (key directories and files)
3. Existing patterns relevant to the task
4. Dependencies that may be needed
5. Files that will likely need modification"#,
        task = task,
        repo = repo_path.display(),
    );

    AgentDefinition {
        name: "explorer".into(),
        kind: AgentKind::Llm(Box::new(LlmAgentConfig {
            system_prompt,
            tools: vec![
                Arc::new(FileReadTool::new(repo_path)),
                Arc::new(TreeTool::new(repo_path)),
                Arc::new(SearchTool::new(repo_path)),
            ],
            lens: Lens::new(["code", "architecture"]),
            llm_config: LlmConfig::new(provider_name, model),
            experience_extractor: None,
            refresh_every_n_tool_calls: None,
        })),
    }
}
