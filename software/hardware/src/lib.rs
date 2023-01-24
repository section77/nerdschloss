mod lock;
mod lockswitch;
mod motor;

pub use lock::DorLock;
pub use lockswitch::LockSwitch;
pub use motor::{run_motor, Direction};
