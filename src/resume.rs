use anyhow::Result;
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use inquire::Select;
use crate::injector::sync_project_context;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionHandoff {
    pub id: String,
    pub date: String,
    pub project_name: String,
    pub goal: String,
    pub files_modified: Vec<String>,
    pub key_decisions: Vec<String>,
    pub unfinished_todos: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SessionStore {
    pub sessions: Vec<SessionHandoff>,
}

pub struct ResumeManager {
    store_path: PathBuf,
}

impl ResumeManager {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        let dir = home.join(".agent-brain");
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        let store_path = dir.join("sessions.json");
        Ok(Self { store_path })
    }

    pub fn load_sessions(&self) -> Result<Vec<SessionHandoff>> {
        if !self.store_path.exists() {
            return Ok(Vec::new());
        }
        let content = fs::read_to_string(&self.store_path)?;
        // Support both JSON Array format and SessionStore wrapper object format
        if let Ok(vec) = serde_json::from_str::<Vec<SessionHandoff>>(&content) {
            return Ok(vec);
        }
        if let Ok(store) = serde_json::from_str::<SessionStore>(&content) {
            return Ok(store.sessions);
        }
        Ok(Vec::new())
    }

    pub fn add_session(&self, handoff: SessionHandoff) -> Result<()> {
        let mut sessions = self.load_sessions()?;
        sessions.insert(0, handoff); // newest first
        let content = serde_json::to_string_pretty(&sessions)?;
        fs::write(&self.store_path, content)?;
        Ok(())
    }

    pub fn render_session_card(&self, s: &SessionHandoff) {
        println!("{}", "─────────────────────────────────────────────────────────────".cyan());
        println!("📅 [{}] Project: {}", s.date.bold().magenta(), s.project_name.bold().yellow());
        println!("🎯 Goal: {}", s.goal.cyan());
        println!("🛠️ Files: {}", if s.files_modified.is_empty() { "None".dimmed() } else { s.files_modified.join(", ").dimmed() });
        println!("💡 Decisions:");
        for d in &s.key_decisions {
            println!("   • {}", d);
        }
        println!("⚠️ Unfinished TODOs:");
        for t in &s.unfinished_todos {
            println!("   • {}", t.yellow());
        }
        println!("{}", "─────────────────────────────────────────────────────────────".cyan());
    }

    pub fn select_and_resume_session(&self, is_interactive: bool, target_index: Option<usize>) -> Result<()> {
        let sessions = self.load_sessions()?;
        if sessions.is_empty() {
            println!("{}", "   (No session handoffs recorded yet. Create one via `agent-brain handoff --auto`)".dimmed());
            return Ok(());
        }

        // Direct index selection (e.g. `agent-brain resume 1`)
        if let Some(idx) = target_index {
            if idx > 0 && idx <= sessions.len() {
                let selected = &sessions[idx - 1];
                println!();
                println!("{}", format!("✨ Restoring Selected Session #{} Memory:", idx).bold().green());
                self.render_session_card(selected);

                // Sync selected session into AGENTS.md
                sync_project_context()?;
                println!("{}", format!("🚀 Successfully restored session state [{}] into AGENTS.md!", selected.id).bold().magenta());
                return Ok(());
            } else {
                println!("{}", format!("⚠️ Invalid session index {}. Total sessions available: {}", idx, sessions.len()).yellow());
                return Ok(());
            }
        }

        if is_interactive {
            let options: Vec<String> = sessions
                .iter()
                .enumerate()
                .map(|(i, s)| format!("{}. [{}] {} - {}", i + 1, s.date, s.project_name, s.goal))
                .collect();

            let ans = Select::new("📜 Select a past session snapshot to inspect & inject into AGENTS.md:", options).prompt()?;

            if let Some(idx_str) = ans.split('.').next() {
                if let Ok(idx) = idx_str.parse::<usize>() {
                    if idx > 0 && idx <= sessions.len() {
                        let selected = &sessions[idx - 1];
                        println!();
                        println!("{}", "✨ Loaded Selected Session Memory:".bold().green());
                        self.render_session_card(selected);

                        // Sync selected session into AGENTS.md
                        sync_project_context()?;
                        println!("{}", format!("🚀 Successfully restored session state [{}] into AGENTS.md!", selected.id).bold().magenta());
                    }
                }
            }
        } else {
            // Non-interactive fallback: list recent sessions with clear numbers
            println!("{}", "📜 Past Session Handoff Snapshots (Use `! agent-brain resume <number>` to load a specific session):".bold().magenta());
            println!();
            for (i, s) in sessions.iter().enumerate().take(5) {
                println!("{}", format!("👉 Session #{}: Type `! agent-brain resume {}` to restore this session", i + 1, i + 1).bold().cyan());
                self.render_session_card(s);
            }
        }

        Ok(())
    }
}
