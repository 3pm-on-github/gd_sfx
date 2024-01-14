use std::{collections::HashSet, path::PathBuf};

use anyhow::Result;
use gdsfx_data::paths;
use gdsfx_library::EntryId;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};

static FAVORITES_FILE: Lazy<PathBuf> = Lazy::new(|| {
    paths::runtime::PROJECT_DIRS.config_local_dir()
        .join("favorites.json")
});

#[derive(Serialize, Deserialize)]
pub struct Favorites(HashSet<EntryId>);

impl Default for Favorites {
    fn default() -> Self {
        let mut favorites = HashSet::new();
        favorites.insert(4451); // FIRE IN THE HOLE
        Self(favorites)
    }
}

impl Favorites {
    pub fn load_or_default() -> Self {
        gdsfx_data::read_json_file(&*FAVORITES_FILE).unwrap_or_default()
    }

    fn try_save(&self) -> Result<()> {
        let json_data = serde_json::to_string(self).expect("derived serialization shouldn't fail");

        gdsfx_data::create_parent_dirs(&*FAVORITES_FILE)?;
        gdsfx_data::write_file(&*FAVORITES_FILE, json_data)?;

        Ok(())
    }

    pub fn has_favorite(&self, id: EntryId) -> bool {
        self.0.contains(&id)
    }

    pub fn add_favorite(&mut self, id: EntryId) {
        if self.0.insert(id) && self.try_save().is_err() {
            // undo on failure
            self.0.remove(&id);
        }
    }

    pub fn remove_favorite(&mut self, id: EntryId) {
        if self.0.remove(&id) && self.try_save().is_err() {
            // undo on failure
            self.0.insert(id);
        }
    }

    pub fn clear_favorites(&mut self) {
        self.0.clear();
        let _ = self.try_save();
    }
}
