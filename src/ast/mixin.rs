use ast::{Node, TransformedNode, CompileErr, AstLocation};
use ast::transform::{Transform, TransformContext, TransformResult};
use scope::{ScopeData, ScopeValue};
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

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
        -> Result<Option<TransformedNode>, CompileErr> {
        let mut t_params = Vec::new();
        for param in &self.parameters {
            if let Some(ref default_value) = param.default {
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
    pub parameters: Vec<Vec<Node>>,
    pub location: AstLocation
}

impl <'a> fmt::Debug for MixinCall {
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
        -> Result<Option<TransformedNode>, CompileErr> {
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
            let mut t_params : Vec<ScopeValue> = Vec::new();
            for param in &self.parameters {
                let transform_results : Vec<Result<Option<TransformedNode>, CompileErr>> = param.iter().map(|p| p.transform(ctx.clone())).collect();
                // return on any transform errors in the param values
                if transform_results.iter().any(|r| r.is_err()) {
                    return transform_results.into_iter().find(|r| r.is_err()).unwrap();
                }


                // get results from transform return values. unwrap is ok since we already checked for None values
                let transform_options : Vec<Option<TransformedNode>> = transform_results.into_iter().map(|o| o.unwrap()).collect();

                if transform_options.iter().any(|r| r.is_none()) {
                    return Err(CompileErr::MissingValue(format!("{:?}", param), self.location.clone()));
                }

                let return_values : Vec<ScopeValue> = transform_options.into_iter().map(|r| r.unwrap().return_value()).collect();

                if return_values.len() == 1 {
                    t_params.push(return_values[0].clone());
                } else {
                    t_params.push(ScopeValue::List(return_values))
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
            Err(CompileErr::MissingVarRef(self.name.clone(), self.location.clone()))
        }
    }

    fn location(&self) -> AstLocation {
        self.location.clone()
    }
}
