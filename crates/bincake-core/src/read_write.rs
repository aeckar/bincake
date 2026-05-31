//! Defines convenience traits and macros for serialization and deserialization,
//! and implements them for source and destination types.

use taped::Tape;

use crate::{
    error::{DecodeError, EncodeError},
    serialize::Serialize,
};

pub trait Read {
    fn read<T: Serialize>(&mut self) -> Result<T, DecodeError>;
}

pub trait Write {
    fn write<T: Serialize>(&mut self, value: &T) -> Result<(), EncodeError>;
}

impl Read for Tape<'_, u8> {
    fn read<T: Serialize>(&mut self) -> Result<T, DecodeError> {
        T::decode(self)
    }
}

impl Write for Vec<u8> {
    fn write<T: Serialize>(&mut self, value: &T) -> Result<(), EncodeError> {
        T::encode(value, self)
    }
}
