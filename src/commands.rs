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
    let length = app.get_habits().len();
    let index = ui::choose_by_number("\nSelect habit to mark complete (by number): ", length)?;
    let habit = app
        .get_mut_habit(index)
        .ok_or(anyhow::anyhow!("index out of range"))?;
    habit.mark_complete(helper::now()?);
    Ok(())
}

fn change_name(app: &mut App) -> anyhow::Result<()> {
    let length = app.get_habits().len();
    let index = ui::choose_by_number("\nSelect habit to change name (by number): ", length)?;
    let habit = app
        .get_mut_habit(index)
        .ok_or(anyhow::anyhow!("index out of range"))?;
    let new_name = ui::input("\nEnter the new name: ")?;
    habit.change_name(new_name);
    Ok(())
}

fn remove_habit(app: &mut App) -> anyhow::Result<()> {
    let length = app.get_habits().len();
    let index: usize = ui::choose_by_number("\nSelect habit to remove (by number): ", length)?;
    let _ = app.remove_habit(index); // choose_by_number ensures the index returned is valid
    Ok(())
}

// todo: "view history" command -- last 30 days: just a list or a calendar-style grid
