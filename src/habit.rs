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

    pub fn done_on_date(&self, date: Date) -> bool {
        self.timestamps
            .iter()
            .any(|datetime| datetime.date() == date)
    }

    pub fn change_name(&mut self, new_name: String) {
        self.name = new_name
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

    pub fn ending_streak(&self, today: Date) -> usize {
        if self.timestamps.is_empty() {
            return 0;
        }

        // 1. Prepare the data
        let mut dates: Vec<Date> = self.timestamps.iter().map(|dt| dt.date()).collect();
        // The timestamps are pushed to self.timestamps in chronological order, and never reordered.
        dates.dedup();

        // 2. Identify our anchor points
        let yesterday = today - Duration::DAY;

        // 3. Determine the starting point of the walk
        // The streak is active if the most recent entry is Today OR Yesterday.
        let mut current_check = if dates.contains(&today) {
            today
        } else if dates.contains(&yesterday) {
            yesterday
        } else {
            // If the last entry was 2+ days ago, the streak is broken.
            return 0;
        };

        // 4. Walk backward through the calendar
        let mut count = 0;
        while dates.contains(&current_check) {
            count += 1;
            current_check -= Duration::DAY;
        }

        count
    }

    // Count the number of days with a completion in the last 30 days
    pub fn last_30_days(&self, today: Date) -> usize {
        let mut dates: Vec<Date> = self.timestamps.iter().map(|dt| dt.date()).collect();
        // The timestamps are pushed to self.timestamps in chronological order, and never reordered.
        dates.dedup();

        let month_ago: Date = today - Duration::DAY * 30;
        dates
            .iter()
            .filter(|&&dt| dt > month_ago) // should it be >= ?
            .count()
    }
}

impl fmt::Display for Habit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&self.name)
    }
}
