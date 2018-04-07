#![feature(proc_macro)]
#![recursion_limit = "128"]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::*;

#[proc_macro_attribute]
pub fn deco(attr: TokenStream, func: TokenStream) -> TokenStream {
    let func = Function::parse(func);
    let attr = parse_attr(attr);
    let vis = &func.vis;
    let fn_token = &func.fn_token;
    let ident = &func.ident;
    let inputs = &func.inputs;
    let output = &func.output;
    let block = &func.block;

    let input_values = func.input_values();

    let caller = quote!{
        #vis #fn_token #ident(#inputs) #output {
            let f = #attr(deco_internal);
            return f(#(#input_values,) *);

            #fn_token deco_internal(#inputs) #output #block
        }
    };
    caller.into()
}

#[derive(Debug)]
struct Function {
    attrs: Vec<Attribute>,
    ident: Ident,
    vis: Visibility,
    block: Box<Block>,
    unsafety: Option<token::Unsafe>,
    inputs: punctuated::Punctuated<FnArg, token::Comma>,
    output: ReturnType,
    fn_token: token::Fn,
}

impl Function {
    pub fn parse(func: TokenStream) -> Self {
        let ItemFn {
            attrs,
            ident,
            vis,
            block,
            decl,
            unsafety,
            ..
        } = ::syn::parse(func.clone()).unwrap();
        let FnDecl {
            inputs,
            output,
            fn_token,
            ..
        } = { *decl };
        Function {
            attrs,
            ident,
            vis,
            block,
            unsafety,
            inputs,
            output,
            fn_token,
        }
    }

    pub fn input_values(&self) -> Vec<&Pat> {
        self.inputs
            .iter()
            .map(|arg| match arg {
                &FnArg::Captured(ref val) => &val.pat,
                _ => unreachable!(""),
            })
            .collect()
    }
}

fn parse_attr(attr: TokenStream) -> Ident {
    let pat: &[_] = &['"', '(', ')', ' '];
    let s = attr.to_string().trim_matches(pat).to_string();
    Ident::from(s)
}
