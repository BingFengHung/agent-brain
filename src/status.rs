use anyhow::Result;
use colored::*;
use std::env;
use crate::memory::MemoryManager;
use crate::resume::ResumeManager;

pub fn render_brain_status() -> Result<()> {
    println!();
    println!("{}", "═══════════════════════════════════════════════════════════════".cyan());
    println!("{}", "               🧠 agent-brain Memory Dashboard                 ".bold().cyan());
    println!("{}", "═══════════════════════════════════════════════════════════════".cyan());
    println!();

    let memory_mgr = MemoryManager::new()?;
    let store = memory_mgr.load_store()?;

    // 1. Global Preferences / Rules
    println!("{}", "🧠 1. Stored Long-Term Memory Rules & Developer Habits:".bold().yellow());
    if store.preferences.is_empty() {
        println!("{}", "   (No rules stored yet. Run `agent-brain learn --global` to auto-learn!)".dimmed());
    } else {
        for p in &store.preferences {
            println!("   • [ID {}] {} ({})", p.id, p.content.green(), p.created_at.dimmed());
        }
    }
    println!();

    // 2. Local Project Sync Status
    let cwd = env::current_dir()?;
    println!("{} {}", "📂 2. Current Project Context Status:".bold().yellow(), cwd.display().to_string().cyan());

    let agents_md = cwd.join("AGENTS.md");
    let copilot_rules = cwd.join(".copilotrules");
    let copilot_instr = cwd.join(".github").join("copilot-instructions.md");

    let status_str = |path: &std::path::Path| {
        if path.exists() {
            "✅ Synced".green()
        } else {
            "❌ Missing (Run `agent-brain sync`)".yellow()
        }
    };

    println!("   • AGENTS.md (agy / Copilot CLI): {}", status_str(&agents_md));
    println!("   • .copilotrules (Copilot CLI): {}", status_str(&copilot_rules));
    println!("   • .github/copilot-instructions.md (VS Code Copilot): {}", status_str(&copilot_instr));
    println!();

    // 3. Past Sessions & Handoff Snapshots
    let resume_mgr = ResumeManager::new()?;
    let sessions = resume_mgr.load_sessions()?;
    println!("{}", "📜 3. Recorded Past Session Handoff Snapshots:".bold().yellow());
    if sessions.is_empty() {
        println!("{}", "   (No session handoffs recorded. Create one via `agent-brain handoff --auto`)".dimmed());
    } else {
        println!("   • Total Session Snapshots: {}", sessions.len().to_string().bold().green());
        if let Some(latest) = sessions.first() {
            println!("   • Latest Session Date: {}", latest.date.bold().magenta());
            println!("   • Latest Session Goal: {}", latest.goal.dimmed());
        }
    }

    println!();
    println!("{}", "═══════════════════════════════════════════════════════════════".cyan());
    println!("{}", "💡 Tip: Run `agent-brain sync` to push rules into your project!".bold().magenta());
    println!();

    Ok(())
}
