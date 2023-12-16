#[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
use std::{fs, io::prelude::*};

#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    target_env = "musl",
    target_os = "linux"
))]
use rppal::gpio::{Gpio, InputPin, Level};

#[derive(Debug, Default, Clone, Copy)]
pub enum LockSwitchState {
    #[default]
    Locked,
    Unlocked,
}

impl std::convert::From<bool> for LockSwitchState {
    fn from(b: bool) -> Self {
        match b {
            true => LockSwitchState::Unlocked,
            false => LockSwitchState::Locked,
        }
    }
}

impl std::convert::From<LockSwitchState> for bool {
    fn from(dlss: LockSwitchState) -> Self {
        match dlss {
            LockSwitchState::Unlocked => true,
            LockSwitchState::Locked => false,
        }
    }
}

pub trait LockSwitchStateTrait {
    fn state(&self) -> LockSwitchState;
}

#[derive(Debug)]
pub struct LockSwitch {
    #[cfg(all(
        any(target_arch = "arm", target_arch = "aarch64"),
        target_env = "musl",
        target_os = "linux"
    ))]
    lockswitch_gpio: InputPin,
}

impl LockSwitch {
    #[cfg(all(
        any(target_arch = "arm", target_arch = "aarch64"),
        target_env = "musl",
        target_os = "linux"
    ))]
    pub fn new() -> Self {
        Self {
            lockswitch_gpio: Gpio::new().unwrap().get(22).unwrap().into_input_pullup(),
        }
    }

    #[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
    pub fn new() -> Self {
        Self::check_state_file();

        Self {}
    }

    #[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
    fn check_state_file() {
        if fs::metadata(super::STATE_FILE).is_err() {
            let mut file = fs::File::create(super::STATE_FILE).unwrap();
            write!(file, "false").unwrap();
        }
    }
}

impl LockSwitchStateTrait for LockSwitch {
    #[cfg(all(
        any(target_arch = "arm", target_arch = "aarch64"),
        target_env = "musl",
        target_os = "linux"
    ))]
    fn state(&self) -> LockSwitchState {
        match &self.lockswitch_gpio.read() {
            Level::Low => LockSwitchState::Locked,
            Level::High => LockSwitchState::Unlocked,
        }
    }

    #[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
    fn state(&self) -> LockSwitchState {
        let mut file = fs::File::open(super::STATE_FILE).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let s: bool = contents.trim().parse().unwrap();

        s.into()
    }
}

impl Default for LockSwitch {
    fn default() -> Self {
        Self::new()
    }
}
