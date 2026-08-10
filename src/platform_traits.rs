use crate::error_types::MutexError;

pub trait PlatformMutex {
    fn lock(&self) -> Result<(), MutexError>;
    fn unlock(&self) -> Result<(), MutexError>;
    fn try_lock(&self) -> Result<(), MutexError>;
}