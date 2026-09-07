use crate::linux::traits::ShmCompatible;
use nix::libc::{pthread_mutex_t, PTHREAD_MUTEX_ERRORCHECK, PTHREAD_PROCESS_SHARED, pthread_mutexattr_init, pthread_mutexattr_settype, pthread_mutexattr_destroy, pthread_mutexattr_setpshared, pthread_mutex_init, pthread_mutex_destroy};
use std::cell::UnsafeCell;
use std::marker::PhantomPinned;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::ptr;
use hashed_type_def::{start_hash_fnv1a, HashedTypeDef};
use nix::errno::Errno;
use crate::error_types::SendableError;

#[repr(transparent)]
pub struct PthreadMutexT(pthread_mutex_t);

impl HashedTypeDef for PthreadMutexT {
    const TYPE_HASH_NATIVE: u128 = start_hash_fnv1a(b"libc::pthread_mutex_t");
}

/// This type provides a scaffolding and is meant to be specialized in the `libgarn` crate, i.e.
/// be equipped with the necessary interface.
#[repr(transparent)]
#[derive(HashedTypeDef)]
pub struct PthreadMutex {
    pub mutex: UnsafeCell<PthreadMutexT>,
    _pin: PhantomPinned
}

impl PthreadMutex {
    // todo: decide whether the mutex should be robust or not or if the user should choose
    // robust (or letting the user choose) would be better for debugging purposes,
    // non-robust would be better for educational purposes

    /// # Note
    /// * on `Err`, the caller must treat dest as uninitialized.
    pub fn init(dest: Pin<&mut MaybeUninit<Self>>) -> Result<(), SendableError> {
        let mut attr = MaybeUninit::uninit();
        // SAFETY: MaybeUninit guarantees validity, writeability, size and align of attr
        if unsafe {pthread_mutexattr_init(attr.as_mut_ptr())} != 0 {
            return Err(Box::new(Errno::last()));
        }
        // SAFETY: the last return guarantees that attr is correctly initialized
        if unsafe {pthread_mutexattr_settype(attr.as_mut_ptr(), PTHREAD_MUTEX_ERRORCHECK)} != 0 {
            // SAFETY: see above
            unsafe {pthread_mutexattr_destroy(attr.as_mut_ptr());}
            return Err(Box::new(Errno::last()));
        }
        // SAFETY: see above
        if unsafe {pthread_mutexattr_setpshared(attr.as_mut_ptr(), PTHREAD_PROCESS_SHARED)} != 0 {
            // SAFETY: see above
            unsafe {pthread_mutexattr_destroy(attr.as_mut_ptr());}
            return Err(Box::new(Errno::last()));
        }
        
        // SAFETY: The pinned value won't move as it isn't moved inside this function and the
        // pointer is discarded upon returning
        let dest = unsafe {dest.get_unchecked_mut()}.as_mut_ptr().cast::<Self>();
        // SAFETY: dest is still valid, large enough and aligned
        // We only dereference to obtain a pointer to the mutex field, not to
        // access the uninitialized memory.
        let mutex_ptr = unsafe {ptr::addr_of_mut!((*dest).mutex)}.cast::<pthread_mutex_t>();
        // SAFETY: mutex_ptr is valid, writeable, large enough, aligned and uninitialized
        // attr is still alive
        let status = unsafe {pthread_mutex_init(mutex_ptr, attr.as_ptr())};
        // SAFETY: attr is still alive
        unsafe {pthread_mutexattr_destroy(attr.as_mut_ptr());}
        if status != 0 {
            return Err(Box::new(Errno::last()));
        }

        Ok(())
    }
}

impl Drop for PthreadMutex {
    fn drop(&mut self) {
        let status = unsafe {pthread_mutex_destroy(self.mutex.get().cast::<pthread_mutex_t>())};
        debug_assert_eq!(status, 0, "pthread_mutex_destroy() failed");
    }
}

unsafe impl Sync for PthreadMutex {}

unsafe impl ShmCompatible for PthreadMutex {}