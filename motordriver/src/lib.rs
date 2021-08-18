use rppal::gpio::Gpio;
use std::sync::mpsc;

pub enum Direction {
    Open,
    Close,
}

pub fn run_stepper(channel: mpsc::Receiver<Direction>) {
    let mut direction = Gpio::new().unwrap().get(24).unwrap().into_output();
    let mut is_open = false;
    loop {
        match channel.recv() {
            Err(_) => return,
            Ok(Direction::Open) => {
                if !is_open {
                    direction.set_high();
                    do_steps();
                    is_open = true;
                }
            }
            Ok(Direction::Close) => {
                if is_open {
                    direction.set_low();
                    do_steps();
                    is_open = false;
                }
            }
        }
    }
}

fn do_steps() {
    let mut stepper = Gpio::new().unwrap().get(23).unwrap().into_output();
    println!("Start stepper");
    for _ in 1..279 {
        stepper.set_high();
        std::thread::sleep(std::time::Duration::from_millis(5));
        stepper.set_low();
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    println!("Stepper done");
}
