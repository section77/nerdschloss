mod lock;
mod lockswitch;
mod motor;

pub use lock::{DorLock, DorLockState};
pub use lockswitch::{DorLockSwitch, DorLockSwitchState, DorLockSwitchStateTrait};
pub use motor::Direction;

const STATE_FILE: &str = "/tmp/nerdschloss";
//const STATE_FILE: &str = "/var/run/nerdschloss";
