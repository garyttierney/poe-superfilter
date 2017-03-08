//! Contains the Expression trait and its implementations for the various abstract syntax tree
//! structures.

use translate::*;
use ast::NumberBox;
use ast::Block;
use ast::block_statements::Statement;

/// This trait needs to be implemented for any abstract syntax tree structure, it contains the
/// functions to transform the structure's representation into the final structure before it gets
/// rendered into plain GGG syntax tree output
pub trait Expression<'a> {
    /// Perform any transformations that need to be done before rendering this structure into
    /// plain GGG loot filter syntax
    fn transform(&mut self, parent_scope:&ScopeData) -> Result<(), TransformErr<'a>>;

    /// Render this structure into a string
    fn render(&'a self) -> Result<String, TransformErr<'a>>;
}

impl <'a> Expression<'a> for Block {
    fn transform(&mut self, parent_scope: &ScopeData) -> Result<(), TransformErr<'a>> {
        let block_scope = ScopeData::from_parent(parent_scope);
        match *self {
            Block::Show(ref mut statements) | Block::Hide(ref mut statements) => {
                for statement in statements {
                    statement.transform(&block_scope);
                }
            }
            _ => unreachable!()
        }
        Ok(())
    }

    fn render(&'a self) -> Result<String, TransformErr<'a>> {
        unimplemented!()
    }
}

impl <'a> Expression<'a> for Statement {
    fn transform(&mut self, scope: &ScopeData) -> Result<(), TransformErr<'a>> {
        unimplemented!()
    }

    fn render(&'a self) -> Result<String, TransformErr<'a>> {
        unimplemented!()
    }
}
