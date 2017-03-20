use ast;
use ast::{TransformedNode,TransformErr};
use ast::transform::Transform;
use scope::{ScopeData, ScopeValue};
use std::rc::Rc;
use std::cell::RefCell;
use arena::TypedArena;

#[derive(Debug)]
pub struct VarReference {
    pub identifier: String
}
impl <'a> ast::Value<'a> for VarReference {}
impl <'a> Transform<'a> for VarReference {
    fn transform<'t>(&'a self, parent_scope: Rc<RefCell<ScopeData>>, transformed_arena: &'t TypedArena<TransformedNode<'t>>)
                     -> Result<&'t TransformedNode<'t>, TransformErr> {
        // try to resolve variable reference
        match parent_scope.borrow().var(&self.identifier) {
            Some(val) => Ok(transformed_arena.alloc(TransformedNode::Value(val))),
            None => Err(TransformErr::MissingVarRef(self.identifier.clone()))
        }
    }
}
