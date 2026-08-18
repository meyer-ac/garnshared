use crate::constants::{MAX_NAME_LEN, MNEMONIC_LEN, WELCOME_REQUEST_SIZE};
use crate::error_types::SerializeError;

const OPEN_ENVIRONMENT_MNEMONIC: &str = "OPEN_ENV";

const OPEN_ENVIRONMENT_OK_MNEMONIC: &str = "OPENV_OK";
const MALFORMED_REQUEST_MNEMONIC: &str = "MALFORMD";
const INTERNAL_ERROR_MNEMONIC: &str = "INTRNERR";

pub enum WelcomeRequest {
    OpenEnvironment(String)
}

impl WelcomeRequest {
    pub fn serialize(&self) -> Result<String, SerializeError> {
        match self {
            Self::OpenEnvironment(name) => {
                if name.as_bytes().len() > MAX_NAME_LEN {
                    Err(SerializeError::NameTooLongError)
                } else {
                    Ok(format!("{}\n{}", OPEN_ENVIRONMENT_MNEMONIC, name))
                }
            },
        }
    }

    pub fn deserialize(message: &str) -> Option<Self> {
        if message.len() < MNEMONIC_LEN {
            return None;
        }

        let mnemonic = &message[..MNEMONIC_LEN];
        let first_null = message.find('\0').unwrap_or(WELCOME_REQUEST_SIZE);
        match mnemonic {
            OPEN_ENVIRONMENT_MNEMONIC => Some(Self::OpenEnvironment(message[MNEMONIC_LEN+1..first_null].to_owned())),
            _ => None,
        }
    }
}

pub enum WelcomeResponse {
    OpenEnvironmentOk,
    MalformedRequest,
    InternalError,
}

impl WelcomeResponse {
    pub fn serialize(&self) -> String {
        match self {
            Self::OpenEnvironmentOk => OPEN_ENVIRONMENT_OK_MNEMONIC.to_owned(),
            Self::MalformedRequest => MALFORMED_REQUEST_MNEMONIC.to_owned(),
            Self::InternalError => INTERNAL_ERROR_MNEMONIC.to_owned(),
        }
    }
    
    pub fn deserialize(message: &str) -> Option<Self> {
        if message.len() < MNEMONIC_LEN {
            return None;
        }

        let mnemonic = &message[..MNEMONIC_LEN];
        match mnemonic {
            OPEN_ENVIRONMENT_OK_MNEMONIC => Some(Self::OpenEnvironmentOk),
            MALFORMED_REQUEST_MNEMONIC => Some(Self::MalformedRequest),
            INTERNAL_ERROR_MNEMONIC => Some(Self::InternalError),
            _ => None,
        }
    }
}