use anyhow::Result;
use colored::*;
use std::process::Command;
use crate::resume::{ResumeManager, SessionHandoff};

pub fn generate_auto_handoff() -> Result<()> {
    println!("{}", "🤖 Auto-generating Session Handoff Snapshot via Git & Environment...".bold().cyan());

    // 1. Detect project name from current working directory
    let cwd = std::env::current_dir()?;
    let project_name = cwd
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "agent-brain".to_string());

    // 2. Auto-detect modified files via git status
    let mut files_modified = Vec::new();
    if let Ok(out) = Command::new("git").arg("status").arg("--porcelain").output() {
        if out.status.success() {
            let stdout = String::from_utf8_lossy(&out.stdout);
            for line in stdout.lines() {
                let trimmed = line.trim();
                if trimmed.len() > 3 {
                    let file_path = trimmed[3..].trim().to_string();
                    files_modified.push(file_path);
                }
            }
        }
    }

    if files_modified.is_empty() {
        files_modified.push("No uncommitted file changes".to_string());
    }

    // 3. Auto-detect latest git commit message for goal
    let mut goal = format!("Session work in {}", project_name);
    if let Ok(out) = Command::new("git").arg("log").arg("-1").arg("--pretty=format:%s").output() {
        if out.status.success() {
            let commit_msg = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !commit_msg.is_empty() {
                goal = format!("Git Commit: {}", commit_msg);
            }
        }
    }

    // 4. Auto-detect key decisions & TODOs
    let key_decisions = vec![
        format!("Auto-detected {} modified file(s) in codebase", files_modified.len()),
        "Recorded project session state via agent-brain auto-handoff".to_string(),
    ];

    let unfinished_todos = vec![
        "Continue session development and verify unit tests".to_string(),
    ];

    let session_id = format!("session-{}", chrono::Local::now().format("%Y%m%d-%H%M%S"));
    let date = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();

    let handoff = SessionHandoff {
        id: session_id,
        date,
        project_name: project_name.clone(),
        goal: goal.clone(),
        files_modified,
        key_decisions,
        unfinished_todos,
    };

    let resume_mgr = ResumeManager::new()?;
    resume_mgr.add_session(handoff.clone())?;

    println!();
    println!("{}", "✨ Auto-Handoff Snapshot Created Successfully (Zero Manual Typing)!".bold().green());
    resume_mgr.render_session_card(&handoff);

    Ok(())
}
