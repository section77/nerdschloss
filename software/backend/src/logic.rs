use tokio::sync::mpsc::Receiver;

use hardware::{Direction, DorLock, DorLockSwitch};

async fn spaceapi(state: bool) {
    let Ok(_) = reqwest::Client::new()
        .put("http://api.section77.de/sensors/people_now_present/")
        .body(format!("value={}", i8::from(state)))
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

                    tokio::task::spawn(async {
                        spaceapi(true).await;
                    });
                }
            }
            Direction::Close => {
                if is_open {
                    println!("Closing ...");
                    dorlock.lock();
                    is_open = false;

                    tokio::task::spawn(async {
                        spaceapi(false).await;
                    });
                }
            }
        }
    }
}
