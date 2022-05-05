use syn::{parse_macro_input, ItemStruct};
use quote::quote;

use proc_macro::TokenStream;

mod gen;

#[proc_macro_derive(IdLike)]
pub fn gen_strategies(input: TokenStream) -> TokenStream
{
    let preset: ItemStruct = parse_macro_input!(input);

    let strategies = gen::generate(preset);

    let ret = quote!{
        #strategies
    };

    ret.into()
}
