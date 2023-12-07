use std::time::Duration;

use chrono::{DateTime, Local};

use super::{BarItem, Color};

pub struct Clock {
    icon_color: Color,
    date_color: Color,
}

impl BarItem for Clock {
    fn get_bar_text(&mut self) -> String {
        let time = self.curr_time();
        return format!(
            "{}󰥔  {}{} ",
            self.icon_color.apply_fg(),
            self.date_color.apply_fg(),
            time
        );
    }
}

impl Clock {
    pub fn new() -> Self {
        Clock {
            icon_color: Color::nord_orange(),
            date_color: Color::nord_orange(),
        }
    }

    pub fn curr_time(&self) -> String {
        let dt: DateTime<Local> = Local::now();
        dt.format("%a %d %h %H:%M").to_string()
    }
}
