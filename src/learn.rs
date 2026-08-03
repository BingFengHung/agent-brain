use anyhow::Result;
use colored::*;
use std::env;
use std::fs;
use std::path::PathBuf;
use crate::memory::MemoryManager;

pub fn auto_learn_codebase() -> Result<()> {
    let cwd = env::current_dir()?;
    println!("{}", format!("🔍 Analyzing codebase at '{}' for auto-learning...", cwd.display()).bold().cyan());

    let memory_mgr = MemoryManager::new()?;
    let mut learned_rules = Vec::new();

    // 1. Detect Rust project
    if cwd.join("Cargo.toml").exists() {
        learned_rules.push("Project Language: Rust. Follow Rust 2021 idiomatic conventions and safety standards.".to_string());
        if let Ok(cargo_content) = fs::read_to_string(cwd.join("Cargo.toml")) {
            if cargo_content.contains("tokio") {
                learned_rules.push("Async Runtime: Tokio. Prefer async/await non-blocking patterns.".to_string());
            }
            if cargo_content.contains("serde") {
                learned_rules.push("Data Serialization: Serde. Use Derive macros for JSON serialization.".to_string());
            }
        }
    }

    // 2. Detect Node.js / TypeScript project
    if cwd.join("package.json").exists() {
        learned_rules.push("Project Ecosystem: Node.js / JavaScript.".to_string());
        if let Ok(pkg_content) = fs::read_to_string(cwd.join("package.json")) {
            if pkg_content.contains("typescript") || cwd.join("tsconfig.json").exists() {
                learned_rules.push("Type System: TypeScript strictly enabled. Avoid using 'any'.".to_string());
            }
            if pkg_content.contains("tailwind") {
                learned_rules.push("Styling: Tailwind CSS. Prefer utility-first styling classes.".to_string());
            }
            if pkg_content.contains("react") || pkg_content.contains("next") {
                learned_rules.push("Frontend Framework: React / Next.js. Use functional components and hooks.".to_string());
            }
        }
    }

    // 3. Detect Python project
    if cwd.join("pyproject.toml").exists() || cwd.join("requirements.txt").exists() {
        learned_rules.push("Project Language: Python 3. Follow PEP 8 style guide and use type hinting.".to_string());
    }

    // 4. Detect Git repository
    if cwd.join(".git").exists() {
        learned_rules.push("Version Control: Git. Follow Conventional Commits (feat, fix, docs, refactor).".to_string());
    }

    // 5. Save learned rules to memory
    if learned_rules.is_empty() {
        println!("{}", "ℹ️ No specific manifest files detected. Applied clean code defaults.".yellow());
    } else {
        println!();
        println!("{}", "✨ Auto-Learned Rules from your Existing Codebase:".bold().green());
        for rule in &learned_rules {
            memory_mgr.add_preference(rule)?;
            println!("  • {}", rule.cyan());
        }
    }

    println!();
    println!("{}", "💡 Run `agent-brain sync` to inject these learned rules into your project AGENTS.md!".bold().magenta());

    Ok(())
}

pub fn auto_learn_global_history() -> Result<()> {
    println!("{}", "🌐 Scanning Global Shell & AI Session History across your system...".bold().cyan());

    let memory_mgr = MemoryManager::new()?;
    let mut global_insights = Vec::new();

    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));

    // 1. Scan PowerShell History on Windows
    let ps_history = home
        .join("AppData")
        .join("Roaming")
        .join("Microsoft")
        .join("Windows")
        .join("PowerShell")
        .join("PSReadLine")
        .join("ConsoleHost_history.txt");

    if ps_history.exists() {
        if let Ok(content) = fs::read_to_string(ps_history) {
            if content.contains("pnpm") {
                global_insights.push("Preferred Node Package Manager: pnpm (detected from shell history)".to_string());
            } else if content.contains("yarn") {
                global_insights.push("Preferred Node Package Manager: yarn (detected from shell history)".to_string());
            }
            if content.contains("cargo") {
                global_insights.push("Active Toolchain: Rust / Cargo CLI".to_string());
            }
            if content.contains("docker") || content.contains("docker-compose") {
                global_insights.push("Workflow Habit: Frequently uses Docker containerization".to_string());
            }
        }
    }

    // 2. Scan Unix Shell History (.bash_history / .zsh_history)
    let bash_history = home.join(".bash_history");
    let zsh_history = home.join(".zsh_history");
    let unix_history = if zsh_history.exists() { Some(zsh_history) } else if bash_history.exists() { Some(bash_history) } else { None };

    if let Some(hist_path) = unix_history {
        if let Ok(content) = fs::read_to_string(hist_path) {
            if content.contains("git commit") {
                global_insights.push("Version Control Habit: Frequent Git Commits with concise messages".to_string());
            }
        }
    }

    // Always include user language & quality baseline
    global_insights.push("Default Response Language: Traditional Chinese (繁體中文)".to_string());
    global_insights.push("Code Quality Preference: Self-documenting code with clear type definitions".to_string());

    println!();
    println!("{}", "✨ Auto-Learned Global Habits from your System History:".bold().green());
    for insight in &global_insights {
        memory_mgr.add_preference(insight)?;
        println!("  • {}", insight.cyan());
    }

    println!();
    println!("{}", "🎉 Global memory updated! Now every new Copilot CLI / agy session will automatically inherit these habits!".bold().magenta());

    Ok(())
}
