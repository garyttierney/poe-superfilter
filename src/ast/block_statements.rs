
use ast::{Value, TransformedNode};
use ast::expressions::{Expression, TransformedExpression};
use ast::mixin::MixinCall;
use ast::VarDefinition;
use translate::{TransformErr, ScopeData, ExpressionValue};
use std::fmt::Debug;
use arena::TypedArena;
use std::cell::RefCell;
use std::rc::Rc;

pub trait BlockStatement<'a> : Debug + Expression<'a> {}

#[derive(Debug, Clone)]
pub struct SetValueStatement<'a> {
    pub name : String,
    pub values : Vec<&'a Value<'a>>
}

#[derive(Debug)]
pub struct PlainSetValueStatement {
    pub name: String,
    pub values: Vec<ExpressionValue>
}

impl <'a> BlockStatement<'a> for SetValueStatement<'a> {}
impl <'a> Expression<'a> for SetValueStatement<'a> {
    fn transform<'t>(&'a self, parent_scope: Rc<RefCell<ScopeData>>, transformed_arena: &'t TypedArena<TransformedNode<'t>>)
        -> Result<&'t TransformedNode<'t>, TransformErr> {
        let mut transformed_values: Vec<ExpressionValue> = vec![];
        for value in &self.values {
            let t_value = try!(value.transform(parent_scope.clone(), transformed_arena));
            transformed_values.push(t_value.return_value());
        }

        Ok(transformed_arena.alloc(TransformedNode::SetValueStmt(
            PlainSetValueStatement {
                name: self.name.clone(),
                values: transformed_values
            }
        )))
    }
}

impl TransformedExpression for PlainSetValueStatement {}

#[derive(Debug, Clone)]
pub struct ConditionStatement<'a> {
    pub name : String,
    pub condition : Condition<'a>
}

#[derive(Debug, Clone)]
pub struct PlainConditionStatement {
    pub name : String,
    pub condition: PlainCondition
}

impl <'a> BlockStatement<'a> for ConditionStatement<'a> {}
impl <'a> Expression<'a> for ConditionStatement<'a> {
    fn transform<'t>(&'a self, parent_scope: Rc<RefCell<ScopeData>>, transformed_arena: &'t TypedArena<TransformedNode<'t>>)
        -> Result<&'t TransformedNode<'t>, TransformErr> {
        let t_value = self.condition.value.transform(parent_scope.clone(), transformed_arena)?;
        Ok(transformed_arena.alloc(TransformedNode::ConditionStmt(
            PlainConditionStatement {
                name: self.name.clone(),
                condition: PlainCondition { value: t_value.return_value(), operator: self.condition.operator },
            }
        )))
    }
}

impl TransformedExpression for PlainConditionStatement {}

#[derive(Debug, Clone)]
pub struct Condition<'a> {
    pub value: &'a Value<'a>,
    pub operator: ComparisonOperator
}

#[derive(Debug, Clone)]
pub struct PlainCondition {
    pub value: ExpressionValue,
    pub operator: ComparisonOperator
}

#[derive(Debug, Clone, Copy)]
pub enum ComparisonOperator {
    Eql,
    Lt,
    Lte,
    Gt,
    Gte
}
