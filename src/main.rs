use crate::baritem::{
    audio::Audio, backlight::Backlight, charge::Bat, network::Network, time::Clock,
};
use baritem::spotify::Spotify;
use baritem::weather::Weather;
use baritem::BarItem;
use std::process::Command;
use std::thread::sleep;
mod baritem;

fn main() {
    let mut bat = Bat::new(0);
    let mut clock = Clock::new();
    let mut backlight = Backlight::new("intel_backlight");
    let mut audio = Audio::new();
    let mut network = Network::new("wlp0s20f3");
    let mut spotify = Spotify::new();
    let mut weather = Weather::new("Gothenburg");

    let mut baritems: Vec<&mut dyn BarItem> = vec![
        &mut weather,
        &mut spotify,
        &mut clock,
        &mut network,
        &mut audio,
        &mut bat,
        &mut backlight,
    ];

    let mut bar_text = String::new();
    loop {
        sleep(std::time::Duration::from_millis(100));
        bar_text.clear();
        bar_text.push_str("                    ^c#D8DEE9^");
        for item in baritems.iter_mut() {
            bar_text.push_str(&item.get_bar_text());
        }
        let _ = Command::new("xsetroot").args(["-name", &bar_text]).output();
    }
}
