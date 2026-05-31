mod error;
mod num;
mod serializable;
mod traits;
mod tuple_n;
mod vec_n;

/// The type used to represent the length of a string when serialized.
pub type StrLen = u32;

/// The size of the type used to represent the length of a string when serialized.
pub const STR_LEN_SIZE: usize = std::mem::size_of::<StrLen>();

pub use self::{error::*, serializable::*, traits::*, vec_n::*};

#[cfg(test)]
mod tests {
    use taped::Tape;

    use crate::error::{DeserializeError, SerializeError};
    use crate::serializable::Serializable;
    use crate::traits::{Read, Write};
    use crate::vec_n::{Vec8, Vec16, Vec32};
    use crate::write_all;

    // Helper function to round-trip test
    fn round_trip<T: Serializable + PartialEq + std::fmt::Debug>(value: T) {
        let mut buffer = vec![];
        buffer.write(&value).expect("Failed to write");

        let mut src = Tape::new(&buffer);
        let decoded = src.read::<T>().expect("Failed to read");

        dbg!(&src);
        assert_eq!(value, decoded, "Round-trip failed");
        assert_eq!(src.rest().len(), 0, "Not all bytes consumed");
    }

    #[test]
    fn test_bool_serialization() {
        round_trip(true);
        round_trip(false);
    }

    #[test]
    fn test_u8_serialization() {
        round_trip(0u8);
        round_trip(127u8);
        round_trip(255u8);
    }

    #[test]
    fn test_u16_serialization() {
        round_trip(0u16);
        round_trip(256u16);
        round_trip(u16::MAX);
    }

    #[test]
    fn test_u32_serialization() {
        round_trip(0u32);
        round_trip(65536u32);
        round_trip(u32::MAX);
    }

    #[test]
    fn test_signed_integers() {
        round_trip(-128i8);
        round_trip(127i8);
        round_trip(-32768i16);
        round_trip(32767i16);
        round_trip(i32::MIN);
        round_trip(i32::MAX);
        round_trip(i64::MIN);
        round_trip(i64::MAX);
        round_trip(i128::MIN);
        round_trip(i128::MAX);
    }

    #[test]
    fn test_floats() {
        round_trip(0.0f32);
        round_trip(-0.0f32);
        round_trip(std::f32::consts::PI);
        round_trip(f32::INFINITY);
        round_trip(f32::NEG_INFINITY);

        round_trip(0.0f64);
        round_trip(std::f64::consts::E);
        round_trip(f64::INFINITY);
        round_trip(f64::NEG_INFINITY);
    }

    #[test]
    fn test_float_nan() {
        let mut buffer = vec![];
        buffer.write(&f32::NAN).unwrap();

        let mut src = Tape::new(&buffer);
        let decoded = src.read::<f32>().unwrap();

        assert!(decoded.is_nan(), "NaN not preserved");
    }

    #[test]
    fn test_string_serialization() {
        round_trip(String::new());
        round_trip("hello".to_string());
        round_trip("Hello, World! 🦀".to_string());
        round_trip("Multi\nLine\nString".to_string());

        // Long string
        let long = "a".repeat(1000);
        round_trip(long);
    }

    #[test]
    fn test_string_utf8() {
        round_trip("日本語".to_string());
        round_trip("Ñoño español".to_string());
        round_trip("🎉🎊✨".to_string());
    }

    #[test]
    fn test_tuples() {
        round_trip((42u32, true));
        round_trip((1u8, 2u16, 3u32));
        round_trip(("hello".to_string(), 123i32));
    }

    #[test]
    fn test_nested_tuples() {
        round_trip(((1u8, 2u8), (3u8, 4u8)));
    }

    #[test]
    fn test_vec8() {
        let empty: Vec8<u32> = Vec8::new(vec![]);
        round_trip(empty);

        let small = Vec8::new(vec![1u32, 2, 3, 4, 5]);
        round_trip(small);

        // Max size for Vec8
        let max_vec8 = Vec8::new(vec![0u8; 255]);
        round_trip(max_vec8);
    }

    #[test]
    fn test_vec16() {
        let empty: Vec16<u32> = Vec16::new(vec![]);
        round_trip(empty);

        let medium = Vec16::new(vec![100u32; 300]);
        round_trip(medium);
    }

    #[test]
    fn test_vec32() {
        let empty: Vec32<u32> = Vec32::new(vec![]);
        round_trip(empty);

        let large = Vec32::new(vec![42u32; 10000]);
        round_trip(large);
    }

    #[test]
    fn test_vec_of_strings() {
        let strings = Vec32::new(vec![
            "hello".to_string(),
            "world".to_string(),
            "🦀".to_string(),
        ]);
        round_trip(strings);
    }

    #[test]
    fn test_nested_vecs() {
        let nested = Vec32::new(vec![
            Vec8::new(vec![1u8, 2, 3]),
            Vec8::new(vec![4, 5]),
            Vec8::new(vec![]),
        ]);
        round_trip(nested);
    }

    #[test]
    fn test_little_endian_encoding() {
        // Verify little-endian byte order
        let mut buffer = vec![];
        buffer.write(&0x12345678u32).unwrap();

        assert_eq!(buffer, vec![0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn test_string_length_prefix() {
        // Verify string is prefixed with u32 length
        let mut buffer = vec![];
        buffer.write(&"abc".to_string()).unwrap();

        // First 4 bytes should be length (3 in little-endian)
        assert_eq!(buffer[0..4], [3, 0, 0, 0]);
        // Next 3 bytes should be the string
        assert_eq!(buffer[4..7], [b'a', b'b', b'c']);
    }

    #[test]
    fn test_vec_length_prefix() {
        let mut buffer = vec![];
        buffer.write(&Vec32::new(vec![1u8, 2, 3])).unwrap();

        // First 4 bytes should be length (3 in little-endian)
        assert_eq!(buffer[0..4], [3, 0, 0, 0]);
        // Next 3 bytes should be the elements
        assert_eq!(buffer[4..7], [1, 2, 3]);
    }

    #[test]
    fn test_empty_buffer_error() {
        let buffer = vec![];
        let mut src = Tape::new(&buffer);

        let result = src.read::<u32>();
        assert!(matches!(result, Err(DeserializeError::Exhausted { .. })));
    }

    #[test]
    fn test_truncated_data() {
        let mut buffer = vec![];
        buffer.write(&42u32).unwrap();

        // Only provide 2 bytes instead of 4
        let truncated = &buffer[0..2];
        let mut src = Tape::new(truncated);

        let result = src.read::<u32>();
        assert!(matches!(result, Err(DeserializeError::Exhausted { .. })));
    }

    #[test]
    fn test_truncated_string() {
        let mut buffer = vec![];
        buffer.write(&"hello".to_string()).unwrap();

        // Truncate the buffer
        let truncated = &buffer[0..6]; // Length prefix + only 2 chars
        let mut src = Tape::new(truncated);

        let result = src.read::<String>();
        assert!(matches!(result, Err(DeserializeError::Exhausted { .. })));
    }

    #[test]
    fn test_invalid_utf8() {
        // Manually construct invalid UTF-8 in string format
        let mut buffer = vec![];
        // Length: 2
        buffer.extend_from_slice(&[2, 0, 0, 0]);
        // Invalid UTF-8 sequence
        buffer.extend_from_slice(&[0xFF, 0xFE]);

        let mut src = Tape::new(&buffer);
        let result = src.read::<String>();

        assert!(matches!(result, Err(DeserializeError::Other { .. })));
    }

    #[test]
    fn test_truncated_vec() {
        let mut buffer = vec![];
        buffer.write(&Vec32::new(vec![1u32, 2, 3])).unwrap();

        // Truncate so there's not enough data for all elements
        let truncated = &buffer[0..8]; // Length prefix + only 1 element
        let mut src = Tape::new(truncated);

        let result = src.read::<Vec32<u32>>();
        assert!(matches!(result, Err(DeserializeError::Exhausted { .. })));
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn test_array_size_overflow() {
        // Test that writing a string larger than u32::MAX fails
        let huge_string = "a".repeat(u32::MAX as usize + 1);
        let mut buffer = vec![];

        let result = buffer.write(&huge_string);
        assert!(matches!(
            result,
            Err(SerializeError::LengthExceedsPrefix { .. })
        ));
    }

    #[test]
    fn test_byte_stream_operations() {
        let data = vec![1, 2, 3, 4, 5];
        let mut src = Tape::new(&data);

        assert_eq!(src.pos, 0);
        assert_eq!(src.rest().len(), 5);

        assert_eq!(src.next(), Some(1));
        assert_eq!(src.pos, 1);
        assert_eq!(src.rest().len(), 4);

        src.pos += 2;
        assert_eq!(src.pos, 3);
        assert_eq!(src.rest().len(), 2);

        assert_eq!(src.next(), Some(4));
        assert_eq!(src.next(), Some(5));
        assert_eq!(src.next(), None);
        assert_eq!(src.rest().len(), 0);
    }

    #[test]
    fn test_multiple_values_in_sequence() {
        let mut buffer = vec![];

        // Write multiple values
        write_all!(buffer;
            &42u32,
            &"hello".to_string(),
            &true,
            &Vec8::new(vec![1u8, 2, 3]),
        )
        .unwrap();

        // Read them back
        let mut src = Tape::new(&buffer);

        let num = src.read::<u32>().unwrap();
        assert_eq!(num, 42);

        let string = src.read::<String>().unwrap();
        assert_eq!(string, "hello");

        let boolean = src.read::<bool>().unwrap();
        assert!(boolean);

        let vec = src.read::<Vec8<u8>>().unwrap();
        assert_eq!(vec.into_inner(), vec![1, 2, 3]);

        assert_eq!(src.rest().len(), 0);
    }

    #[test]
    fn test_complex_structure() {
        // Simulate a complex bytecode structure
        let mut buffer = vec![];

        // Header
        buffer.write(&0xCAFEBABEu32).unwrap();

        // Version
        buffer.write(&(1u16, 0u16)).unwrap();

        // Function table
        let functions = Vec32::new(vec![
            ("main".to_string(), 0u32),
            ("helper".to_string(), 100u32),
        ]);
        buffer.write(&functions).unwrap();

        // Constants
        let constants = Vec16::new(vec!["Hello".to_string(), "World".to_string()]);
        buffer.write(&constants).unwrap();

        // Read it all back
        let mut src = Tape::new(&buffer);

        let magic = src.read::<u32>().unwrap();
        assert_eq!(magic, 0xCAFEBABE);

        let version = src.read::<(u16, u16)>().unwrap();
        assert_eq!(version, (1, 0));

        let funcs = src.read::<Vec32<(String, u32)>>().unwrap();
        assert_eq!(funcs.len(), 2);
        assert_eq!(funcs[0].0, "main");
        assert_eq!(funcs[1].1, 100);

        let consts = src.read::<Vec16<String>>().unwrap();
        assert_eq!(consts.len(), 2);
        assert_eq!(consts[0], "Hello");

        assert_eq!(src.rest().len(), 0);
    }

    #[test]
    fn test_zero_values() {
        round_trip(0u8);
        round_trip(0u16);
        round_trip(0u32);
        round_trip(0i8);
        round_trip(0i16);
        round_trip(0i32);
        round_trip(0i64);
        round_trip(0i128);
        round_trip(0.0f32);
        round_trip(0.0f64);
    }

    #[test]
    fn test_max_values() {
        round_trip(u8::MAX);
        round_trip(u16::MAX);
        round_trip(u32::MAX);
        round_trip(i8::MAX);
        round_trip(i16::MAX);
        round_trip(i32::MAX);
        round_trip(i64::MAX);
        round_trip(i128::MAX);
    }

    #[test]
    fn test_min_values() {
        round_trip(i8::MIN);
        round_trip(i16::MIN);
        round_trip(i32::MIN);
        round_trip(i64::MIN);
        round_trip(i128::MIN);
    }
}
