//! Defines wrapper types for vectors with sizes of various encodings.
//!
//! These types implement the `Serializable` trait, allowing them to be serialized to and deserialized from bytecode
//! with the appropriate size prefix.
//! They also dereference to `Vec<T>` for ease of use.

use crate::{
    error::{DeserializeError, SerializeError},
    serializable::Serializable,
    traits::{Read, Write},
    write_le_num,
};

use pastey::paste;
use taped::Tape;

/// Implements Serializable for sequences with sizes of various encodings.
macro_rules! impl_vec_n {
    ($($width:literal),* $(,)?) => { paste! { $(
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct [<Vec $width>]<T>(Vec<T>);

        impl<T> [<Vec $width>]<T> {
            /// Creates a new VecN from a Vec.
            pub const fn new(vec: Vec<T>) -> Self {
                Self(vec)
            }

            /// Consumes self and returns the inner Vec.
            pub fn into_inner(self) -> Vec<T> {
                self.0
            }
        }

        impl<T: Serializable> Serializable for [<Vec $width>]<T> {
            fn write_to(&self, dest: &mut Vec<u8>) -> Result<(), SerializeError> {
                let len = self.len();
                #[cfg(target_pointer_width = "64")]
                if len > 0xFFFFFFFFusize /* u32::MAX */ {
                    return Err(SerializeError::LengthExceedsPrefix { prefix_size: $width, len })
                }
                write_le_num!([<u $width>]; dest, len);
                for d in self.iter() {
                    dest.write(d)?;
                }
                Ok(())
            }

            fn read_from(src: &mut Tape<'_, u8>) -> Result<Self, DeserializeError> {
                let len = src.read::<[<u $width>]>()? as usize;
                let mut data = Vec::with_capacity(len);
                for _ in 0..len {
                    data.push(src.read::<T>()?);
                }
                Ok(Self(data))
            }
        }

        // Dereference to underlying data.
        impl<T> std::ops::Deref for [<Vec $width>]<T> {
            type Target = Vec<T>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        // Mutable dereference to underlying data.
        impl<T> std::ops::DerefMut for [<Vec $width>]<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    )*}};
}

impl_vec_n!(8, 16, 32, 64, 128);
