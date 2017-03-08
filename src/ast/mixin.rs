use ast::Value;
use ast::block_statements::Statement;
use ast::block_statements::BlockStatement;

/// Name and parameter specs for a mixin
#[derive(Debug, Clone)]
pub struct Mixin {
    pub name: String,
    pub parameters: Vec<Param>,
    pub statements: Vec<Statement>
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

impl BlockStatement for MixinCall {}
