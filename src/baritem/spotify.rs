use std::{
    process::Command,
    sync::mpsc::{channel, Receiver},
    thread::{self, sleep},
    time::Duration,
};

use regex::Regex;

use super::BarItem;

pub struct Spotify {
    ch: Receiver<String>,
    bar_text: String,
}

fn getTitle(re: &Regex) -> Option<String> {
    let command = Command::new("playerctl").args(["metadata"]).output();
    let text = String::from_utf8(command.ok()?.stdout).ok()?;
    Some(re.captures(&text)?.name("title")?.as_str().to_string())
}

impl Spotify {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        thread::spawn(move || {
            let re = Regex::new(r"title\s+(?<title>\w+.*)").unwrap();
            for i in 0.. {
                dbg!(getTitle(&re));
                sleep(Duration::from_secs(1));
                let _ = sender.send(String::from(format!("{}", i)));
            }
        });
        Spotify {
            ch: receiver,
            bar_text: String::new(),
        }
    }

    fn update(&mut self) {
        match self.ch.try_recv() {
            Ok(text) => self.bar_text = text,
            Err(_) => {}
        }
    }
}

impl BarItem for Spotify {
    fn get_bar_text(&mut self) -> String {
        self.update();
        self.bar_text.clone()
    }
}
