//! A preprocessor for Path of Exile loot filters that adds variables, mixins, arithmetics and lots
//! of other useful things to GGG's loot filter syntax and compiles filters
//! written in the extended syntax down to a pure loot filter that can be used
//! in the game.

#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
mod filter;
#[allow(dead_code)]
mod ast;
#[allow(dead_code)]
mod tok;

fn main() {
    let mut file = File::open("test_filter.filter").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{:?}", contents);
    let tokens = Box::new(tok::tokenize(&contents));
    for tok in tokens.iter() {
        println!("{:?}", tok);
    }
    let filter = filter::parse_Filter(tokens.into_iter());
    println!("{:?}", filter);
}
