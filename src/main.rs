pub mod commands;
pub mod habit;
pub mod helper;
pub mod storage;
pub mod ui;

use anyhow::Context;
use habit::Habit;

fn main() -> anyhow::Result<()> {
    // load data from app_data.json, otherwise, create a new App
    let mut habits: Vec<Habit> = match storage::load_data() {
        Ok(data) => data,
        Err(err) => {
            eprintln!("\nNotice: Starting with a fresh database (Reason: {})", err);
            Vec::new()
        }
    };

    loop {
        ui::clear_screen()?;
        ui::list_habits(&habits)?;

        ui::show_menu();

        // get the user's menu choice
        let item = ui::input("> ")?;

        if item == "q" {
            break;
        }

        if let Err(e) = commands::do_command(&mut habits, &item) {
            eprintln!("Error: {}", e);
            ui::input("Press <Enter> to continue...")?;
        }
    }

    // save data to a file
    storage::save_data(&habits).context("Error: could not save app data")?;

    Ok(())
}
