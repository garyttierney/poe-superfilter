//! Contains the Expression trait and its implementations for the various abstract syntax tree
//! structures.

use scope::{ScopeData, ScopeValue};
use arena::TypedArena;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::Write;
use ast::{RenderErr,TransformErr,TransformedNode};

/// This trait needs to be implemented for any abstract syntax tree structure, it contains the
/// functions to transform the structure's representation into the final structure before it gets
/// rendered into plain GGG syntax tree output
#[allow(unused_variables)]
pub trait Transform<'a> {
    /// Perform any transformations that need to be done before rendering this structure into
    /// plain GGG loot filter syntax
    fn transform(&'a self, parent_scope: Rc<RefCell<ScopeData<'a>>>, transformed_arena: &'a TypedArena<TransformedNode<'a>>)
        -> Result<Option<&'a TransformedNode<'a>>, TransformErr>;
}

/// Fully transformed AST structures should implement this trait in order to be renderable and make
/// their result available for use in parent nodes
pub trait TransformResult {
    /// Returns the return value of this AST structure if it has one. The return value of this
    /// function can be used in parent structures that base their transformation on the result
    /// of child nodes.
    fn return_value(&self) -> ScopeValue {
        ScopeValue::None
    }

    /// Renders the output for this node into a writable stream.
    fn render(&self, buf: &mut Write) -> Result<(), RenderErr>;
}
