//! A preprocessor for Path of Exile loot filters that adds variables, mixins, arithmetics and lots
//! of other useful things to GGG's loot filter syntax and compiles filters
//! written in the extended syntax down to a pure loot filter that can be used
//! in the game.

#![recursion_limit = "1024"]

#[macro_use]
extern crate superfilter_macro;
#[macro_use]
extern crate error_chain;
use filter::FilterParser;
use lalrpop_util::lalrpop_mod;

use crate::ast::transform::{RenderConfig, RenderContext, TransformResult};
use crate::scope::ScopeData;
use std::cell::RefCell;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

lalrpop_mod!(
    #[allow(unused)]
    #[allow(clippy::all)]
    filter
);

#[allow(dead_code)]
pub mod ast;

#[allow(dead_code)]
mod tok;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
mod scope;

#[allow(dead_code)]
mod errors;

use crate::errors::{Result, ResultExt};

#[cfg(windows)]
pub const LINE_END: &'static [u8] = b"\r\n";
#[cfg(not(windows))]
pub const LINE_END: &'static [u8] = b"\n";

/// Compiles a complete filter into vanilla loot filter syntax
pub fn compile(
    contents: &str,
    file: PathBuf,
    out_buf: &mut dyn Write,
    render_config: &RenderConfig,
) -> Result<()> {
    let tokens = Box::new(tok::tokenize(contents));
    let root_scope = Rc::new(RefCell::new(ScopeData::new(None)));

    let render_ctx = RenderContext {
        config: render_config,
        indent_level: 0,
    };

    let parser = FilterParser::new();

    match parser.parse(&Arc::new(file), tokens.into_iter()) {
        Ok(ref filter) => {
            let transformed_tree = filter
                .transform_begin(root_scope, Rc::new(render_config.base_path.clone()))?
                .expect("Expected a transform result");
            transformed_tree.render(render_ctx, out_buf)
        }
        Err(err) => Err(err).chain_err(|| "Parse Error"),
    }
}
