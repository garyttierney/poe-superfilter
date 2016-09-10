
#[derive(Debug, Clone)]
pub enum Block {
    Show(Vec<Statement>),
    Hide(Vec<Statement>),
    Mixin(MixinSpec, Vec<Statement>),
    Var(VarDefinition),
    Import(String),
}

#[derive(Debug, Clone)]
pub struct MixinSpec {
    pub name: String,
    pub parameters: Vec<Param>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub default: Option<Value>
}

#[derive(Debug, Clone)]
pub struct MixinCall {
    pub name: String,
    pub parameters: Vec<Value>
}

#[derive(Debug, Clone)]
pub enum NumberBox {
    IntValue(i32),
    Decimal(f32),
    Var(String)
}

#[derive(Debug, Clone)]
pub enum StringBox {
    Value(String),
    Var(String)
}

#[derive(Debug, Clone)]
pub struct VarDefinition {
    pub identifier: String,
    pub values: Vec<Value>
}

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
    Eq,
    Lt,
    Lte,
    Gt,
    Gte
}

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