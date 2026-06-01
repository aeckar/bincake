//! Provides the `Serializable` trait for handling serialization to/from a consistent, opinionated binary format.
//!
//! The binary format specified by this crate is used to emit RVM bytecode,
//! and exhibits the following properties to ensure cross-platform compatibility:
//!
//! - Byte-aligned data
//! - Little-endian encoding for numeric types
//! - Strings encoded in UTF-8 with a `u32` length prefix (see `StrLen`)
//! - Arrays encoded with a length prefix of varying size (e.g., `u8`, `u16`, `u32`), followed by the serialized elements
//!
//! This module also provides trivial implementations of `Serializable` for `bool` and `String`.

use taped::Tape;

use crate::{DecodeError, EncodeError, Read, StringSize, Vec32, Write};

/// Writes all arguments to a byte buffer, taking each as a reference.
#[macro_export]
macro_rules! stream {
    ($($data:expr),* $(,)? => $dest:expr) => {
        {
            let mut result;
            'cases: {
                $(
                    result = $dest.write(&$data);
                    if result.is_err() {
                        break 'cases;
                    }
                )*
            }
            result
        }
    };
}

/// A value that can be serialized to and deserialized from a byte buffer.
pub trait Serialize: Sized {
    /// Encodes this value by appending it to the buffer.
    fn encode(&self, dest: &mut Vec<u8>) -> Result<(), EncodeError>;

    /// Decodes the value by consuming the next one in the buffer.
    ///
    /// Returns `bincake::DeserializeError` if the encoded data is malformed.
    fn decode(src: &mut Tape<'_, u8>) -> Result<Self, DecodeError>;
}

impl Serialize for bool {
    fn encode(&self, dest: &mut Vec<u8>) -> Result<(), EncodeError> {
        dest.push(if *self { 1u8 } else { 0u8 });
        Ok(())
    }

    fn decode(src: &mut Tape<'_, u8>) -> Result<Self, DecodeError> {
        src.next()
            .ok_or(DecodeError::Exhausted { pos: src.pos })
            .map(|b| b == 1)
    }
}

impl Serialize for String {
    fn encode(&self, dest: &mut Vec<u8>) -> Result<(), EncodeError> {
        let len = self.len();
        let len32 = StringSize::try_from(len).map_err(|_| EncodeError::LengthExceedsPrefix {
            prefix_size: 32,
            len,
        })?;
        dest.write(&len32)?;
        dest.extend_from_slice(self.as_bytes());
        Ok(())
    }

    fn decode(src: &mut Tape<'_, u8>) -> Result<Self, DecodeError> {
        let start = src.pos;
        let data = src.read::<Vec32<u8>>()?.into();
        String::from_utf8(data).map_err(|e| DecodeError::Other {
            pos: start,
            cause: format!("Invalid UTF-8 ({e})"),
        })
    }
}
