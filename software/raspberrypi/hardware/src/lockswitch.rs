#[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
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
    pub fn new(configuration: Configuration, sender: tokio::sync::mpsc::Sender<bool>) -> Self {
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
    #[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
    pub fn new(_configuration: Configuration, _sender: tokio::sync::mpsc::Sender<bool>) -> Self {
        Self::check_state_file();

        use notify::{Error, RecommendedWatcher, RecursiveMode, Watcher};
        use notify_debouncer_full::{
            new_debouncer, DebounceEventResult, DebouncedEvent, Debouncer, FileIdMap,
        };
        use std::{path::Path, time::Duration};
        use tokio::{runtime::Handle, sync::mpsc::Receiver};

        pub struct NotifyHandler {
            pub notify_watcher: Option<Debouncer<RecommendedWatcher, FileIdMap>>,
            pub receiver: Option<Receiver<Result<Vec<DebouncedEvent>, Vec<Error>>>>,
        }

        impl NotifyHandler {
            pub async fn initialize_notify_scheduler(&mut self) {
                let (tx, rx) = tokio::sync::mpsc::channel(1);
                let rt = Handle::current();

                let debouncer = new_debouncer(
                    Duration::from_secs(3),
                    None,
                    move |result: DebounceEventResult| {
                        let tx = tx.clone();

                        println!("calling by notify -> {:?}", &result);
                        rt.spawn(async move {
                            if let Err(e) = tx.send(result).await {
                                println!("Error sending event result: {:?}", e);
                            }
                        });
                    },
                );

                match debouncer {
                    Ok(watcher) => {
                        println!("Initialize notify watcher success");
                        self.notify_watcher = Some(watcher);

                        self.receiver = Some(rx);
                    }
                    Err(error) => {
                        println!("{:?}", error);
                    }
                }
            }

            pub async fn watch(&mut self, path: &str) -> notify::Result<()> {
                let watch_path = Path::new(path);

                if watch_path.exists() {
                    let is_file = watch_path.is_file();
                    println!("Valid path {} is file {}", path, is_file);
                } else {
                    println!("watch path {:?} not exists", watch_path);
                }

                if let Some(watcher) = self.notify_watcher.as_mut() {
                    watcher
                        .watcher()
                        .watch(watch_path, RecursiveMode::Recursive)?;

                    watcher
                        .cache()
                        .add_root(watch_path, RecursiveMode::Recursive);

                    if let Some(mut rx) = self.receiver.take() {
                        tokio::spawn(async move {
                            while let Some(res) = rx.recv().await {
                                match res {
                                    Ok(events) => {
                                        println!("events: {:?}", events);
                                    }
                                    Err(errors) => {
                                        println!("errors: {:?}", errors)
                                    }
                                }
                            }
                        });
                    }
                }

                Ok(())
            }
        }

        tokio::spawn(async move {
            let mut notifier: NotifyHandler = NotifyHandler {
                notify_watcher: None,
                receiver: None,
            };

            notifier.initialize_notify_scheduler().await;
            notifier.watch(super::LOCKSWITCH_STATE_FILE).await.unwrap();

            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                // let time: DateTime<Local> = Local::now();
                // println!("{}: Hello, world!", time.format("%Y-%m-%d %H:%M:%S"));
            }
        });
        println!("aha");

        Self {}
    }

    #[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
    fn check_state_file() {
        let state_file = path::Path::new(super::LOCKSWITCH_STATE_FILE);
        if !state_file.parent().unwrap().exists() {
            fs::create_dir_all(state_file.parent().unwrap()).unwrap();
        }

        let mut file;
        if fs::metadata(state_file).is_err() {
            file = fs::File::create(super::LOCKSWITCH_STATE_FILE).unwrap();
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

    #[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
    fn state(&self) -> State {
        let mut file = fs::File::open(super::LOCKSWITCH_STATE_FILE).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents.trim().parse::<bool>().unwrap().into()
    }
}
