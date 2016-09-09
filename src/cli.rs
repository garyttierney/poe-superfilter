//! A preprocessor for Path of Exile loot filters that adds variables, mixins, arithmetics and lots
//! of other useful things to GGG's loot filter syntax and compiles filters
//! written in the extended syntax down to a pure loot filter that can be used
//! in the game.
//!
//! This crate is a CLI application that exposes the functionality of the compiler library. 

extern crate superfilter;

use std::fs::File;
use std::io::prelude::*;

/// Compiles one or several files
pub fn main() {
    let mut file = File::open("test_filter.filter").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{:?}", superfilter::compile(&contents));
}
