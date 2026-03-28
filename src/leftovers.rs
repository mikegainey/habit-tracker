// originally in commands.rs
// rework this to display in calendar format
fn habit_history(app: &mut App) -> anyhow::Result<()> {
    let index = choose_habit(app, "\nSelect habit to view history (by number): ")?;
    let habit = app
        .get_habit(index)
        .ok_or(anyhow::anyhow!("index out of range"))?;

    println!("\n{}:\n", habit);
    let days = 10;
    let mut date = helper::today()? - Duration::DAY * (days - 1);
    for _ in 0..days {
        let done = match habit.done_on_date(date) {
            true => "done",
            false => "",
        };
        println!("{}: {}", date, done);
        date += Duration::DAY;
    }
    ui::input("\nPress <Enter> to continue.")?;
    Ok(())
}
