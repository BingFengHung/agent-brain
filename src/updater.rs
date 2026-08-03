use anyhow::{anyhow, Result};
use colored::*;
use serde::Deserialize;
use std::env;
use tempfile::NamedTempFile;

#[derive(Deserialize, Debug)]
struct ReleaseAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Deserialize, Debug)]
struct GitHubRelease {
    tag_name: String,
    name: Option<String>,
    assets: Vec<ReleaseAsset>,
}

pub async fn check_and_update() -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION");
    println!("{}", format!("🔍 Checking for latest updates on GitHub Releases (Current: v{})...", current_version).bold().cyan());

    let repo_url = "https://api.github.com/repos/BingFengHung/agent-brain/releases/latest";
    let client = reqwest::Client::builder()
        .user_agent("agent-brain-cli-updater")
        .build()?;

    let res = client.get(repo_url).send().await?;
    if !res.status().is_success() {
        return Err(anyhow!("Failed to fetch release info from GitHub (HTTP {})", res.status()));
    }

    let release: GitHubRelease = res.json().await?;
    let latest_tag = release.tag_name.trim_start_matches('v');

    println!("  • Latest Version on GitHub: {}", format!("v{}", latest_tag).yellow());

    if latest_tag == current_version {
        println!("{}", "✨ agent-brain is already on the latest version!".bold().green());
        return Ok(());
    }

    println!("{}", format!("🚀 New version v{} is available! Downloading update...", latest_tag).bold().green());

    // Determine exact binary asset name for current OS
    let target_asset_name = if cfg!(target_os = "windows") {
        "agent-brain-windows-amd64.exe"
    } else if cfg!(target_os = "macos") {
        "agent-brain-macos-arm64"
    } else {
        "agent-brain-linux-amd64"
    };

    let asset = release
        .assets
        .iter()
        .find(|a| a.name == target_asset_name || a.name.ends_with(target_asset_name))
        .ok_or_else(|| anyhow!("Could not find binary asset '{}' in release", target_asset_name))?;

    println!("  • Downloading asset: {}", asset.browser_download_url.dimmed());

    let bytes = client
        .get(&asset.browser_download_url)
        .send()
        .await?
        .bytes()
        .await?;

    // Create a temporary file and self-replace the running binary
    let temp_file = NamedTempFile::new()?;
    std::fs::write(temp_file.path(), &bytes)?;

    self_replace::self_replace(temp_file.path())?;

    println!();
    println!("{}", format!("✨ Successfully updated agent-brain to v{}!", latest_tag).bold().green());

    Ok(())
}
