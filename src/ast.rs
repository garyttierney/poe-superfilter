
#[derive(Debug)]
pub enum Block {
    Show(Box<Vec<Instruction>>),
    Hide(Box<Vec<Instruction>>),
    Empty
}

#[derive(Debug)]
pub struct Instruction {
    pub name: String,
    pub value: InstructionExpression
}

#[derive(Debug)]
pub enum InstructionExpression {
    Condition(Condition),
    Value(Value)
}

#[derive(Debug)]
pub enum Value {
    Numbers(Vec<NumberExpression>),
    Color(Color),
    Names(Vec<String>)
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
    Number(i32)
}
