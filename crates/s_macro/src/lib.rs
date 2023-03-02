extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

struct ScriptMain {
    block: syn::ExprBlock,
}

impl syn::parse::Parse for ScriptMain {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            block: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn script_main(tokens: TokenStream) -> TokenStream {
    let block = parse_macro_input!(tokens as ScriptMain).block;

    quote! {
        use sysinfo::{System, SystemExt};
        use xshell::{cmd, Shell};
        use lexopt::{Parser,prelude::*,Arg};

        pub fn main(sh: &Shell, sys: &System,args: &mut Parser) -> anyhow::Result<()> {
            #block

            Ok(())
        }


    }
    .into()
}

#[proc_macro_attribute]
pub fn test_attribute(args: TokenStream, tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as syn::ItemFn);
    let sig = &input.sig;
    let body = &input.block;
    let output = &sig.output;

    let time_ms = inner_get_val(&parse_macro_input!(args as syn::AttributeArgs));

    inner_check(&input);

    return quote! {
        #sig {
            fn inner() #output #body

            println!("{}",#time_ms);

            inner()
        }
    }
    .into();

    fn inner_get_val(args: &syn::AttributeArgs) -> u64 {
        if args.len() != 1 {
            panic!("Only one integer expected. Example: #[timeout(10)]");
        }

        match &args[0] {
            syn::NestedMeta::Lit(lit) => match lit {
                syn::Lit::Int(int) => int.base10_parse::<u64>().expect("Integer expected"),
                _ => {
                    panic!("Integer as timeout in ms expected. Example: #[timeout(10)]");
                }
            },

            syn::NestedMeta::Meta(_) => {
                panic!("Integer expected. Example: #[timeout(10)]");
            }
        }
    }

    fn inner_check(tokens: &syn::ItemFn) {
        for attribute in &tokens.attrs {
            let meta = attribute.parse_meta();
            match meta {
                Ok(m) => match m {
                    syn::Meta::Path(p) => {
                        let identifier = p.get_ident().expect("Expected identifier!");
                        if identifier == "timeout" {
                            panic!("Timeout attribute is only allowed once");
                        }
                    }
                    syn::Meta::List(ml) => {
                        let identifier = ml.path.get_ident().expect("Expected identifier!");
                        if identifier == "timeout" {
                            panic!("Timeout attribute is only allowed once");
                        }
                    }
                    syn::Meta::NameValue(nv) => {
                        let identifier = nv.path.get_ident().expect("Expected identifier!");
                        if identifier == "timeout" {
                            panic!("Timeout attribute is only allowed once");
                        }
                    }
                },
                Err(e) => panic!("Could not determine meta data. Error {}.", e),
            }
        }
    }
}
