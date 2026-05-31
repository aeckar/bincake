mod derive_serialize;

use proc_macro::TokenStream;

use crate::derive_serialize::derive_serialize;

/// Derive macro for automatically implementing the `Serializable` trait.
///
/// Works with `struct`s and `enum`s. For `enum`s, uses a `u8` discriminant prefix.
///
/// # Example
///
/// ```rust
/// use bincake::Serialize;
/// 
/// #[derive(Serialize)]
/// struct MyStruct {
///     name: String,
///     age: u32,
///     active: bool,
/// }
///
/// #[derive(Serialize)]
/// enum MyEnum {
///     Unit,
///     Tuple(u32, String),
///     Struct { x: i32, y: i32 },
/// }
/// ```
#[proc_macro_derive(Serialize)]
pub fn derive_serialize_macro(input: TokenStream) -> TokenStream {
    derive_serialize(input)
}
