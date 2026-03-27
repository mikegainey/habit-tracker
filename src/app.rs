use crate::habit::Habit;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    habits: Vec<Habit>,
}

impl App {
    pub fn new() -> App {
        App { habits: Vec::new() }
    }

    pub fn get_habits(&self) -> &[Habit] {
        &self.habits
    }

    pub fn remove_habit(&mut self, index: usize) -> anyhow::Result<()> {
        if index >= self.habits.len() {
            anyhow::bail!("selection out of range: {}", index + 1);
        }
        self.habits.remove(index);
        Ok(())
    }

    pub fn add_habit(&mut self, name: String) {
        let habit = Habit::new(name);
        self.habits.push(habit);
    }

    pub fn get_habit(&mut self, index: usize) -> Option<&Habit> {
        self.habits.get(index)
    }

    pub fn get_mut_habit(&mut self, index: usize) -> Option<&mut Habit> {
        self.habits.get_mut(index)
    }
}
