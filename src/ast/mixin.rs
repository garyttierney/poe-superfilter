use ast::{Node, TransformedNode, TransformErr};
use ast::transform::{Transform, TransformResult};
use scope::{ScopeData};
use std::cell::RefCell;
use std::rc::Rc;
use arena::TypedArena;

/// Name and parameter specs for a mixin
#[derive(Debug, Clone)]
pub struct Mixin<'a> {
    pub name: String,
    pub parameters: Vec<Param<'a>>,
    pub statements: Vec<&'a Node<'a>>
}

#[derive(Debug, Clone)]
pub struct PreparedMixin<'a> {
    pub name: String,
    pub parameters: Vec<PlainParam<'a>>,
    pub statements: Vec<&'a Node<'a>>
}

impl <'a> Transform<'a> for Mixin<'a> {
    fn transform(&'a self, parent_scope: Rc<RefCell<ScopeData<'a>>>, transformed_arena: &'a TypedArena<TransformedNode<'a>>)
        -> Result<Option<&'a TransformedNode<'a>>, TransformErr> {
        let mut t_params = Vec::new();
        for param in &self.parameters {
            if let Some(default_value) = param.default {
                if let Some(t_value) = default_value.transform(parent_scope.clone(), transformed_arena)? {
                    t_params.push(PlainParam {
                        name: param.name.clone(),
                        default: Some(t_value.clone()),
                    })
                } else {
                    return Err(TransformErr::MissingValue(format!("{:?}", self)))
                }
            } else {
                t_params.push(PlainParam {
                    name: param.name.clone(),
                    default: None,
                })
            }
        }

        parent_scope.borrow_mut().push_mixin(
            self.name.clone(),
            PreparedMixin {
                name: self.name.clone(),
                parameters: t_params,
                statements: self.statements.clone(),
            }
        );

        Ok(None)
    }
}

/// (Mixin) Parameter name and default values
#[derive(Debug, Clone)]
pub struct Param<'a> {
    pub name: String,
    pub default: Option<&'a Node<'a>>
}

/// (Mixin) Parameter name and default values
#[derive(Debug, Clone)]
pub struct PlainParam<'a> {
    pub name: String,
    pub default: Option<TransformedNode<'a>>
}

/// Represents a mixin include with name and parameters
#[derive(Debug, Clone)]
pub struct MixinCall<'a> {
    pub name: String,
    pub parameters: Vec<&'a Node<'a>>
}

pub type ResolvedMixin<'a> = Vec<&'a TransformedNode<'a>>;

impl <'a> Transform<'a> for MixinCall<'a> {
    #[allow(unused_variables)]
    fn transform(&'a self, parent_scope: Rc<RefCell<ScopeData<'a>>>, transformed_arena: &'a TypedArena<TransformedNode<'a>>)
        -> Result<Option<&'a TransformedNode<'a>>, TransformErr> {
        if let Some(mixin) = parent_scope.borrow().mixin(&self.name) {
            // catch parameter count mismatch
            if mixin.parameters.len() != self.parameters.len() {
                return Err(TransformErr::WrongParameterCount(format!("{:?}", self), mixin.parameters.len(), self.parameters.len()));
            }

            // transform parameters in this call
            let mut t_params = Vec::new();
            for param in &self.parameters {
                if let Some(t_param) = param.transform(parent_scope.clone(), transformed_arena)? {
                    t_params.push(t_param);
                } else {
                    return Err(TransformErr::MissingValue(format!("{:?}", param)))
                }
            }

            let mut mixin_inner_scope = ScopeData::new(Some(parent_scope.clone()));
            for i in 0..self.parameters.len() {
                mixin_inner_scope.push_var(mixin.parameters[i].name.clone(), t_params[i].return_value())
            }

            let mixin_inner_scope_ref = Rc::new(RefCell::new(mixin_inner_scope));

            // collect transformed statements from lines in the mixin
            let mut t_statements: ResolvedMixin = Vec::new();
            for statement in &mixin.statements {
                if let Some(t_statement) = statement.transform(mixin_inner_scope_ref.clone(), transformed_arena)? {
                    t_statements.push(t_statement);
                }
            }
            Ok(Some(
                transformed_arena.alloc(TransformedNode::ResolvedMixin(t_statements))
            ))
        } else {
            Err(TransformErr::MissingVarRef(self.name.clone()))
        }
    }
}
