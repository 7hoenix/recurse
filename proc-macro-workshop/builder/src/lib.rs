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
    let attrs: proc_macro2::TokenStream = ast.attrs.into_iter().map(|cake| { quote! { #cake: None, }}).collect();
    // for attr in &ast.attrs {
    //     attrs.append(quote! { #attr });
    // };


    let expanded = quote! {
        pub struct #as_ident {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        impl #module_ident {
            // fn executable (&mut self, executable: String) -> &mut Self {
            //     self.executable = Some(executable);
            //     self
            // }

            // fn args (&mut self, args: Vec<String>) -> &mut Self {
            //     self.args = Some(args);
            //     self
            // }

            // fn env (&mut self, env: Vec<String>) -> &mut Self {
            //     self.env = Some(env);
            //     self
            // }

            // fn current_dir (&mut self, current_dir: String) -> &mut Self {
            //     self.current_dir = Some(current_dir);
            //     self
            // }

            pub fn builder() -> #as_ident {
                #as_ident {
                    #attrs
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
