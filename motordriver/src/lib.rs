#[cfg(feature = "hardware")]
use rppal::gpio::Gpio;

#[cfg(not(feature = "hardware"))]
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use std::sync::mpsc;

pub enum Direction {
    Open,
    Close,
}

pub fn run_stepper(channel: mpsc::Receiver<Direction>) {
    #[cfg(feature = "hardware")]
    let mut direction = Gpio::new().unwrap().get(24).unwrap().into_output();
    let mut is_open = false;
    loop {
        match channel.recv() {
            Err(_) => return,
            Ok(Direction::Open) => {
                if !is_open {
                    #[cfg(feature = "hardware")]
                    {
                        direction.set_high();
                        do_steps();
                    }
                    #[cfg(not(feature = "hardware"))]
                    {
                        print!("Simulated motor opens the door ...");
                        io::stdout().flush().unwrap();
                        thread::sleep(Duration::from_secs(5));
                        println!(" open!");
                    }
                    is_open = true;
                }
            }
            Ok(Direction::Close) => {
                if is_open {
                    #[cfg(feature = "hardware")]
                    {
                        direction.set_low();
                        do_steps();
                    }
                    #[cfg(not(feature = "hardware"))]
                    {
                        print!("Simulated motor closes the door ...");
                        io::stdout().flush().unwrap();
                        thread::sleep(Duration::from_secs(5));
                        println!(" closed!");
                    }
                    is_open = false;
                }
            }
        }
    }
}

#[cfg(feature = "hardware")]
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
