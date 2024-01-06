use serde::{Deserialize, Serialize};

use crate::motor::{run_motor, Direction};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LockMotorConfiguration {
    pub pin: u8,
    pub direction: u8,
    pub driver: u8,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum LockState {
    #[default]
    Locked,
    Locking,
    Unlocked,
    Unlocking,
}

impl std::convert::From<bool> for LockState {
    fn from(b: bool) -> Self {
        match b {
            true => Self::Unlocked,
            false => Self::Locked,
        }
    }
}

impl std::convert::From<LockState> for bool {
    fn from(dls: LockState) -> Self {
        match dls {
            LockState::Unlocked => true,
            LockState::Unlocking => false,
            LockState::Locked => false,
            LockState::Locking => false,
        }
    }
}

pub trait Lockable {
    fn lock(&self);
}

pub trait Unlockable {
    fn unlock(&self);
}

pub trait LockStateTrait {
    fn state(&self) -> LockState;
}

#[derive(Debug, Clone, Copy)]
pub struct Lock {
    state: LockState,
    configuration: LockMotorConfiguration,
}

impl Lock {
    pub fn new(configuration: LockMotorConfiguration) -> Self {
        Self {
            state: LockState::Locked,
            configuration,
        }
    }

    pub fn lock(&mut self) {
        self.state = LockState::Locking;
        run_motor(self.configuration, Direction::Close);
        self.state = LockState::Locked;
    }

    pub fn unlock(&mut self) {
        self.state = LockState::Unlocking;
        run_motor(self.configuration, Direction::Open);
        self.state = LockState::Unlocked;
    }

    pub fn state(&self) -> LockState {
        self.state
    }
}
