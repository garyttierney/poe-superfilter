//! Contains the Expression trait and its implementations for the various abstract syntax tree
//! structures.

use scope::{ScopeData, ScopeValue};
use arena::TypedArena;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::Write;
use ast::{CompileErr,TransformedNode,Node};

/// This trait needs to be implemented for any abstract syntax tree structure, it contains the
/// functions to transform the structure's representation into the final structure before it gets
/// rendered into plain GGG syntax tree output
pub trait Transform<'a> {
    /// Perform any transformations that need to be done before rendering this structure into
    /// plain GGG loot filter syntax
    fn transform(&'a self, ctx: TransformContext<'a>)
        -> Result<Option<&'a TransformedNode<'a>>, CompileErr>;
}

#[derive(Clone)]
pub struct TransformContext<'a> {
    pub parent_scope: Rc<RefCell<ScopeData<'a>>>,
    pub transform_arena: &'a TypedArena<TransformedNode<'a>>,
    pub ast_arena: &'a TypedArena<Node<'a>>
}

impl <'a> TransformContext<'a> {
    pub fn alloc_transformed(&self, node: TransformedNode<'a>)
        -> &'a TransformedNode<'a> {
        self.transform_arena.alloc(node)
    }

    pub fn alloc_ast(&self, node: Node<'a>)
                             -> &'a Node<'a> {
        self.ast_arena.alloc(node)
    }
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
    fn render(&self, buf: &mut Write) -> Result<(), CompileErr>;
}
