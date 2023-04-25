use tokio::sync::mpsc::Receiver;

use hardware::{Direction, DorLock, DorLockSwitch};

pub fn run_stepper(mut receiver: Receiver<Direction>) {
    let mut is_open = false;
    let _dorlockswitch = DorLockSwitch::default();
    let mut dorlock = DorLock::default();

    loop {
        let Some(msg) = receiver.blocking_recv() else {
            continue;
        };
        match msg {
            Direction::Open => {
                if !is_open {
                    println!("Opening ...");
                    dorlock.unlock();
                    is_open = true;
                }
            }
            Direction::Close => {
                if is_open {
                    println!("Closing ...");
                    dorlock.lock();
                    is_open = false;
                }
            }
        }
    }
}
