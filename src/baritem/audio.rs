use std::process::Command;

use regex::Regex;

use super::BarItem;

pub struct Audio {
    regex: Regex,
}

impl Audio {
    pub fn new() -> Self {
        Audio {
            regex: Regex::new(r"(?<perc>\d+)%").unwrap(),
        }
    }
}
impl BarItem for Audio {
    fn get_bar_text(&self) -> String {
        let amixer = Command::new("amixer").args(["sget", "Master"]).output();
        if let Ok(out) = amixer {
            let text = String::from_utf8(out.stdout).unwrap();
            let icon = if text.contains("off") { "󰝛" } else { "󰝚" };
            let cap = self
                .regex
                .captures(&text)
                .unwrap()
                .name("perc")
                .unwrap()
                .as_str();
            return format!("{} {}% ", icon, cap);
        }
        return String::from("");
    }
}
