use proc_macro::TokenStream;
use syn::*;

#[proc_macro_attribute]
pub fn deco(attr: TokenStream, func: TokenStream) -> TokenStream {
    let func = func.into();
    let decorator: Ident = syn::parse(attr).expect("attribute must be a function name");
    let item_fn: ItemFn = syn::parse(func).expect("Input is not a function");
    let vis = &item_fn.vis;
    let ident = &item_fn.sig.ident;
    let block = &item_fn.block;

    let inputs = item_fn.sig.inputs;
    let output = item_fn.sig.output;

    let input_values: Vec<_> = inputs
        .iter()
        .map(|arg| match arg {
            &FnArg::Typed(ref val) => &val.pat,
            _ => unimplemented!("#[deco] cannot be used with associated function"),
        })
        .collect();

    let caller = quote::quote! {
        #vis fn #ident(#inputs) #output {
            let f = #decorator(deco_internal);
            return f(#(#input_values,) *);

            fn deco_internal(#inputs) #output #block
        }
    };
    caller.into()
}
