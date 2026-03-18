use crate::ui;
use serde::{Deserialize, Serialize};
use std::fmt;
use time::{Date, OffsetDateTime, macros::format_description};

#[derive(Serialize, Deserialize, Debug)]
pub struct Habit {
    name: String,
    timestamps: Vec<OffsetDateTime>,
}

impl Habit {
    pub fn new(name: String) -> Habit {
        Habit {
            name,
            timestamps: Vec::new(),
        }
    }

    pub fn mark_complete(&mut self, date: OffsetDateTime) {
        self.timestamps.push(date);
    }

    pub fn check_date(&self, date: Date) -> bool {
        self.timestamps
            .iter()
            .any(|datetime| datetime.date() == date)
    }

    pub fn done_today(&self) -> bool {
        let today = ui::today();
        self.check_date(today)
    }

    pub fn change_name(&mut self, new_name: String) {
        self.name = new_name
    }

    pub fn get_timestamps(&self) -> &Vec<OffsetDateTime> {
        &self.timestamps
    }

    pub fn list_times(&self, date: Date) -> String {
        let format = format_description!("[hour]:[minute]");
        self.timestamps
            .iter()
            .filter(|&odt| odt.date() == date)
            .map(|odt| odt.format(&format).expect("failed to format time"))
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn total_completions(&self) -> u16 {
        self.timestamps.len() as u16
    }
}

impl fmt::Display for Habit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
