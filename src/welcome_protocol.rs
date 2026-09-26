use crate::constants::{MAX_NAME_LEN, MNEMONIC_LEN};
use crate::message_parser::{MessageProtocol, MessageProtocolError};
use std::collections::HashMap;
use std::sync::LazyLock;

const OPEN_ENVIRONMENT_MNEMONIC: &str = "OPEN_ENV";

const OPEN_ENVIRONMENT_OK_MNEMONIC: &str = "OPENV_OK";
const MALFORMED_REQUEST_MNEMONIC: &str = "MALFORMD";
const INTERNAL_ERROR_MNEMONIC: &str = "INTRNERR";

pub static WELCOME_REQUEST_PROTOCOL: LazyLock<MessageProtocol> = LazyLock::new(
    || {
        MessageProtocol::new(
            MNEMONIC_LEN,
            HashMap::from([(OPEN_ENVIRONMENT_MNEMONIC, vec![MAX_NAME_LEN].into_boxed_slice())]),
        )
        .unwrap()
    }, /* If something goes wrong here, it is a configuration error in the source code; it should panic. */
);

pub static WELCOME_RESPONSE_PROTOCOL: LazyLock<MessageProtocol> = LazyLock::new(
    || {
        MessageProtocol::new(
            MNEMONIC_LEN,
            HashMap::from([
                (OPEN_ENVIRONMENT_OK_MNEMONIC, vec![].into_boxed_slice()),
                (MALFORMED_REQUEST_MNEMONIC, vec![].into_boxed_slice()),
                (INTERNAL_ERROR_MNEMONIC, vec![].into_boxed_slice()),
            ]),
        )
        .unwrap()
    }, /* If something goes wrong here, it is a configuration error in the source code; it should panic. */
);

pub enum WelcomeRequest {
    OpenEnvironment(String),
}

impl WelcomeRequest {
    pub fn serialize(&self) -> Result<String, MessageProtocolError> {
        match self {
            Self::OpenEnvironment(name) => {
                WELCOME_REQUEST_PROTOCOL.serialize(OPEN_ENVIRONMENT_MNEMONIC, &[name])
            }
        }
    }

    pub fn deserialize(message: &str) -> Result<Self, MessageProtocolError> {
        let deserialization = WELCOME_REQUEST_PROTOCOL.deserialize(message)?;
        match deserialization.mnemonic() {
            OPEN_ENVIRONMENT_MNEMONIC => {
                Ok(Self::OpenEnvironment(deserialization.args()[0].clone()))
            }
            _ => unreachable!(),
        }
    }
}

pub enum WelcomeResponse {
    OpenEnvironmentOk,
    MalformedRequest,
    InternalError,
}

impl WelcomeResponse {
    #[must_use]
    pub fn serialize_internal_error() -> String {
        Self::InternalError.serialize().unwrap()
    }

    pub fn serialize(&self) -> Result<String, MessageProtocolError> {
        match self {
            Self::OpenEnvironmentOk => WELCOME_RESPONSE_PROTOCOL.serialize(OPEN_ENVIRONMENT_OK_MNEMONIC, &[]),
            Self::MalformedRequest => WELCOME_RESPONSE_PROTOCOL.serialize(MALFORMED_REQUEST_MNEMONIC, &[]),
            Self::InternalError => WELCOME_RESPONSE_PROTOCOL.serialize(INTERNAL_ERROR_MNEMONIC, &[]),
        }
    }

    pub fn deserialize(message: &str) -> Result<Self, MessageProtocolError> {
        let deserialization = WELCOME_RESPONSE_PROTOCOL.deserialize(message)?;
        match deserialization.mnemonic() {
            OPEN_ENVIRONMENT_OK_MNEMONIC => Ok(Self::OpenEnvironmentOk),
            MALFORMED_REQUEST_MNEMONIC => Ok(Self::MalformedRequest),
            INTERNAL_ERROR_MNEMONIC => Ok(Self::InternalError),
            _ => unreachable!(),
        }
    }
}
