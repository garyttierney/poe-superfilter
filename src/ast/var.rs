use ast::{TransformedNode, CompileErr, Node, AstLocation};
use ast::transform::{Transform, TransformContext};
use scope::ScopeValue;
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

impl Transform for VarReference {
    #[allow(unused_variables)]
    fn transform(&self, ctx: TransformContext)
                     -> Result<Option<TransformedNode>, CompileErr> {
        // try to resolve variable reference
        match ctx.ref_scope().var(&self.identifier) {
            Some(val) => Ok(Some(
                TransformedNode::Value(val)
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
pub struct VarDefinition {
    pub identifier: String,
    pub values: Vec<Node>,
    pub location: AstLocation
}

impl Transform for VarDefinition {
    fn transform(&self, ctx: TransformContext)
                     -> Result<Option<TransformedNode>, CompileErr> {
        // transform values and collect them in a vec
        let mut return_values = Vec::new();
        for val in &self.values {
            if let Some(t_val) = val.transform(ctx.clone())? {
                return_values.push(t_val.return_value())
            }
        }

        // export variable to parent scope
        ctx.mut_scope()
            .push_var(self.identifier.clone(), ScopeValue::List(return_values));
        Ok(None)
    }

    fn location(&self) -> AstLocation {
        self.location.clone()
    }
}
