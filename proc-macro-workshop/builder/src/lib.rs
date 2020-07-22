extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::{quote};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let module_ident = &ast.ident;
    let with_builder = format!("{}Builder", module_ident);
    let as_ident = syn::Ident::new(&with_builder, module_ident.span());

    let expanded = quote! {
        pub struct #as_ident {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        impl #module_ident {
            pub fn builder() -> #as_ident {
                #as_ident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
