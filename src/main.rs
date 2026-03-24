pub mod app;
pub mod commands;
pub mod habit;
pub mod helper;
pub mod storage;
pub mod ui;

use anyhow::Context;
use app::App;

fn main() -> anyhow::Result<()> {
    // load data from app_date.json, otherwise, create a new App
    let mut app = match storage::load_data() {
        Ok(app) => app,
        Err(err) => {
            eprintln!("\nNotice: Starting with a fresh database (Reason: {})", err);
            App::new()
        }
    };

    loop {
        ui::clear_screen();
        ui::list_habits(&app);

        ui::show_menu();

        // get the user's menu choice
        let item = ui::input("> ")?;

        if item == "q" {
            break;
        }

        commands::do_command(&mut app, &item)?;
    }

    // save data to a file
    storage::save_data(app).context("Error: could not save app data")?;

    Ok(())
}
