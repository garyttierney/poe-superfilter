//! Contains the Expression trait and its implementations for the various abstract syntax tree
//! structures.

use translate::*;
use ast::Block;

/// This trait needs to be implemented for any abstract syntax tree structure, it contains the
/// functions to transform the structure's representation into the final structure before it gets
/// rendered into plain GGG syntax tree output
pub trait Expression<'a> {
    /// Perform any transformations that need to be done before rendering this structure into
    /// plain GGG loot filter syntax
    fn transform<'b, 'c>(&'a self, parent_scope: &'b ScopeData<'b>) -> Result<Rc<TransformedExpression<'c>>, TransformErr> {
        unimplemented!();
    }
}

pub trait TransformedExpression<'a> {
    fn return_value(&self) -> &'a ExpressionValue {
        unimplemented!();
    }
    fn export_scope(&self) -> Option<&'a ScopeData<'a>> {
        None
    }
    fn render(&self) -> Result<String, TransformErr> {
        unimplemented!();
    }
}
