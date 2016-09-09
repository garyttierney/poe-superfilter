
#[derive(Debug)]
pub enum Block {
    Show(Vec<Statement>),
    Hide(Vec<Statement>),
    Head(Vec<Statement>),
    Mixin(MixinSpec, Vec<Statement>),
}

#[derive(Debug)]
pub struct MixinSpec {
    pub name: String,
    pub parameters: Vec<Param>,
}

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub default: Option<Value>
}

#[derive(Debug)]
pub struct MixinCall {
    pub name: String,
    pub parameters: Vec<Value>
}

#[derive(Debug)]
pub enum NumberBox {
    IntValue(i32),
    Decimal(f32),
    Var(String)
}

#[derive(Debug)]
pub enum StringBox {
    Value(String),
    Var(String)
}

#[derive(Debug)]
pub struct VarDefinition {
    pub identifier: String,
    pub values: Vec<Value>
}

#[derive(Debug)]
pub enum Statement {
    SetValue(String, Vec<Value>),
    Condition(String, Condition),
    Var(VarDefinition),
    Include(MixinCall),
    Import(String),
}

#[derive(Debug)]
pub enum Value {
    Num(NumberExpression),
    Str(StringBox),
    Var(String),
    Color(Color)
}

#[derive(Debug)]
pub struct Color {
    pub r: NumberExpression,
    pub g: NumberExpression,
    pub b: NumberExpression,
    pub a: NumberExpression
}

#[derive(Debug)]
pub struct Condition {
    pub value: Value,
    pub operator: ComparisonOperator
}

#[derive(Debug)]
pub enum ComparisonOperator {
    Eq,
    Lt,
    Lte,
    Gt,
    Gte
}

#[derive(Debug)]
pub enum NumberExpression {
    Number(NumberBox),
    Op(Box<NumberExpression>, NumberOperation, Box<NumberExpression>)
}

#[derive(Debug)]
pub enum NumberOperation {
    Mul,
    Div,
    Add,
    Sub
}