use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, Meta, NestedMeta};

macro_rules! macro_error {
    ($msg:literal) => {
        quote::quote! {

            compile_error!($msg);
        }
        .into()
    };
}

#[proc_macro_attribute]
pub fn time_fn(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let fn_decl = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as AttributeArgs);

    let mut return_time = false;

    for arg in args {
        match arg {
            NestedMeta::Meta(meta) => {
                match meta {
                    Meta::NameValue(value) => {
                        let val = value.lit;
                        let path = value.path.segments.first().unwrap();

                        if path.ident.to_string() != String::from("return_time") {
                            return macro_error!("Invalid attribute argument");
                        } else {
                            match val {
                                Lit::Bool(v) => {
                                    return_time = v.value;
                                }
                                _ => {
                                    return macro_error!("Invalid value, expected boolean");
                                }
                            }
                        }
                    }
                    _ => (),
                };
            }
            _ => {
                return macro_error!("Did not expect unnamed arguments");
            }
        }
    }

    let import_time = if cfg!(feature = "std") {
        quote! {use std::time::SystemTime;}
    } else {
        quote! {}
    };

    let ident = &fn_decl.sig.ident;
    let async_key = &fn_decl.sig.asyncness;
    let fn_args = &fn_decl.sig.inputs;
    let fn_return = if return_time {
        quote! {-> u128}
    } else {
        let return_type = &fn_decl.sig.output;
        quote! {#return_type}
    };
    let fn_generics = {
        let generics = &fn_decl.sig.generics;

        if generics.lt_token.is_none() || generics.gt_token.is_none() {
            TokenStream::new()
        } else {
            quote! {<#generics>}
        }
    };
    let fn_body = &fn_decl.block;

    quote! {
        #async_key fn #ident #fn_generics (#fn_args) #fn_return {
            #import_time
            let start_time = SystemTime::now();
            let return_val = {#fn_body};
            let end_time = SystemTime::now();
            let time_taken = end_time.duration_since(start_time).expect("time went backwards");

            println!("{}", time_taken.as_nanos());

            if #return_time {
                time_taken.as_nanos()
            } else {
                return_val
            }
        }
    }
    .into()
}
