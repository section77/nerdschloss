use tokio::sync::mpsc::Receiver;

use hardware::{Direction, Lock, LockSwitch, LockSwitchStateTrait};

const SPACEAPI: bool = false;

async fn spaceapi(state: bool) {
    let Ok(_) = reqwest::Client::new()
        .put("http://api.section77.de/sensors/people_now_present/")
        .body(format!("value={}", u8::from(state)))
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .send()
        .await else {
            eprintln!("Failed to set SpaceAPI");
            return
        };
}

pub fn logic(mut receiver: Receiver<Direction>) {
    let lockswitch = LockSwitch::default();
    let mut lock = Lock::default();
    let mut is_open: bool;

    loop {
        let Some(msg) = receiver.blocking_recv() else {
            continue;
        };
        is_open = bool::from(lockswitch.state());
        match msg {
            Direction::Open => {
                if !is_open {
                    println!("Opening ...");
                    lock.unlock();
                }
            }
            Direction::Close => {
                if is_open {
                    println!("Closing ...");
                    lock.lock();
                }
            }
        }

        let state = SPACEAPI && lockswitch.state().into() && lock.state().into();
        tokio::task::spawn(async move {
            spaceapi(state).await;
        });
    }
}
