use crate::app::App;
use crate::commands::COMMANDS;
use crate::helper;
use colored::Colorize;
use std::io::{self, Write};

pub fn list_habits(app: &App) {
    println!("\n{}", "===== List of Habits =====".cyan());
    println!("{}", "Done today:".cyan());
    let today = helper::today();
    let mut count = 0;
    for (number, habit) in app.get_habits().iter().enumerate() {
        if habit.done_today() {
            count += 1;
            let streak = habit.ending_streak();
            let last30days = habit.last_30_days();
            println!(
                "{}) {:30}  (streak: {})  (last 30 days: {})  (time: {})",
                number + 1,
                habit,
                streak,
                last30days,
                habit.list_times(today),
            );
        }
    }
    if count == 0 {
        println!("(none)");
    }
    println!("\n{}", "Not done today:".cyan());
    let mut count = 0;
    for (number, habit) in app.get_habits().iter().enumerate() {
        if !habit.done_today() {
            count += 1;
            let streak = habit.ending_streak();
            let last30days = habit.last_30_days();
            println!(
                "{}) {:30}  (streak: {})  (last 30 days: {})",
                number + 1,
                habit,
                streak,
                last30days
            )
        }
    }
    if count == 0 {
        println!("(none)");
    }
    println!("{}", "==========================".cyan());
}

pub fn show_menu() {
    println!("\nWhat do you want to do?");
    for cmd in COMMANDS {
        println!("{}) {}", cmd.key, cmd.desc);
    }
    println!("q) Quit this program");
    println!();
}

pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // this unwrap is okay

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line"); // this expect is okay
    let input = input.trim().to_string();
    input
}

pub fn input_number<T>(prompt: &str) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    loop {
        let input = input(prompt);
        match input.parse() {
            Ok(val) => return val,
            Err(e) => println!("Invalid input: {}", e),
        }
    }
}
