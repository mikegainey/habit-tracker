#![allow(unused_mut)]
// call into ui, app, and storage
// handle top-level errors

pub mod app;
pub mod commands;
pub mod error;
pub mod habit;
pub mod journal;
pub mod storage;
pub mod ui;

use app::App;

fn main() {
    let mut app = App::new();

    loop {
        ui::list_habits(&app);

        ui::show_menu();

        // get the user's menu choice
        let item = ui::get_input("> ");

        commands::do_command(&mut app, &item);
    }
}
