//! Abstract syntax tree structures

/// Top level statements, can be Blocks of instructions
/// like Show/Hide/Mixin or single top-level statements,
/// e.g. variable definitions and imports
#[derive(Debug, Clone)]
pub enum Block {
    Show(Vec<Statement>),
    Hide(Vec<Statement>),
    Mixin(MixinSpec, Vec<Statement>),
    Var(VarDefinition),
    Import(String),
}

/// Name and parameter specs for a mixin 
#[derive(Debug, Clone)]
pub struct MixinSpec {
    pub name: String,
    pub parameters: Vec<Param>,
}

/// (Mixin) Parameter name and default values
#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub default: Option<Value>
}

/// Represents a mixin include with name and
/// parameters
#[derive(Debug, Clone)]
pub struct MixinCall {
    pub name: String,
    pub parameters: Vec<Value>
}

/// Number value or variable reference
#[derive(Debug, Clone)]
pub enum NumberBox {
    IntValue(i32),
    Decimal(f32),
    Var(String)
}

/// String value or variable reference
#[derive(Debug, Clone)]
pub enum StringBox {
    Value(String),
    Var(String)
}

/// Variable definition
#[derive(Debug, Clone)]
pub struct VarDefinition {
    pub identifier: String,
    pub values: Vec<Value>
}

/// Any statement that can occur inside of a Show/Hide/Mixin block
#[derive(Debug, Clone)]
pub enum Statement {
    SetValue(String, Vec<Value>),
    Condition(String, Condition),
    Var(VarDefinition),
    Include(MixinCall),
}

#[derive(Debug, Clone)]
pub enum Value {
    Num(NumberExpression),
    Str(StringBox),
    Var(String),
    Color(Color)
}

#[derive(Debug, Clone)]
pub struct Color {
    pub r: NumberExpression,
    pub g: NumberExpression,
    pub b: NumberExpression,
    pub a: NumberExpression
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

/// Number value or expression
#[derive(Debug, Clone)]
pub enum NumberExpression {
    Number(NumberBox),
    Op(Box<NumberExpression>, NumberOperation, Box<NumberExpression>)
}

#[derive(Debug, Clone)]
pub enum NumberOperation {
    Mul,
    Div,
    Add,
    Sub
}