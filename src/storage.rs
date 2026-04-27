use crate::habit::Habit;
use anyhow::{Context, anyhow};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct AppData {
    habits: Vec<Habit>,
}

// For me, this is: /home/michael/.local/share/habit-tracker
pub fn data_file_path() -> anyhow::Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "mikegainey", "habit-tracker")
        .ok_or_else(|| anyhow!("Could not determine an app data directory for this platform"))?;

    let data_dir = proj_dirs.data_local_dir();
    fs::create_dir_all(data_dir)
        .with_context(|| format!("Failed to create app data directory at {:?}", data_dir))?;

    Ok(data_dir.join("app_data.toml"))
}

pub fn load_data() -> anyhow::Result<Vec<Habit>> {
    let path = data_file_path()?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read app data file at {:?}", &path))?;

    let data: AppData =
        toml::from_str(&contents).context("Failed to parse the TOML data into AppData")?;

    // If the TOML can't be parsed (because of a bad user edit), reopen the editor
    // letting the user fix it.

    Ok(data.habits)
}

// todo: Save to a temp file. If successful, rename to app_data.toml.
pub fn save_data(habits: &[Habit]) -> anyhow::Result<()> {
    let path = data_file_path()?;

    let data = AppData {
        habits: habits.to_vec(),
    };

    let toml_string =
        toml::to_string_pretty(&data).context("Failed to generate TOML data from &[Habit]")?;

    fs::write(&path, &toml_string).with_context(|| format!("Failed to write {:?}", &path))?;

    Ok(())
}
