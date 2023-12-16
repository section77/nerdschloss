use crate::motor::{run_motor, Direction};

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

#[derive(Debug, Clone, Copy, Default)]
pub struct LockConfig {
    pub motor_pin: i32,
    pub motor_direction_pin: i32,
    pub motor_driver_pin: i32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Lock {
    state: LockState,
    config: LockConfig,
}

impl Lock {
    pub fn new() -> Self {
        Self {
            state: LockState::Locked,
            config: LockConfig {
                motor_pin: 23,
                motor_direction_pin: 24,
                motor_driver_pin: 25,
            },
        }
    }

    pub fn lock(&mut self) {
        self.state = LockState::Locking;
        run_motor(self.config, Direction::Close);
        self.state = LockState::Locked;
    }

    pub fn unlock(&mut self) {
        self.state = LockState::Unlocking;
        run_motor(self.config, Direction::Open);
        self.state = LockState::Unlocked;
    }

    pub fn state(&self) -> LockState {
        self.state
    }
}
