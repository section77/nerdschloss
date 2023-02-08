#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    target_env = "musl",
    target_os = "linux"
))]
use rppal::gpio::Gpio;

#[derive(Debug)]
pub enum Direction {
    Open,
    Close,
}

#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    target_env = "musl",
    target_os = "linux"
))]
pub fn run_motor(direction: Direction) {
    println!("Hardware {direction:?}");

    // set motor direction
    let mut motor_direction_gpio = Gpio::new().unwrap().get(24).unwrap().into_output();
    match direction {
        Direction::Open => motor_direction_gpio.set_high(),
        Direction::Close => motor_direction_gpio.set_low(),
    }

    let mut motor_gpio = Gpio::new().unwrap().get(23).unwrap().into_output();
    let mut motor_driver_gpio = Gpio::new().unwrap().get(25).unwrap().into_output();

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

#[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
pub fn run_motor(direction: Direction) {
    println!("Debug {direction:?}");
}
