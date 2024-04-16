use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{info, instrument, warn};

use hardware::{doorswitch, lock, lockswitch, lockswitch::StateTrait, Direction};

use crate::configuration::Configuration;

#[instrument]
pub fn logic(
    configuration: Configuration,
    mut receiver: Receiver<Direction>,
    spaceapi_sender: Sender<bool>,
) {
    let lockswitch = lockswitch::LockSwitch::new(configuration.lockswitch, spaceapi_sender);
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
    }
}
