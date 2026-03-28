use crate::app::App;
use crate::helper;
use crate::ui;
use anyhow::Context;
use colored::*;
use time::Duration;

pub struct Command {
    pub key: &'static str,
    pub desc: &'static str,
    action: fn(&mut App) -> Result<(), anyhow::Error>,
}
pub const COMMANDS: [Command; 6] = [
    Command {
        key: "1",
        desc: "Mark a habit complete",
        action: mark_complete,
    },
    Command {
        key: "2",
        desc: "Add a habit",
        action: add_habit,
    },
    Command {
        key: "3",
        desc: "Remove a habit",
        action: remove_habit,
    },
    Command {
        key: "4",
        desc: "Change a habit name",
        action: change_name,
    },
    Command {
        key: "5",
        desc: "View habit 1-year chart",
        action: habit_chart,
    },
    Command {
        key: "rhc", // len > 1 is a hidden command
        desc: "Reset habit completions",
        action: reset_completions,
    },
];

pub fn do_command(app: &mut App, item: &str) -> anyhow::Result<()> {
    if let Some(cmd) = COMMANDS.iter().find(|c| c.key == item) {
        (cmd.action)(app)?;
    } else {
        println!("\nInvalid choice: {}", item);
        ui::input("\nPress <Enter> to continue.")?;
    }
    Ok(())
}

fn mark_complete(app: &mut App) -> anyhow::Result<()> {
    let index = choose_habit(app, "\nSelect habit to mark complete (by number): ")?;
    let habit = app
        .get_mut_habit(index)
        .ok_or(anyhow::anyhow!("index out of range"))
        .context("could not mark habit complete")?;
    habit.mark_complete(helper::now()?);
    Ok(())
}

fn add_habit(app: &mut App) -> anyhow::Result<()> {
    println!("\nAdd a habit:");
    let name = ui::input("name: ")?;
    app.add_habit(name);
    println!("habit added");
    Ok(())
}

fn remove_habit(app: &mut App) -> anyhow::Result<()> {
    let index = choose_habit(app, "\nSelect habit to remove (by number): ")?;
    app.remove_habit(index).context("could not remove habit")?;
    Ok(())
}

fn change_name(app: &mut App) -> anyhow::Result<()> {
    let index = choose_habit(app, "\nSelect habit to change name (by number): ")?;
    let habit = app
        .get_mut_habit(index)
        .ok_or(anyhow::anyhow!("index out of range"))
        .context("could not change habit name")?;
    let new_name = ui::input("\nEnter the new name: ")?;
    habit.change_name(new_name);
    Ok(())
}

fn habit_chart(app: &mut App) -> anyhow::Result<()> {
    let index = choose_habit(app, "\nSelect habit to view chart (by number): ")?;
    let habit = app
        .get_habit(index)
        .ok_or(anyhow::anyhow!("index out of range"))
        .context("could not print habit chart")?;

    println!("\n{}:\n", habit.to_string().bold());

    let today = helper::today()?;
    let days_since_monday = today.weekday().number_days_from_monday();
    let first_monday = (today - Duration::days(days_since_monday as i64)) - Duration::weeks(51);

    // --- PART 1: PRINT MONTH HEADERS ---
    print!("    "); // Offset for "Mon " labels

    let mut week = 0;
    while week < 52 {
        let monday = first_monday + Duration::weeks(week as i64);
        let next_monday = monday + Duration::days(6);

        // If the month changes within this week (the 1st is between Mon and Sun)
        if monday.month() != next_monday.month() || monday.day() == 1 {
            let month_name = next_monday.month().to_string();
            let display_name = &month_name[..3]; // "Jan", "Feb", etc.

            // The three-letter month plus an extra space takes up two weeks of the chart
            print!("{} ", display_name.cyan());

            // To keep the grid, we must skip the "next" week's space to compensate
            // Calculation: Month(3) + 1 space = 4 chars (exactly 2 weeks of grid)
            week += 2;
        } else {
            print!("  "); // Two spaces to match "■ "
            week += 1;
        }
    }
    println!();

    // --- PART 2: PRINT THE GRID ---
    let day_labels = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    for day_index in 0..7 {
        print!("{:3} ", day_labels[day_index].bright_black());
        for week_index in 0..52 {
            let current_date = first_monday + Duration::days((week_index * 7 + day_index) as i64);
            let symbol = "■";
            if habit.done_on_date(current_date) {
                print!("{} ", symbol.truecolor(33, 191, 84));
            } else {
                print!("{} ", symbol.truecolor(60, 60, 60));
            }
        }
        println!();
    }
    ui::input("\nPress <Enter> to continue.")?;
    Ok(())
}

fn reset_completions(app: &mut App) -> anyhow::Result<()> {
    let index = choose_habit(app, "\nSelect habit to delete completions (by number): ")?;
    let habit = app
        .get_mut_habit(index)
        .ok_or(anyhow::anyhow!("index out of range"))?;
    habit.delete_completions();
    Ok(())
}

fn choose_habit(app: &mut App, prompt: &str) -> anyhow::Result<usize> {
    let length = app.get_habits().len();
    let index: usize = ui::choose_by_number(prompt, length)?;
    Ok(index)
}
