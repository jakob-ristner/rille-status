pub mod audio;
pub mod backlight;
pub mod charge;
pub mod network;
pub mod time;

pub trait BarItem {
    fn get_bar_text(&self) -> String;
}
