use ast::{TransformedNode, Node, AstLocation};
use ast::transform::{Transform, TransformContext, TransformResult};
use std::fmt;
use errors::{Result, ErrorKind};

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

impl Transform for VarReference {
    #[allow(unused_variables)]
    fn transform(&self, ctx: TransformContext)
                 -> Result<Option<TransformedNode>> {
        // try to resolve variable reference
        match ctx.ref_scope().var(&self.identifier) {
            Some(val) => Ok(Some(
                TransformedNode::Value(val)
            )),
            None => Err(ErrorKind::MissingVarRef(format!("{:?}", self), self.location()).into())
        }
    }

    fn location(&self) -> AstLocation {
        self.location.clone()
    }
}

/// AST node for variable definitions
#[derive(Debug, Clone)]
pub struct VarDefinition {
    pub identifier: String,
    pub values: Box<Node>,
    pub location: AstLocation
}

impl Transform for VarDefinition {
    fn transform(&self, ctx: TransformContext)
                 -> Result<Option<TransformedNode>> {
        if let Some(transformed_value) = self.values.transform(ctx.clone())? {
            // export variable to parent scope
            ctx.mut_scope()
                .push_var(self.identifier.clone(), transformed_value.return_value());
        }

        Ok(None)
    }

    fn location(&self) -> AstLocation {
        self.location.clone()
    }
}
