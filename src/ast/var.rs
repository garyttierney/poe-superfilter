use ast::{TransformedNode, CompileErr, Node, AstLocation};
use ast::transform::{Transform, TransformContext};
use std::fmt;

/// AST Structure that represents a reference to a variable
#[derive(Clone)]
pub struct VarReference {
    pub identifier: String,
    pub location: AstLocation
}

impl fmt::Debug for VarReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${}", self.identifier)
    }
}

impl <'a> Transform<'a> for VarReference {
    #[allow(unused_variables)]
    fn transform(&self, ctx: TransformContext<'a>)
                     -> Result<Option<&'a TransformedNode<'a>>, CompileErr> {
        // try to resolve variable reference
        match ctx.ref_scope().var(&self.identifier) {
            Some(val) => Ok(Some(
                ctx.alloc_transformed(TransformedNode::Value(val))
            )),
            None => Err(CompileErr::MissingVarRef(format!("{:?}", self), self.location()))
        }
    }

    fn location(&self) -> AstLocation {
        self.location.clone()
    }
}

/// AST node for variable definitions
#[derive(Debug, Clone)]
pub struct VarDefinition<'a> {
    pub identifier: String,
    pub values: Vec<&'a Node<'a>>,
    pub location: AstLocation
}

impl <'a> Transform<'a> for VarDefinition<'a> {
    fn transform(&self, ctx: TransformContext<'a>)
                     -> Result<Option<&'a TransformedNode<'a>>, CompileErr> {
        for val in &self.values {
            if let Some(t_val) = val.transform(ctx.clone())? {
                // export variable to parent scope
                ctx.mut_scope()
                    .push_var(self.identifier.clone(), t_val.return_value());
            }
        }
        Ok(None)
    }

    fn location(&self) -> AstLocation {
        self.location.clone()
    }
}