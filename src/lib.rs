extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(DefaultFields)]
pub fn derive_default_fields(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item);
    let struct_name = input.ident;

    if let Data::Struct(data) = input.data {
        let default_field_getters = data.fields.into_iter().filter_map(|field| {
            if let Some(field_name) = field.ident {
                let field_type = field.ty;
                let func_name = format_ident!("get_default_{}", field_name);
                let quote = quote! {
                    pub fn #func_name () -> #field_type {
                        let default: #struct_name = Default::default();
                        default.#field_name
                    }
                };
                Some(quote)
            } else {
                None
            }
        });

        let output = quote! {
            impl #struct_name {
                #(#default_field_getters)

                *
            }
        };
        output.into()
    } else {
        "".parse().unwrap()
    }
}
