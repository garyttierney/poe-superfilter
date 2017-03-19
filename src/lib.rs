//! A preprocessor for Path of Exile loot filters that adds variables, mixins, arithmetics and lots
//! of other useful things to GGG's loot filter syntax and compiles filters
//! written in the extended syntax down to a pure loot filter that can be used
//! in the game.

#![feature(rustc_private)]
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate arena;

use arena::TypedArena;

#[allow(dead_code)]
mod filter;

#[allow(dead_code)]
pub mod ast;

#[allow(dead_code)]
mod tok;

#[allow(dead_code)]
mod translate;

/// Compiles a complete filter into vanilla loot filter syntax
pub fn compile(contents: &str) -> Box<String> {
    let tokens = Box::new(tok::tokenize(contents));
    for tok in tokens.iter() {
        println!("{:?}", tok);
    }
    let ast_arena = TypedArena::new();
    let filter = filter::parse_Filter(&ast_arena, tokens.into_iter());
    let transformed_ast_arena = TypedArena::new();
    let transformed_tree = filter.unwrap().transform(&transformed_ast_arena);
    println!("{:?}", transformed_tree);
    // TODO: return actual compiled result
    Box::new(String::from(""))
}
