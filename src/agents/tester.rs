use pulsehive::prelude::*;
use std::path::Path;
use std::sync::Arc;

use crate::tools::{FileReadTool, FileWriteTool, ShellExecTool};

pub fn build_tester(
    task: &str,
    repo_path: &Path,
    provider_name: &str,
    model: &str,
) -> AgentDefinition {
    let system_prompt = format!(
        r#"You are a testing agent. Based on the code changes made by the coder agent
(available in your context), write appropriate tests and run them.

Task: "{task}"

Rules:
- Read the modified/created files to understand what was implemented
- Write tests that cover the main functionality
- Follow the project's existing test conventions (framework, directory, naming)
- Use shell_exec to run the test suite
- Report test results clearly: passed, failed, errors
- If tests fail, describe what went wrong (but don't fix the code)"#,
        task = task,
    );

    let mut lens = Lens::new(["testing", "quality"]);
    lens.attention_budget = 100;

    AgentDefinition {
        name: "tester".into(),
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
