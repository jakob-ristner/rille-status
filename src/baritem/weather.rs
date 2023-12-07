use std::{
    process::Command,
    sync::mpsc::{channel, Receiver},
    thread::{self, sleep},
    time::{Duration, SystemTime},
};

use regex::Regex;

use super::{BarItem, Color};

pub struct Weather {
    ch: Receiver<Option<String>>,
    bar_text: Option<String>,
    color: Color,
}

fn get_data(region: &str) -> Option<String> {
    let command = Command::new("curl")
        .arg(format!("wttr.in/{}?format=1", region))
        .output()
        .ok()?;
    let text = String::from_utf8(command.stdout).ok()?.trim().to_string();
    dbg!(text.chars().collect::<Vec<char>>());
    Some(text.split_whitespace().collect::<Vec<&str>>().join(" "))
}

impl Weather {
    pub fn new(region: &str) -> Self {
        let region = region.to_string();
        let (sender, receiver) = channel();
        thread::spawn(move || loop {
            let _ = sender.send(get_data(&region));
            sleep(Duration::from_secs(1));
        });
        Weather {
            ch: receiver,
            bar_text: None,
            color: Color::nord_red(),
        }
    }

    fn update(&mut self) {
        match self.ch.try_recv() {
            Ok(text) => self.bar_text = text,
            Err(_) => {}
        }
    }
}

impl BarItem for Weather {
    fn get_bar_text(&mut self) -> String {
        self.update();

        match &self.bar_text {
            Some(text) => format!("{}{} ", self.color.apply_fg(), text.to_string()),
            None => String::from(""),
        }
    }
}
