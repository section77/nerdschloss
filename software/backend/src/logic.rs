use hardware::do_steps;
use hardware::Direction;

use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use tokio::sync::mpsc::Receiver;

pub fn run_stepper(mut receiver: Receiver<Direction>) {
    let mut is_open = false;
    loop {
        let msg = match receiver.blocking_recv() {
            Some(m) => m,
            None => continue,
        };
        match msg {
            Direction::Open => {
                if !is_open {
                    println!("Opening ...");
                    do_steps(msg);
                    {
                        print!("Simulated motor opens the door ...");
                        io::stdout().flush().unwrap();
                        thread::sleep(Duration::from_secs(5));
                        println!(" open!");
                    }
                    is_open = true;
                }
            }
            Direction::Close => {
                if is_open {
                    println!("Closing ...");
                    do_steps(msg);
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
