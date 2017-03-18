
use ast;
use ast::Node;
use ast::expressions::Expression;
use translate::{TransformErr, ScopeData, ExpressionValue};

#[derive(Debug, Clone)]
pub enum NumberOperation {
    Mul,
    Div,
    Add,
    Sub
}

/// Number value or expression
#[derive(Debug, Clone)]
pub enum NumberExpression<'ast> {
    Number(NumberBox),
    Op(&'ast Node<'ast>, NumberOperation, &'ast Node<'ast>)
}

impl <'a> ast::Value<'a> for NumberExpression<'a> {}
impl <'a> Expression<'a> for NumberExpression<'a> {}

/// Number value or variable reference
#[derive(Debug, Clone)]
pub enum NumberBox {
    IntValue(i32),
    Decimal(f32),
    Var(String)
}

impl <'a> Expression<'a> for NumberBox {}
