#[must_use]
pub const fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[must_use]
pub const fn lcm(a: NonZero<usize>, b: NonZero<usize>) -> NonZero<usize> {
    // Unwrap: LCM can't be zero because `a` and `b` are non-zero
    NonZero::new((a.get() / gcd(a.get(), b.get())) * b.get()).unwrap()
}

pub fn try_extract_error_message(payload: &(dyn std::any::Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .copied()
        .map(str::to_owned)
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or(format!("{payload:?}"))
}

#[macro_export]
macro_rules! pthread_result_errno {
    ($expr:expr) => {
        match $expr {
            0 => ::std::result::Result::Ok(()),
            e => ::std::result::Result::Err(::nix::errno::Errno::from_raw(e)),
        }
    };
}

#[macro_export]
macro_rules! pthread_result_detailed {
    ($expr:expr) => {
        <_ as $crate::error_types::ResultMetadata<()>>::add_metadata(
            $crate::util::pthread_result_errno!($expr),
        )
    };
}

use std::num::NonZero;
pub use {pthread_result_errno, pthread_result_detailed};
