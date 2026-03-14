#![allow(dead_code)]
// This should define the big App struct: the full state of the program.
// Possible methods:
// remove a habit
// mark a habit done
// add a journal entry
// search habits by text
// create reports
// maybe return “habits done today”
// maybe return “habits not done today”
// Rule of thumb: if the operation needs to look across the whole collection of habits, it belongs here.
use crate::habit::Habit;
use crate::journal::JournalEntry;

#[derive(Debug)]
pub struct App {
    habits: Vec<Habit>,
    journal_entries: Vec<JournalEntry>,
}

impl App {
    pub fn new() -> App {
        App {
            habits: Vec::new(),
            journal_entries: Vec::new(),
        }
    }

    pub fn get_habits(&self) -> &Vec<Habit> {
        &self.habits
    }

    pub fn add_habit(&mut self, habit: Habit) {
        self.habits.push(habit);
    }
}

// Test things like:
// adding habits increments IDs properly
// removing the correct habit
// marking the correct habit done
// searching habits works
