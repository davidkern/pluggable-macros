#![feature(proc_macro_diagnostic)]

extern crate proc_macro;
use self::proc_macro::TokenStream;

use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, parenthesized, parse_macro_input, Ident, Lit, Token};

mod keyword {
    syn::custom_keyword!(interface);
}

struct Interface {
    name: Ident,
    iid: Lit,
}

impl Parse for Interface {
    fn parse(input: ParseStream) -> Result<Self> {
        let body;

        input.parse::<keyword::interface>()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let iid: Lit = input.parse()?;
        braced!(body in input);

        Ok(Interface { 
            name,
            iid,
        })
    }
}

#[proc_macro]
pub fn pluggable(input: TokenStream) -> TokenStream {
    let Interface {
        name,
        iid,
    } = parse_macro_input!(input as Interface);

    let expanded = quote! {
        fn #name () -> i32 { 42 }
    };

    TokenStream::from(expanded)
}
