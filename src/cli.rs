//! A preprocessor for Path of Exile loot filters that adds variables, mixins, arithmetics and lots
//! of other useful things to GGG's loot filter syntax and compiles filters
//! written in the extended syntax down to a pure loot filter that can be used
//! in the game.
//!
//! This crate is a CLI application that exposes the functionality of the compiler library.

extern crate superfilter;
extern crate clap;

use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::Path;
use superfilter::ast::transform::RenderConfig;
use std::time::SystemTime;
use clap::{Arg, App};

pub fn main() {
    let start_time = SystemTime::now();

    let matches = App::new("PoE Superfilter Compiler")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Stefan Kaufhold <cere@fastmail.fm>")
        .arg(Arg::with_name("PATH")
            .help("Path of the input file")
            .required(true)
            .index(1))
        .arg(Arg::with_name("pretty")
            .help("Include indentation and other formatting in the output")
            .short("p")
            .long("pretty"))
        .arg(Arg::with_name("output")
            .help("Output file. If this option is omitted, the output will be printed to the console.")
            .short("o")
            .long("output")
            .takes_value(true)
            .value_name("FILE"))
        .get_matches();

    let input_path = Path::new(matches.value_of("PATH").unwrap()).to_owned();

    let mut file = File::open(&input_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let base_path = input_path
        .parent()
        .unwrap()
        .to_owned();

    let render_config = RenderConfig {
        pretty: matches.is_present("pretty"),
        indent_str: "    ",
        base_path: base_path
    };

    let result = match matches.value_of("output") {
        Some(output) => superfilter::compile(&contents, input_path, &mut File::create(output).unwrap(), &render_config),
        _ => superfilter::compile(&contents, input_path, &mut io::stdout(), &render_config)
    };

    let compile_time = start_time.elapsed().unwrap();

    match result {
        Ok(_) => {
            println!("Compilation successful ({}.{}s)", compile_time.as_secs(), compile_time.subsec_nanos() / 100000);
        }
        Err(err) => {
            println!("Compilation failed:");
            println!("{}", err)
        }
    }
}
