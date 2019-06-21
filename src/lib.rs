#![recursion_limit="128"]
#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

mod interface;
mod interface_method;
mod interface_method_arg;

use self::proc_macro::TokenStream;

use quote::quote;
use syn::{braced, parenthesized, parse_macro_input, Ident, Lit, ReturnType, Token, Type};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;

#[proc_macro]
pub fn pluggable(input: TokenStream) -> TokenStream {
    let interface::Interface {
        interface_kw,
        name,
        colon_tok,
        iid,
        methods,
    } = parse_macro_input!(input as interface::Interface);

    let expanded = quote! {
        // Function call virtual table
        pub struct Vtbl<TComponent> {
            pub component_vtbl: ::pluggable::IComponentVtbl,
            // $(
            //     pub $method_name: fn(component: &TComponent),
            // )+
            _component_marker: std::marker::PhantomData<TComponent>,
        }

        impl<TComponent> AsRef<::pluggable::IComponentVtbl> for Vtbl<TComponent> {
            fn as_ref(&self) -> &::pluggable::IComponentVtbl {
                &self.component_vtbl
            }
        }

        impl<TComponent> ::pluggable::IID for Vtbl<TComponent> {
            // The interface identifier
            const IID: u128 = #iid;
        }

        pub struct #name(::pluggable::Interface<Vtbl<::pluggable::OpaqueComponent>>);

        // Function calls forward to the implementation
        impl #name {
            pub fn get_interface<TVtbl>(&self) -> Option<::pluggable::Interface<TVtbl>>
                where TVtbl: ::pluggable::IID + AsRef<::pluggable::IComponentVtbl>
            {
                unsafe {
                    match (self.0.vtbl.component_vtbl.get_interface)(&*self.0.component, TVtbl::IID) {
                        Some(interface) => Some(::std::mem::transmute(interface)),
                        None => None,
                    }
                }
            }

            // $(
            //     pub fn $method_name(&self) {
            //         unsafe { (self.vtbl.$method_name)(&*self.component) }
            //     }
            // )*
        }
    };

    TokenStream::from(expanded)
}
