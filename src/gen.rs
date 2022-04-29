use proc_macro2::TokenStream;
use quote::quote_spanned;

use crate::parse::ParseInfo;

pub fn generate(input: ParseInfo) -> TokenStream
{
    let name = input.name;
    let inner_type = input.inner_type;

    quote_spanned! { name.span() =>
        impl nicole::IdLike for #name
        {
            fn null(&self) -> Self { Self(0.wrapping_sub(1)) }
        }

        impl Into<usize> for #name
        {
            fn into(self) -> usize { self.0 as usize }
        }

        impl From<usize> for #name
        {
            fn from(x: usize) -> Self { Self(x as #inner_type) }
        }
    }
}
