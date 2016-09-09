
#[derive(Debug)]
pub enum Block {
    Show(Vec<FilterInstruction>),
    Hide(Vec<FilterInstruction>),
    Definitions(Vec<VarDefinition>),
    Mixin {
        name: String,
        parameters: Vec<String>,
        instructions: Vec<FilterInstruction>
    }
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
pub enum FilterInstruction {
    SetValue(String, Vec<Value>),
    Condition(String, Condition),
    Var(VarDefinition)
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