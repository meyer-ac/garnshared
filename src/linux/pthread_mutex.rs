use std::debug_assert_matches;
use crate::error_types::DetailedError;
use crate::linux::traits::ShmCompatible;
use hashed_type_def::{HashedTypeDef, start_hash_fnv1a};
use nix::libc::{
    PTHREAD_MUTEX_ERRORCHECK, PTHREAD_MUTEX_ROBUST, PTHREAD_PROCESS_SHARED, pthread_mutex_destroy,
    pthread_mutex_init, pthread_mutex_t, pthread_mutexattr_destroy, pthread_mutexattr_init,
    pthread_mutexattr_setpshared, pthread_mutexattr_setrobust, pthread_mutexattr_settype,
};
use std::cell::UnsafeCell;
use std::marker::PhantomPinned;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::ptr;
use crate::pthread_result_detailed;

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
    _pin: PhantomPinned,
}

impl PthreadMutex {
    /// # Note
    /// * on `Err`, the caller must treat dest as uninitialized.
    pub fn init(dest: Pin<&mut MaybeUninit<Self>>) -> Result<&Self, DetailedError> {
        let mut attr = MaybeUninit::uninit();
        // SAFETY: MaybeUninit guarantees validity, writeability, size and align of attr
        pthread_result_detailed!(unsafe {pthread_mutexattr_init(attr.as_mut_ptr())})?;
        // SAFETY: the last return guarantees that attr is correctly initialized
        if let Err(e) =  pthread_result_detailed!(unsafe { pthread_mutexattr_settype(attr.as_mut_ptr(), PTHREAD_MUTEX_ERRORCHECK) }) {
            // SAFETY: see above
            unsafe {
                let _ = pthread_mutexattr_destroy(attr.as_mut_ptr());
            }
            return Err(e);
        }
        // SAFETY: see above
        if let Err(e) = pthread_result_detailed!(unsafe { pthread_mutexattr_setpshared(attr.as_mut_ptr(), PTHREAD_PROCESS_SHARED) }) {
            // SAFETY: see above
            unsafe {
                let _ = pthread_mutexattr_destroy(attr.as_mut_ptr());
            }
            return Err(e);
        }
        // SAFETY: see above
        if let Err(e) =  pthread_result_detailed!(unsafe { pthread_mutexattr_setrobust(attr.as_mut_ptr(), PTHREAD_MUTEX_ROBUST) }) {
            // SAFETY: see above
            unsafe {
                let _ = pthread_mutexattr_destroy(attr.as_mut_ptr());
            }
            return Err(e);
        }

        // SAFETY: The pinned value won't move as it isn't moved inside this function and the
        // pointer is discarded upon returning
        let dest = unsafe { dest.get_unchecked_mut() }
            .as_mut_ptr()
            .cast::<Self>();
        // SAFETY: dest is still valid, large enough and aligned
        // We only dereference to obtain a pointer to the mutex field, not to
        // access the uninitialized memory.
        let mutex_ptr = unsafe { ptr::addr_of_mut!((*dest).mutex) }.cast::<pthread_mutex_t>();
        // SAFETY: pthread_mutex_init: mutex_ptr is valid, writeable, large enough, aligned and uninitialized
        //     attr is still alive
        // as_ref_unchecked: Initialization was successful
        let init_result = pthread_result_detailed!(unsafe { pthread_mutex_init(mutex_ptr, attr.as_ptr()) }).map(|()| unsafe { dest.as_ref_unchecked() });
        // SAFETY: attr is still alive
        unsafe {
            // Doesn't really matter if this fails, the mutex is already up and running
            let _ = pthread_mutexattr_destroy(attr.as_mut_ptr());
        }
        init_result
    }
}

impl Drop for PthreadMutex {
    fn drop(&mut self) {
        let result = pthread_result_detailed!(unsafe { pthread_mutex_destroy(self.mutex.get().cast::<pthread_mutex_t>()) });
        debug_assert_matches!(result, Ok(()), "pthread_mutex_destroy() failed");
    }
}

unsafe impl Sync for PthreadMutex {}

unsafe impl ShmCompatible for PthreadMutex {}

// todo: !Unpin
