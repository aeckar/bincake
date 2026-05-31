//! Implements `Serialize` for numeric types.
//!
//! This module also provides the `write_le_num` macro for writing numbers in little-endian format to byte vectors.

use taped::Tape;

use crate::{
    Serialize,
    error::{DecodeError, EncodeError},
};

/// Writes a numeric value to the buffer in little-endian format.
#[macro_export]
macro_rules! write_le_num {
    ($T:ty; $dest:expr, $num:expr) => {
        $dest.extend_from_slice(&($num as $T).to_le_bytes())
    };
}

/// Implements `Serialize` for numeric types.
macro_rules! impl_serialize_num {
    ($($T:ty),* $(,)?) => { $(
        impl Serialize for $T {
            fn encode(&self, dest: &mut Vec<u8>) -> Result<(), EncodeError> {
                write_le_num!($T; dest, *self);
                Ok(())
            }

            fn decode(src: &mut Tape<'_, u8>) -> Result<Self, DecodeError> {
                let size = size_of::<Self>();
                let pos = src.pos;
                let data = &src;
                if pos + size > data.len() {
                    return Err(DecodeError::Exhausted { pos });
                }
                let slice = &data[pos..pos + size];
                src.pos += size;
                let bytes = slice.try_into()
                    .map_err(|_| DecodeError::Other {
                        pos,
                        cause: format!("Invalid length at index {}", pos)
                    })?;
                Ok(Self::from_le_bytes(bytes))
            }
        }
    )*};
}

impl_serialize_num!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64,
);
