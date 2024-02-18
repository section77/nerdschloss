use secrecy::ExposeSecret;
use tokio::sync::mpsc::Receiver;
use tracing::{error, info, instrument, warn};

use hardware::{doorswitch, lock, lockswitch, lockswitch::StateTrait, Direction};

use crate::configuration::{Configuration, SpaceAPI};

#[instrument]
async fn spaceapi(configuration: &SpaceAPI, state: bool) {
    let status = if state {
        String::from("open")
    } else {
        String::from("closed")
    };
    info!("Set SpaceAPI status");
    match reqwest::Client::new()
        .put(format!("{}?status={status}", configuration.url))
        .basic_auth(
            &configuration.username,
            Some(&configuration.password.expose_secret()),
        )
        .send()
        .await
    {
        Ok(_) => {
            info!("Successfully set SpaceAPI status");
        }
        Err(e) => {
            error!("Failed to set SpaceAPI status: {e:?}");
        }
    };
}

#[instrument]
pub fn logic(configuration: Configuration, mut receiver: Receiver<Direction>) {
    let lockswitch = lockswitch::LockSwitch::new(configuration.lockswitch);
    let _doorswitch = doorswitch::DoorSwitch::new(configuration.doorswitch);
    let mut lock = lock::Lock::new(configuration.lockmotor);
    let mut is_open;

    loop {
        let Some(msg) = receiver.blocking_recv() else {
            continue;
        };
        is_open = bool::from(lockswitch.state());
        match msg {
            Direction::Open => {
                if !is_open {
                    info!("Opening ...");
                    lock.unlock();
                }
            }
            Direction::Close => {
                if is_open {
                    info!("Closing ...");
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
