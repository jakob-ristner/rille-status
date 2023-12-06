use brightness::blocking::{brightness_devices, Brightness, BrightnessDevice};

use super::BarItem;

pub struct Backlight {
    device: Option<BrightnessDevice>,
    icons: Vec<char>,
}
impl BarItem for Backlight {
    fn get_bar_text(&self) -> String {
        if let Some(brightness) = self.get_brightness() {
            return format!("{} ", self.get_icon(brightness));
        }
        return String::from("");
    }
}

impl Backlight {
    pub fn new(id: &str) -> Self {
        Backlight {
            device: get_device(id),
            icons: vec![
                '󰽢', '', '', '', '', '', '', '', '', '', '', '', '', '',
            ],
        }
    }

    fn get_icon(&self, percent: u32) -> char {
        let index = (percent as f64) * (self.icons.len() as f64 / 100.0);
        self.icons[(self.icons.len() - index as usize).min(self.icons.len() - 1)]
    }

    pub fn get_brightness(&self) -> Option<u32> {
        println!(
            "{}",
            self.get_icon(self.device.as_ref().unwrap().get().unwrap())
        );
        self.device.as_ref()?.get().ok()
    }
}

fn get_device(id: &str) -> Option<BrightnessDevice> {
    for d in brightness_devices() {
        if d.as_ref().ok()?.device_name().ok()? == id {
            return d.ok();
        }
    }
    None
}
