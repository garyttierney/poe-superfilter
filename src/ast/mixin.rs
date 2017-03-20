use ast::{Value,TransformedNode};
use ast::block_statements::BlockStatement;
use ast::expressions::Expression;
use translate::{TransformErr, ScopeData, ExpressionValue};
use std::cell::RefCell;
use std::rc::Rc;
use arena::TypedArena;

/// Name and parameter specs for a mixin
#[derive(Debug, Clone)]
pub struct Mixin<'a> {
    pub name: String,
    pub parameters: Vec<Param<'a>>,
    pub statements: Vec<&'a BlockStatement<'a>>
}

/// (Mixin) Parameter name and default values
#[derive(Debug, Clone)]
pub struct Param<'a> {
    pub name: String,
    pub default: Option<&'a Value<'a>>
}

/// Represents a mixin include with name and
/// parameters
#[derive(Debug, Clone)]
pub struct MixinCall<'a> {
    pub name: String,
    pub parameters: Vec<&'a Value<'a>>
}

impl <'a> Expression<'a> for MixinCall<'a> {
    fn transform<'t>(&'a self, parent_scope: Rc<RefCell<ScopeData>>, transformed_arena: &'t TypedArena<TransformedNode<'t>>)
        -> Result<&'t TransformedNode<'t>, TransformErr> {
        unimplemented!()
    }
}
impl <'a> BlockStatement<'a> for MixinCall<'a> {}
