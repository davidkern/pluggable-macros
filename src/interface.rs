use crate::proc_macro::TokenStream;

use crate::interface_method::InterfaceMethod;
use quote::quote;
use syn::{braced, parenthesized, parse_macro_input, Ident, Lit, ReturnType, Token, Type};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;

mod keyword {
    syn::custom_keyword!(interface);
}

pub struct Interface {
    pub interface_kw: keyword::interface,
    pub name: Ident,
    pub colon_tok: Token![:],
    pub iid: Lit,
    pub methods: Punctuated<InterfaceMethod, Token![;]>,
}

impl Parse for Interface {
    fn parse(input: ParseStream) -> Result<Self> {
        let body;

        let interface_kw = input.parse()?;
        let name = input.parse()?;
        let colon_tok = input.parse()?;
        let iid = input.parse()?;
        
        braced!(body in input);
        let methods = Punctuated::<InterfaceMethod, Token![;]>::parse_terminated(&body)?;

        Ok(Interface {
            interface_kw,
            name,
            colon_tok,
            iid,
            methods,
        })
    }
}
