use pulsehive::prelude::*;
use std::path::Path;

use crate::agents::{build_coder, build_explorer, build_planner, build_tester};

/// Build the full 4-agent sequential pipeline: Explorer → Planner → Coder → Tester
pub fn build_pipeline(
    task: &str,
    repo_path: &Path,
    provider_name: &str,
    model: &str,
) -> AgentDefinition {
    AgentDefinition {
        name: "devstudio-pipeline".into(),
        kind: AgentKind::Sequential(vec![
            build_explorer(task, repo_path, provider_name, model),
            build_planner(task, repo_path, provider_name, model),
            build_coder(task, repo_path, provider_name, model),
            build_tester(task, repo_path, provider_name, model),
        ]),
    }
}

/// Build a dry-run pipeline: Explorer → Planner only (no code changes)
pub fn build_dry_run_pipeline(
    task: &str,
    repo_path: &Path,
    provider_name: &str,
    model: &str,
) -> AgentDefinition {
    AgentDefinition {
        name: "devstudio-pipeline".into(),
        kind: AgentKind::Sequential(vec![
            build_explorer(task, repo_path, provider_name, model),
            build_planner(task, repo_path, provider_name, model),
        ]),
    }
}
