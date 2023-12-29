use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemFn, LitStr, AttributeArgs, NestedMeta, FnArg, PatType, Type, Lit};
use syn::parse::Parser;
use quote::{quote, format_ident};
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use syn::__private::TokenStream2;

#[proc_macro_attribute]
pub fn yarn_command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemFn);
    let command_name_map = get_func_name(&input, &attr_args);
    let command_name = input.sig.ident.clone();
    let arg_parsers = parse_args(&input);

    let func_invocation = if !arg_parsers.is_empty() {
        let args = (0..arg_parsers.len())
            .map(|i| format_ident!("arg{}", i))
            .collect::<Vec<_>>();
        quote! { #command_name(#(#args),*); }
    } else {
        quote! { #command_name(); }
    };

    let arg_parsers_lets = arg_parsers
        .iter()
        .enumerate()
        .map(|(i, parser)| {
            let arg = format_ident!("arg{}", i);
            let new_tstream = TokenStream2::from(parser.clone());
            quote! { let #arg = #new_tstream; }
        })
        .collect::<Vec<_>>();

    let registration = quote! {
                COMMAND_REGISTRY.lock().unwrap().insert(
                    String::from(#command_name_map),
                    Box::new(|commands, tokens| {
                        #(#arg_parsers_lets)*
                        #func_invocation
                    })
                );
    };

    let output = quote! {
        #input
        #registration
    };

    TokenStream::from(output)
}

fn get_func_name(input: &ItemFn, attr_args: &AttributeArgs) -> String {
    if let Some(NestedMeta::Lit(Lit::Str(name))) = attr_args.first() {
        name.value()
    } else {
        input.sig.ident.to_string()
    }
}

fn parse_args(input: &ItemFn) -> Vec<TokenStream> {
    let mut arg_parsers = Vec::new();
    for arg in input.sig.inputs.iter() {
        if let FnArg::Typed(PatType { ty, .. }) = arg {
            match &**ty {
                Type::Path(tp) if tp.path.is_ident("String") => arg_parsers.push(
                    quote! {
                    tokens.next().unwrap().to_string()  }
                    .into(),
                ),
                Type::Path(tp) if tp.path.is_ident("i32") => arg_parsers.push(
                    quote! {
                    tokens.next().unwrap().parse::<i32>().unwrap()  }
                    .into(),
                ),
                Type::Reference(tr) => {
                    if let Type::Path(tp) = &*tr.elem {
                        if tp.path.is_ident("Commands") {
                            arg_parsers.push(quote! { commands }.into())
                        } else {
                            panic!(
                                "Unsupported function's argument reference type: {}",
                                tp.path.get_ident().unwrap()
                            );
                        }
                    }
                }
                _ => panic!("Unsupported function's argument type: UNKNOWN"),
            }
        }
    }
    arg_parsers
}
