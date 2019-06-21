use crate::proc_macro::TokenStream;

use quote::quote;
use syn::{braced, parenthesized, parse_macro_input, Ident, Lit, ReturnType, Token, Type};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;

pub struct InterfaceMethodArg {
    pub name: Ident,
    pub colon_tok: Token![:],
    pub ty: Type,
}

impl Parse for InterfaceMethodArg {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        let colon_tok = input.parse()?;
        let ty = input.parse()?;

        Ok(InterfaceMethodArg {
            name,
            colon_tok,
            ty,
        })
    }
}
