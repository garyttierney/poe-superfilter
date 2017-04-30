//! This is a support library for poe-superfilter, it contains procedural macros needed in it.

#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use syn::{DeriveInput, Variant};
use proc_macro::TokenStream;

fn impl_match_variants<F>(ast: &DeriveInput, gen_code: F) -> quote::Tokens
    where F: Fn(&Variant) -> quote::Tokens {
    if let &syn::Body::Enum(ref variants) = &ast.body {
        let name = &ast.ident;
        let mut impl_variants = quote::Tokens::new();
        for v in variants {
            let v_ident = &v.ident;
            let code = gen_code(&v);
            impl_variants.append(quote! {
                #name::#v_ident #code,
            });
        }

        impl_variants
    } else {
        panic!("This derive only works for enums")
    }
}

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
    let transform_variants = impl_match_variants(&ast, |_| {
        quote! { (ref n) => { n.transform(ctx) } }
    });

    let location_variants = impl_match_variants(&ast, |_| {
        quote! { (ref n) => { n.location() } }
    });

    quote! {
        impl Transform for #name {
            fn transform(&self, ctx: TransformContext)
                -> Result<Option<TransformedNode>, CompileErr> {
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
            fn render(&self, ctx: RenderContext, buf: &mut Write) -> Result<(), CompileErr> {
                match *self {
                    #render_variants
                }
            }
        }
    }
}

/// Macro for custom derive of the TransformResult Trait
#[proc_macro_derive(InnerScopeValue)]
pub fn inner_scope_value(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = impl_inner_scope_value(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_inner_scope_value(ast: &DeriveInput) -> quote::Tokens {
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
            fn try_add(self, other: Self) -> CompileResult<Self> {
                match self {
                    #add_variants
                }
            }

            fn try_sub(self, other: Self) -> CompileResult<Self> {
                match self {
                    #sub_variants
                }
            }

            fn try_mul(self, other: Self) -> CompileResult<Self> {
                match self {
                    #mul_variants
                }
            }

            fn try_div(self, other: Self) -> CompileResult<Self> {
                match self {
                    #div_variants
                }
            }

            fn try_cmp(&self, other: Self) -> CompileResult<Ordering> {
                match *self {
                    #cmp_variants
                }
            }

            fn try_eq(&self, other: Self) -> CompileResult<bool> {
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