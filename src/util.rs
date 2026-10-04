pub fn try_extract_error_message(payload: &(dyn std::any::Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .copied()
        .map(str::to_owned)
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or(format!("{payload:?}"))
}

#[macro_export]
macro_rules! pthread_result {
    ($expr:expr) => {
        match $expr {
            0 => ::std::result::Result::Ok(()),
            e => ::std::result::Result::Err($crate::error_types::DetailedError::add_metadata(
                ::nix::errno::Errno::from_raw(e),
            )),
        }
    };
}
