pub mod commands;
pub mod datetime;
pub mod habit;
pub mod helper;
pub mod storage;
pub mod ui;

use habit::Habit;

fn main() -> anyhow::Result<()> {
    // load data from app_data.json, otherwise, create a new App
    let mut habits: Vec<Habit> = storage::load_data()?;

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
    if let Err(err) = storage::save_data(&habits) {
        eprintln!("Error: could not save app data (Reason: {})", err);
    }

    Ok(())
}
