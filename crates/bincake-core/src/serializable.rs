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

use crate::{
    error::{DeserializeError, SerializeError},
    traits::Read,
    vec_n::Vec32,
};

/// Writes all provided data to the destination.
#[macro_export]
macro_rules! write_all {
    ($dest:expr; $($data:expr),* $(,)?) => {
        {
            let mut result;
            'cases: {
                $(
                    result = $dest.write($data);
                    if result.is_err() {
                        break 'cases;
                    }
                )*
            }
            result
        }
    };
}

/// A value that can be serialized to and deserialized from bytecode.
pub trait Serializable: Sized {
    /// Encodes this value to bytecode by appending it to the buffer.
    fn write_to(&self, dest: &mut Vec<u8>) -> Result<(), SerializeError>;

    /// Decodes the value from bytecode.
    ///
    /// Returns `crate::bytecode::Error` if the encoded data is malformed.
    fn read_from(src: &mut Tape<'_, u8>) -> Result<Self, DeserializeError>;
}

impl Serializable for bool {
    fn write_to(&self, dest: &mut Vec<u8>) -> Result<(), SerializeError> {
        dest.push(if *self { 1u8 } else { 0u8 });
        Ok(())
    }

    fn read_from(src: &mut Tape<'_, u8>) -> Result<Self, DeserializeError> {
        src.next()
            .ok_or(DeserializeError::Exhausted { pos: src.pos })
            .map(|b| b == 1)
    }
}

impl Serializable for String {
    fn write_to(&self, dest: &mut Vec<u8>) -> Result<(), SerializeError> {
        let len = self.len();
        #[cfg(target_pointer_width = "64")]
        if len > u32::MAX as usize {
            return Err(SerializeError::LengthExceedsPrefix {
                prefix_size: 32,
                len,
            });
        }
        crate::write_le_num!(u32; dest, len);
        dest.extend_from_slice(self.as_bytes());
        Ok(())
    }

    fn read_from(src: &mut Tape<'_, u8>) -> Result<Self, DeserializeError> {
        let start = src.pos;
        let data = src.read::<Vec32<u8>>()?.into_inner();
        String::from_utf8(data).map_err(|e| DeserializeError::Other {
            pos: start,
            cause: format!("Invalid UTF-8 ({e})"),
        })
    }
}
