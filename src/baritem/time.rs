use chrono::{DateTime, Local};

use super::BarItem;

pub struct Time {}

impl BarItem for Time {
    fn get_bar_text(&self) -> String {
        let time = self.curr_time();
        return format!("󰥔  {} ", time);
    }
}

impl Time {
    pub fn new() -> Self {
        Time {}
    }

    pub fn curr_time(&self) -> String {
        let dt: DateTime<Local> = Local::now();
        dt.format("%a %d %h %H:%M").to_string()
    }
}
