use std::process::Command;

use dbus::blocking::Connection;
use networkmanager::{
    devices::{Any, Device},
    NetworkManager,
};

pub struct Network {
    dbus: Connection,
    id: String,
}

impl Network {
    pub fn new(id: &str) -> Self {
        let dbus = Connection::new_system().unwrap();
        Network {
            dbus,
            id: id.to_string(),
        }
    }

    pub fn get_essid(&self) -> Option<String> {
        let nm = NetworkManager::new(&self.dbus);
        let enp0s2 = nm.get_device_by_ip_iface(&self.id).unwrap();
        match enp0s2 {
            Device::WiFi(x) => {
                if let Ok(conn) = x.active_connection() {
                    return conn.id().ok();
                }
                None
            }
            _ => None,
        }
    }
}
