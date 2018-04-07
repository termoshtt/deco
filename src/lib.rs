#![feature(proc_macro)]
#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::*;

#[proc_macro_attribute]
pub fn deco(attr: TokenStream, func: TokenStream) -> TokenStream {
    let func = func.into();
    let attr = parse_attr(attr);
    let item_fn: ItemFn = syn::parse2(func).expect("Input is not a function");
    let vis = &item_fn.vis;
    let ident = &item_fn.ident;
    let block = &item_fn.block;

    let decl: FnDecl = *item_fn.decl;
    let inputs = &decl.inputs;
    let output = &decl.output;

    let input_values: Vec<_> = inputs
        .iter()
        .map(|arg| match arg {
            &FnArg::Captured(ref val) => &val.pat,
            _ => unreachable!(""),
        })
        .collect();

    let caller = quote!{
        #vis fn #ident(#inputs) #output {
            let f = #attr(deco_internal);
            return f(#(#input_values,) *);

            fn deco_internal(#inputs) #output #block
        }
    };
    caller.into()
}

fn parse_attr(attr: TokenStream) -> Ident {
    let pat: &[_] = &['"', '(', ')', ' '];
    let s = attr.to_string().trim_matches(pat).to_string();
    Ident::from(s)
}
