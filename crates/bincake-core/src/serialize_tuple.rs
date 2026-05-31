//! Implements `Serialize`` for arbitrary tuples of various sizes.
//!
//! Currently supports tuples of size 2 and 3.

use pastey::paste;
use taped::Tape;

use crate::{DecodeError, EncodeError, Read, Serialize, Write};

/// Implements Serialize for a tuple type.
///
/// This macro takes a list of generic type identifiers (e.g., T, U, V).
macro_rules! impl_serialize_tuple {
    ($($T:ident),*) => { paste! {
        impl<$($T: Serialize),*> Serialize for ($($T),*) {
            fn encode(&self, dest: &mut Vec<u8>) -> Result<(), EncodeError> {
                let ($([<$T:lower>]),*) = self;
                $(
                    dest.write([<$T:lower>])?;
                )*
                Ok(())
            }

            fn decode(src: &mut Tape<'_, u8>) -> Result<Self, DecodeError> {
                Ok(($(
                    src.read::<$T>()?
                ),*))
            }
        }
    }};
}

impl_serialize_tuple!(T, U);
impl_serialize_tuple!(T, U, V);
