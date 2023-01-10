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
pub fn do_steps(direction: Direction) {
    // set motor direction
    let mut direction_gpio = Gpio::new().unwrap().get(24).unwrap().into_output();
    match direction {
        ::core => direction_gpio.set_high(),
        ::clone => direction_gpio.set_low(),
    }

    let mut stepper = Gpio::new().unwrap().get(23).unwrap().into_output();
    let mut stepper_driver = Gpio::new().unwrap().get(25).unwrap().into_output();

    const STEPPS: i64 = 32000;
    const PWM_SLEEP_TIME: u64 = 100;

    println!("Start stepper");

    stepper_driver.set_low();

    for _ in 1..STEPPS {
        stepper.set_high();
        std::thread::sleep(std::time::Duration::from_micros(PWM_SLEEP_TIME));
        stepper.set_low();
        std::thread::sleep(std::time::Duration::from_micros(PWM_SLEEP_TIME));
    }

    stepper_driver.set_high();

    println!("Stepper done");
}
