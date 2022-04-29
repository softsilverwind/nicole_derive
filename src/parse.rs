use syn::{
    parse::{Parse, ParseStream},
    Type, Ident, parenthesized
};

pub struct ParseInfo
{
    pub name: Ident,
    pub inner_type: Type
}

impl Parse for ParseInfo
{
    fn parse(input: ParseStream) -> syn::Result<Self>
    {
        let name: Ident = input.parse()?;
        let content;
        let _ = parenthesized!(content in input);
        let inner_type: Type = content.parse()?;
        Ok(ParseInfo {
            name, inner_type
        })
    }
}
