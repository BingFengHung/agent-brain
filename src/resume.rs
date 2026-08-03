use anyhow::{anyhow, Result};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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

pub struct ResumeManager {
    base_dir: PathBuf,
}

impl ResumeManager {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().ok_or_else(|| anyhow!("Could not locate home directory"))?;
        let base_dir = home.join(".agent-brain");

        if !base_dir.exists() {
            fs::create_dir_all(&base_dir)?;
        }

        Ok(Self { base_dir })
    }

    fn sessions_file_path(&self) -> PathBuf {
        self.base_dir.join("sessions.json")
    }

    pub fn load_sessions(&self) -> Result<Vec<SessionHandoff>> {
        let path = self.sessions_file_path();
        if !path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(path)?;
        let sessions: Vec<SessionHandoff> = serde_json::from_str(&content)?;
        Ok(sessions)
    }

    pub fn save_sessions(&self, sessions: &[SessionHandoff]) -> Result<()> {
        let path = self.sessions_file_path();
        let json = serde_json::to_string_pretty(sessions)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn add_session(&self, session: SessionHandoff) -> Result<()> {
        let mut sessions = self.load_sessions()?;
        sessions.retain(|s| s.id != session.id);
        sessions.insert(0, session); // latest session on top
        self.save_sessions(&sessions)?;
        Ok(())
    }

    pub fn render_session_card(&self, s: &SessionHandoff) {
        println!("{}", "─".repeat(65).dimmed());
        println!(
            "{} [{}] {} {}",
            "📅".cyan(),
            s.date.bold().yellow(),
            "Project:".dimmed(),
            s.project_name.bold().green()
        );
        println!("{} {}", "🎯 Goal:".bold().magenta(), s.goal);
        if !s.files_modified.is_empty() {
            println!("{} {}", "🛠️ Files:".bold().blue(), s.files_modified.join(", ").dimmed());
        }
        if !s.key_decisions.is_empty() {
            println!("{}", "💡 Decisions:".bold().yellow());
            for d in &s.key_decisions {
                println!("   • {}", d);
            }
        }
        if !s.unfinished_todos.is_empty() {
            println!("{}", "⚠️ Unfinished TODOs:".bold().red());
            for todo in &s.unfinished_todos {
                println!("   • {}", todo);
            }
        }
        println!("{}", "─".repeat(65).dimmed());
        println!();
    }
}
