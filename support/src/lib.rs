//! This is a support library for poe-superfilter, it contains procedural macros needed in it.

extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use syn::{DeriveInput};
use proc_macro::TokenStream;

/// Macro for custom derive of the Transform Trait
#[proc_macro_derive(InnerTransform)]
pub fn inner_transform(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = impl_transform(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_transform(ast: &DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let mut variant_tokens = quote::Tokens::new();
    match &ast.body {
        &syn::Body::Enum(ref variants) => {
            for v in variants {
                let v_ident = &v.ident;
                variant_tokens.append(quote! {
                    #name::#v_ident(ref n) => { n as &Transform },
                });
            }
        }
        _ => panic!("This derive only works for enums")
    };

    let tokens = quote! {
        impl <'a> Deref for #name<'a> {
            type Target = Transform<'a> + 'a;
            fn deref(&self) -> &Self::Target {
                match *self {
                    #variant_tokens
                }
            }
        }
    };

    tokens
}

/// Macro for custom derive of the TransformResult Trait
#[proc_macro_derive(InnerTransformResult)]
pub fn inner_transform_result(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = impl_transform_result(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_transform_result(ast: &DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let mut variant_tokens = quote::Tokens::new();
    match &ast.body {
        &syn::Body::Enum(ref variants) => {
            for v in variants {
                let v_ident = &v.ident;
                variant_tokens.append(quote! {
                    #name::#v_ident(ref n) => { n as &TransformResult },
                });
            }
        }
        _ => panic!("This derive only works for enums")
    };

    let tokens = quote! {
        impl <'a> Deref for #name<'a> {
            type Target = TransformResult + 'a;
            fn deref(&self) -> &Self::Target {
                match *self {
                    #variant_tokens
                }
            }
        }
    };

    tokens
}