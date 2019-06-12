#![recursion_limit="128"]
#![feature(proc_macro_diagnostic)]

extern crate proc_macro;
use self::proc_macro::TokenStream;

use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, parse_macro_input, Ident, Lit, Token};

mod keyword {
    syn::custom_keyword!(interface);
}

struct Interface {
    name: Ident,
    iid: Lit,
}

impl Parse for Interface {
    fn parse(input: ParseStream) -> Result<Self> {
        let _body;

        input.parse::<keyword::interface>()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let iid: Lit = input.parse()?;
        braced!(_body in input);

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
