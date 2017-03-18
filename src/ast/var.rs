use ast;
use ast::expressions::Expression;
use translate::{TransformErr, ScopeData, ExpressionValue};

#[derive(Debug)]
pub struct VarReference {
    pub identifier: String
}
impl <'a> ast::Value<'a> for VarReference {}
impl <'a> Expression<'a> for VarReference {}
