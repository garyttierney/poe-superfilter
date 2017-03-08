
use ast::Value;
use ast::expressions::Expression;
use ast::mixin::MixinCall;
use ast::VarDefinition;
use translate::TransformErr;
use translate::ScopeData;

/// Any statement that can occur inside of a Show/Hide/Mixin block
#[derive(Debug, Clone)]
pub enum Statement {
    SetValue(SetValueStatement),
    Condition(ConditionStatement),
    Var(VarDefinition),
    Include(MixinCall),
}

pub trait BlockStatement {}

#[derive(Debug, Clone)]
pub struct SetValueStatement {
    pub name : String,
    pub values : Vec<Value>
}

impl BlockStatement for SetValueStatement {}
impl <'a> Expression<'a> for SetValueStatement {
    fn transform(&mut self, parent_scope: &ScopeData) -> Result<(), TransformErr<'a>> {
        unimplemented!()
    }

    fn render(&'a self) -> Result<String, TransformErr<'a>> {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct ConditionStatement {
    pub name : String,
    pub condition : Condition
}

impl BlockStatement for ConditionStatement {}
impl <'a> Expression<'a> for ConditionStatement {
    fn transform(&mut self, parent_scope: &ScopeData) -> Result<(), TransformErr<'a>> {
        unimplemented!()
    }

    fn render(&'a self) -> Result<String, TransformErr<'a>> {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct Condition {
    pub value: Value,
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
