#![allow(dead_code, unused_imports)]
// This module defines the menu commands.
// Possible contents:
// a function that turns user input into a Command
// For example, if the user types "1", you map that to something like “list habits”.
// This keeps command parsing separate from the main loop.
use crate::app::{self, App};
use crate::habit::{self, Habit};
use crate::ui;

pub const COMMANDS: [(&str, &str); 1] = [("1", "add a habit")];

pub fn do_command(app: &mut App, item: &str) {
    match item {
        "1" => add_habit(app),
        _ => println!("invalid choice"),
    }
}

fn add_habit(app: &mut App) {
    println!("\nAdd a habit:");
    let name = ui::get_input("name: ");
    let description = ui::get_input("desctiption: ");
    let habit = Habit::new(name, description);
    app.add_habit(habit);
    println!("habit added");
}
