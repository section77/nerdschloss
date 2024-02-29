#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    target_env = "musl",
    target_os = "linux"
))]
use std::time::Duration;
#[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
use std::{fs, io::prelude::*, path};

use serde::{Deserialize, Serialize};

#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    target_env = "musl",
    target_os = "linux"
))]
use debounce::EventDebouncer;
#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    target_env = "musl",
    target_os = "linux"
))]
use rppal::gpio::{Gpio, InputPin, Level};
#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    target_env = "musl",
    target_os = "linux"
))]
use tracing::info;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Configuration {
    pub pin: u8,
    pub interruptdelay: u64,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum State {
    #[default]
    Locked,
    Unlocked,
}

impl std::convert::From<bool> for State {
    fn from(b: bool) -> Self {
        if b {
            State::Unlocked
        } else {
            State::Locked
        }
    }
}

impl std::convert::From<State> for bool {
    fn from(dlss: State) -> Self {
        match dlss {
            State::Unlocked => true,
            State::Locked => false,
        }
    }
}

pub trait StateTrait {
    fn state(&self) -> State;
}

pub struct LockSwitch {
    #[cfg(all(
        any(target_arch = "arm", target_arch = "aarch64"),
        target_env = "musl",
        target_os = "linux"
    ))]
    lockswitch_gpio: InputPin,
}

impl LockSwitch {
    #[must_use]
    #[cfg(all(
        any(target_arch = "arm", target_arch = "aarch64"),
        target_env = "musl",
        target_os = "linux"
    ))]
    pub fn new(configuration: Configuration) -> Self {
        let delay = Duration::from_millis(configuration.interruptdelay);
        let debouncer = EventDebouncer::new(delay, |v: ()| {
            info!("Debounced Interrupt LockSwitchState: {v:?}");
        });

        let mut gpio = Gpio::new()
            .unwrap()
            .get(configuration.pin)
            .unwrap()
            .into_input_pullup();
        gpio.set_async_interrupt(rppal::gpio::Trigger::Both, move |_| {
            // info!("Interrupt LockSwitchState: {level:?}");
            debouncer.put(());
        })
        .unwrap();

        Self {
            lockswitch_gpio: gpio,
        }
    }

    #[must_use]
    #[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
    pub fn new(_configuration: Configuration) -> Self {
        Self::check_state_file();
        Self {}
    }

    #[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
    fn check_state_file() {
        let state_file = path::Path::new(super::LOCKSWITCH_STATE_FILE);
        if !state_file.parent().unwrap().exists() {
            fs::create_dir_all(state_file.parent().unwrap()).unwrap();
        }

        if fs::metadata(state_file).is_err() {
            let mut file = fs::File::create(super::LOCKSWITCH_STATE_FILE).unwrap();
            writeln!(file, "false").unwrap();
        }
    }
}

impl StateTrait for LockSwitch {
    #[cfg(all(
        any(target_arch = "arm", target_arch = "aarch64"),
        target_env = "musl",
        target_os = "linux"
    ))]
    fn state(&self) -> State {
        match &self.lockswitch_gpio.read() {
            Level::Low => State::Locked,
            Level::High => State::Unlocked,
        }
    }

    #[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
    fn state(&self) -> State {
        let mut file = fs::File::open(super::LOCKSWITCH_STATE_FILE).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents.trim().parse::<bool>().unwrap().into()
    }
}
