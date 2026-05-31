use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Index, parse_macro_input};

pub fn derive_serializable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let (write_impl, read_impl) = match &input.data {
        Data::Struct(data_struct) => generate_struct_impl(&data_struct.fields),
        Data::Enum(data_enum) => generate_enum_impl(name, data_enum),
        Data::Union(_) => {
            return syn::Error::new_spanned(name, "Unions cannot be serialized")
                .to_compile_error()
                .into();
        }
    };

    let expanded = quote! {
        impl #impl_generics Serializable for #name #ty_generics #where_clause {
            fn write_to(&self, dest: &mut Vec<u8>) -> Result<(), SerializeError> {
                #write_impl
            }

            fn read_from(src: &mut taped::Tape<'_, u8>) -> Result<Self, DeserializeError> {
                #read_impl
            }
        }
    };

    TokenStream::from(expanded)
}

/// Generates the `write_to` and `read_from` implementations for a `struct`.
pub(crate) fn generate_struct_impl(
    fields: &Fields,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    match fields {
        Fields::Named(fields_named) => {
            let field_names: Vec<_> = fields_named.named.iter().map(|f| &f.ident).collect();
            let write_fields = field_names.iter().map(|name| {
                quote! {
                    self.#name.write_to(dest)?;
                }
            });
            let read_fields = field_names.iter().map(|name| {
                quote! {
                    #name: Serializable::read_from(src)?
                }
            });
            let write_impl = quote! {
                #(#write_fields)*
                Ok(())
            };
            let read_impl = quote! {
                Ok(Self {
                    #(#read_fields),*
                })
            };
            (write_impl, read_impl)
        }
        Fields::Unnamed(fields_unnamed) => {
            let field_count = fields_unnamed.unnamed.len();
            let field_indices: Vec<_> = (0..field_count).map(Index::from).collect();
            let write_fields = field_indices.iter().map(|i| {
                quote! {
                    self.#i.write_to(dest)?;
                }
            });
            let read_fields = (0..field_count).map(|_| {
                quote! {
                    Serializable::read_from(src)?
                }
            });
            let write_impl = quote! {
                #(#write_fields)*
                Ok(())
            };
            let read_impl = quote! {
                Ok(Self(
                    #(#read_fields),*
                ))
            };
            (write_impl, read_impl)
        }
        Fields::Unit => {
            let write_impl = quote! {
                Ok(())
            };
            let read_impl = quote! {
                Ok(Self)
            };
            (write_impl, read_impl)
        }
    }
}

/// Generates the `write_to` and `read_from` implementations for an `enum`.
fn generate_enum_impl(
    name: &syn::Ident,
    data_enum: &syn::DataEnum,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let variant_count = data_enum.variants.len();

    if variant_count > 256 {
        return (
            syn::Error::new_spanned(name, "Enums with more than 256 variants are not supported")
                .to_compile_error(),
            quote! { unreachable!() },
        );
    }

    // Generate write match arms
    let write_variants = data_enum
        .variants
        .iter()
        .enumerate()
        .map(|(disc, variant)| {
            let variant_name = &variant.ident;
            let discriminant = disc as u8;
            match &variant.fields {
                Fields::Named(fields) => {
                    let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                    let write_fields = field_names.iter().map(|fname| {
                        quote! { #fname.write_to(dest)?; }
                    });
                    quote! {
                        Self::#variant_name { #(#field_names),* } => {
                            dest.push(#discriminant);
                            #(#write_fields)*
                        }
                    }
                }
                Fields::Unnamed(fields) => {
                    let field_count = fields.unnamed.len();
                    let field_bindings: Vec<_> = (0..field_count)
                        .map(|i| {
                            syn::Ident::new(&format!("f{}", i), proc_macro2::Span::call_site())
                        })
                        .collect();
                    let write_fields = field_bindings.iter().map(|f| {
                        quote! { #f.write_to(dest)?; }
                    });
                    quote! {
                        Self::#variant_name(#(#field_bindings),*) => {
                            dest.push(#discriminant);
                            #(#write_fields)*
                        }
                    }
                }
                Fields::Unit => {
                    quote! {
                        Self::#variant_name => {
                            dest.push(#discriminant);
                        }
                    }
                }
            }
        });

    // Generate read match arms
    let read_variants = data_enum
        .variants
        .iter()
        .enumerate()
        .map(|(disc, variant)| {
            let variant_name = &variant.ident;
            let discriminant = disc as u8;
            match &variant.fields {
                Fields::Named(fields) => {
                    let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                    let read_fields = field_names.iter().map(|fname| {
                        quote! {
                            #fname: Serializable::read_from(src)?
                        }
                    });
                    quote! {
                        #discriminant => Ok(Self::#variant_name {
                            #(#read_fields),*
                        })
                    }
                }
                Fields::Unnamed(fields) => {
                    let field_count = fields.unnamed.len();
                    let read_fields = (0..field_count).map(|_| {
                        quote! { Serializable::read_from(src)? }
                    });
                    quote! {
                        #discriminant => Ok(Self::#variant_name(
                            #(#read_fields),*
                        ))
                    }
                }
                Fields::Unit => {
                    quote! {
                        #discriminant => Ok(Self::#variant_name)
                    }
                }
            }
        });

    let write_impl = quote! {
        match self {
            #(#write_variants)*
        }
        Ok(())
    };

    let read_impl = quote! {
        let discriminant = u8::read_from(src)?;
        match discriminant {
            #(#read_variants,)*
            _ => Err(DeserializeError::Other {
                pos: src.pos - 1,
                cause: format!("Invalid discriminant {} for enum {}", discriminant, stringify!(#name)),
            })
        }
    };

    (write_impl, read_impl)
}
