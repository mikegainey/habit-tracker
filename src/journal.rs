#![allow(dead_code)]

#[derive(Debug)]
pub struct JournalEntry {
    date: String,
    text: String,
}

impl JournalEntry {
    fn new(date: String, text: String) -> JournalEntry {
        JournalEntry { date, text }
    }
}
