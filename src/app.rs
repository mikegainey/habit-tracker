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

    pub fn get_mut_habits(&mut self) -> &mut Vec<Habit> {
        &mut self.habits
    }

    pub fn add_habit(&mut self, habit: Habit) {
        self.habits.push(habit);
    }

    pub fn get_mut_habit(&mut self, index: usize) -> Option<&mut Habit> {
        self.habits.get_mut(index)
    }
}
