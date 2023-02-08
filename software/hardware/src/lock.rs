#[derive(Debug)]
pub enum LockState {
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
    fn state(&self) -> LockState;
}

pub struct DorLock {
    motor_pin: i32,
    motor_direction_pin: i32,
    motor_driver_pin: i32,
}

impl Lockable for DorLock {
    fn lock(&self) {}
}

impl Unlockable for DorLock {
    fn unlock(&self) {}
}

impl LockUnlockState for DorLock {
    fn state(&self) -> LockState {
        LockState::Locked
    }
}
