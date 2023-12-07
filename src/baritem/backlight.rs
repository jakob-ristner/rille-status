use brightness::blocking::{brightness_devices, Brightness, BrightnessDevice};

use crate::baritem::icon_from_percent;

use super::{BarItem, Color};

pub struct Backlight {
    device: Option<BrightnessDevice>,
    icons: Vec<char>,
    color: Color,
}
impl BarItem for Backlight {
    fn get_bar_text(&mut self) -> String {
        if let Some(brightness) = self.get_brightness() {
            return format!(
                "{}{} ",
                self.color.apply_fg(),
                icon_from_percent(&self.icons, brightness)
            );
        }
        return String::from("");
    }
}

impl Backlight {
    pub fn new(id: &str) -> Self {
        Backlight {
            device: get_device(id),
            icons: vec![
                '', '', '', '', '', '', '', '', '', '', '', '', '', '', '',
            ],
            color: Color::nord_white(),
        }
    }

    pub fn get_brightness(&self) -> Option<u32> {
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
