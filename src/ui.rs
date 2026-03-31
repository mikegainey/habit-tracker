use crate::commands::COMMANDS;
use crate::habit::Habit;
use crate::helper;
use colored::Colorize;
use std::io::{self, Write};

pub fn list_habits(habits: &[Habit]) -> anyhow::Result<()> {
    let today = helper::today();
    println!("{}", "======== List of Habits ========".cyan());
    println!("{}", "Done today:".cyan());
    let mut count = 0;
    for (index, habit) in habits.iter().enumerate() {
        if habit.done_on_date(today) {
            count += 1;
            let streak = habit.ending_streak(today);
            let last30days = habit.last_30_days(today);
            println!(
                "{}) {:30}  (streak: {:2})  (last 30 days: {:2})  (time: {})",
                index + 1,
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
    for (index, habit) in habits.iter().enumerate() {
        if !habit.done_on_date(today) {
            count += 1;
            let streak = habit.ending_streak(today);
            let last30days = habit.last_30_days(today);
            println!(
                "{}) {:30}  (streak: {:2})  (last 30 days: {:2})",
                index + 1,
                habit,
                streak,
                last30days
            )
        }
    }
    if count == 0 {
        println!("(none)");
    }
    println!("{}", "================================".cyan());
    Ok(())
}

pub fn show_menu() {
    println!("\n{}", "What do you want to do?".cyan());
    for cmd in COMMANDS {
        if cmd.key.len() > 1 {
            // commands of length > 1 are hidden commands
            continue;
        }
        println!("{}) {}", cmd.key, cmd.desc);
    }
    println!("{}", "q) Quit this program".cyan());
    println!();
}

pub fn input(prompt: &str) -> anyhow::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();
    Ok(input)
}

pub fn choose_by_number(prompt: &str, len: usize) -> anyhow::Result<usize> {
    loop {
        let input = input(prompt)?; // user input: 1-based
        match input.parse() {
            Err(e) => println!("Invalid input: {}", e),
            Ok(0) => println!("Invalid input: 0"),
            Ok(val) if val > len => println!("Invalid input: {}", val),
            Ok(val) => return Ok(val - 1), // return value: 0-based
        }
    }
}

// Clear screen and move cursor to home position (1,1)
pub fn clear_screen() -> anyhow::Result<()> {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush()?;
    Ok(())
}
