use tokio::sync::mpsc::Receiver;

use hardware::{Direction, DoorSwitch, Lock, LockSwitch, LockSwitchStateTrait};

use crate::configuration::{Configuration, SpaceAPI};

async fn spaceapi(spaceapi: &SpaceAPI, state: bool) {
    let status = if state {
        String::from("open")
    } else {
        String::from("closed")
    };
    match reqwest::Client::new()
        .put(format!(
            "https://api.section77.de/set_door_status.php?status={}",
            status
        ))
        .basic_auth(&spaceapi.username, Some(&spaceapi.password))
        .send()
        .await
    {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to set SpaceAPI: {:?}", e);
        }
    };
}

pub fn logic(configuration: Configuration, mut receiver: Receiver<Direction>) {
    let lockswitch = LockSwitch::new(&configuration.lockswitch);
    let _doorswitch = DoorSwitch::new(&configuration.doorswitch);
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

        if configuration.spaceapi.enable {
            let spaceapi_configuration = configuration.spaceapi.clone();
            let state = lockswitch.state().into();

            tokio::task::spawn(async move {
                spaceapi(&spaceapi_configuration, state).await;
            });
        }
    }
}
