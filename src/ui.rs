use crate::app::App;
use crate::commands::COMMANDS;
use std::io::{self, Write};
use time::{Date, OffsetDateTime};

pub fn list_habits(app: &App) {
    println!("\n\nHabits done today:");
    let today = today();
    for (number, habit) in app.get_habits().iter().enumerate() {
        if habit.done_today() {
            println!(
                "{}) {} (time: {})",
                number + 1,
                habit,
                habit.list_times(today)
            )
        }
    }
    println!("\nNot done today:");
    for (number, habit) in app.get_habits().iter().enumerate() {
        if !habit.done_today() {
            println!("{}) {}", number + 1, habit)
        }
    }
    println!();
}

pub fn show_menu() {
    println!("\nWhat do you want to do?");
    for (item, command) in COMMANDS {
        println!("{}) {}", item, command);
    }
    println!("q) Quit this program");
    println!();
}

pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
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

pub fn today() -> Date {
    OffsetDateTime::now_local().unwrap().date()
}
