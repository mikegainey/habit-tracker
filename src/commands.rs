use crate::app::App;
use crate::habit::Habit;
use crate::helper;
use crate::ui;

pub struct Command {
    pub key: &'static str,
    pub desc: &'static str,
    action: fn(&mut App),
}
pub const COMMANDS: [Command; 4] = [
    Command {
        key: "1",
        desc: "Add a habit",
        action: add_habit,
    },
    Command {
        key: "2",
        desc: "Remove a habit",
        action: remove_habit,
    },
    Command {
        key: "3",
        desc: "Change a habit name",
        action: change_name,
    },
    Command {
        key: "4",
        desc: "Mark a habit complete",
        action: mark_complete,
    },
];

pub fn do_command(app: &mut App, item: &str) {
    if let Some(cmd) = COMMANDS.iter().find(|c| c.key == item) {
        (cmd.action)(app);
    } else {
        println!("\nInvalid choice");
    }
}

fn add_habit(app: &mut App) {
    println!("\nAdd a habit:");
    let name = ui::input("name: ");
    let habit = Habit::new(name); // maybe this should be part of app::add_habit
    app.add_habit(habit);
    println!("habit added");
}

fn mark_complete(app: &mut App) {
    ui::list_habits(app);
    let selection: usize = ui::input_number("\nSelect habit to mark complete (by number): ");
    if selection == 0 {
        println!("Error: selection out of range: {}", selection);
        return;
    }
    let index: usize = selection - 1;
    let habit = match app.get_mut_habit(index) {
        Some(h) => h,
        None => {
            println!("Error: selection out of range: {}", selection);
            return;
        }
    };
    habit.mark_complete(helper::now());
}

fn change_name(app: &mut App) {
    ui::list_habits(app);
    let selection: usize = ui::input_number("\nSelect habit to change name (by number): ");
    if selection == 0 {
        println!("Error: selection out of range: {}", selection);
        return;
    }
    let index: usize = selection - 1;
    let habit = match app.get_mut_habit(index) {
        Some(h) => h,
        None => {
            println!("Error: selection out of range: {}", selection);
            return;
        }
    };
    let new_name = ui::input("\nEnter the new name: ");
    habit.change_name(new_name);
}

fn remove_habit(app: &mut App) {
    ui::list_habits(app);
    let selection: usize = ui::input_number("\nSelect habit to remove (by number): ");
    if selection == 0 {
        println!("Error: selection out of range: {}", selection);
        return;
    }
    let index: usize = selection - 1;
    app.get_mut_habits().remove(index);
}
