use std::error::Error;
use std::fmt::Display;
use std::panic::Location;

#[macro_export]
macro_rules! add_metadata_to_boxed_error {
    ($err:expr) => {
        $crate::error_types::SendableErrorWithMetadata::new(
            file!(),
            line!(),
            column!(),
            $err,
        )
    };
}

#[macro_export]
macro_rules! add_metadata_to_error {
    ($err:expr) => {
        $crate::error_types::add_metadata_to_boxed_error!(::std::boxed::Box::new($err))
    };
}

pub use {add_metadata_to_boxed_error, add_metadata_to_error};

#[derive(Debug)]
pub struct SendableErrorWithMetadata {
    file: &'static str,
    line: u32,
    column: u32,
    error: Box<dyn Error + Send + Sync>,
}

impl SendableErrorWithMetadata {
    pub fn new(
        file: &'static str,
        line: u32,
        column: u32,
        error: Box<dyn Error + Send + Sync>,
    ) -> Self {
        SendableErrorWithMetadata {
            file,
            line,
            column,
            error,
        }
    }

    pub fn error(&self) -> &Box<dyn Error + Send + Sync> {
        &self.error
    }
}

impl Display for SendableErrorWithMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "In file {} on line {}, column {}: {}", self.file, self.line, self.column, self.error)
    }
}

impl Error for SendableErrorWithMetadata {}

pub trait ResultMetadata<T> {
    fn add_metadata(self) -> Result<T, SendableErrorWithMetadata>;
}

impl<T, E> ResultMetadata<T> for Result<T, E>
where E: Error + Send + Sync + 'static {
    #[track_caller]
    fn add_metadata(self) -> Result<T, SendableErrorWithMetadata> {
        self.map_err(|e| {
            let loc = Location::caller();
            SendableErrorWithMetadata::new(
                loc.file(),
                loc.line(),
                loc.column(),
                Box::new(e),
            )
        })
    }
}
