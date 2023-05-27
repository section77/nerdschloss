use std::{fs, io::prelude::*};

#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    target_env = "musl",
    target_os = "linux"
))]
use rppal::gpio::{Gpio, InputPin, Level};

#[derive(Debug, Default, Clone, Copy)]
pub enum DorLockSwitchState {
    #[default]
    Locked,
    Unlocked,
}

impl std::convert::From<bool> for DorLockSwitchState {
    fn from(b: bool) -> Self {
        match b {
            true => DorLockSwitchState::Unlocked,
            false => DorLockSwitchState::Locked,
        }
    }
}

impl std::convert::From<DorLockSwitchState> for bool {
    fn from(dlss: DorLockSwitchState) -> Self {
        match dlss {
            DorLockSwitchState::Unlocked => true,
            DorLockSwitchState::Locked => false,
        }
    }
}

pub trait DorLockSwitchStateTrait {
    fn state(&self) -> DorLockSwitchState;
}

#[derive(Debug)]
pub struct DorLockSwitch {
    #[cfg(all(
        any(target_arch = "arm", target_arch = "aarch64"),
        target_env = "musl",
        target_os = "linux"
    ))]
    dorlockswitch_gpio: InputPin,
}

impl DorLockSwitch {
    #[cfg(all(
        any(target_arch = "arm", target_arch = "aarch64"),
        target_env = "musl",
        target_os = "linux"
    ))]
    pub fn new() -> Self {
        Self {
            dorlockswitch_gpio: Gpio::new().unwrap().get(22).unwrap().into_input(),
        }
    }

    #[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
    pub fn new() -> Self {
        Self::check_state_file();

        Self {}
    }

    fn check_state_file() {
        if fs::metadata(super::STATE_FILE).is_err() {
            let mut file = fs::File::create(super::STATE_FILE).unwrap();
            write!(file, "false").unwrap();
        }
    }
}

impl DorLockSwitchStateTrait for DorLockSwitch {
    #[cfg(all(
        any(target_arch = "arm", target_arch = "aarch64"),
        target_env = "musl",
        target_os = "linux"
    ))]
    fn state(&self) -> DorLockSwitchState {
        match self.dorlockswitch_gpio.read() {
            Level::High => DorLockSwitchState::Locked,
            Level::Low => DorLockSwitchState::Unlocked,
        }
    }

    #[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
    fn state(&self) -> DorLockSwitchState {
        let mut file = fs::File::open(super::STATE_FILE).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let s: bool = contents.trim().parse().unwrap();

        s.into()
    }
}

impl Default for DorLockSwitch {
    fn default() -> Self {
        Self::new()
    }
}
