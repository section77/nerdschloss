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

pub trait DorLockSwitchTrait {
    fn state(&self) -> DorLockSwitchState;
}

#[derive(Debug)]
pub struct DorLockSwitch {
    state: DorLockSwitchState,
    dorlockswitch_pin: i32,
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
            state: DorLockSwitchState::default(),
            dorlockswitch_pin: 32,
            dorlockswitch_gpio: Gpio::new().unwrap().get(42).unwrap().into_input(),
        }
    }

    #[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
    pub fn new() -> Self {
        Self {
            state: DorLockSwitchState::default(),
            dorlockswitch_pin: 32,
        }
    }

    #[cfg(all(
        any(target_arch = "arm", target_arch = "aarch64"),
        target_env = "musl",
        target_os = "linux"
    ))]
    fn state(&self) -> DorLockSwitchState {
        match self.lockswitch_gpio.read() {
            Level::High => DorLockSwitchState::Locked,
            Level::Low => DorLockSwitchState::Unlocked,
        }
    }

    #[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
    fn state(&self) -> DorLockSwitchState {
        self.state
    }
}

impl Default for DorLockSwitch {
    fn default() -> Self {
        Self::new()
    }
}
