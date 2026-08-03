use anyhow::Result;
use colored::*;
use std::env;
use std::fs;
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
