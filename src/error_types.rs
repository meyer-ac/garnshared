use std::error::Error;
use std::fmt::Display;
use std::panic::Location;

#[derive(Debug)]
pub struct DetailedError {
    file: &'static str,
    line: u32,
    column: u32,
    error: Box<dyn Error + Send + Sync>,
}

impl DetailedError {
    #[must_use]
    pub fn new(
        file: &'static str,
        line: u32,
        column: u32,
        error: Box<dyn Error + Send + Sync>,
    ) -> Self {
        DetailedError {
            file,
            line,
            column,
            error,
        }
    }

    #[must_use]
    pub fn error(&self) -> &(dyn Error + Send + Sync + 'static) {
        &*self.error
    }

    #[track_caller]
    pub fn add_metadata<T: Error + Send + Sync + 'static>(err: T) -> DetailedError {
        let loc = Location::caller();
        DetailedError::new(
            loc.file(),
            loc.line(),
            loc.column(),
            Box::new(err),
        )
    }
}

impl Display for DetailedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "In file {} on line {}, column {}: {}", self.file, self.line, self.column, self.error)
    }
}

impl Error for DetailedError {}

pub trait ResultMetadata<T> {
    fn add_metadata(self) -> Result<T, DetailedError>;
}

impl<T, E> ResultMetadata<T> for Result<T, E>
where E: Error + Send + Sync + 'static {
    #[track_caller]
    fn add_metadata(self) -> Result<T, DetailedError> {
        self.map_err(DetailedError::add_metadata)
    }
}
