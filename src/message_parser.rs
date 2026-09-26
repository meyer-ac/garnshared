use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum MessageProtocolError {
    NoMnemonicsProvided,
    MnemonicLenMismatch {
        mnemonic: &'static str,
        mnemonic_len: usize,
    },
    UnknownMnemonic {
        mnemonic: String,
    },
    WrongNumberOfArguments {
        mnemonic: String,
        expected: usize,
        provided: usize,
    },
    ArgumentTooLong {
        mnemonic: String,
        argument_number: usize,
        max_len: usize,
        actual_len: usize,
    },
    MessageTooShort {
        message: String,
        mnemonic_len: usize,
    },
    CharacterCrossesMnemonicBoundary {
        message: String,
        mnemonic_len: usize,
    },
    ExpectedEndOfMessage {
        message: String,
        mnemonic: String,
    },
    ExpectedNewlineAfterMnemonic {
        message: String,
        mnemonic: String,
    },
    NoNullCharacterFound {
        message: String,
    },
    FormattingError,
    WrongArgumentFormat {
        mnemonic: String,
        argument_number: usize,
        error_message: String,
    }, // Meant to be constructed from outside
}

impl Display for MessageProtocolError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            MessageProtocolError::NoMnemonicsProvided => {
                write!(f, "no mnemonics provided; an empty protocol is not allowed")
            }
            MessageProtocolError::MnemonicLenMismatch {
                mnemonic,
                mnemonic_len,
            } => write!(
                f,
                "provided mnemonic \"{mnemonic}\" does not match the required mnemonic length of {mnemonic_len}"
            ),
            MessageProtocolError::UnknownMnemonic { mnemonic } => {
                write!(f, "unknown mnemonic \"{mnemonic}\"")
            }
            MessageProtocolError::WrongNumberOfArguments {
                mnemonic,
                expected,
                provided,
            } => write!(
                f,
                "wrong number of arguments for mnemonic \"{mnemonic}\": expected {expected}, got {provided}"
            ),
            MessageProtocolError::ArgumentTooLong {
                mnemonic,
                argument_number,
                max_len,
                actual_len,
            } => write!(
                f,
                "argument number {argument_number} for mnemonic \"{mnemonic}\" of length {actual_len} exceeds the maximum allowed length of {max_len}"
            ),
            MessageProtocolError::MessageTooShort {
                message,
                mnemonic_len,
            } => write!(
                f,
                "message \"{message}\" (length: {}) is too short to store a mnemonic of length {mnemonic_len}",
                message.len()
            ),
            MessageProtocolError::CharacterCrossesMnemonicBoundary {
                message,
                mnemonic_len
            } => write!(f, "a character must not cross the mnemonic boundary between byte {mnemonic_len} and byte {} in the message \"{message}\"", mnemonic_len + 1),
            MessageProtocolError::ExpectedEndOfMessage { message, mnemonic } => write!(
                f,
                "expected end of message after \"{mnemonic}\" in message \"{message}\""
            ),
            MessageProtocolError::ExpectedNewlineAfterMnemonic { message, mnemonic } => write!(
                f,
                "expected newline after mnemonic \"{mnemonic}\" in message \"{message}\""
            ),
            MessageProtocolError::NoNullCharacterFound { message } => write!(f, "message \"{message}\" must be terminated by a null character"),
            MessageProtocolError::FormattingError => {
                write!(f, "an error occurred upon writing to a string buffer")
            }
            MessageProtocolError::WrongArgumentFormat {
                mnemonic,
                argument_number,
                error_message,
            } => {
                write!(
                    f,
                    "argument number {argument_number} for mnemonic \"{mnemonic}\" has the wrong format: {error_message}"
                )
            }
        }
    }
}

impl Error for MessageProtocolError {}

pub struct Deserialization {
    mnemonic: String,
    args: Box<[String]>,
}

impl Deserialization {
    #[must_use]
    pub fn mnemonic(&self) -> &str {
        &self.mnemonic
    }

    #[must_use]
    pub fn args(&self) -> &[String] {
        &self.args
    }

    #[must_use]
    pub fn wrong_argument_format(
        &self,
        argument_number: usize,
        error_message: String,
    ) -> MessageProtocolError {
        MessageProtocolError::WrongArgumentFormat {
            mnemonic: self.mnemonic.clone(),
            argument_number,
            error_message,
        }
    }
}

pub struct MessageProtocol {
    mnemonic_len: usize,
    mnemonic_args_map: HashMap<&'static str, Box<[usize]>>, // Maps a mnemonic to a slice containing the maximum length of all arguments
}

impl MessageProtocol {
    pub fn new(
        mnemonic_len: usize,
        mnemonic_args_map: HashMap<&'static str, Box<[usize]>>,
    ) -> Result<MessageProtocol, MessageProtocolError> {
        if mnemonic_args_map.is_empty() {
            return Err(MessageProtocolError::NoMnemonicsProvided);
        }
        for mnemonic in mnemonic_args_map.keys() {
            if mnemonic.len() != mnemonic_len {
                return Err(MessageProtocolError::MnemonicLenMismatch {
                    mnemonic,
                    mnemonic_len,
                });
            }
        }
        Ok(MessageProtocol {
            mnemonic_len,
            mnemonic_args_map,
        })
    }

    #[must_use]
    pub fn get_max_size(&self) -> usize {
        self.mnemonic_args_map
            .values()
            .map(
                |v| v.iter().sum::<usize>() + v.len(), /* leading newlines */
            )
            .max()
            .unwrap()
            + self.mnemonic_len
            + 1 /* terminating NULL */
    }

    pub fn serialize(
        &self,
        mnemonic: &'static str,
        args: &[&str],
    ) -> Result<String, MessageProtocolError> {
        let Some(args_lengths) = self.mnemonic_args_map.get(mnemonic) else {
            return Err(MessageProtocolError::UnknownMnemonic {
                mnemonic: mnemonic.to_owned(),
            });
        };
        if args.len() != args_lengths.len() {
            return Err(MessageProtocolError::WrongNumberOfArguments {
                mnemonic: mnemonic.to_owned(),
                expected: args_lengths.len(),
                provided: args.len(),
            });
        }
        let mut s = String::new();
        write!(s, "{mnemonic}").map_err(|_| MessageProtocolError::FormattingError)?;
        for (i, (arg, max_len)) in args.iter().zip(args_lengths.iter()).enumerate() {
            if arg.len() > *max_len {
                return Err(MessageProtocolError::ArgumentTooLong {
                    mnemonic: mnemonic.to_owned(),
                    argument_number: i,
                    max_len: *max_len,
                    actual_len: arg.len(),
                });
            }
            write!(s, "\n{arg}").map_err(|_| MessageProtocolError::FormattingError)?;
        }
        write!(s, "\0").map_err(|_| MessageProtocolError::FormattingError)?;
        Ok(s)
    }

    pub fn deserialize(&self, message: &str) -> Result<Deserialization, MessageProtocolError> {
        if message.len() < self.mnemonic_len {
            return Err(MessageProtocolError::MessageTooShort {
                message: message.to_owned(),
                mnemonic_len: self.mnemonic_len,
            });
        }
        if !message.is_char_boundary(self.mnemonic_len) {
            return Err(MessageProtocolError::CharacterCrossesMnemonicBoundary {message: message.to_owned(), mnemonic_len: self.mnemonic_len});
        }
        let mnemonic = &message[..self.mnemonic_len];
        let Some(args_lengths) = self.mnemonic_args_map.get(mnemonic) else {
            return Err(MessageProtocolError::UnknownMnemonic {
                mnemonic: mnemonic.to_owned(),
            });
        };
        if args_lengths.is_empty() {
            if message.len() > self.mnemonic_len {
                return Err(MessageProtocolError::ExpectedEndOfMessage {
                    message: message.to_owned(),
                    mnemonic: mnemonic.to_owned(),
                });
            }
            return Ok(Deserialization {
                mnemonic: mnemonic.to_owned(),
                args: Box::new([]),
            });
        }
        let null_position = message.find('\0').ok_or(MessageProtocolError::NoNullCharacterFound {message: message.to_owned()})?;
        let message = &message[..null_position];
        let mut delimiter_positions = message
            .match_indices('\n')
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        if delimiter_positions.is_empty() || delimiter_positions[0] != self.mnemonic_len {
            return Err(MessageProtocolError::ExpectedNewlineAfterMnemonic {
                message: message.to_owned(),
                mnemonic: mnemonic.to_owned(),
            });
        }
        if delimiter_positions.len() != args_lengths.len() {
            return Err(MessageProtocolError::WrongNumberOfArguments {
                mnemonic: mnemonic.to_owned(),
                expected: args_lengths.len(),
                provided: delimiter_positions.len(),
            });
        }
        delimiter_positions.push(null_position);
        let mut args = Vec::with_capacity(args_lengths.len());
        for i in 0..delimiter_positions.len() - 1 {
            let arg = message[delimiter_positions[i] + 1..delimiter_positions[i + 1]].to_owned();
            if arg.len() > args_lengths[i] {
                return Err(MessageProtocolError::ArgumentTooLong {
                    mnemonic: mnemonic.to_owned(),
                    argument_number: i,
                    max_len: args_lengths[i],
                    actual_len: arg.len(),
                });
            }
            args.push(arg);
        }
        Ok(Deserialization {
            mnemonic: mnemonic.to_owned(),
            args: args.into_boxed_slice(),
        })
    }
}
