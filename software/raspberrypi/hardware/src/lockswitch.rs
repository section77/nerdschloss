#[cfg(all(
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "macos", target_os = "linux")
))]
use std::{fs, io::prelude::*, path};

use serde::{Deserialize, Serialize};

#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    target_env = "musl",
    target_os = "linux"
))]
use tracing::{debug, error};

#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    target_env = "musl",
    target_os = "linux"
))]
use rppal::gpio::{Gpio, InputPin, Level, Trigger};

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
    lockswitch_gpio: InputPin,
}

impl LockSwitch {
    #[must_use]
    #[cfg(all(
        any(target_arch = "arm", target_arch = "aarch64"),
        target_env = "musl",
        target_os = "linux"
    ))]
    pub fn new(
        configuration: &'static Configuration,
        sender: tokio::sync::mpsc::Sender<bool>,
    ) -> Self {
        let mut gpio = Gpio::new()
            .unwrap()
            .get(configuration.pin)
            .unwrap()
            .into_input_pullup();

        let delay = std::time::Duration::from_millis(configuration.interruptdelay);

        gpio.set_async_interrupt(rppal::gpio::Trigger::Both, Some(delay), move |event| {
            debug!("Interrupt LockSwitchState: {event:?}");
            let state = match event.trigger {
                Trigger::RisingEdge => true,
                Trigger::FallingEdge => false,
                s => {
                    error!("Trigger value is not supported {s}");
                    false
                }
            };
            sender.blocking_send(state).unwrap();
        })
        .unwrap();

        Self {
            lockswitch_gpio: gpio,
        }
    }

    #[must_use]
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "aarch64"),
        any(target_os = "macos", target_os = "linux")
    ))]
    pub fn new(
        _configuration: &'static Configuration,
        sender: tokio::sync::mpsc::Sender<bool>,
    ) -> Self {
        Self::check_state_file();

        std::thread::spawn(move || {
            let (tx, rx) = std::sync::mpsc::channel();

            let mut debouncer = notify_debouncer_full::new_debouncer(
                std::time::Duration::from_millis(10),
                None,
                tx,
            )
            .unwrap();

            debouncer
                .watch(
                    super::LOCKSWITCH_STATE_FILE,
                    notify::RecursiveMode::Recursive,
                )
                .unwrap();

            // print all events and errors
            for result in rx {
                match result {
                    Ok(events) => events
                        .iter()
                        .filter(|event| {
                            event.kind
                                == notify::EventKind::Modify(notify::event::ModifyKind::Data(
                                    notify::event::DataChange::Any,
                                ))
                        })
                        .for_each(|event| {
                            tracing::info!("{event:?}");
                            let state = fs::read_to_string(super::LOCKSWITCH_STATE_FILE)
                                .unwrap()
                                .trim()
                                .parse::<bool>()
                                .unwrap();
                            sender.blocking_send(state).unwrap();
                        }),
                    Err(errors) => errors.iter().for_each(|error| tracing::error!("{error:?}")),
                }
            }
        });

        Self {}
    }

    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "aarch64"),
        any(target_os = "macos", target_os = "linux")
    ))]
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
        let value = &self.lockswitch_gpio.read();
        match value {
            Level::Low => State::Locked,
            Level::High => State::Unlocked,
        }
    }

    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "aarch64"),
        any(target_os = "macos", target_os = "linux")
    ))]
    fn state(&self) -> State {
        fs::read_to_string(super::LOCKSWITCH_STATE_FILE)
            .unwrap()
            .trim()
            .parse::<bool>()
            .unwrap()
            .into()
    }
}
