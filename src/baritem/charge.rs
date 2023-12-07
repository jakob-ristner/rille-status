use anyhow::{bail, Result};
use battery::{units::ratio::percent, Battery, Manager, State};

use super::{icon_from_percent, BarItem};

pub struct Bat {
    bat_id: usize,
    icons: Vec<char>,
    discharging_icon: char,
}

impl BarItem for Bat {
    fn get_bar_text(&mut self) -> String {
        let m_state = self.state();
        let m_charge = self.charge_percent();

        if let (Some(state), Some(charge)) = (m_state, m_charge) {
            let icon = match state {
                State::Discharging => icon_from_percent(&self.icons, charge),
                _ => &self.discharging_icon,
            };
            return format!("{}  {}% ", icon, charge);
        }
        return String::from("");
    }
}

impl Bat {
    pub fn new(bat_id: usize) -> Self {
        Bat {
            bat_id,
            icons: vec!['', '', '', '', ''],
            discharging_icon: '󰠠',
        }
    }

    fn get_bat(id: usize) -> Result<Battery> {
        if let Some((_, maybe_bat)) = Manager::new()?
            .batteries()?
            .enumerate()
            .find(|(i, _)| i == &id)
        {
            return Ok(maybe_bat?);
        }
        bail!("Could not find battery")
    }

    pub fn charge_percent(&self) -> Option<u32> {
        let bat = Self::get_bat(self.bat_id).ok()?;
        let float_percent: f64 = bat.state_of_charge().get::<percent>().into();
        Some(float_percent as u32)
    }

    pub fn state(&self) -> Option<State> {
        let bat = Self::get_bat(self.bat_id).ok();
        Some(bat?.state())
    }
}
