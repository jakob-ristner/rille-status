use std::process::Command;

use battery::State;
use regex::Regex;

use self::{audio::Audio, backlight::Backlight, charge::Bat, network::Network, time::Time};

pub mod audio;
pub mod backlight;
pub mod charge;
pub mod network;
pub mod time;

pub trait BarItem {
    fn get_bar_text(&self) -> String;
}

impl BarItem for Audio {
    fn get_bar_text(&self) -> String {
        let amixer = Command::new("amixer").args(["sget", "Master"]).output();
        let re_perc = Regex::new(r"(?<perc>\d+)%").unwrap();
        if let Ok(out) = amixer {
            let text = String::from_utf8(out.stdout).unwrap();
            let icon = if text.contains("off") { "󰝛" } else { "󰝚" };
            let cap = re_perc
                .captures(&text)
                .unwrap()
                .name("perc")
                .unwrap()
                .as_str();
            return format!("{} {}% ", icon, cap);
        }
        return String::from("");
    }
}

impl BarItem for Backlight {
    fn get_bar_text(&self) -> String {
        if let Some(brightness) = self.get_brightness() {
            return format!("󰖙 {}% ", brightness);
        }
        return String::from("");
    }
}

impl BarItem for Bat {
    fn get_bar_text(&self) -> String {
        let m_state = self.state();
        let m_charge = self.charge_percent();

        if let (Some(state), Some(charge)) = (m_state, m_charge) {
            let icon = match state {
                State::Discharging => "",
                _ => "󰠠",
            };
            return format!("{}  {}% ", icon, charge);
        }
        return String::from("");
    }
}

impl BarItem for Network {
    fn get_bar_text(&self) -> String {
        //TODO clean up add id to network
        let is_conn = Command::new("cat")
            .args(["/sys/class/net/wlp0s20f3/operstate"])
            .output()
            .unwrap()
            .stdout;
        // let is_conn = Command::new("iwgetid").args(["-r"]).output();
        dbg!(String::from_utf8(is_conn).unwrap().trim());
        if let Some(essid) = self.get_essid() {
            return format!("󰤨  {} ", essid);
        }
        return String::from("󰤭  disconnected ");
    }
}

impl BarItem for Time {
    fn get_bar_text(&self) -> String {
        let time = self.curr_time();
        return format!("󰥔  {} ", time);
    }
}
