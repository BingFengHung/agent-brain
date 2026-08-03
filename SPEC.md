# Technical Specification: `agent-brain` Long-Term Memory & Smart Resume Agent CLI

## Problem Statement

AI Agent CLI tools (such as Copilot CLI, `agy`, Claude Code, Cursor) suffer from context amnesia:
1. Every time a new session starts, developers must repeatedly re-teach coding style, project architecture, and technology preferences.
2. The native session resumption feature (e.g. `/resume`) displays vague session IDs or truncated first messages, leaving developers unable to recall past decisions, modified files, or unfinished tasks.

## Solution

`agent-brain` is a lightweight, zero-dependency Rust CLI tool that serves as a persistent Long-Term Memory Gateway and Smart Resume assistant for AI Agent CLI tools:
- **`remember` / `sync`**: Stores user coding preferences and project architecture rules, auto-generating `AGENTS.md` and `.copilotrules` so AI tools recognize project context from second 1.
- **`resume`**: Replaces vague session IDs with structured, human-readable timeline cards (Goal, Files Modified, Key Decisions, Unfinished TODOs).
- **`find`**: Allows searching through past session memories and decisions using keywords or natural language.
- **`handoff`**: Saves end-of-day progress snapshots for seamless continuation tomorrow.

---

## User Stories

1. As a developer, I want to type `agent-brain remember "<preference>"` so that my coding style rules are saved permanently.
2. As a Copilot CLI user, I want `agent-brain sync` to automatically generate `AGENTS.md` / `.copilotrules` in my project root so that Copilot never forgets my project rules.
3. As a developer resuming work after days, I want `agent-brain resume` to display structured cards of past sessions showing modified files, decisions made, and unfinished tasks.
4. As an engineer searching past context, I want `agent-brain find "<topic>"` to locate previous session summaries and decisions instantly.
5. As a developer wrapping up the workday, I want `agent-brain handoff` to save today's accomplishments and tomorrow's next steps.

---

## Implementation Decisions

### 1. Architecture & Core Modules (Rust Crate)

- **`main.rs`**: Entry point and subcommand routing (`clap`).
- **`memory.rs`**: Manages global user rules (`~/.agent-brain/global_preferences.json`) and project context (`~/.agent-brain/projects/`).
- **`resume.rs`**: Parses session handoff logs and renders structured CLI timeline cards.
- **`injector.rs`**: Generates and updates `AGENTS.md` and `.copilotrules` files in the current working directory.
- **`search.rs`**: Provides fast search indexing over stored memories and past session handoffs.

### 2. File Storage Layout

- `~/.agent-brain/global_preferences.json`
- `~/.agent-brain/sessions.json`
- Local project root: `AGENTS.md` & `.copilotrules`

---

## Out of Scope

- Cloud database syncing (v1 is 100% privacy-first local-first).
- IDE GUI extension (v1 focuses on CLI interface).

---

## Further Notes

- Cargo dependencies: `clap`, `tokio`, `serde`, `serde_json`, `colored`, `inquire`, `anyhow`, `async-trait`.
