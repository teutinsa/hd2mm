use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input,
    FnArg,
    Ident,
    ItemFn,
    Pat
};

#[proc_macro_attribute]
pub fn trace_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
	if !attr.is_empty() {
		panic!("`trace_fn` takes no arguments");
	}

    let input = parse_macro_input!(item as ItemFn);
    let fn_name = input.sig.ident.to_string();

    let arg_names: Vec<String> = input.sig.inputs.iter().filter_map(|arg| {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                Some(pat_ident.ident.to_string())
            } else {
                None
            }
        } else {
            None
        }
    }).collect();
    
    let trace_format = format!(
        "{}({})",
        fn_name,
        arg_names
            .iter()
            .map(|name| format!("{}: {{:?}}", name))
            .collect::<Vec<String>>()
            .join(", ")
    );

    let arg_idents: Vec<Ident> = arg_names
        .iter()
        .map(|name| Ident::new(name, Span::call_site()))
        .collect();

    let fn_vis = &input.vis;
    let fn_sig = &input.sig;
    let fn_block = &input.block;
    let output = quote! {
        #fn_vis #fn_sig {
            log::trace!(#trace_format, #(#arg_idents),*);
            #fn_block
        }
    };

    TokenStream::from(output)
}