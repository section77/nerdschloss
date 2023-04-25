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

                    let client = reqwest::blocking::Client::new();
                    let res = client.put("http://api.section77.de/sensors/people_now_present/")
                        .body("value=1")
                        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                        .send()
                    ;

                }
            }
            Direction::Close => {
                if is_open {
                    println!("Closing ...");
                    dorlock.lock();
                    is_open = false;

                    let client = reqwest::blocking::Client::new();
                    let res = client.put("http://api.section77.de/sensors/people_now_present/")
                        .body("value=0")
                        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                        .send()
                    ;
                }
            }
        }
    }
}
