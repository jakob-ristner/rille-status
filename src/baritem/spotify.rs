use std::{
    process::Command,
    sync::mpsc::{channel, Receiver},
    thread::{self, sleep},
    time::{Duration, SystemTime},
};

use regex::Regex;

use super::{BarItem, Color};

pub struct Spotify {
    ch: Receiver<Option<String>>,
    bar_text: Option<String>,
    max_length: usize,
    ctr: f64,
    time: SystemTime,
    wrap_ctr: usize,
    wrap_speed: f64,
    color: Color,
}

#[derive(Debug)]
struct MetaData {
    title: String,
    album: String,
    artist: String,
}
impl ToString for MetaData {
    fn to_string(&self) -> String {
        format!("{} - {}", self.title, self.album)
    }
}

fn get_data(title_re: &Regex, artist_re: &Regex, album_re: &Regex) -> Option<MetaData> {
    let command = Command::new("playerctl").args(["metadata"]).output();
    let text = String::from_utf8(command.ok()?.stdout).ok()?;
    let title = title_re
        .captures(&text)?
        .name("title")?
        .as_str()
        .to_string();
    let artist = artist_re
        .captures(&text)?
        .name("artist")?
        .as_str()
        .to_string();
    let album = album_re
        .captures(&text)?
        .name("album")?
        .as_str()
        .to_string();
    Some(MetaData {
        title,
        album,
        artist,
    })
}

impl Spotify {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        thread::spawn(move || {
            let title_re = Regex::new(r"title\s+(?<title>\w+.*)").unwrap();
            let artist_re = Regex::new(r"artist\s+(?<artist>\w+.*)").unwrap();
            let album_re = Regex::new(r"album\s+(?<album>\w+.*)").unwrap();
            loop {
                if let Some(metadata) = get_data(&title_re, &artist_re, &album_re) {
                    let _ = sender.send(Some(metadata.to_string()));
                } else {
                    let _ = sender.send(None);
                }
                sleep(Duration::from_secs(1));
            }
        });
        Spotify {
            ch: receiver,
            bar_text: None,
            max_length: 20,
            ctr: 0.0,
            time: SystemTime::now(),
            wrap_ctr: 0,
            wrap_speed: 0.3,
            color: Color::nord_green(),
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
        self.ctr += self.time.elapsed().unwrap().as_secs_f64();
        self.time = SystemTime::now();
        self.update();

        if self.ctr > self.wrap_speed {
            self.wrap_ctr += 1;
            self.ctr = 0.0;
        }

        match &self.bar_text {
            Some(text) => {
                self.wrap_ctr %= text.len();
                let mut out: String = if text.len() > self.max_length {
                    let mut chars: Vec<char> = text.chars().collect();
                    chars.push(' ');
                    chars.rotate_right(self.wrap_ctr);
                    let slice = &chars[0..self.max_length];
                    slice.iter().collect()
                } else {
                    text.clone()
                };
                format!("{}󰓇  {} ", self.color.apply_fg(), out)
            }
            None => String::from(""),
        }
    }
}
