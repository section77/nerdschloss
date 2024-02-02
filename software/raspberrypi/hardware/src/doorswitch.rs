#[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
use std::{fs, io::prelude::*};

use serde::{Deserialize, Serialize};

#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    target_env = "musl",
    target_os = "linux"
))]
use rppal::gpio::{Gpio, InputPin, Level};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DoorSwitchConfiguration {
    pub pin: u8,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum DoorSwitchState {
    #[default]
    Locked,
    Unlocked,
}

impl std::convert::From<bool> for DoorSwitchState {
    fn from(b: bool) -> Self {
        match b {
            true => DoorSwitchState::Unlocked,
            false => DoorSwitchState::Locked,
        }
    }
}

impl std::convert::From<DoorSwitchState> for bool {
    fn from(dlss: DoorSwitchState) -> Self {
        match dlss {
            DoorSwitchState::Unlocked => true,
            DoorSwitchState::Locked => false,
        }
    }
}

pub trait DoorSwitchStateTrait {
    fn state(&self) -> DoorSwitchState;
}

#[derive(Debug)]
pub struct DoorSwitch {
    #[cfg(all(
        any(target_arch = "arm", target_arch = "aarch64"),
        target_env = "musl",
        target_os = "linux"
    ))]
    doorswitch_gpio: InputPin,
}

impl DoorSwitch {
    #[cfg(all(
        any(target_arch = "arm", target_arch = "aarch64"),
        target_env = "musl",
        target_os = "linux"
    ))]
    pub fn new(configuration: DoorSwitchConfiguration) -> Self {
        Self {
            doorswitch_gpio: Gpio::new()
                .unwrap()
                .get(configuration.pin)
                .unwrap()
                .into_input_pullup(),
        }
    }

    #[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
    pub fn new(_configuration: DoorSwitchConfiguration) -> Self {
        Self::check_state_file();
        Self {}
    }

    #[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
    fn check_state_file() {
        if fs::metadata(super::DOORSWITCH_STATE_FILE).is_err() {
            let mut file = fs::File::create(super::DOORSWITCH_STATE_FILE).unwrap();
            write!(file, "false").unwrap();
        }
    }
}

impl DoorSwitchStateTrait for DoorSwitch {
    #[cfg(all(
        any(target_arch = "arm", target_arch = "aarch64"),
        target_env = "musl",
        target_os = "linux"
    ))]
    fn state(&self) -> DoorSwitchState {
        match &self.doorswitch_gpio.read() {
            Level::Low => DoorSwitchState::Locked,
            Level::High => DoorSwitchState::Unlocked,
        }
    }

    #[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
    fn state(&self) -> DoorSwitchState {
        let mut file = fs::File::open(super::DOORSWITCH_STATE_FILE).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let s: bool = contents.trim().parse().unwrap();

        s.into()
    }
}
