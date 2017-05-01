use ast::{Node, TransformedNode, AstLocation};
use ast::transform::{Transform, TransformContext, TransformResult};
use scope::{ScopeData, ScopeValue};
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;
use errors::*;

/// Name and parameter specs for a mixin
#[derive(Debug, Clone)]
pub struct Mixin {
    pub name: String,
    pub parameters: Vec<Param>,
    pub statements: Vec<Node>,
    pub location: AstLocation
}

#[derive(Debug, Clone)]
pub struct PreparedMixin {
    pub name: String,
    pub parameters: Vec<PlainParam>,
    pub statements: Vec<Node>
}

impl Transform for Mixin {
    fn transform(&self, ctx: TransformContext)
                 -> Result<Option<TransformedNode>> {
        let mut t_params = Vec::new();
        for param in &self.parameters {
            if let Some(ref default_value) = param.default {
                if let Some(t_value) = default_value.transform(ctx.clone())? {
                    t_params.push(PlainParam {
                        name: param.name.clone(),
                        default: Some(t_value.clone()),
                    })
                } else {
                    return Err(ErrorKind::MissingValue(format!("{:?}", self), self.location.clone()).into())
                }
            } else {
                t_params.push(PlainParam {
                    name: param.name.clone(),
                    default: None,
                })
            }
        }

        ctx.mut_scope().push_mixin(
            self.name.clone(),
            PreparedMixin {
                name: self.name.clone(),
                parameters: t_params,
                statements: self.statements.clone(),
            }
        );

        Ok(None)
    }

    fn location(&self) -> AstLocation {
        self.location.clone()
    }
}

/// (Mixin) Parameter name and default values
#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub default: Option<Node>
}

/// (Mixin) Parameter name and default values
#[derive(Debug, Clone)]
pub struct PlainParam {
    pub name: String,
    pub default: Option<TransformedNode>
}

/// Represents a mixin include with name and parameters
#[derive(Clone)]
pub struct MixinCall {
    pub name: String,
    pub parameters: Vec<Node>,
    pub location: AstLocation
}

impl<'a> fmt::Debug for MixinCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MixinCall +{}(", self.name)?;
        for param in &self.parameters {
            write!(f, "{:?}, ", param)?;
        }
        write!(f, ")")
    }
}

pub type ResolvedMixin<'a> = Vec<TransformedNode>;

impl Transform for MixinCall {
    fn transform(&self, ctx: TransformContext)
                 -> Result<Option<TransformedNode>> {
        if let Some(mixin) = ctx.ref_scope().mixin(&self.name) {
            // catch parameter count mismatch
            if mixin.parameters.len() != self.parameters.len() {
                return Err(ErrorKind::WrongParameterCount(format!("{:?}", self),
                                                           mixin.parameters.len(),
                                                           self.parameters.len(),
                                                           self.location.clone()
                ).into());
            }

            // transform parameters in this call
            let mut t_params: Vec<ScopeValue> = Vec::new();

            for param in &self.parameters {
                match param.transform(ctx.clone())? {
                    Some(transformed_param) => t_params.push(transformed_param.return_value()),
                    None => return Err(
                        ErrorKind::MissingValue(format!("{:?}", param), param.location().clone()).into()
                    )
                }
            }

            // create scope for this mixin call, include parameter values
            let mut mixin_inner_scope = ScopeData::new(Some(ctx.scope.clone()));
            for i in 0..self.parameters.len() {
                mixin_inner_scope.push_var(mixin.parameters[i].name.clone(), t_params[i].return_value())
            }

            let inner_ctx = TransformContext {
                scope: Rc::new(RefCell::new(mixin_inner_scope)),
                path: ctx.path.clone(),
            };

            // collect transformed statements from lines in the mixin
            let mut t_statements: ResolvedMixin = Vec::new();
            for statement in &mixin.statements {
                if let Some(t_statement) = statement.transform(inner_ctx.clone())? {
                    t_statements.push(t_statement);
                }
            }
            Ok(Some(
                TransformedNode::ExpandedNodes(t_statements)
            ))
        } else {
            Err(
                ErrorKind::MissingVarRef(self.name.clone(), self.location.clone()).into()
            )
        }
    }

    fn location(&self) -> AstLocation {
        self.location.clone()
    }
}
