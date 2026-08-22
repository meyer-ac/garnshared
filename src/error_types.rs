use std::error::Error;

pub type SendableError = Box<dyn Error + Send + Sync>;

pub enum SerializeError {
    NameTooLongError,
}