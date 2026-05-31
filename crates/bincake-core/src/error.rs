//! Provides types to denote errors during serialization/deserialization.

/// Denotes a fatal error during decoding.
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    #[error("[{pos:#04x}]: Stream is exhausted")]
    Exhausted { pos: usize },

    #[error("[{pos:#04x}]: {cause}")]
    Other { pos: usize, cause: String },
}

/// Denotes a fatal error during encoding.
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum EncodeError {
    /// Denotes that the length of an array is to large to be serialized as a prefix of its elements.
    ///
    /// Here, an array refers to any contiguous sequence of elements.
    ///
    /// This error may be raised when the length of a `String` is too large for a `u32` prefix,
    /// or when the length of a `VecN` is too large for its specified prefix.
    #[error("Length {len} cannot fit in prefix of size {prefix_size} bytes")]
    LengthExceedsPrefix { prefix_size: u8, len: usize },

    #[error("{cause}")]
    Other { cause: String },
}
