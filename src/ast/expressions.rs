//! Contains the Expression trait and its implementations for the various abstract syntax tree
//! structures.

use translate::*;
use ast::TransformedNode;
use arena::TypedArena;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::Write;
use std::io::Error as IoError;

/// This trait needs to be implemented for any abstract syntax tree structure, it contains the
/// functions to transform the structure's representation into the final structure before it gets
/// rendered into plain GGG syntax tree output
#[allow(unused_variables)]
pub trait Expression<'a> {
    /// Perform any transformations that need to be done before rendering this structure into
    /// plain GGG loot filter syntax
    fn transform<'t>(&'a self, parent_scope: Rc<RefCell<ScopeData>>, transformed_arena: &'t TypedArena<TransformedNode<'t>>)
        -> Result<&'t TransformedNode<'t>, TransformErr>;
}

pub trait TransformedExpression {
    fn return_value(&self) -> ExpressionValue {
        ExpressionValue::None
    }
    fn render(&self, buf: &mut Write) -> Result<(), RenderErr>;
}

quick_error! {
    #[derive(Debug)]
    pub enum RenderErr {
        Io(inner: IoError) {
            description("IO Error")
            display("IO Error: {}", inner)
            cause(inner)
        }
    }
}

impl From<IoError> for RenderErr {
    fn from(io_err: IoError) -> RenderErr {
        RenderErr::Io(io_err)
    }
}