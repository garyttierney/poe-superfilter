//! A preprocessor for Path of Exile loot filters that adds variables, mixins, arithmetics and lots
//! of other useful things to GGG's loot filter syntax and compiles filters
//! written in the extended syntax down to a pure loot filter that can be used
//! in the game.

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate quick_error;
#[macro_use] extern crate superfilter_macro;

extern crate regex;
extern crate lalrpop_util;

use ast::transform::{RenderContext, RenderConfig};
use ast::CompileErr;
use std::io::Write;
use ast::Node;
use std::rc::Rc;
use std::cell::RefCell;
use scope::ScopeData;

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

const LINE_END : &'static [u8] = b"\r\n";

/// Compiles a complete filter into vanilla loot filter syntax
pub fn compile(contents: &str, file: String, out_buf: &mut Write, render_config: &RenderConfig)
        -> Result<(),CompileErr> {
    let tokens = Box::new(tok::tokenize(contents));
    let root_scope = Rc::new(RefCell::new(ScopeData::new(None)));

    let render_ctx = RenderContext {
        config: render_config,
        indent_level: 0,
    };

    match filter::parse_Filter(file.as_str(), tokens.into_iter()) {
        Ok(Node::Filter(ref filter)) => {
            let result = filter.transform_begin(root_scope,
                                                Rc::new(render_config.base_path.clone()))?;
            if let Some(transformed_tree) = result {
                transformed_tree.render(render_ctx, out_buf).unwrap();
            }
        },
        Err(err) => return Err(CompileErr::ParseError(err)),
        _ => panic!()
    }
    Ok(())
}
