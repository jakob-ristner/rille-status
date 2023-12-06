use brightness::blocking::{brightness_devices, Brightness, BrightnessDevice};

pub struct Backlight {
    device: Option<BrightnessDevice>,
}

impl Backlight {
    pub fn new(id: &str) -> Self {
        Backlight {
            device: get_device(id),
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
