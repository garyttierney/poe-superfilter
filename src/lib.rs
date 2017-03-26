//! A preprocessor for Path of Exile loot filters that adds variables, mixins, arithmetics and lots
//! of other useful things to GGG's loot filter syntax and compiles filters
//! written in the extended syntax down to a pure loot filter that can be used
//! in the game.

#![feature(rustc_private)]
#![feature(plugin)]
#![plugin(indoc)]
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate quick_error;

extern crate regex;
extern crate arena;
extern crate lalrpop_util;

use arena::TypedArena;
use ast::transform::TransformResult;
use std::io::Write;

#[allow(dead_code)]
mod filter;

#[allow(dead_code)]
pub mod ast;

#[allow(dead_code)]
mod tok;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
mod scope;

/// Compiles a complete filter into vanilla loot filter syntax
pub fn compile(contents: &str, out_buf: &mut Write) {
    let tokens = Box::new(tok::tokenize(contents));
    let ast_arena = TypedArena::new();
    let filter = filter::parse_Filter(&ast_arena, tokens.into_iter());
    if let Some(transformed_tree) = filter.unwrap().transform().unwrap() {
        transformed_tree.render(out_buf).unwrap();
    }
}
