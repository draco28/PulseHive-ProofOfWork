use pulsehive::prelude::*;
use std::path::Path;
use std::sync::Arc;

use crate::tools::{FileReadTool, FileWriteTool, ShellExecTool};

pub fn build_coder(
    task: &str,
    repo_path: &Path,
    provider_name: &str,
    model: &str,
) -> AgentDefinition {
    let system_prompt = format!(
        r#"You are a code implementation agent. Follow the implementation plan from the
planning phase (available in your context) and write the actual code.

Task: "{task}"

Rules:
- Follow the plan step by step
- Write production-quality code with proper error handling
- Follow existing code conventions in the repository
- Use file_write to create or modify files
- Use shell_exec for package installations (npm install, pip install, etc.)
- Do NOT run tests — the tester agent handles that
- After each file modification, briefly note what you changed and why"#,
        task = task,
    );

    let mut lens = Lens::new(["code", "implementation"]);
    lens.attention_budget = 100;

    AgentDefinition {
        name: "coder".into(),
        kind: AgentKind::Llm(Box::new(LlmAgentConfig {
            system_prompt,
            tools: vec![
                Arc::new(FileReadTool::new(repo_path)),
                Arc::new(FileWriteTool::new(repo_path)),
                Arc::new(ShellExecTool::new(repo_path)),
            ],
            lens,
            llm_config: LlmConfig::new(provider_name, model),
            experience_extractor: None,
            refresh_every_n_tool_calls: None,
        })),
    }
}
