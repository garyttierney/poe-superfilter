//! A preprocessor for Path of Exile loot filters that adds variables, mixins, arithmetics and lots
//! of other useful things to GGG's loot filter syntax and compiles filters
//! written in the extended syntax down to a pure loot filter that can be used
//! in the game.
//!
//! This crate is a CLI application that exposes the functionality of the compiler library. 

extern crate superfilter;
extern crate rustc_serialize;
extern crate docopt;

use std::fs::File;
use std::io::prelude::*;
use std::io;
use docopt::Docopt;
use std::path::Path;
use superfilter::ast::transform::RenderConfig;

const USAGE: &'static str = "
PoE Superfilter compiler

Usage:
  superfilter <name> [--output=<file>] [-p]
  superfilter (-h | --help)
  superfilter --version

Options:
  -h --help        Show this screen.
  --version, -v    Show version.
  --pretty, -p     Include indentation and other formatting in the output
  --output=<file>  Output file.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_name: String,
    flag_output: Option<String>,
    flag_pretty: bool
}

pub fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let mut file = File::open(&args.arg_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // set directory to dir of the loaded file
    ::std::env::set_current_dir(Path::new(&args.arg_name).parent().unwrap()).unwrap();

    let render_config = RenderConfig {
        pretty: args.flag_pretty,
        indent_str: "    ",
    };

    if let Some(out_file) = args.flag_output {
        let mut file = File::create(out_file).unwrap();
        superfilter::compile(&contents, &mut file, &render_config);
    } else {
        superfilter::compile(&contents, &mut io::stdout(), &render_config);
    }
}
