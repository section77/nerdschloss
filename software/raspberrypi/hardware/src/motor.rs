#[cfg(all(
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "macos", target_os = "linux")
))]
use std::{fs, io::prelude::*};

#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    target_env = "musl",
    target_os = "linux"
))]
use rppal::gpio::Gpio;

use crate::lock::Configuration;

#[derive(Debug, Default, Clone, Copy)]
pub enum Direction {
    Open,
    #[default]
    Close,
}

#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    target_env = "musl",
    target_os = "linux"
))]
pub fn run(configuration: &'static Configuration, direction: Direction) {
    println!("Hardware {direction:?}");

    // set motor direction
    let mut motor_direction_gpio = Gpio::new()
        .unwrap()
        .get(configuration.direction)
        .unwrap()
        .into_output();
    match direction {
        Direction::Open => motor_direction_gpio.set_high(),
        Direction::Close => motor_direction_gpio.set_low(),
    }

    let mut motor_gpio = Gpio::new()
        .unwrap()
        .get(configuration.pin)
        .unwrap()
        .into_output();
    let mut motor_driver_gpio = Gpio::new()
        .unwrap()
        .get(configuration.driver)
        .unwrap()
        .into_output();

    const STEPPS: i64 = 32000;
    const PWM_SLEEP_TIME: u64 = 100;

    println!("Start motor");

    motor_driver_gpio.set_low();

    for _ in 1..STEPPS {
        motor_gpio.set_high();
        std::thread::sleep(std::time::Duration::from_micros(PWM_SLEEP_TIME));
        motor_gpio.set_low();
        std::thread::sleep(std::time::Duration::from_micros(PWM_SLEEP_TIME));
    }

    motor_driver_gpio.set_high();

    println!("Stop motor");
}

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "macos", target_os = "linux")
))]
pub fn run(configuration: &'static Configuration, direction: Direction) {
    dbg!(configuration);
    println!("Debug {direction:?}");

    let mut file = fs::File::options()
        .write(true)
        .truncate(true)
        .open(super::LOCKSWITCH_STATE_FILE)
        .unwrap();
    match direction {
        Direction::Open => writeln!(file, "true").unwrap(),
        Direction::Close => writeln!(file, "false").unwrap(),
    }
}
