pub enum MutexError {
    TryLockError,
    NestedLockError,
    UnauthorizedUnlockError,
    UnknownError
}