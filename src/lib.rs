use anyhow::{bail, Result};
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
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

#[derive(Debug, PartialEq)]
enum DecoratorAttr {
    Ident(Ident),
    Parametric { ident: Ident, args: Vec<Expr> },
}

impl DecoratorAttr {
    fn parse(attr: proc_macro2::TokenStream) -> Result<Self> {
        let mut ident = None;
        let mut args = Vec::new();
        for at in attr {
            match at {
                TokenTree::Ident(id) => {
                    ident = Some(id);
                }
                TokenTree::Group(grp) => {
                    if ident.is_none() {
                        bail!("Invalid token stream");
                    }
                    for t in grp.stream() {
                        if let Ok(expr) = syn::parse2(t.into()) {
                            args.push(expr);
                        }
                    }
                }
                _ => bail!("Invalid token stream"),
            }
        }
        if let Some(ident) = ident {
            if args.is_empty() {
                Ok(DecoratorAttr::Ident(ident))
            } else {
                Ok(DecoratorAttr::Parametric { ident, args })
            }
        } else {
            bail!("Decorator name not found");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parse_attr() -> Result<()> {
        let ts = proc_macro2::TokenStream::from_str("logging").unwrap();
        assert!(matches!(DecoratorAttr::parse(ts)?, DecoratorAttr::Ident(_)));
        Ok(())
    }

    #[test]
    fn parse_attr_parametric_literal() -> Result<()> {
        let ts = proc_macro2::TokenStream::from_str(r#"logging("test.log", 2)"#).unwrap();
        match DecoratorAttr::parse(ts)? {
            DecoratorAttr::Ident(_) => bail!("Failed to parse args"),
            DecoratorAttr::Parametric { args, .. } => {
                assert_eq!(args.len(), 2);
            }
        }
        Ok(())
    }

    #[test]
    fn parse_attr_parametric_variable() -> Result<()> {
        let ts =
            proc_macro2::TokenStream::from_str(r#"logging("test.log", some_variable)"#).unwrap();
        match DecoratorAttr::parse(ts)? {
            DecoratorAttr::Ident(_) => bail!("Failed to parse args"),
            DecoratorAttr::Parametric { args, .. } => {
                assert_eq!(args.len(), 2);
            }
        }
        Ok(())
    }

    #[test]
    fn parse_attr_parametric_expr() -> Result<()> {
        let ts = proc_macro2::TokenStream::from_str(r#"logging("test.log", (1 + 2))"#).unwrap();
        match DecoratorAttr::parse(ts)? {
            DecoratorAttr::Ident(_) => bail!("Failed to parse args"),
            DecoratorAttr::Parametric { args, .. } => {
                assert_eq!(args.len(), 2);
            }
        }
        Ok(())
    }

    #[test]
    fn parse_attr_empty() -> Result<()> {
        let ts = proc_macro2::TokenStream::from_str("").unwrap();
        assert!(DecoratorAttr::parse(ts).is_err());
        Ok(())
    }

    #[test]
    fn parse_attr_invalid() -> Result<()> {
        // inverse order
        let ts = proc_macro2::TokenStream::from_str(r#"("test.log", 2)logging"#).unwrap();
        assert!(DecoratorAttr::parse(ts).is_err());
        Ok(())
    }
}
