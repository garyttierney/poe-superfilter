//! A preprocessor for Path of Exile loot filters that adds variables, mixins, arithmetics and lots
//! of other useful things to GGG's loot filter syntax and compiles filters
//! written in the extended syntax down to a pure loot filter that can be used
//! in the game.

#[macro_use] extern crate lazy_static;
extern crate regex;

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
    let filter = filter::parse_Filter(tokens.into_iter());
    println!("{:?}", filter);
    // TODO: return actual compiled result
    Box::new(String::from(""))
}
