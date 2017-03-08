//! Abstract syntax tree structures

pub mod expressions;
pub mod mixin;
pub mod strings;
pub mod numbers;
pub mod block_statements;

use self::strings::*;
use self::numbers::*;
use self::block_statements::*;
use self::mixin::*;

/// Top level statements, can be Blocks of instructions
/// like Show/Hide/Mixin or single top-level statements,
/// e.g. variable definitions and imports
#[derive(Debug, Clone)]
pub enum Block {
    Show(Vec<Statement>),
    Hide(Vec<Statement>),
    Mixin(Mixin),
    Var(VarDefinition),
    Import(String),
}

/// Variable definition
#[derive(Debug, Clone)]
pub struct VarDefinition {
    pub identifier: String,
    pub values: Vec<Value>
}

impl BlockStatement for VarDefinition {}

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
