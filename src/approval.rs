use async_trait::async_trait;
use colored::Colorize;
use pulsehive::prelude::*;
use std::io::{self, BufRead, Write};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct CliApprovalHandler {
    approve_all: AtomicBool,
}

impl CliApprovalHandler {
    pub fn new(approve_all: bool) -> Self {
        Self {
            approve_all: AtomicBool::new(approve_all),
        }
    }
}

#[async_trait]
impl ApprovalHandler for CliApprovalHandler {
    async fn request_approval(&self, action: &PendingAction) -> Result<ApprovalResult> {
        if self.approve_all.load(Ordering::Relaxed) {
            return Ok(ApprovalResult::Approved);
        }

        eprintln!();
        eprintln!(
            "{}",
            format!("  {} wants to execute: {}", action.tool_name, action.description)
                .yellow()
                .bold()
        );

        // Show relevant params
        if let Some(path) = action.params.get("path").and_then(|v| v.as_str()) {
            eprintln!("  {} {}", "Path:".dimmed(), path);
        }
        if let Some(command) = action.params.get("command").and_then(|v| v.as_str()) {
            eprintln!("  {} {}", "Command:".dimmed(), command);
        }
        if let Some(content) = action.params.get("content").and_then(|v| v.as_str()) {
            let preview: Vec<&str> = content.lines().take(5).collect();
            for line in &preview {
                eprintln!("  {} {}", "+".green(), line);
            }
            let total_lines = content.lines().count();
            if total_lines > 5 {
                eprintln!("  {} ... ({} more lines)", "+".green(), total_lines - 5);
            }
        }

        eprintln!();
        eprint!("  {} ", "[y]es / [n]o / [a]pprove all:".cyan().bold());
        io::stderr().flush().ok();

        let stdin = io::stdin();
        let mut line = String::new();
        stdin
            .lock()
            .read_line(&mut line)
            .map_err(|e| PulseHiveError::tool(format!("Failed to read input: {e}")))?;

        match line.trim().to_lowercase().as_str() {
            "y" | "yes" => Ok(ApprovalResult::Approved),
            "a" | "all" => {
                self.approve_all.store(true, Ordering::Relaxed);
                Ok(ApprovalResult::Approved)
            }
            _ => Ok(ApprovalResult::Denied {
                reason: "User denied the action".to_string(),
            }),
        }
    }
}
