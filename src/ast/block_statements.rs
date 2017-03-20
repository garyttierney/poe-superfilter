
use ast::{Value, TransformedNode, TransformErr};
use ast::transform::{Transform, TransformResult};
use ast::mixin::MixinCall;
use ast::VarDefinition;
use scope::{ScopeData, ScopeValue};
use std::fmt::Debug;
use arena::TypedArena;
use std::cell::RefCell;
use std::rc::Rc;
use std::io::Write;
use std::string::ToString;
use ast::RenderErr;

pub trait BlockStatement<'a> : Debug + Transform<'a> {}

#[derive(Debug, Clone)]
pub struct SetValueStatement<'a> {
    pub name : String,
    pub values : Vec<&'a Value<'a>>
}

#[derive(Debug)]
pub struct PlainSetValueStatement {
    pub name: String,
    pub values: Vec<ScopeValue>
}

impl <'a> BlockStatement<'a> for SetValueStatement<'a> {}
impl <'a> Transform<'a> for SetValueStatement<'a> {
    fn transform<'t>(&'a self, parent_scope: Rc<RefCell<ScopeData>>, transformed_arena: &'t TypedArena<TransformedNode<'t>>)
        -> Result<&'t TransformedNode<'t>, TransformErr> {
        let mut transformed_values: Vec<ScopeValue> = vec![];
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

impl TransformResult for PlainSetValueStatement {
    fn render(&self, buf: &mut Write) -> Result<(), RenderErr> {
        buf.write(self.name.as_ref())?;
        for val in &self.values {
            buf.write(" ".as_ref())?;
            val.render(buf)?;
        }
        buf.write("\n".as_ref())?;
        Ok(())
    }
}

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
impl <'a> Transform<'a> for ConditionStatement<'a> {
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

impl TransformResult for PlainConditionStatement {
    fn render(&self, buf: &mut Write) -> Result<(), RenderErr> {
        buf.write(self.name.as_ref())?;
        buf.write(" ".as_ref())?;
        self.condition.operator.render(buf)?;
        buf.write(" ".as_ref())?;
        self.condition.value.render(buf)?;
        buf.write("\n".as_ref())?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Condition<'a> {
    pub value: &'a Value<'a>,
    pub operator: ComparisonOperator
}

#[derive(Debug, Clone)]
pub struct PlainCondition {
    pub value: ScopeValue,
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

impl TransformResult for ComparisonOperator {
    fn render(&self, buf: &mut Write) -> Result<(), RenderErr> {
        buf.write(match *self {
            ComparisonOperator::Eql => "=",
            ComparisonOperator::Gt => ">",
            ComparisonOperator::Gte => ">=",
            ComparisonOperator::Lt => "<",
            ComparisonOperator::Lte => "<=",
        }.as_ref())?;
        Ok(())
    }
}
