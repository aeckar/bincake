//! Implements `Serializable` for numeric types.
//!
//! This module also provides the `write_le_num` macro for writing numbers in little-endian format to byte vectors.

use crate::{error::DeserializeError, error::SerializeError, serializable::Serializable};
use taped::Tape;

/// Writes a numeric value to the `Vec<u8>` in little-endian format,
/// as required by the bytecode specification.
#[macro_export]
macro_rules! write_le_num {
    ($T:ty; $dest:expr, $num:expr) => {
        $dest.extend_from_slice(&($num as $T).to_le_bytes())
    };
}

/// Implements `Serializable` for numeric types.
macro_rules! impl_num_serializers {
    ($($T:ty),* $(,)?) => { $(
        impl Serializable for $T {
            fn write_to(&self, dest: &mut Vec<u8>) -> Result<(), SerializeError> {
                write_le_num!($T; dest, *self);
                Ok(())
            }

            fn read_from(src: &mut Tape<'_, u8>) -> Result<Self, DeserializeError> {
                let size = size_of::<Self>();
                let pos = src.pos;
                let data = &src;
                if pos + size > data.len() {
                    return Err(DeserializeError::Exhausted { pos });
                }
                let slice = &data[pos..pos + size];
                src.pos += size;
                let bytes = slice.try_into()
                    .map_err(|_| DeserializeError::Other {
                        pos,
                        cause: format!("Invalid length at index {}", pos)
                    })?;
                Ok(Self::from_le_bytes(bytes))
            }
        }
    )*};
}

impl_num_serializers!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64,
);
