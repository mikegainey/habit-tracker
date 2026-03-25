use crate::app::App;
use crate::commands::COMMANDS;
use crate::helper;
use colored::Colorize;
use std::io::{self, Write};

pub fn list_habits(app: &App) -> anyhow::Result<()> {
    println!("{}", "======== List of Habits ========".cyan());
    println!("{}", "Done today:".cyan());
    let today = helper::today()?;
    let mut count = 0;
    for (index, habit) in app.get_habits().iter().enumerate() {
        if habit.done_today()? {
            count += 1;
            let streak = habit.ending_streak()?;
            let last30days = habit.last_30_days()?;
            println!(
                "{}) {:30}  (streak: {})  (last 30 days: {})  (time: {})",
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
    for (index, habit) in app.get_habits().iter().enumerate() {
        if !habit.done_today()? {
            count += 1;
            let streak = habit.ending_streak()?;
            let last30days = habit.last_30_days()?;
            println!(
                "{}) {:30}  (streak: {})  (last 30 days: {})",
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

pub fn choose_by_number(prompt: &str) -> anyhow::Result<usize> {
    loop {
        let input = input(prompt)?;
        match input.parse() {
            Err(e) => println!("Invalid input: {}", e),
            Ok(val) => match val {
                0 => print!("Invalid input: {}", val),
                _ => return Ok(val),
            },
        }
    }
}

// Clear screen and move cursor to home position (1,1)
pub fn clear_screen() -> anyhow::Result<()> {
    print!("\x1B[2J\x1B[1;1H");
    std::io::Write::flush(&mut std::io::stdout())?;
    Ok(())
}
