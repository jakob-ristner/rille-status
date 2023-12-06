use std::process::Command;

use regex::Regex;

use super::{regtangle, BarItem, Color, BAR_HEIGHT};

pub struct Audio {
    regex: Regex,
    fill_color: Color,
    sep_color: Color,
    emp_color: Color,
    sep_height: u32,
    fill_height: u32,
    sep_width: u32,
}

impl Audio {
    pub fn new() -> Self {
        Audio {
            regex: Regex::new(r"(?<perc>\d+)%").unwrap(),
            fill_color: Color::white(),
            sep_color: Color::white(),
            emp_color: Color::white(),
            sep_height: 22,
            fill_height: 2,
            sep_width: 2,
        }
    }
}
impl BarItem for Audio {
    fn get_bar_text(&self) -> String {
        let amixer = Command::new("amixer").args(["sget", "Master"]).output();
        if let Ok(out) = amixer {
            let text = String::from_utf8(out.stdout).unwrap();
            let percent: u32 = self
                .regex
                .captures(&text)
                .unwrap()
                .name("perc")
                .unwrap()
                .as_str()
                .parse()
                .unwrap();

            let rec = regtangle(
                0,
                (BAR_HEIGHT - self.fill_height) / 2,
                (100 - percent),
                self.fill_height,
                &self.emp_color.as_str(),
            );
            let mid = regtangle(0, 20, 2, 22, &self.sep_color.as_str());
            let rec2 = regtangle(
                0,
                (BAR_HEIGHT - self.fill_height) / 2,
                (percent),
                self.fill_height,
                &self.fill_color.as_str(),
            );

            let icon = if text.contains("off") { "󰝛" } else { "󰝚" };
            return format!("{}  {}{}{} ", icon, rec2, mid, rec);
        }
        return String::from("");
    }
}
