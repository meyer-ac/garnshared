//! The protocol a client uses to communicate with an opened environment.
//! An environment does not expose itself as a state machine to the outside world, i.e. there is no
//! way of inferring the internal state of an environment through use of this protocol.
//! Hence, it is perfectly safe for a client connection to unexpectedly break down and in turn for
//! requests and responses to get lost.

use crate::constants::{MNEMONIC_LEN, ENVIRONMENT_REQUEST_SIZE, MAX_NAME_LEN};
use crate::error_types::SerializeError;

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
                if name.len() > MAX_NAME_LEN {
                    Err(SerializeError::NameTooLongError)
                } else {
                    Ok(format!("{OPEN_MUTEX_MNEMONIC}\n{name}"))
                }
            },
        }
    }
    
    #[must_use]
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
    #[must_use]
    pub fn serialize(&self) -> String {
        match self {
            Self::OpenMutexOk(page, offset) => format!("{OPEN_MUTEX_OK_MNEMONIC}\n{page}\n{offset}"),
            Self::MalformedRequest => MALFORMED_REQUEST_MNEMONIC.to_owned(),
            Self::InternalError => INTERNAL_ERROR_MNEMONIC.to_owned(),
        }
    }

    #[must_use]
    pub fn deserialize(message: &str) -> Option<Self> {
        if message.len() < MNEMONIC_LEN {
            return None;
        }

        let mnemonic = &message[..MNEMONIC_LEN];
        match mnemonic {
            OPEN_MUTEX_OK_MNEMONIC => {
                let second_lf = message[MNEMONIC_LEN+1..].find('\n').map(|pos| pos + MNEMONIC_LEN+1)?;
                let first_null = message.find('\0')?;
                let page = message[MNEMONIC_LEN+1..second_lf].parse::<usize>().ok()?;
                let offset = message[second_lf+1..first_null].parse::<usize>().ok()?;
                Some(Self::OpenMutexOk(page, offset))
            }
            MALFORMED_REQUEST_MNEMONIC => Some(Self::MalformedRequest),
            INTERNAL_ERROR_MNEMONIC => Some(Self::InternalError),
            _ => None,
        }
    }
}