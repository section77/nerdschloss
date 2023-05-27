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

impl std::convert::From<bool> for DorLockState {
    fn from(b: bool) -> Self {
        match b {
            true => Self::Unlocked,
            false => Self::Locked,
        }
    }
}

impl std::convert::From<DorLockState> for bool {
    fn from(dls: DorLockState) -> Self {
        match dls {
            DorLockState::Unlocked => true,
            DorLockState::Unlocking => false,
            DorLockState::Locked => false,
            DorLockState::Locking => false,
        }
    }
}

pub trait Lockable {
    fn lock(&self);
}

pub trait Unlockable {
    fn unlock(&self);
}

pub trait DorLockStateTrait {
    fn state(&self) -> DorLockState;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DorLockConfig {
    pub motor_pin: i32,
    pub motor_direction_pin: i32,
    pub motor_driver_pin: i32,
}

#[derive(Debug, Clone, Copy, Default)]
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
        self.state = DorLockState::Locking;
        run_motor(self.config, Direction::Close);
        self.state = DorLockState::Locked;
    }

    pub fn unlock(&mut self) {
        self.state = DorLockState::Unlocking;
        run_motor(self.config, Direction::Open);
        self.state = DorLockState::Unlocked;
    }

    pub fn state(&self) -> DorLockState {
        self.state
    }
}
