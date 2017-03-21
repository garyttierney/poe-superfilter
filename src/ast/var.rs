use ast;
use ast::{TransformedNode, TransformErr, Value};
use ast::transform::{Transform, TransformResult};
use scope::{ScopeData, ScopeValue};
use std::rc::Rc;
use std::cell::RefCell;
use arena::TypedArena;
use ast::block_statements::BlockStatement;

#[derive(Debug)]
pub struct VarReference {
    pub identifier: String
}
impl <'a> ast::Value<'a> for VarReference {}
impl <'a> Transform<'a> for VarReference {
    fn transform(&'a self, parent_scope: Rc<RefCell<ScopeData<'a>>>, transformed_arena: &'a TypedArena<TransformedNode<'a>>)
                     -> Result<Option<&'a TransformedNode<'a>>, TransformErr> {
        // try to resolve variable reference
        match parent_scope.borrow().var(&self.identifier) {
            Some(val) => Ok(Some(
                transformed_arena.alloc(TransformedNode::Value(val))
            )),
            None => Err(TransformErr::MissingVarRef(self.identifier.clone()))
        }
    }
}

/// Variable definition
#[derive(Debug, Clone)]
pub struct VarDefinition<'a> {
    pub identifier: String,
    pub values: Vec<&'a Value<'a>>
}

impl <'a> BlockStatement<'a> for VarDefinition<'a> {}

impl <'a> Transform<'a> for VarDefinition<'a> {
    fn transform(&'a self, parent_scope: Rc<RefCell<ScopeData<'a>>>, transformed_arena: &'a TypedArena<TransformedNode<'a>>)
                     -> Result<Option<&'a TransformedNode<'a>>, TransformErr> {
        for val in &self.values {
            if let Some(t_val) = val.transform(parent_scope.clone(), transformed_arena)? {
                parent_scope.borrow_mut().push_var(self.identifier.clone(), t_val.return_value());
            }
        }
        Ok(None)
    }
}