use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Ident, Fields, Type, ItemStruct};

pub fn generate(input: ItemStruct) -> TokenStream
{
    let name: Ident = input.ident.clone();
    let fields = &input.fields;

    let parsed_type: Option<Type> = match fields {
        Fields::Unnamed(un) => 
            if un.unnamed.len() != 1 {
                None
            }
            else {
                Some(un.unnamed[0].ty.clone())
            },
        _ => None
    };

    let inner_type = match parsed_type {
        Some(pt) => pt,
        None => return quote! {
            compile_error!(r#"IdLike can only be derived for single member tuple structs"#);
        }
    };

    quote_spanned! { input.span() =>
        impl nicole::IdLike for #name
        {
            fn null() -> Self { Self((0 as #inner_type).wrapping_sub(1)) }
        }

        impl From<#name> for usize
        {
            fn from(x: #name) -> usize { x.0 as usize }
        }

        impl From<usize> for #name
        {
            fn from(x: usize) -> Self { Self(x as #inner_type) }
        }
    }
}
