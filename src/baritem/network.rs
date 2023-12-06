use std::{ops::Div, process::Command};

use regex::Regex;

use super::BarItem;

pub struct Network {
    id: String,
    quality_re: Regex,
}

#[derive(Debug)]
pub enum NetworkState {
    Up,
    Down,
}

impl BarItem for Network {
    fn get_bar_text(&self) -> String {
        //TODO clean up add id to network
        let state = self.get_state();
        let disconnected = String::from("󰤭  disconnected ");
        match state {
            NetworkState::Up => {
                let m_essid = self.get_essid();
                let m_link = self.link_quality();
                if let (Some(essid), Some(link)) = (m_essid, m_link) {
                    return format!("󰤨  {} {}% ", essid, link);
                } else {
                    disconnected
                    // TODO print to stderr
                }
            }
            NetworkState::Down => disconnected,
        }
    }
}

impl Network {
    pub fn new(id: &str) -> Self {
        Network {
            id: id.to_string(),
            quality_re: Regex::new(r"Link Quality=(?<qual>\d+)[/](?<max>\d+)").unwrap(),
        }
    }

    pub fn get_state(&self) -> NetworkState {
        let link = self.link_quality(); // TEST
        self.try_get_state().unwrap_or(NetworkState::Down)
    }

    fn link_quality(&self) -> Option<u32> {
        let command = Command::new("iwconfig").output().ok()?;
        let text = String::from_utf8(command.stdout).ok()?;
        let cap = self.quality_re.captures(&text)?;
        let qual: f64 = cap.name("qual")?.as_str().parse().ok()?;
        let max: f64 = cap.name("max")?.as_str().parse().ok()?;
        Some((qual.div(max) * 100.0) as u32)
    }

    fn try_get_state(&self) -> Option<NetworkState> {
        let state = Command::new("cat")
            .args([format!("/sys/class/net/{}/operstate", self.id)])
            .output()
            .ok()?;
        let text = String::from_utf8(state.stdout).ok()?;
        if text.contains("up") {
            Some(NetworkState::Up)
        } else {
            Some(NetworkState::Down)
        }
    }

    pub fn get_essid(&self) -> Option<String> {
        let command = Command::new("iwgetid").args(["-r"]).output().ok()?;
        let text = String::from_utf8(command.stdout).ok()?.trim().to_string();
        Some(text)
    }
}
