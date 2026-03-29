use crate::habit::Habit;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

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

    pub fn get_habit(&self, index: usize) -> Option<&Habit> {
        self.habits.get(index)
    }

    pub fn get_mut_habit(&mut self, index: usize) -> Option<&mut Habit> {
        self.habits.get_mut(index)
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

    pub fn mark_habit_complete(&mut self, index: usize, datetime: OffsetDateTime) -> Result<()> {
        let habit: &mut Habit = self
            .habits
            .get_mut(index)
            .ok_or(anyhow!("index not found"))?;
        habit.mark_complete(datetime);
        Ok(())
    }

    pub fn change_name(&mut self, index: usize, new_name: String) -> anyhow::Result<()> {
        let habit: &mut Habit = self
            .habits
            .get_mut(index)
            .ok_or(anyhow!("index not found"))?;
        habit.change_name(new_name);
        Ok(())
    }

    pub fn reset_completions(&mut self, index: usize) -> anyhow::Result<()> {
        let habit: &mut Habit = self
            .habits
            .get_mut(index)
            .ok_or(anyhow!("index not found"))?;
        habit.reset_completions();
        Ok(())
    }
}
