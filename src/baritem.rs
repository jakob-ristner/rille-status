pub mod audio;
pub mod backlight;
pub mod charge;
pub mod network;
pub mod time;

pub static BAR_HEIGHT: u32 = 62;

pub trait BarItem {
    fn get_bar_text(&self) -> String;
}

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub fn icon_from_percent(icons: &[char], percent: u32) -> &char {
    // 100% will pick the first item in the array
    let index = (percent as f64) * (icons.len() as f64 / 100.0);
    &icons[(icons.len() - index as usize).min(icons.len() - 1)]
}

impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn white() -> Self {
        Color {
            r: 255,
            g: 255,
            b: 255,
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
