//! Defines convenience traits and macros for serialization and deserialization,
//! and implements them for source and destination types.

use taped::Tape;

use crate::{
    error::{DeserializeError, SerializeError},
    serializable::Serializable,
};

pub trait Read {
    fn read<T: Serializable>(&mut self) -> Result<T, DeserializeError>;
}

pub trait Write {
    fn write<T: Serializable>(&mut self, value: &T) -> Result<(), SerializeError>;
}

impl Read for Tape<'_, u8> {
    fn read<T: Serializable>(&mut self) -> Result<T, DeserializeError> {
        T::read_from(self)
    }
}

impl Write for Vec<u8> {
    fn write<T: Serializable>(&mut self, value: &T) -> Result<(), SerializeError> {
        T::write_to(value, self)
    }
}
