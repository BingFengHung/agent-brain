use anyhow::Result;
use colored::*;
use crate::memory::MemoryManager;
use crate::resume::ResumeManager;

pub fn search_brain(query: &str) -> Result<()> {
    let query_lower = query.to_lowercase();
    println!("{}", format!("🔍 Searching brain for keyword: '{}'...", query).bold().cyan());
    println!();

    let memory_mgr = MemoryManager::new()?;
    let store = memory_mgr.load_store()?;

    let mut found_any = false;

    // 1. Search Preferences
    let matched_prefs: Vec<_> = store
        .preferences
        .iter()
        .filter(|p| p.content.to_lowercase().contains(&query_lower))
        .collect();

    if !matched_prefs.is_empty() {
        found_any = true;
        println!("{}", "🧠 Matching Preferences:".bold().yellow());
        for p in matched_prefs {
            println!("  [ID {}] {} ({})", p.id, p.content.green(), p.created_at.dimmed());
        }
        println!();
    }

    // 2. Search Session History
    let resume_mgr = ResumeManager::new()?;
    let sessions = resume_mgr.load_sessions()?;

    let matched_sessions: Vec<_> = sessions
        .iter()
        .filter(|s| {
            s.goal.to_lowercase().contains(&query_lower)
                || s.project_name.to_lowercase().contains(&query_lower)
                || s.key_decisions.iter().any(|d| d.to_lowercase().contains(&query_lower))
                || s.unfinished_todos.iter().any(|t| t.to_lowercase().contains(&query_lower))
        })
        .collect();

    if !matched_sessions.is_empty() {
        found_any = true;
        println!("{}", "📅 Matching Past Sessions:".bold().magenta());
        for s in matched_sessions {
            resume_mgr.render_session_card(s);
        }
    }

    if !found_any {
        println!("{}", "ℹ️ No matching memory or past sessions found.".dimmed());
    }

    Ok(())
}
