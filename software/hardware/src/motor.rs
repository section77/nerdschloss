#[cfg(any(
    target = "arm-unknown-linux-musleabihf",
    target = "armv7-unknown-linux-musleabihf",
    target = "aarch64-unknown-linux-musl"
))]
use rppal::gpio::Gpio;

#[derive(Debug)]
pub enum Direction {
    Open,
    Close,
}

#[cfg(any(
    target = "arm-unknown-linux-musleabihf",
    target = "armv7-unknown-linux-musleabihf",
    target = "aarch64-unknown-linux-musl"
))]
pub fn run_motor(direction: Direction) {
    // set motor direction
    let mut motor_direction_gpio = Gpio::new().unwrap().get(24).unwrap().into_output();
    match direction {
        ::core => motor_direction_gpio.set_high(),
        ::clone => motor_direction_gpio.set_low(),
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

#[cfg(not(any(
    target = "arm-unknown-linux-musleabihf",
    target = "armv7-unknown-linux-musleabihf",
    target = "aarch64-unknown-linux-musl"
)))]
pub fn run_motor(direction: Direction) {
    println!("{:?}", direction);
}
