use std::error::Error;

#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        name.strip_suffix("::f").unwrap()
    }}
}

#[macro_export]
macro_rules! add_metadata_to_boxed_error {
    ($err:expr) => {
        SendableErrorWithMetadata {
            file: file!(),
            function: $crate::error_types::function!(),
            line: line!(),
            column: column!(),
            error: $err,
        }
    }
}

#[macro_export]
macro_rules! add_metadata_to_error {
    ($err:expr) => {
        SendableErrorWithMetadata {
            file: file!(),
            function: $crate::error_types::function!(),
            line: line!(),
            column: column!(),
            error: Box::new($err),
        }
    }
}

pub use {function, add_metadata_to_boxed_error, add_metadata_to_error};

pub struct SendableErrorWithMetadata {
    pub file: &'static str,
    pub function: &'static str,
    pub line: u32,
    pub column: u32,
    pub error: Box<dyn Error + Send + Sync>,
}