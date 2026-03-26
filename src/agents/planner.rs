use pulsehive::prelude::*;
use std::path::Path;
use std::sync::Arc;

use crate::tools::FileReadTool;

pub fn build_planner(
    task: &str,
    repo_path: &Path,
    provider_name: &str,
    model: &str,
) -> AgentDefinition {
    let system_prompt = format!(
        r#"You are an implementation planning agent. Based on the codebase understanding
from the exploration phase (available in your context), create a detailed
step-by-step implementation plan.

Task: "{task}"

Your plan should include:
1. Files to create (with purpose)
2. Files to modify (with specific changes)
3. Dependencies to install
4. Configuration changes needed
5. Order of operations (what to do first)
6. Potential risks or edge cases

Be specific — the coder agent will follow your plan exactly.
Do NOT write code. Only describe what needs to be done."#,
        task = task,
    );

    let mut lens = Lens::new(["architecture", "requirements"]);
    lens.attention_budget = 100;

    AgentDefinition {
        name: "planner".into(),
        kind: AgentKind::Llm(Box::new(LlmAgentConfig {
            system_prompt,
            tools: vec![Arc::new(FileReadTool::new(repo_path))],
            lens,
            llm_config: LlmConfig::new(provider_name, model),
            experience_extractor: None,
            refresh_every_n_tool_calls: None,
        })),
    }
}
