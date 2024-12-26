//! This is a support library for poe-superfilter, it contains procedural macros needed in it.

#![recursion_limit="128"]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use] extern crate quote;

use quote::ToTokens;
use syn::{DeriveInput, Variant};
use proc_macro2::TokenStream;

fn impl_match_variants<F>(ast: &DeriveInput, gen_code: F) -> TokenStream
    where F: Fn(&Variant) -> TokenStream {
    if let &syn::Data::Enum(ref variants) = &ast.data {
        let name = &ast.ident;
        let mut impl_variants = TokenStream::new();
        for v in &variants.variants {
            let v_ident = &v.ident;
            let code = gen_code(&v);
            let variant = quote! {
                #name::#v_ident #code,
            };

            variant.to_tokens(&mut impl_variants);
        }

        impl_variants
    } else {
        panic!("This derive only works for enums")
    }
}

/// Macro for custom derive of the Transform Trait
#[proc_macro_derive(Transform)]
pub fn inner_transform(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the string representation
    let ast = syn::parse_macro_input!(input);

    // Build the impl
    let gen = impl_transform(&ast);

    // Return the generated impl
    proc_macro::TokenStream::from(gen)
}

fn impl_transform(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let transform_variants = impl_match_variants(&ast, |_| {
        quote! { (ref n) => { n.transform(ctx) } }
    });

    let location_variants = impl_match_variants(&ast, |_| {
        quote! { (ref n) => { n.location() } }
    });

    quote! {
        impl Transform for #name {
            fn transform(&self, ctx: TransformContext)
                -> Result<Option<TransformedNode>> {
                match *self {
                    #transform_variants
                }
            }

            fn location(&self) -> AstLocation {
                match *self {
                    #location_variants
                }
            }
        }
    }
}

/// Macro for custom derive of the TransformResult Trait
#[proc_macro_derive(TransformResult)]
pub fn inner_transform_result(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the string representation
    let ast = syn::parse_macro_input!(input);

    // Build the impl
    let gen = impl_transform_result(&ast);

    // Return the generated impl
    proc_macro::TokenStream::from(gen)
}

fn impl_transform_result(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let return_variants = impl_match_variants(&ast, |_| {
        quote! { (ref n) => { n.return_value() } }
    });

    let render_variants = impl_match_variants(&ast, |_| {
        quote! { (ref n) => { n.render(ctx, buf) } }
    });

    quote! {
        impl TransformResult for #name {
            fn return_value(&self) -> ScopeValue {
                match *self {
                    #return_variants
                }
            }

            /// Renders the output for this node into a writable stream.
            fn render(&self, ctx: RenderContext, buf: &mut dyn Write) -> Result<()> {
                match *self {
                    #render_variants
                }
            }
        }
    }
}

/// Macro for custom derive of the TransformResult Trait
#[proc_macro_derive(InnerScopeValue)]
pub fn inner_scope_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the string representation
    let ast = syn::parse_macro_input!(input);

    // Build the impl
    let gen = impl_inner_scope_value(&ast);

    // Return the generated impl
    proc_macro::TokenStream::from(gen)
}

fn impl_inner_scope_value(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let add_variants = impl_match_variants(&ast, |_| {
        quote! { (v) => { v.try_add(other.try_into()?) } }
    });
    let mul_variants = impl_match_variants(&ast, |_| {
        quote! { (v) => { v.try_mul(other.try_into()?) } }
    });
    let sub_variants = impl_match_variants(&ast, |_| {
        quote! { (v) => { v.try_sub(other.try_into()?) } }
    });
    let div_variants = impl_match_variants(&ast, |_| {
        quote! { (v) => { v.try_div(other.try_into()?) } }
    });
    let cmp_variants = impl_match_variants(&ast, |_| {
        quote! { (ref v) => { v.try_cmp(other.try_into()?) } }
    });
    let eq_variants = impl_match_variants(&ast, |_| {
        quote! { (ref v) => { v.try_eq(other.try_into()?) } }
    });
    let type_name_variants = impl_match_variants(&ast, |_| {
        quote! { (ref v) => { v.type_name() } }
    });

    quote! {
        impl InnerScopeValue for #name {
            fn try_add(self, other: Self) -> Result<Self> {
                match self {
                    #add_variants
                }
            }

            fn try_sub(self, other: Self) -> Result<Self> {
                match self {
                    #sub_variants
                }
            }

            fn try_mul(self, other: Self) -> Result<Self> {
                match self {
                    #mul_variants
                }
            }

            fn try_div(self, other: Self) -> Result<Self> {
                match self {
                    #div_variants
                }
            }

            fn try_cmp(&self, other: Self) -> Result<Ordering> {
                match *self {
                    #cmp_variants
                }
            }

            fn try_eq(&self, other: Self) -> Result<bool> {
                match *self {
                    #eq_variants
                }
            }

            fn type_name(&self) -> &'static str {
                match *self {
                    #type_name_variants
                }
            }
        }
    }
}