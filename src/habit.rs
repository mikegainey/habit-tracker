#![allow(dead_code)]
// Possible methods:
// compute current streak
// compute longest streak later if you want
// Rule of thumb: if a function only needs one habit, it probably belongs here.
// later: struct Date(String) should be used to validate a date (make a date module)

#[derive(Debug)]
pub struct Habit {
    name: String,
    description: String,
    completed_dates: Vec<String>, // like "2026-03-11"
}

impl Habit {
    pub fn new(name: String, description: String) -> Habit {
        Habit {
            name,
            description,
            completed_dates: Vec::new(),
        }
    }

    pub fn mark_done(&mut self, date: String) {
        match self.check_date(&date) {
            true => return,
            false => self.completed_dates.push(date),
        }
    }

    pub fn total_completions(&self) -> u16 {
        self.completed_dates.len() as u16
    }

    pub fn check_date(&self, date: &String) -> bool {
        self.completed_dates.contains(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let habit = Habit::new(
            "1hr cardio".into(),
            "1hr on the eliptical at level 6".into(),
        );
        println!("{:?}", habit) // use cargo test -- --nocapture to see this output
    }
}
// Test things like:
// marking done once
// checking whether a habit was done on a date
// counting completions
