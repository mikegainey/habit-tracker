// This module handles terminal interaction.
// Its job:
// ask the user for a habit name
// ask for a date
// print habit lists
// print reports
// print errors in a friendly way
// Important design point:
// Try to keep ui.rs from knowing too much about file formats or business rules.
// It should mostly gather input and display output.

// You should probably have functions whose purposes are:
// read one line of user input
// ask for a string field
// ask for a date string
// print one habit nicely
// print a list of habits
// print journal entries
// print a report section
// Try to keep these focused and boring. Boring UI code is good UI code.

use crate::app::App;
use crate::commands::COMMANDS;
use std::io::{self, Write};

pub fn list_habits(app: &App) {
    let habits = app.get_habits();
    if habits.is_empty() {
        println!("\nNo habits to list. You should add one.");
        return;
    }

    println!("\nList of Habits:");
    for (i, habit) in habits.iter().enumerate() {
        println!("{}) {:?}", i + 1, habit);
    }
}

pub fn show_menu() {
    println!("\nMenu:");
    for (item, command) in COMMANDS {
        println!("{}) {}", item, command);
    }
    println!();
}

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim().to_string();
    input
}
