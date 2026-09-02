use std::error::Error;

pub type SendableError = Box<dyn Error + Send + Sync>;

#[derive(Debug)]
pub enum SerializeError {
    NameTooLongError,
}