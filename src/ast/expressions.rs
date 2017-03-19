//! Contains the Expression trait and its implementations for the various abstract syntax tree
//! structures.

use translate::*;
use ast::TransformedNode;
use arena::TypedArena;
use std::rc::Rc;
use std::cell::RefCell;

/// This trait needs to be implemented for any abstract syntax tree structure, it contains the
/// functions to transform the structure's representation into the final structure before it gets
/// rendered into plain GGG syntax tree output
pub trait Expression<'a> {
    /// Perform any transformations that need to be done before rendering this structure into
    /// plain GGG loot filter syntax
    fn transform<'t>(&'a self, parent_scope: Rc<RefCell<ScopeData>>, transformed_arena: &'t TypedArena<TransformedNode<'t>>)
        -> Result<&'t TransformedNode<'t>, TransformErr> {
        unimplemented!();
    }
}

pub trait TransformedExpression {
    fn return_value(&self) -> ExpressionValue {
        ExpressionValue::None
    }
    fn render(&self) -> Result<String, TransformErr> {
        unimplemented!();
    }
}
