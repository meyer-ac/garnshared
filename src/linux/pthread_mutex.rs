use crate::error_types::MutexError;
use crate::linux::miri::{pthread_mutex_destroy, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_t, pthread_mutex_trylock, pthread_mutex_unlock, pthread_mutexattr_destroy, pthread_mutexattr_init, pthread_mutexattr_setpshared, pthread_mutexattr_settype};
use crate::linux::traits::ShmSync;
use nix::libc::{self, PTHREAD_MUTEX_ERRORCHECK, PTHREAD_PROCESS_SHARED};
use std::cell::UnsafeCell;
use std::marker::PhantomPinned;
use std::mem::MaybeUninit;
use std::ptr;
use hashed_type_def::{start_hash_fnv1a, HashedTypeDef};

#[repr(transparent)]
pub struct PthreadMutexT(pthread_mutex_t);

impl HashedTypeDef for PthreadMutexT {
    const TYPE_HASH_NATIVE: u128 = start_hash_fnv1a(b"libc::pthread_mutex_t");
}

#[repr(transparent)]
#[derive(HashedTypeDef)]
pub struct PthreadMutex {
    pub mutex: UnsafeCell<PthreadMutexT>,
    pin_: PhantomPinned
}

impl PthreadMutex {
    /// # Safety
    /// All the following invariants must be satisfied:
    /// * `dest` must be valid for reads and writes as well as correctly aligned and large enough for `PthreadMutex`
    /// * there must not be concurrent access to `dest` during the call
    /// * `dest` stays at the same memory location and remains valid for the whole program execution
    /// * on `Err`, the caller must treat dest as uninitialized.
    pub unsafe fn init(dest: *mut MaybeUninit<Self>) -> Result<(), MutexError> {
        let mut attr = MaybeUninit::uninit();
        // SAFETY: MaybeUninit guarantees validity, writeability, size and align of attr
        if unsafe {pthread_mutexattr_init(attr.as_mut_ptr())} != 0 {
            return Err(MutexError::UnknownError);
        }
        // SAFETY: the last return guarantees that attr is correctly initialized
        if unsafe {pthread_mutexattr_settype(attr.as_mut_ptr(), PTHREAD_MUTEX_ERRORCHECK)} != 0 {
            // SAFETY: see above
            unsafe {pthread_mutexattr_destroy(attr.as_mut_ptr());}
            return Err(MutexError::UnknownError);
        }
        // SAFETY: see above
        if unsafe {pthread_mutexattr_setpshared(attr.as_mut_ptr(), PTHREAD_PROCESS_SHARED)} != 0 {
            // SAFETY: see above
            unsafe {pthread_mutexattr_destroy(attr.as_mut_ptr());}
            return Err(MutexError::UnknownError);
        }

        let dest = dest as *mut Self;
        // SAFETY: dest is still valid, large enough and aligned
        // We only dereference to obtain a pointer to the mutex field, not to
        // access the uninitialized memory.
        let mutex_ptr = unsafe {ptr::addr_of_mut!((*dest).mutex)} as *mut pthread_mutex_t;
        // SAFETY: mutex_ptr is valid, writeable, large enough, aligned and uninitialized
        // attr is still alive
        let status = unsafe {pthread_mutex_init(mutex_ptr, attr.as_ptr())};
        // SAFETY: attr is still alive
        unsafe {pthread_mutexattr_destroy(attr.as_mut_ptr());}
        if status != 0 {
            return Err(MutexError::UnknownError);
        }

        Ok(())
    }
}

impl Drop for PthreadMutex {
    fn drop(&mut self) {
        let status = unsafe {pthread_mutex_destroy(self.mutex.get() as *mut pthread_mutex_t)};
        debug_assert_eq!(status, 0, "pthread_mutex_destroy() failed");
    }
}

unsafe impl Sync for PthreadMutex {}

unsafe impl ShmSync for PthreadMutex {}