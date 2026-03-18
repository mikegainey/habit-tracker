use crate::app::App;
use crate::habit::Habit;
use crate::ui;

struct Command {
    key: &'static str,
    desc: &'static str,
    action: fn(&mut App),
}
pub const COMMANDS: [(&str, &str); 2] = [("1", "Add a habit"), ("2", "Mark a habit done")];
pub fn do_command(app: &mut App, item: &str) {
    match item {
        "1" => add_habit(app),
        "2" => mark_complete(app),
        _ => println!("invalid choice"),
    }
}

fn add_habit(app: &mut App) {
    println!("\nAdd a habit:");
    let name = ui::input("name: ");
    let habit = Habit::new(name);
    app.add_habit(habit);
    println!("habit added");
}

fn mark_complete(app: &mut App) {
    ui::list_habits(app);
    let selection: usize = ui::input_number("\nSelect habit to mark complete (by number): ");
    let index: usize = selection.saturating_sub(1);
    let habit = app.get_mut_habit(index).unwrap(); // handle this error better
    let now = time::OffsetDateTime::now_local().unwrap();
    habit.mark_complete(now);
}
