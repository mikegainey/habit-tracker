use crate::habit::Habit;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub fn load_data() -> Result<Vec<Habit>> {
    let path = "app_data.json";
    let file =
        File::open(path).with_context(|| format!("Failed to open app data file at {}", path))?;
    let reader = BufReader::new(file);
    let habits: Vec<Habit> =
        serde_json::from_reader(reader).context("Failed to parse the JSON data into Vec<Habit>")?;
    Ok(habits)
}

pub fn save_data(habits: &[Habit]) -> Result<()> {
    let path = "app_data.json";
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, habits)?;
    Ok(())
}
