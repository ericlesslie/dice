use crate::die::DieKind;
use gtk::glib;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct RollEntry {
    pub id: u64,
    pub dice: Vec<(DieKind, u32)>,
    pub total: u32,
}

pub struct RollHistory {
    pub recents: Vec<RollEntry>,
    pub favorites: Vec<RollEntry>,
    next_id: u64,
}

impl RollHistory {
    pub fn new() -> Self {
        let favorites = Self::load_favorites();
        let next_id = favorites.iter().map(|e| e.id).max().unwrap_or(0) + 1;
        Self {
            recents: Vec::new(),
            favorites,
            next_id,
        }
    }

    pub fn add_recent(&mut self, dice: Vec<(DieKind, u32)>) -> RollEntry {
        let total = dice.iter().map(|(_, v)| v).sum();
        let entry = RollEntry {
            id: self.next_id,
            dice,
            total,
        };
        self.next_id += 1;
        self.recents.insert(0, entry.clone());
        if self.recents.len() > 50 {
            self.recents.truncate(50);
        }
        entry
    }

    pub fn favorite(&mut self, id: u64) {
        if let Some(entry) = self.recents.iter().find(|e| e.id == id) {
            if !self.favorites.iter().any(|e| e.id == id) {
                self.favorites.push(entry.clone());
                self.save_favorites();
            }
        }
    }

    pub fn remove_favorite(&mut self, id: u64) {
        self.favorites.retain(|e| e.id != id);
        self.save_favorites();
    }

    fn favorites_path() -> PathBuf {
        let mut path = glib::user_data_dir();
        path.push("dice");
        path.push("favorites.json");
        path
    }

    fn save_favorites(&self) {
        let path = Self::favorites_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok();
        }
        if let Ok(json) = serde_json::to_string_pretty(&self.favorites) {
            fs::write(&path, json).ok();
        }
    }

    fn load_favorites() -> Vec<RollEntry> {
        let path = Self::favorites_path();
        fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn format_roll(entry: &RollEntry) -> (String, String) {
        let mut counts: BTreeMap<&str, u32> = BTreeMap::new();
        for (kind, _) in &entry.dice {
            let name = match kind {
                DieKind::Four => "d4",
                DieKind::Six => "d6",
                DieKind::Eight => "d8",
                DieKind::Ten => "d10",
                DieKind::Twelve => "d12",
                DieKind::Twenty => "d20",
            };
            *counts.entry(name).or_insert(0) += 1;
        }
        let title: Vec<String> = counts.iter().map(|(k, v)| format!("{}{}", v, k)).collect();
        (title.join(" + "), format!("= {}", entry.total))
    }
}
