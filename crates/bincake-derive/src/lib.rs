mod serialize_derive;

use proc_macro::TokenStream;

/// Derive macro for automatically implementing the `Serializable` trait.
///
/// Works with `struct`s and `enum`s. For `enum`s, uses a `u8` discriminant prefix.
///
/// # Example
///
/// ```rust
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
pub fn derive_serializable(input: TokenStream) -> TokenStream {
    serialize_derive::derive_serialize(input)
}
