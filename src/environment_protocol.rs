use uuid::Uuid;
use crate::constants::{MNEMONIC_LEN, ENVIRONMENT_REQUEST_SIZE, MAX_NAME_LEN};
use crate::error_types::SerializeError;
use crate::welcome_protocol::WelcomeResponse;

const OPEN_MUTEX_MNEMONIC: &str = "OPEN_MUT";

const OPEN_MUTEX_OK_MNEMONIC: &str = "OPMUT_OK";
const MALFORMED_REQUEST_MNEMONIC: &str = "MALFORMD";
const INTERNAL_ERROR_MNEMONIC: &str = "INTRNERR";

pub enum EnvironmentRequest {
    OpenMutex(String)
}

impl EnvironmentRequest {
    pub fn serialize(&self) -> Result<String, SerializeError> {
        match self {
            Self::OpenMutex(name) => {
                if name.as_bytes().len() > MAX_NAME_LEN {
                    Err(SerializeError::NameTooLongError)
                } else {
                    Ok(format!("{}\n{}", OPEN_MUTEX_MNEMONIC, name))
                }
            },
        }
    }
    
    pub fn deserialize(message: &str) -> Option<Self> {
        if message.len() < MNEMONIC_LEN {
            return None;
        }
        
        let mnemonic = &message[..MNEMONIC_LEN];
        let first_null = message.find('\0').unwrap_or(ENVIRONMENT_REQUEST_SIZE);
        match mnemonic {
            OPEN_MUTEX_MNEMONIC => Some(Self::OpenMutex(message[MNEMONIC_LEN+1..first_null].to_owned())),
            _ => None,
        }
    }
}

pub enum EnvironmentResponse {
    OpenMutexOk(usize, usize),
    MalformedRequest,
    InternalError
}

impl EnvironmentResponse {
    pub fn serialize(&self) -> String {
        match self {
            Self::OpenMutexOk(page, offset) => format!("{}\n{}\n{}", OPEN_MUTEX_OK_MNEMONIC, page, offset),
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
            OPEN_MUTEX_MNEMONIC => {
                let second_lf = message[MNEMONIC_LEN+1..].find('\n');
                if second_lf.is_none() {
                    return None;
                }
                let first_null = message.find('\0');
                if first_null.is_none() {
                    return None;
                }
                let page = usize::from_str_radix(&message[MNEMONIC_LEN+1..second_lf.unwrap()], 10);
                let offset = usize::from_str_radix(&message[second_lf.unwrap()+1..first_null.unwrap()], 10);
                if page.is_err() || offset.is_err() {
                    return None;
                }
                Some(Self::OpenMutexOk(page.unwrap(), offset.unwrap()))
            }
            MALFORMED_REQUEST_MNEMONIC => Some(Self::MalformedRequest),
            INTERNAL_ERROR_MNEMONIC => Some(Self::InternalError),
            _ => None,
        }
    }
}