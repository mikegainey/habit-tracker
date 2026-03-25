use crate::app::App;
use crate::helper;
use crate::ui;

pub struct Command {
    pub key: &'static str,
    pub desc: &'static str,
    action: fn(&mut App) -> Result<(), anyhow::Error>,
}
pub const COMMANDS: [Command; 4] = [
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
];

pub fn do_command(app: &mut App, item: &str) -> anyhow::Result<()> {
    if let Some(cmd) = COMMANDS.iter().find(|c| c.key == item) {
        (cmd.action)(app)?;
    } else {
        println!("\nInvalid choice");
    }
    Ok(())
}

fn add_habit(app: &mut App) -> anyhow::Result<()> {
    println!("\nAdd a habit:");
    let name = ui::input("name: ")?;
    app.add_habit(name);
    println!("habit added");
    Ok(())
}

fn mark_complete(app: &mut App) -> anyhow::Result<()> {
    // ui::list_habits(app);
    let selection = ui::choose_by_number("\nSelect habit to mark complete (by number): ")?;
    let index: usize = selection - 1; // choose_by_number returns usize >= 1
    let habit = match app.get_mut_habit(index) {
        Some(h) => h,
        None => {
            println!("Error: selection out of range: {}", selection);
            return Ok(());
        }
    };
    habit.mark_complete(helper::now()?);
    Ok(())
}

fn change_name(app: &mut App) -> anyhow::Result<()> {
    // ui::list_habits(app);
    let selection = ui::choose_by_number("\nSelect habit to change name (by number): ")?;
    let index: usize = selection - 1; // choose_by_number returns usize >= 1
    let habit = match app.get_mut_habit(index) {
        Some(h) => h,
        None => {
            println!("Error: selection out of range: {}", selection);
            return Ok(());
        }
    };
    let new_name = ui::input("\nEnter the new name: ")?;
    habit.change_name(new_name);
    Ok(())
}

fn remove_habit(app: &mut App) -> anyhow::Result<()> {
    // ui::list_habits(app);
    let selection: usize = ui::choose_by_number("\nSelect habit to remove (by number): ")?;
    let index: usize = selection - 1; // choose_by_number returns usize >= 1
    let habits = app.get_mut_habits();
    if index >= habits.len() {
        println!("Error: selection out of range: {}", selection);
        return Ok(());
    }
    app.get_mut_habits().remove(index);
    Ok(())
}
