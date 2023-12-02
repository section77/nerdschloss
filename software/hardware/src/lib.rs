mod lock;
mod lockswitch;
mod motor;

pub use lock::{Lock, LockState};
pub use lockswitch::{LockSwitch, LockSwitchState, LockSwitchStateTrait};
pub use motor::Direction;

#[cfg(all(target_arch = "x86_64", any(target_os = "macos", target_os = "linux")))]
const STATE_FILE: &str = "/tmp/nerdschloss";
//const STATE_FILE: &str = "/var/run/nerdschloss";
