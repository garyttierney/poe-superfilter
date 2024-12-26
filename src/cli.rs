//! A preprocessor for Path of Exile loot filters that adds variables, mixins, arithmetics and lots
//! of other useful things to GGG's loot filter syntax and compiles filters
//! written in the extended syntax down to a pure loot filter that can be used
//! in the game.
//!
//! This crate is a CLI application that exposes the functionality of the compiler library.

extern crate clap;
extern crate superfilter;

use clap::{Arg, ArgAction};
use miette::{miette, LabeledSpan, Severity, SourceOffset};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use superfilter::ast::transform::RenderConfig;
use superfilter::{Error, LINE_END};

use superfilter::tok::Location as TokenLocation;
use superfilter::tok::Tok;

pub fn main() {
    //let start_time = SystemTime::now();
    miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(false)
                .context_lines(3)
                .tab_width(4)
                .break_words(true)
                .build(),
        )
    }))
    .expect("failed to install miette");

    let matches = clap::Command::new("PoE Superfilter Compiler")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Stefan Kaufhold <cere@fastmail.fm>")
        .arg(Arg::new("PATH")
            .help("Path of the input file")
            .required(true)
            .index(1))
        .arg(Arg::new("pretty")
            .help("Include indentation and other formatting in the output")
            .short('p')
            .long("pretty")
            .action(ArgAction::SetTrue))
        .arg(Arg::new("comments")
            .help("Include comments in the output")
            .short('c')
            .long("comments")
            .action(ArgAction::SetTrue))
        .arg(Arg::new("output")
            .help("Output file. If this option is omitted, the output will be printed to the console.")
            .short('o')
            .long("output")
            .value_name("FILE"))
        .arg(Arg::new("line_endings")
            .help("Type of line ending used (LF OR CRLF) defaults to the platform line ending")
            .short('l')
            .long("line-endings")
            .default_values(["lf", "crlf"])
            .value_name("LINE_ENDING"))
        .get_matches();

    let line_ending: &'static [u8] = match matches
        .get_one::<String>("line_endings")
        .map(String::as_str)
    {
        Some("crlf") => b"\r\n",
        Some("lf") => b"\n",
        None => LINE_END,
        _ => panic!("Invalid line ending"),
    };

    let input_path = Path::new(matches.get_one::<String>("PATH").unwrap()).to_owned();

    let mut file = File::open(&input_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let base_path = input_path.parent().unwrap().to_owned();

    let render_config = RenderConfig {
        pretty: matches.get_flag("pretty"),
        indent_str: "    ",
        base_path,
        line_ending,
        comments: matches.get_flag("comments"),
    };

    let result = match matches.get_one::<String>("output") {
        Some(output) => superfilter::compile(
            &contents,
            input_path,
            &mut File::create(output).unwrap(),
            &render_config,
        ),
        _ => superfilter::compile(&contents, input_path, &mut io::stdout(), &render_config),
    };

    //let compile_time = start_time.elapsed().unwrap();

    if let Err(Error(_, state)) = result {
        let parse_error = state.next_error.and_then(|err| {
            err.downcast::<lalrpop_util::ParseError<TokenLocation, Tok, char>>()
                .ok()
        });

        let Some(error) = parse_error else { return };

        match error.as_ref() {
            lalrpop_util::ParseError::UnrecognizedToken {
                token: (l, tok, _),
                expected,
            } => {
                let start = SourceOffset::from_location(&contents, l.line, l.pos);
                let unrecognized = LabeledSpan::at(start, format!("{} wasn't expected here", tok));

                let report = miette!(
                    labels = vec![unrecognized],
                    help = format!("expected one of: {}", expected.join(", ")),
                    severity = Severity::Error,
                    "Unexpected token"
                );
                let report = report.with_source_code(contents);

                println!("{:?}", report);
            }
            _ => {
                println!("parse error");
            }
        }

        println!("Compilation failed:");
    }
}
