//! Defines wrapper types for vectors with sizes of various encodings.
//!
//! These types implement the `Serialize` trait, allowing them to be serialized to and deserialized
//! to a byte buffer with the appropriate size prefix.
//!
//! They also dereference to `Vec<T>` for ease of use.

use pastey::paste;
use taped::Tape;

use crate::{DecodeError, EncodeError, Read, Serialize, Write};

/// Initializes a vector with an 8-bit size.
#[macro_export]
macro_rules! vec8 {
    () => { $crate::Vec8::from(vec![]) };
    ($($x:expr),+ $(,)?) => { $crate::Vec8::from(vec![$($x),+]) };
    ($x:expr; $n:expr) => { $crate::Vec8::from(vec![$x; $n]) };
}

/// Initializes a vector with an 16-bit size.
#[macro_export]
macro_rules! vec16 {
    () => { $crate::Vec16::from(vec![]) };
    ($($x:expr),+ $(,)?) => { $crate::Vec16::from(vec![$($x),+]) };
    ($x:expr; $n:expr) => { $crate::Vec16::from(vec![$x; $n]) };
}

/// Initializes a vector with an 32-bit size.
#[macro_export]
macro_rules! vec32 {
    () => { $crate::Vec32::from(vec![]) };
    ($($x:expr),+ $(,)?) => { $crate::Vec32::from(vec![$($x),+]) };
    ($x:expr; $n:expr) => { $crate::Vec32::from(vec![$x; $n]) };
}

/// Initializes a vector with an 64-bit size.
#[macro_export]
macro_rules! vec64 {
    () => { $crate::Vec64::from(vec![]) };
    ($($x:expr),+ $(,)?) => { $crate::Vec64::from(vec![$($x),+]) };
    ($x:expr; $n:expr) => { $crate::Vec64::from(vec![$x; $n]) };
}

/// Initializes a vector with an 128-bit size.
#[macro_export]
macro_rules! vec128 {
    () => { $crate::Vec128::from(vec![]) };
    ($($x:expr),+ $(,)?) => { $crate::Vec128::from(vec![$($x),+]) };
    ($x:expr; $n:expr) => { $crate::Vec128::from(vec![$x; $n]) };
}

/// Implements Serialize for vecs with sizes of various encodings.
macro_rules! impl_vec_n {
    ($($width:literal),* $(,)?) => { paste! { $(
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct [<Vec $width>]<T>(Vec<T>);

        impl<T> [<Vec $width>]<T> {
            /// Consumes self and returns the inner Vec.
            pub fn into_inner(self) -> Vec<T> {
                self.0
            }
        }

        // From/Into conversions
        impl<T> From<Vec<T>> for [<Vec $width>]<T> {
            fn from(v: Vec<T>) -> Self {
                Self(v)
            }
        }

        impl<T> From<[<Vec $width>]<T>> for Vec<T> {
            fn from(v: [<Vec $width>]<T>) -> Self {
                v.0
            }
        }

        impl<T: Serialize> Serialize for [<Vec $width>]<T> {
            fn encode(&self, dest: &mut Vec<u8>) -> Result<(), EncodeError> {
                let len = self.len();
                #[cfg(target_pointer_width = "64")]
                if len > 0xFFFFFFFFusize {
                    return Err(EncodeError::LengthExceedsPrefix { prefix_size: $width, len })
                }
                len.encode(dest)?;
                for d in self.iter() {
                    dest.write(d)?;
                }
                Ok(())
            }

            fn decode(src: &mut Tape<'_, u8>) -> Result<Self, DecodeError> {
                let len = src.read::<[<u $width>]>()? as usize;
                let mut data = Vec::with_capacity(len);
                for _ in 0..len {
                    data.push(src.read::<T>()?);
                }
                Ok(Self(data))
            }
        }

        impl<T> std::ops::Deref for [<Vec $width>]<T> {
            type Target = Vec<T>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> std::ops::DerefMut for [<Vec $width>]<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    )*}};
}

impl_vec_n!(8, 16, 32, 64, 128);
