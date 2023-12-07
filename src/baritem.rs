use std::{cmp::min, time::Duration};

pub mod audio;
pub mod backlight;
pub mod charge;
pub mod network;
pub mod spotify;
pub mod time;

pub static BAR_HEIGHT: u32 = 62;

pub trait BarItem {
    fn get_bar_text(&mut self) -> String;
}

#[derive(Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub fn icon_from_percent(icons: &[char], percent: u32) -> &char {
    let act_index = (((percent as f64) / 100.0) * (icons.len() - 1) as f64) as usize;
    &icons[act_index]
}

impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn nord_white() -> Self {
        Color {
            r: 216,
            g: 222,
            b: 233,
        }
    }
    pub fn nord_red() -> Self {
        Color {
            r: 191,
            g: 97,
            b: 106,
        }
    }
    pub fn nord_orange() -> Self {
        Color {
            r: 208,
            g: 135,
            b: 112,
        }
    }
    pub fn nord_yellow() -> Self {
        Color {
            r: 235,
            g: 203,
            b: 129,
        }
    }
    pub fn nord_green() -> Self {
        Color {
            r: 163,
            g: 190,
            b: 140,
        }
    }
    pub fn nord_purple() -> Self {
        Color {
            r: 180,
            g: 142,
            b: 173,
        }
    }
    pub fn nord_blue() -> Self {
        Color {
            r: 129,
            g: 161,
            b: 193,
        }
    }

    pub fn as_str(&self) -> String {
        format!("#{:x}{:x}{:x}", self.r, self.g, self.b)
    }

    pub fn apply_fg(&self) -> String {
        format!("^c{}^", self.as_str())
    }
    pub fn apply_bg(&self) -> String {
        format!("^b{}^", self.as_str())
    }
}

pub fn regtangle(x: u32, y: u32, w: u32, h: u32, col: &str) -> String {
    format!("^c{}^^r{},{},{},{}^^f{}^", col, x, y, w, h, w)
}
