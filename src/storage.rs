use crate::app::App;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub fn save_data(app: App) -> Result<()> {
    let path = "app_data.json";
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &app)?;
    Ok(())
}

pub fn load_data() -> Result<App> {
    let path = "app_data.json";
    let file =
        File::open(path).with_context(|| format!("Failed to open app data file at {}", path))?;
    let reader = BufReader::new(file);
    let app: App = serde_json::from_reader(reader)
        .context("Failed to parse the JSON data into the App struct")?;
    Ok(app)
}
