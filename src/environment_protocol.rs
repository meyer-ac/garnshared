//! The protocol a client uses to communicate with an opened environment.
//! An environment does not expose itself as a state machine to the outside world, i.e. there is no
//! way of inferring the internal state of an environment through use of this protocol.
//! Hence, it is perfectly safe for a client connection to unexpectedly break down and in turn for
//! requests and responses to get lost.

use crate::constants::{MAX_NAME_LEN, MNEMONIC_LEN};
use crate::message_parser::{MessageProtocol, MessageProtocolError};
use std::collections::HashMap;
use std::sync::LazyLock;

const OPEN_MUTEX_MNEMONIC: &str = "OPEN_MUT";

const OPEN_MUTEX_OK_MNEMONIC: &str = "OPMUT_OK";
const MALFORMED_REQUEST_MNEMONIC: &str = "MALFORMD";
const INTERNAL_ERROR_MNEMONIC: &str = "INTRNERR";

static ENVIRONMENT_REQUEST_PROTOCOL: LazyLock<MessageProtocol> = LazyLock::new(
    || {
        MessageProtocol::new(
            MNEMONIC_LEN,
            HashMap::from([(OPEN_MUTEX_MNEMONIC, vec![MAX_NAME_LEN].into_boxed_slice())]),
        )
        .unwrap()
    }, /* If something goes wrong here, it is a configuration error in the source code; it should panic. */
);

static ENVIRONMENT_RESPONSE_PROTOCOL: LazyLock<MessageProtocol> = LazyLock::new(
    || {
        MessageProtocol::new(
            MNEMONIC_LEN,
            HashMap::from([
                (
                    OPEN_MUTEX_OK_MNEMONIC,
                    vec![
                        usize::MAX.checked_ilog10().unwrap() as usize + 1,
                        usize::MAX.checked_ilog10().unwrap() as usize + 1,
                    ]
                    .into_boxed_slice(),
                ),
                (MALFORMED_REQUEST_MNEMONIC, vec![].into_boxed_slice()),
                (INTERNAL_ERROR_MNEMONIC, vec![].into_boxed_slice()),
            ]),
        )
        .unwrap()
    }, /* If something goes wrong here, it is a configuration error in the source code; it should panic. */
);

pub enum EnvironmentRequest {
    OpenMutex(String),
}

impl EnvironmentRequest {
    pub fn serialize(&self) -> Result<String, MessageProtocolError> {
        match self {
            Self::OpenMutex(name) => {
                ENVIRONMENT_REQUEST_PROTOCOL.serialize(OPEN_MUTEX_MNEMONIC, &[name])
            }
        }
    }

    pub fn deserialize(message: &str) -> Result<Self, MessageProtocolError> {
        let deserialization = ENVIRONMENT_REQUEST_PROTOCOL.deserialize(message)?;
        match deserialization.mnemonic() {
            OPEN_MUTEX_MNEMONIC => Ok(Self::OpenMutex(deserialization.args()[0].clone())),
            _ => unreachable!(),
        }
    }
}

pub enum EnvironmentResponse {
    OpenMutexOk(usize, usize),
    MalformedRequest,
    InternalError,
}

impl EnvironmentResponse {
    #[must_use]
    pub fn serialize_internal_error() -> String {
        Self::InternalError.serialize().unwrap()
    }

    pub fn serialize(&self) -> Result<String, MessageProtocolError> {
        match self {
            Self::OpenMutexOk(page, offset) => ENVIRONMENT_RESPONSE_PROTOCOL.serialize(
                OPEN_MUTEX_OK_MNEMONIC,
                &[&page.to_string(), &offset.to_string()],
            ),
            Self::MalformedRequest => {
                ENVIRONMENT_RESPONSE_PROTOCOL.serialize(MALFORMED_REQUEST_MNEMONIC, &[])
            }
            Self::InternalError => {
                ENVIRONMENT_RESPONSE_PROTOCOL.serialize(INTERNAL_ERROR_MNEMONIC, &[])
            }
        }
    }

    pub fn deserialize(message: &str) -> Result<Self, MessageProtocolError> {
        let deserialization = ENVIRONMENT_RESPONSE_PROTOCOL.deserialize(message)?;
        match deserialization.mnemonic() {
            OPEN_MUTEX_OK_MNEMONIC => {
                let page = deserialization.args()[0].parse::<usize>().map_err(|_| {
                    deserialization.wrong_argument_format(
                        0,
                        "page must be an usize in decimal representation".to_owned(),
                    )
                })?;
                let offset = deserialization.args()[1].parse::<usize>().map_err(|_| {
                    deserialization.wrong_argument_format(
                        1,
                        "offset must be an usize in decimal representation".to_owned(),
                    )
                })?;
                Ok(Self::OpenMutexOk(page, offset))
            }
            MALFORMED_REQUEST_MNEMONIC => Ok(Self::MalformedRequest),
            INTERNAL_ERROR_MNEMONIC => Ok(Self::InternalError),
            _ => unreachable!(),
        }
    }
}
