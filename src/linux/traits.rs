use hashed_type_def::HashedTypeDef;

/// Types for which it is safe to share references between processes.
pub unsafe trait ShmSync: Sync + HashedTypeDef + 'static {}