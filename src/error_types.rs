use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MutexError {
    TryLockError,
    NestedLockError,
    UnauthorizedUnlockError,
    UnknownError
}

impl Display for MutexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TryLockError => Display::fmt("Tried to lock an already locked mutex in a non-blocking manner.", f),
            Self::NestedLockError => Display::fmt("Tried to lock an already locked mutex from the same thread.", f),
            Self::UnauthorizedUnlockError => Display::fmt("Tried to unlock a mutex which isn't owned by this thread.", f),
            Self::UnknownError => Display::fmt("Unknown mutex error occurred.", f),
        }
    }
}

impl Error for MutexError {}

pub enum SerializeError {
    NameTooLongError,
}