use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPreference {
    pub id: usize,
    pub content: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MemoryStore {
    pub preferences: Vec<UserPreference>,
}

pub struct MemoryManager {
    base_dir: PathBuf,
}

impl MemoryManager {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().ok_or_else(|| anyhow!("Could not locate home directory"))?;
        let base_dir = home.join(".agent-brain");

        if !base_dir.exists() {
            fs::create_dir_all(&base_dir)?;
        }

        Ok(Self { base_dir })
    }

    fn pref_file_path(&self) -> PathBuf {
        self.base_dir.join("preferences.json")
    }

    pub fn load_store(&self) -> Result<MemoryStore> {
        let path = self.pref_file_path();
        if !path.exists() {
            return Ok(MemoryStore::default());
        }

        let content = fs::read_to_string(path)?;
        let store: MemoryStore = serde_json::from_str(&content)?;
        Ok(store)
    }

    pub fn save_store(&self, store: &MemoryStore) -> Result<()> {
        let path = self.pref_file_path();
        let json = serde_json::to_string_pretty(store)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn add_preference(&self, content: &str) -> Result<()> {
        let mut store = self.load_store()?;
        let next_id = store.preferences.iter().map(|p| p.id).max().unwrap_or(0) + 1;

        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        store.preferences.push(UserPreference {
            id: next_id,
            content: content.to_string(),
            created_at: now,
        });

        self.save_store(&store)?;
        Ok(())
    }

    pub fn remove_preference(&self, id: usize) -> Result<bool> {
        let mut store = self.load_store()?;
        let len_before = store.preferences.len();
        store.preferences.retain(|p| p.id != id);
        let removed = store.preferences.len() < len_before;

        if removed {
            self.save_store(&store)?;
        }
        Ok(removed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_store_serialization() {
        let mut store = MemoryStore::default();
        store.preferences.push(UserPreference {
            id: 1,
            content: "Use Tailwind and Zustand".to_string(),
            created_at: "2026-08-03 10:00:00".to_string(),
        });

        let json = serde_json::to_string(&store).unwrap();
        assert!(json.contains("Tailwind"));
    }
}
