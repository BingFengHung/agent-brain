mod auto_handoff;
mod injector;
mod learn;
mod memory;
mod resume;
mod search;
mod status;
mod updater;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use auto_handoff::generate_auto_handoff;
use injector::sync_project_context;
use learn::auto_learn_codebase;
use memory::MemoryManager;
use resume::{ResumeManager, SessionHandoff};
use search::search_brain;
use status::render_brain_status;
use updater::check_and_update;
use inquire::Text;
use std::io::IsTerminal;

#[derive(Parser, Debug)]
#[command(
    name = "agent-brain",
    author = "BingFengHung <your.email@example.com>",
    version = "0.1.3",
    about = "Long-Term Memory & Smart Resume Gateway Agent CLI for Copilot CLI, agy & AI Agents."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Display full Memory Dashboard (stored rules, project sync status & session stats)
    Status,
    /// Alias for Status: Inspect stored memory and project context status
    Inspect,
    /// Auto-check GitHub Releases and update agent-brain CLI to the latest version
    Update,
    /// Auto-analyze codebase or system history (use --global to scan shell & system history)
    Learn {
        /// Scan global shell history & system habits across all past sessions
        #[arg(short, long)]
        global: bool,
    },
    /// Save a developer preference or coding convention rule permanently
    Remember {
        #[arg(required = true)]
        content: Vec<String>,
    },
    /// List all stored developer preferences and rules
    List,
    /// Forget a specific preference rule by ID
    Forget {
        #[arg(required = true)]
        id: usize,
    },
    /// Inject & sync memory rules into local AGENTS.md, .copilotrules, and .github/copilot-instructions.md
    Sync,
    /// Create a session handoff snapshot (auto-switches to --auto mode inside agy/pipes)
    Handoff {
        /// Auto generate handoff snapshot using Git diff & session metadata (Zero manual typing)
        #[arg(short, long)]
        auto: bool,
        /// Session main goal / accomplishment
        #[arg(short, long)]
        goal: Option<String>,
        /// Files modified (comma separated)
        #[arg(short, long)]
        files: Option<String>,
        /// Key decisions made (comma separated)
        #[arg(short, long)]
        decisions: Option<String>,
        /// Unfinished TODOs (comma separated)
        #[arg(short, long)]
        todos: Option<String>,
        /// Project name
        #[arg(short, long)]
        project: Option<String>,
    },
    /// Display Smart Resume timeline cards of past sessions
    Resume,
    /// Search memory rules and past sessions by keyword
    Find {
        #[arg(required = true)]
        query: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Status | Commands::Inspect => {
            render_brain_status()?;
        }
        Commands::Update => {
            check_and_update().await?;
        }
        Commands::Learn { global } => {
            if global {
                learn::auto_learn_global_history()?;
            } else {
                auto_learn_codebase()?;
            }
        }
        Commands::Remember { content } => {
            let rule = content.join(" ");
            let memory_mgr = MemoryManager::new()?;
            memory_mgr.add_preference(&rule)?;
            println!("{}", "✨ Saved rule to Long-Term Memory:".bold().green());
            println!("   {}", rule.cyan());
        }
        Commands::List => {
            let memory_mgr = MemoryManager::new()?;
            let store = memory_mgr.load_store()?;
            println!("{}", "🧠 Long-Term Memory Rules:".bold().yellow());
            if store.preferences.is_empty() {
                println!("{}", "   (No rules stored yet. Run `agent-brain learn` to auto-learn from codebase!)".dimmed());
            } else {
                for p in &store.preferences {
                    println!("   [ID {}] {} ({})", p.id, p.content.green(), p.created_at.dimmed());
                }
            }
        }
        Commands::Forget { id } => {
            let memory_mgr = MemoryManager::new()?;
            if memory_mgr.remove_preference(id)? {
                println!("{}", format!("✨ Removed rule ID {} from memory.", id).green());
            } else {
                println!("{}", format!("⚠️ Rule ID {} not found.", id).yellow());
            }
        }
        Commands::Sync => {
            sync_project_context()?;
        }
        Commands::Handoff {
            auto,
            goal,
            files,
            decisions,
            todos,
            project,
        } => {
            let is_interactive = std::io::stdin().is_terminal();

            // Automatically switch to --auto mode inside agy/pipes (when not a TTY terminal)
            if auto || !is_interactive || goal.is_some() || files.is_some() {
                if !is_interactive && !auto && goal.is_none() {
                    println!("{}", "ℹ️ Non-interactive terminal detected inside agy. Auto-switching to --auto handoff mode!".dimmed());
                }

                if goal.is_some() || files.is_some() {
                    let memory_mgr = ResumeManager::new()?;
                    let project_name = project.unwrap_or_else(|| "current-project".to_string());
                    let goal_str = goal.unwrap_or_else(|| "Session work".to_string());
                    let files_modified: Vec<String> = files.map(|f| f.split(',').map(|s| s.trim().to_string()).collect()).unwrap_or_default();
                    let key_decisions: Vec<String> = decisions.map(|d| d.split(',').map(|s| s.trim().to_string()).collect()).unwrap_or_default();
                    let unfinished_todos: Vec<String> = todos.map(|t| t.split(',').map(|s| s.trim().to_string()).collect()).unwrap_or_default();

                    let session_id = format!("session-{}", chrono::Local::now().format("%Y%m%d-%H%M%S"));
                    let date = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();

                    let handoff = SessionHandoff {
                        id: session_id,
                        date,
                        project_name,
                        goal: goal_str,
                        files_modified,
                        key_decisions,
                        unfinished_todos,
                    };

                    memory_mgr.add_session(handoff)?;
                    println!("{}", "✨ Session Handoff Snapshot Saved Successfully!".bold().green());
                } else {
                    generate_auto_handoff()?;
                }
            } else {
                let memory_mgr = ResumeManager::new()?;
                println!("{}", "📝 Creating End-of-Session Handoff Snapshot".bold().cyan());
                let project_name = Text::new("Project Name:").with_initial_value("agent-brain").prompt()?;
                let goal_str = Text::new("Session Main Goal / Accomplishment:").prompt()?;
                let files_str = Text::new("Files Modified (comma separated):").prompt()?;
                let files_modified: Vec<String> = files_str.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                let decisions_str = Text::new("Key Decisions Made (comma separated):").prompt()?;
                let key_decisions: Vec<String> = decisions_str.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                let todos_str = Text::new("Unfinished TODOs for Tomorrow (comma separated):").prompt()?;
                let unfinished_todos: Vec<String> = todos_str.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();

                let session_id = format!("session-{}", chrono::Local::now().format("%Y%m%d-%H%M%S"));
                let date = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();

                let handoff = SessionHandoff {
                    id: session_id,
                    date,
                    project_name,
                    goal: goal_str,
                    files_modified,
                    key_decisions,
                    unfinished_todos,
                };

                memory_mgr.add_session(handoff)?;
                println!();
                println!("{}", "✨ Session Handoff Snapshot Saved Successfully!".bold().green());
            }
        }
        Commands::Resume => {
            let resume_mgr = ResumeManager::new()?;
            let sessions = resume_mgr.load_sessions()?;
            println!("{}", "📜 Smart Resume Timeline:".bold().magenta());
            println!();

            if sessions.is_empty() {
                println!("{}", "   (No session handoffs recorded yet. Create one via `agent-brain handoff --auto`)".dimmed());
            } else {
                for s in &sessions {
                    resume_mgr.render_session_card(s);
                }
            }
        }
        Commands::Find { query } => {
            let q = query.join(" ");
            search_brain(&q)?;
        }
    }

    Ok(())
}
