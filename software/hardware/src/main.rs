#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    target_env = "musl",
    target_os = "linux"
))]
use std::{thread, time};

#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    target_env = "musl",
    target_os = "linux"
))]
use rppal::gpio::Gpio;

#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    target_env = "musl",
    target_os = "linux"
))]
fn main() -> anyhow::Result<()> {
    let gpio = Gpio::new()?.get(22)?.into_input_pullup();

    loop {
        thread::sleep(time::Duration::from_secs(1));
        let state = gpio.read();
        dbg!(state);
    }

    #[allow(unreachable_code)]
    Ok(())
}

#[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
fn main() -> anyhow::Result<()> {
    Ok(())
}
