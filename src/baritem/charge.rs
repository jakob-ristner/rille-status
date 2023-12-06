use anyhow::{bail, Result};
use battery::{units::ratio::percent, Battery, Manager, State};

pub struct Bat {
    bat: usize,
}

impl Bat {
    pub fn new(id: usize) -> Self {
        Bat { bat: id }
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
        let bat = Self::get_bat(self.bat).ok();
        Some(bat?.state_of_charge().get::<percent>().into())
    }

    pub fn state(&self) -> Option<State> {
        let bat = Self::get_bat(self.bat).ok();
        Some(bat?.state())
    }
}
