#[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
use std::{fs, io::prelude::*, path};

#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    target_env = "musl",
    target_os = "linux"
))]
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    target_env = "musl",
    target_os = "linux"
))]
use rppal::gpio::{Gpio, InputPin, Level};

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

#[derive(Debug)]
pub struct LockSwitch {
    #[cfg(all(
        any(target_arch = "arm", target_arch = "aarch64"),
        target_env = "musl",
        target_os = "linux"
    ))]
    lockswitch_gpio: Arc<RwLock<InputPin>>,
}

impl LockSwitch {
    #[must_use]
    #[cfg(all(
        any(target_arch = "arm", target_arch = "aarch64"),
        target_env = "musl",
        target_os = "linux"
    ))]
    pub fn new(configuration: Configuration, sender: tokio::sync::mpsc::Sender<bool>) -> Self {
        let gpio = Arc::new(RwLock::new(
            Gpio::new()
                .unwrap()
                .get(configuration.pin)
                .unwrap()
                .into_input_pullup(),
        ));

        let g = gpio.clone();
        let delay = std::time::Duration::from_millis(configuration.interruptdelay);
        let debouncer = debounce::EventDebouncer::new(delay, move |_| {
            let state = g.read().unwrap().read();
            tracing::debug!("Debounced Interrupt LockSwitchState: {:?}", state);
            let state = match state {
                Level::High => true,
                Level::Low => false,
            };
            sender.blocking_send(state).unwrap();
        });

        gpio.write()
            .unwrap()
            .set_async_interrupt(rppal::gpio::Trigger::Both, move |_| {
                // tracing::debug!("Interrupt LockSwitchState: {level:?}");
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
        let value = &self.lockswitch_gpio.read().unwrap().read();
        match value {
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
