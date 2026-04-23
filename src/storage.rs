use crate::habit::Habit;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

#[derive(Debug, Serialize, Deserialize)]
struct AppData {
    habits: Vec<Habit>,
}

pub fn load_data() -> anyhow::Result<Vec<Habit>> {
    let path = "app_data.toml";
    let file =
        File::open(path).with_context(|| format!("Failed to open app data file at {}", path))?;
    let mut reader = BufReader::new(file);

    let mut contents = String::new();
    reader
        .read_to_string(&mut contents)
        .with_context(|| format!("Failed to read app data file at {}", path))?;

    let data: AppData =
        toml::from_str(&contents).context("Failed to parse the TOML data into AppData")?;

    Ok(data.habits)
}

// todo: Save to a temp file. If successful, rename to app_data.toml.
pub fn save_data(habits: &[Habit]) -> anyhow::Result<()> {
    let path = "app_data.toml";
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    let data = AppData {
        habits: habits.to_vec(),
    };

    let toml_string = toml::to_string_pretty(&data)?;
    writer.write_all(toml_string.as_bytes())?;
    writer.flush()?;

    Ok(())
}
