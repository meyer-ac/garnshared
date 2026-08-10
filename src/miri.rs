/*
 * Minimal shims of certain nix functions to be used with Miri.
 * These functions do NOT provide a viable alternative!
 * They do nothing!
 */

cfg_if::cfg_if! {
    if #[cfg(miri)] {
        use nix::libc::{pthread_mutexattr_t, c_int};

        #[allow(non_camel_case_types)]
        pub struct pthread_mutex_t {
            _placeholder: u8
        }

        // SAFETY: see ptr::write
        pub unsafe fn pthread_mutex_init(lock: *mut pthread_mutex_t, _attr: *const pthread_mutexattr_t) -> c_int {
            unsafe {
                lock.write(pthread_mutex_t {
                    _placeholder: 0
                });
            }
            0
        }

        // SAFETY: Safe. Only marked as `unsafe` to exactly match the signature
        pub unsafe fn pthread_mutex_lock(_lock: *mut pthread_mutex_t) -> c_int {
            0
        }

        // SAFETY: Safe. Only marked as `unsafe` to exactly match the signature
        pub unsafe fn pthread_mutex_unlock(_lock: *mut pthread_mutex_t) -> c_int {
            0
        }

        // SAFETY: Safe. Only marked as `unsafe` to exactly match the signature
        pub unsafe fn pthread_mutex_trylock(_lock: *mut pthread_mutex_t) -> c_int {
            0
        }

        // SAFETY: see ptr::read
        pub unsafe fn pthread_mutex_destroy(lock: *mut pthread_mutex_t) -> c_int {
            let _lock = unsafe {lock.read()};
            0
        }

        // SAFETY: Safe. Only marked as `unsafe` to exactly match the signature
        pub unsafe fn pthread_mutexattr_init(_attr: *mut pthread_mutexattr_t) -> c_int {
            0
        }

        // SAFETY: Safe. Only marked as `unsafe` to exactly match the signature
        pub unsafe fn pthread_mutexattr_settype(_attr: *mut pthread_mutexattr_t, _type: c_int) -> c_int {
            0
        }

        // SAFETY: Safe. Only marked as `unsafe` to exactly match the signature
        pub unsafe fn pthread_mutexattr_setpshared(_attr: *mut pthread_mutexattr_t, _pshared: c_int) -> c_int {
            0
        }

        // SAFETY: Safe. Only marked as `unsafe` to exactly match the signature
        pub unsafe fn pthread_mutexattr_destroy(_attr: *mut pthread_mutexattr_t) -> c_int {
            0
        }
    } else {
        pub use nix::libc::{
            pthread_mutex_t,
            pthread_mutex_init,
            pthread_mutex_lock,
            pthread_mutex_unlock,
            pthread_mutex_trylock,
            pthread_mutex_destroy,
            pthread_mutexattr_init,
            pthread_mutexattr_settype,
            pthread_mutexattr_setpshared,
            pthread_mutexattr_destroy
        };
    }
}