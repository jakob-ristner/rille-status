use anyhow::{bail, Result};
use battery::{units::ratio::percent, Battery, Manager, State};

use super::BarItem;

pub struct Bat {
    bat_id: usize,
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

impl Bat {
    pub fn new(bat_id: usize) -> Self {
        Bat { bat_id }
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

    pub fn charge_percent(&self) -> Option<f64> {
        let bat = Self::get_bat(self.bat_id).ok()?;
        Some(bat.state_of_charge().get::<percent>().into())
    }

    pub fn state(&self) -> Option<State> {
        let bat = Self::get_bat(self.bat_id).ok();
        Some(bat?.state())
    }
}
