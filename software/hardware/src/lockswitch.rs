#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    target_env = "musl",
    target_os = "linux"
))]
use rppal::gpio::{Gpio, InputPin, Level};

#[derive(Debug)]
pub enum LockSwitchState {
    Locked,
    Unlocked,
}

pub trait LockSwitchTrait {
    fn state(&self) -> LockSwitchState;
}

pub struct LockSwitch {
    lockswitch_pin: i32,
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
    fn new(&self) -> Self {
        Self {
            lockswitch_pin: 32,
            lockswitch_gpio: Gpio::new().unwrap().get(42).unwrap().into_input(),
        }
    }

    #[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
    fn new(&self) -> Self {
        Self { lockswitch_pin: 32 }
    }
}

impl LockSwitchTrait for LockSwitch {
    #[cfg(all(
        any(target_arch = "arm", target_arch = "aarch64"),
        target_env = "musl",
        target_os = "linux"
    ))]
    fn state(&self) -> LockSwitchState {
        match self.lockswitch_gpio.read() {
            Level::High => LockSwitchState::Locked,
            Level::Low => LockSwitchState::Unlocked,
        }
    }

    #[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
    fn state(&self) -> LockSwitchState {
        LockSwitchState::Locked
    }
}
