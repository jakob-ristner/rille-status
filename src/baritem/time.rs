use chrono::{DateTime, Local};

pub struct Time {}

impl Time {
    pub fn new() -> Self {
        Time {}
    }

    pub fn curr_time(&self) -> String {
        let dt: DateTime<Local> = Local::now();
        dt.format("%a %d %h %H:%M").to_string()
    }
}
