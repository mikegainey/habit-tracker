use crate::helper;
use serde::{Deserialize, Serialize};
use std::fmt;
use time::{Date, Duration, OffsetDateTime, macros::format_description};

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

    pub fn done_today(&self) -> anyhow::Result<bool> {
        let today = helper::today()?;
        Ok(self.check_date(today))
    }

    pub fn change_name(&mut self, new_name: String) {
        self.name = new_name
    }

    // not used for now
    pub fn get_timestamps(&self) -> &[OffsetDateTime] {
        &self.timestamps
    }

    pub fn list_times(&self, date: Date) -> String {
        let format = format_description!("[hour]:[minute]");
        self.timestamps
            .iter()
            .filter(|&odt| odt.date() == date)
            .map(|d| d.format(&format).unwrap_or("unknown".to_string()))
            .collect::<Vec<_>>()
            .join(", ")
    }

    // not used
    // pub fn total_completions(&self) -> u16 {
    //     self.timestamps.len() as u16
    // }

    pub fn ending_streak(&self) -> anyhow::Result<usize> {
        if self.timestamps.is_empty() {
            return Ok(0);
        }

        // 1. Prepare the data
        let mut dates: Vec<Date> = self.timestamps.iter().map(|dt| dt.date()).collect();
        dates.dedup();

        // 2. Identify our anchor points
        let today = helper::today()?;
        let yesterday = today - Duration::DAY;

        // 3. Determine the starting point of the walk
        // The streak is active if the most recent entry is Today OR Yesterday.
        let mut current_check = if dates.contains(&today) {
            today
        } else if dates.contains(&yesterday) {
            yesterday
        } else {
            // If the last entry was 2+ days ago, the streak is broken.
            return Ok(0);
        };

        // 4. Walk backward through the calendar
        let mut count = 0;
        while dates.contains(&current_check) {
            count += 1;
            current_check -= Duration::DAY;
        }

        Ok(count)
    }

    // this function should be tested for correctness
    pub fn last_30_days(&self) -> anyhow::Result<usize> {
        let month_ago: Date = helper::today()? - Duration::DAY * 30;
        Ok(self
            .timestamps
            .iter()
            .filter(|dt| dt.date() > month_ago) // should it be >= ?
            .count())
    }
}

impl fmt::Display for Habit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&self.name)
    }
}
