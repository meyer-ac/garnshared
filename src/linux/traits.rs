use hashed_type_def::HashedTypeDef;

/// Types for which it is safe to share references between processes.
/// # Safety
/// - Any instance must be able to be re-interpreted from its underlying bit sequence in another
///   process and still be a valid instance
/// - Any public methods and associated functions must be able to be invoked in parallel from any
///   number of concurrent processes
/// - Any public members must be able to be read and modified in parallel from any number of
///   concurrent processes
/// - The type and all its members recursively must be `repr(C)`, `repr(transparent)` or `repr(u*)`
pub unsafe trait ShmCompatible: Sized + Sync + HashedTypeDef + 'static {}