
use ast::Value;
use ast::expressions::Expression;
use ast::mixin::MixinCall;
use ast::VarDefinition;
use translate::{TransformErr, ScopeData, ExpressionValue};
use std::fmt::Debug;

pub trait BlockStatement<'a> : Debug + Expression<'a> {}

#[derive(Debug, Clone)]
pub struct SetValueStatement<'a> {
    pub name : String,
    pub values : Vec<&'a Value<'a>>
}

impl <'a> BlockStatement<'a> for SetValueStatement<'a> {}
impl <'a> Expression<'a> for SetValueStatement<'a> {}

#[derive(Debug, Clone)]
pub struct ConditionStatement<'a> {
    pub name : String,
    pub condition : Condition<'a>
}

impl <'a> BlockStatement<'a> for ConditionStatement<'a> {}
impl <'a> Expression<'a> for ConditionStatement<'a> {}

#[derive(Debug, Clone)]
pub struct Condition<'a> {
    pub value: &'a Value<'a>,
    pub operator: ComparisonOperator
}

#[derive(Debug, Clone)]
pub enum ComparisonOperator {
    Eql,
    Lt,
    Lte,
    Gt,
    Gte
}
