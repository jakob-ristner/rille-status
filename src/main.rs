use std::process::Command;

use crate::baritem::{
    audio::Audio, backlight::Backlight, charge::Bat, network::Network, time::Time,
};

use baritem::BarItem;

mod baritem;

fn main() {
    let _ = Command::new("xsetroot").args(["-name", "abc asd"]).output();
    //
    //

    let bat = Bat::new(0);
    let time = Time::new();
    let backlight = Backlight::new("intel_backlight");
    let audio = Audio::new();
    let network = Network::new("wlp0s20f3");

    let baritems: Vec<&dyn BarItem> = vec![&bat, &time, &backlight, &audio, &network];

    let mut bar_text = String::new();
    loop {
        bar_text.clear();
        bar_text.push_str("                    ^c#D8DEE9^");
        for item in baritems.iter() {
            bar_text.push_str(&item.get_bar_text())
        }
        let _ = Command::new("xsetroot").args(["-name", &bar_text]).output();
    }
}
