mod doorswitch;
mod lock;
mod lockswitch;
mod motor;

pub use self::{
    doorswitch::{DoorSwitch, DoorSwitchState, DoorSwitchStateTrait},
    lock::{Lock, LockState},
    lockswitch::{LockSwitch, LockSwitchState, LockSwitchStateTrait},
    motor::Direction,
};

#[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
const STATE_FILE: &str = "/tmp/nerdschloss";
