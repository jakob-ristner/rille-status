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
}

pub fn regtangle(x: u32, y: u32, w: u32, h: u32, col: &str) -> String {
    format!("^c{}^^r{},{},{},{}^^f{}^^d^", col, x, y, w, h, w)
}
