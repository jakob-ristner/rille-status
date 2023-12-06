use crate::baritem::{
    audio::Audio, backlight::Backlight, charge::Bat, network::Network, time::Time,
};
use baritem::BarItem;
use std::process::Command;
use std::thread::sleep;
use std::{sync::mpsc::channel, thread, time::Duration};

mod baritem;

fn main() {
    let bat = Bat::new(0);
    let time = Time::new();
    let backlight = Backlight::new("intel_backlight");
    let audio = Audio::new();
    let network = Network::new("wlp0s20f3");

    // let (sender, receiver) = channel();
    // thread::spawn(move || {
    //     sender
    //         .send("heavy computation 1")
    //         .expect("receiver hung up :(");
    //     thread::sleep(Duration::from_millis(500));
    //     sender
    //         .send("heavy computation 2")
    //         .expect("receiver hung up :(");
    // });
    let baritems: Vec<&dyn BarItem> = vec![&bat, &time, &audio, &network, &backlight];

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
