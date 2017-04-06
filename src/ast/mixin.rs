use ast::{Node, TransformedNode, CompileErr, AstLocation};
use ast::transform::{Transform, TransformContext};
use scope::{ScopeData};
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

/// Name and parameter specs for a mixin
#[derive(Debug, Clone)]
pub struct Mixin<'a> {
    pub name: String,
    pub parameters: Vec<Param<'a>>,
    pub statements: Vec<&'a Node<'a>>,
    pub location: AstLocation
}

#[derive(Debug, Clone)]
pub struct PreparedMixin<'a> {
    pub name: String,
    pub parameters: Vec<PlainParam<'a>>,
    pub statements: Vec<&'a Node<'a>>
}

impl <'a> Transform<'a> for Mixin<'a> {
    fn transform(&self, ctx: TransformContext<'a>)
        -> Result<Option<&'a TransformedNode<'a>>, CompileErr> {
        let mut t_params = Vec::new();
        for param in &self.parameters {
            if let Some(default_value) = param.default {
                if let Some(t_value) = default_value.transform(ctx.clone())? {
                    t_params.push(PlainParam {
                        name: param.name.clone(),
                        default: Some(t_value.clone()),
                    })
                } else {
                    return Err(CompileErr::MissingValue(format!("{:?}", self), self.location.clone()))
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
#[derive(Clone)]
pub struct MixinCall<'a> {
    pub name: String,
    pub parameters: Vec<&'a Node<'a>>,
    pub location: AstLocation
}

impl <'a> fmt::Debug for MixinCall<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MixinCall +{}(", self.name)?;
        for param in &self.parameters {
            write!(f, "{:?}, ", param)?;
        }
        write!(f, ")")
    }
}

pub type ResolvedMixin<'a> = Vec<&'a TransformedNode<'a>>;

impl <'a> Transform<'a> for MixinCall<'a> {
    fn transform(&self, ctx: TransformContext<'a>)
        -> Result<Option<&'a TransformedNode<'a>>, CompileErr> {
        if let Some(mixin) = ctx.ref_scope().mixin(&self.name) {
            // catch parameter count mismatch
            if mixin.parameters.len() != self.parameters.len() {
                return Err(CompileErr::WrongParameterCount(format!("{:?}", self),
                                                           mixin.parameters.len(),
                                                           self.parameters.len(),
                                                           self.location.clone()
                ));
            }

            // transform parameters in this call
            let mut t_params = Vec::new();
            for param in &self.parameters {
                if let Some(t_param) = param.transform(ctx.clone())? {
                    t_params.push(t_param);
                } else {
                    return Err(CompileErr::MissingValue(format!("{:?}", param), self.location.clone()));
                }
            }

            let mut mixin_inner_scope = ScopeData::new(Some(ctx.scope.clone()));
            for i in 0..self.parameters.len() {
                mixin_inner_scope.push_var(mixin.parameters[i].name.clone(), t_params[i].return_value())
            }

            let inner_ctx = TransformContext {
                scope: Rc::new(RefCell::new(mixin_inner_scope)),
                transform_arena: ctx.transform_arena,
                ast_arena: ctx.ast_arena,
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
                ctx.alloc_transformed(TransformedNode::ExpandedNodes(t_statements))
            ))
        } else {
            Err(CompileErr::MissingVarRef(self.name.clone(), self.location.clone()))
        }
    }
    fn location(&self) -> AstLocation {
        self.location.clone()
    }
}
