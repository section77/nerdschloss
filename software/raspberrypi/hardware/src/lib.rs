pub mod doorswitch;
pub mod lock;
pub mod lockswitch;
mod motor;

pub use self::motor::Direction;

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "macos", target_os = "linux")
))]
const LOCKSWITCH_STATE_FILE: &str = "/tmp/nerdschloss/lockswitch_state";
#[cfg(all(
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "macos", target_os = "linux")
))]
const DOORSWITCH_STATE_FILE: &str = "/tmp/nerdschloss/doorswitch_state";
