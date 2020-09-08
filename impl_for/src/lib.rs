extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Ident, ItemStruct, Meta, NestedMeta};

#[proc_macro_attribute]
pub fn impl_for(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);

    let item = parse_macro_input!(item as ItemStruct);
    let struct_name = item.ident.clone();

    let impls = args
        .iter()
        .map(|arg| match arg {
            NestedMeta::Meta(meta) => match meta {
                Meta::Path(path) => path.get_ident(),
                _ => None,
            },
            _ => None,
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect::<Vec<&Ident>>();

    let output = quote! {
        #(impl From<#struct_name> for #impls {
            fn from(item: #struct_name) -> Self {
                Self::#struct_name(item)
            }
        })*
        // reprint the existing struct this was called on
        #item
    };

    output.into()
}
