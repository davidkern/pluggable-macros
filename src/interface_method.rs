use crate::proc_macro::TokenStream;

use crate::interface_method_arg::InterfaceMethodArg;
use quote::quote;
use syn::{braced, parenthesized, parse_macro_input, Ident, Lit, ReturnType, Token, Type};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;

pub struct InterfaceMethod {
    pub fn_tok: Token![fn],
    pub name: Ident,
    pub amp_tok: Token![&],
    pub self_tok: Token![self],
    pub args: Punctuated<InterfaceMethodArg, Token![,]>,
    pub ret_ty: ReturnType,
}

impl Parse for InterfaceMethod {
    fn parse(input: ParseStream) -> Result<Self> {
        let arg_list;

        let fn_tok = input.parse()?;
        let name = input.parse()?;
        
        parenthesized!(arg_list in input);
        let amp_tok = arg_list.parse()?;
        let self_tok = arg_list.parse()?;
        let args = Punctuated::<InterfaceMethodArg, Token![,]>::parse_terminated(&arg_list)?;
        
        let ret_ty = input.parse()?;

        Ok(InterfaceMethod {
            fn_tok,
            name,
            amp_tok,
            self_tok,
            args,
            ret_ty,
        })
    }
}
