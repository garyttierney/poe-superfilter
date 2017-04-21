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
use std::time::SystemTime;

const USAGE: &'static str = "
PoE Superfilter compiler

Usage:
  superfilter <path> [--output=<file>] [-p]
  superfilter (-h | --help)
  superfilter --version

Options:
  -h --help        Show this screen.
  --version, -v    Show version.
  --pretty, -p     Include indentation and other formatting in the output
  --output=<file>  Output file. If this option is omitted, the output will be printed to the console.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_path: String,
    flag_output: Option<String>,
    flag_pretty: bool
}

pub fn main() {
    let start_time = SystemTime::now();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let mut file = File::open(&args.arg_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let base_path = Path::new(&args.arg_path)
        .parent()
        .unwrap()
        .to_owned();

    let render_config = RenderConfig {
        pretty: args.flag_pretty,
        indent_str: "    ",
        base_path: base_path
    };

    let result = {
        if let Some(out_file) = args.flag_output {
            let mut out_stream = File::create(out_file).unwrap();
            superfilter::compile(&contents, args.arg_path.clone(), &mut out_stream, &render_config)
        } else {
            superfilter::compile(&contents, args.arg_path.clone(), &mut io::stdout(), &render_config)
        }
    };

    let compile_time = start_time.elapsed().unwrap();

    match result {
        Ok(_) => {
            println!("Compilation successful ({}.{}s)", compile_time.as_secs(), compile_time.subsec_nanos() / 100000);
        },
        Err(err) => {
            println!("Compilation failed:");
            println!("{}", err)
        }
    }
}
