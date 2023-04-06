#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    target_env = "musl",
    target_os = "linux"
))]
use rppal::gpio::{Gpio, InputPin, Level};

use crate::motor::{run_motor, Direction};

#[derive(Debug, Default, Clone, Copy)]
pub enum DorLockState {
    #[default]
    Locked,
    Locking,
    Unlocked,
    Unlocking,
}

pub trait Lockable {
    fn lock(&self);
}

pub trait Unlockable {
    fn unlock(&self);
}

pub trait LockUnlockState {
    fn state(&self) -> DorLockState;
}

#[derive(Debug, Clone, Copy)]
pub struct DorLockConfig {
    pub motor_pin: i32,
    pub motor_direction_pin: i32,
    pub motor_driver_pin: i32,
}

#[derive(Debug)]
pub struct DorLock {
    state: DorLockState,
    config: DorLockConfig,
}

impl DorLock {
    pub fn new() -> Self {
        Self {
            state: DorLockState::Locked,
            config: DorLockConfig {
                motor_pin: 23,
                motor_direction_pin: 24,
                motor_driver_pin: 25,
            },
        }
    }

    pub fn lock(&mut self) {
        run_motor(self.config, Direction::Close);
        self.state = DorLockState::Locked;
    }

    pub fn unlock(&mut self) {
        run_motor(self.config, Direction::Open);
        self.state = DorLockState::Unlocked;
    }

    pub fn state(&self) -> DorLockState {
        self.state
    }
}

impl Default for DorLock {
    fn default() -> Self {
        Self::new()
    }
}
