use ast::{TransformedNode, CompileErr, Node};
use ast::transform::{Transform, TransformResult};
use scope::{ScopeData};
use std::rc::Rc;
use std::cell::RefCell;
use arena::TypedArena;

/// AST Structure that represents a reference to a variable
#[derive(Debug)]
pub struct VarReference {
    pub identifier: String
}
impl <'a> Transform<'a> for VarReference {
    #[allow(unused_variables)]
    fn transform(&'a self, parent_scope: Rc<RefCell<ScopeData<'a>>>, transformed_arena: &'a TypedArena<TransformedNode<'a>>, ast_arena: &'a TypedArena<Node<'a>> )
                     -> Result<Option<&'a TransformedNode<'a>>, CompileErr> {
        // try to resolve variable reference
        match parent_scope.borrow().var(&self.identifier) {
            Some(val) => Ok(Some(
                transformed_arena.alloc(TransformedNode::Value(val))
            )),
            None => Err(CompileErr::MissingVarRef(self.identifier.clone()))
        }
    }
}

/// AST node for variable definitions
#[derive(Debug, Clone)]
pub struct VarDefinition<'a> {
    pub identifier: String,
    pub values: Vec<&'a Node<'a>>
}

impl <'a> Transform<'a> for VarDefinition<'a> {
    fn transform(&'a self,
                 parent_scope: Rc<RefCell<ScopeData<'a>>>,
                 transformed_arena: &'a TypedArena<TransformedNode<'a>>,
                 ast_arena: &'a TypedArena<Node<'a>>)
                     -> Result<Option<&'a TransformedNode<'a>>, CompileErr> {
        for val in &self.values {
            if let Some(t_val) = val.transform(parent_scope.clone(), transformed_arena, ast_arena)? {
                // export variable to parent scope
                parent_scope.borrow_mut()
                    .push_var(self.identifier.clone(), t_val.return_value());
            }
        }
        Ok(None)
    }
}