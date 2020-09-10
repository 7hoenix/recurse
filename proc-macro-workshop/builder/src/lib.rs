extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Error, parse_macro_input, DeriveInput, Attribute, Data, DataStruct, Fields, FieldsNamed, Field, Type, PathSegment, Path, Result};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::{Punctuated, Pair};
use syn::token::{Colon2};
use quote::{quote};

#[derive(Debug)]
struct MyMacroInput {
    ident: syn::Ident,
}

impl Parse for MyMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: syn::Ident = input.parse()?;
        Err(Error::new(input.span(), "WE FAILED"))
    }
}

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    eprintln!("HELLOOOOOOOOOOOOOOOOOOOOO");
    let ast = parse_macro_input!(input as MyMacroInput);
    eprintln!("****************************************");
    eprintln!("AST: {:#?}", ast);
    eprintln!("****************************************");
    // let noned_attrs: proc_macro2::TokenStream = retrieve_attrs_typed_or_untyped(&ast, None); // Will return `arg: None`
    // let typed_attrs: proc_macro2::TokenStream = retrieve_attrs_typed_or_untyped(&ast, Some("doesntmatt")); // Will return `arg: Option<S>

    let module_ident = &ast.ident;
    // let with_builder = format!("{}Builder", module_ident);
    // let module_name_as_ident = syn::Ident::new(&with_builder, module_ident.span());

    // let expanded = quote! {
    //     pub struct #module_name_as_ident {
    //         #typed_attrs
    //     }

    //     impl #module_ident {
    //         pub fn builder() -> #module_name_as_ident {
    //             #module_name_as_ident {
    //                 #noned_attrs
    //             }
    //         }
    //     }
    // };
    let expanded = quote! {
        pub struct #module_ident {
            // #typed_attrs
        }
        // input
    };
    return proc_macro::TokenStream::from(expanded)
}


fn retrieve_attrs_typed_or_untyped(ast: &DeriveInput, flag: Option<&str>) -> proc_macro2::TokenStream {
    match &ast.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(attrs) => {
                    let vals: proc_macro2::TokenStream = attrs.named.pairs().map(|puncuated| {
                        match puncuated {
                            Pair::Punctuated(field, _) => {
                                let field_ident = extract_ident(field);
                                let field_type = extract_type(field);

                                match flag {
                                    None => {
                                        let expanded = quote! {
                                           #field_ident: None,
                                        };
                                        proc_macro2::TokenStream::from(expanded)
                                    },
                                    Some(_) => {
                                        let expanded = quote! {
                                           #field_ident: Option<#field_type>,
                                        };
                                        proc_macro2::TokenStream::from(expanded)
                                    }
                                }

                            },
                            Pair::End(_) => {
                                let expanded = quote! {
                                };
                                let cake: proc_macro2::TokenStream = proc_macro2::TokenStream::from(expanded);
                                cake
                            }
                        }
                    }).collect::<proc_macro2::TokenStream>();
                    vals
                },
                // Fields::Named
                _ => {
                    panic!("unhandled");
                }
            }
        },
        // Data::Struct
        _ => {
            panic!("unhandled");
        }
    }
}

fn extract_ident(field: &Field) -> proc_macro2::TokenStream {
    match &field.ident {
        Some(attr_ident) => {
            let expanded = quote! {
                #attr_ident
            };
            let cake: proc_macro2::TokenStream = proc_macro2::TokenStream::from(expanded);
            cake
        },
        None => {
            let expanded = quote! {
            };
            let cake: proc_macro2::TokenStream = proc_macro2::TokenStream::from(expanded);
            cake
        }
    }
}

fn extract_type(field: &Field) -> proc_macro2::TokenStream {
    match &field.ty {
        Type::Path(type_path) => {
            let maybe_ident = type_path.path.get_ident();
            match maybe_ident {
                None => {
                   panic!("unhandled");
                },
                Some(id) => {
                    let expanded = quote! {
                        #id
                    };
                    proc_macro2::TokenStream::from(expanded)
                }
            }
        },
        _ => {
           panic!("unhandled");
        }
    }
}
