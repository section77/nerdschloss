mod doorswitch;
mod lock;
mod lockswitch;
mod motor;

pub use self::{
    doorswitch::{DoorSwitch, DoorSwitchConfiguration, DoorSwitchState, DoorSwitchStateTrait},
    lock::{Lock, LockMotorConfiguration, LockState},
    lockswitch::{LockSwitch, LockSwitchConfiguration, LockSwitchState, LockSwitchStateTrait},
    motor::Direction,
};

#[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
const LOCKSWITCH_STATE_FILE: &str = "/tmp/nerdschloss/lockswitch";
#[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
const DOORSWITCH_STATE_FILE: &str = "/tmp/nerdschloss/doorswitch";
