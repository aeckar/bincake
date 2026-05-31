//! Implements `Serializable`` for arbitrary tuples of various sizes.
//!
//! Currently supports tuples of size 2 and 3.

use crate::{
    error::DeserializeError,
    error::SerializeError,
    serializable::Serializable,
    traits::{Read, Write},
};
use taped::Tape;
use pastey::paste;

/// Implements Serializable for a tuple type.
///
/// This macro takes a list of generic type identifiers (e.g., T, U, V).
macro_rules! impl_tuple_serializer {
    ($($T:ident),*) => { paste! {
        impl<$($T: Serializable),*> Serializable for ($($T),*) {
            fn write_to(&self, dest: &mut Vec<u8>) -> Result<(), SerializeError> {
                let ($([<$T:lower>]),*) = self;
                $(
                    dest.write([<$T:lower>])?;
                )*
                Ok(())
            }

            fn read_from(src: &mut Tape<'_, u8>) -> Result<Self, DeserializeError> {
                Ok(($(
                    src.read::<$T>()?
                ),*))
            }
        }
    }};
}

impl_tuple_serializer!(T, U);
impl_tuple_serializer!(T, U, V);
