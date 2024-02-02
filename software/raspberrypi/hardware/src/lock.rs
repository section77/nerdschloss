use serde::{Deserialize, Serialize};

use crate::motor::{run, Direction};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Configuration {
    pub pin: u8,
    pub direction: u8,
    pub driver: u8,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum State {
    #[default]
    Locked,
    Locking,
    Unlocked,
    Unlocking,
}

impl std::convert::From<bool> for State {
    fn from(b: bool) -> Self {
        if b {
            Self::Unlocked
        } else {
            Self::Locked
        }
    }
}

impl std::convert::From<State> for bool {
    fn from(dls: State) -> Self {
        match dls {
            State::Unlocked => true,
            State::Unlocking | State::Locked | State::Locking => false,
        }
    }
}

pub trait Lockable {
    fn lock(&self);
}

pub trait Unlockable {
    fn unlock(&self);
}

pub trait StateTrait {
    fn state(&self) -> State;
}

#[derive(Debug, Clone, Copy)]
pub struct Lock {
    state: State,
    configuration: Configuration,
}

impl Lock {
    #[must_use]
    pub fn new(configuration: Configuration) -> Self {
        Self {
            state: State::Locked,
            configuration,
        }
    }

    pub fn lock(&mut self) {
        self.state = State::Locking;
        run(self.configuration, Direction::Close);
        self.state = State::Locked;
    }

    pub fn unlock(&mut self) {
        self.state = State::Unlocking;
        run(self.configuration, Direction::Open);
        self.state = State::Unlocked;
    }

    pub fn state(&self) -> State {
        self.state
    }
}
