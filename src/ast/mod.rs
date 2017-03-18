//! Abstract syntax tree structures

pub mod expressions;
pub mod mixin;
pub mod strings;
pub mod numbers;
pub mod block_statements;
pub mod var;

use ast::strings::*;
use ast::numbers::*;
use ast::block_statements::*;
use ast::mixin::*;
use ast::var::*;
use ast::expressions::{Expression, TransformedExpression};
use translate::{TransformErr, ScopeData, ExpressionValue};
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;

pub trait Value<'a> : Debug + Expression<'a> {}

#[derive(Debug)]
pub enum Node<'ast> {
    Block(Block<'ast>),
    SetValueStmt(SetValueStatement<'ast>),
    ConditionStmt(ConditionStatement<'ast>),
    Condition(Condition<'ast>),
    ComparisonOp(ComparisonOperator),

    StringBox(StringBox),

    NumOperation(NumberOperation),
    NumBox(NumberBox),
    NumExpression(NumberExpression<'ast>),

    VarRef(VarReference),
    VarDefinition(VarDefinition<'ast>),

    MixinCall(MixinCall<'ast>),
    Color(Color<'ast>),
}

impl <'a> Expression<'a> for Node<'a> {}
impl <'a> Value<'a> for Node<'a> {}
impl <'a> BlockStatement<'a> for Node<'a> {}

/// Top level statements, can be Blocks of instructions
/// like Show/Hide/Mixin or single top-level statements,
/// e.g. variable definitions and imports
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Show(Vec<&'a BlockStatement<'a>>),
    Hide(Vec<&'a BlockStatement<'a>>),
    Mixin(Mixin<'a>),
    Var(VarDefinition<'a>),
    Import(String),
}

pub struct Arena<'ast> {
    data: RefCell<Vec<Box<Node<'ast>>>>
}

impl<'ast> Arena<'ast> {
    pub fn new() -> Arena<'ast> {
        Arena { data: RefCell::new(vec![]) }
    }

    pub fn alloc(&'ast self, n: Node<'ast>) -> &'ast Node<'ast> {
        let b = Box::new(n);
        let p: *const Node<'ast> = &*b;
        self.data.borrow_mut().push(b);
        unsafe { &*p }
    }
}

pub struct TransformedBlock<'a> {
    stmts: Vec<Box<TransformedExpression<'a>>>
}

impl <'a> TransformedExpression<'a> for TransformedBlock<'a> {}

impl <'a> Expression<'a> for Block<'a> {
    fn transform<'b, 'c>(&'a self, parent_scope: &'b ScopeData<'b>) -> Result<Rc<TransformedExpression<'c>>, TransformErr> {
        /*let mut block_scope = parent_scope.new_child_scope();

        //let mut t_statements = Vec::new();

        match *self {
            Block::Show(statements) | Block::Hide(statements) => {
                for mut statement in statements {
                    match statement.transform(&block_scope) {
                        Ok(t_expression) => {
                            if let Some(exported_scope) = t_expression.export_scope() {
                                block_scope.extend(exported_scope)
                            }
                        }
                    }
                }
            }
            _ => panic!()
        }
        Ok(Rc::new(TransformedBlock {
            stmts: Vec::new(),
        }))*/
        unimplemented!();
    }
}

/// Variable definition
#[derive(Debug, Clone)]
pub struct VarDefinition<'a> {
    pub identifier: String,
    pub values: Vec<&'a Value<'a>>
}

impl <'a> BlockStatement<'a> for VarDefinition<'a> {}

impl <'a> Expression<'a> for VarDefinition<'a> {
    /*fn transform(&self, parent_scope: &ScopeData) -> Result<Box<TransformedExpression<'a>>, TransformErr> {
        let mut evaluated_expressions : Vec<ExpressionValue> = Vec::new();
        evaluated_expressions.reserve(self.values.len());

        for mut val in self.values.iter_mut() {
            match val.transform(parent_scope) {
                Ok(result) => {
                    evaluated_expressions.push(result.return_value);
                    parent_scope = result.parent_scope;
                },
                Err(e) => return Err(e)
            }
        }

        if evaluated_expressions.len() > 1 {
            parent_scope.push_var(self.identifier.clone(), ExpressionValue::List(evaluated_expressions))
        } else {
            parent_scope.push_var(self.identifier.clone(), evaluated_expressions[0].clone())
        }
        return Ok(TransformResult::returns_nothing(parent_scope))
    }*/
}

#[derive(Debug, Clone)]
pub struct Color<'a> {
    pub r: &'a Node<'a>,
    pub g: &'a Node<'a>,
    pub b: &'a Node<'a>,
    pub a: &'a Node<'a>
}

impl <'a> Value<'a> for Color<'a> {}
impl <'a> Expression<'a> for Color<'a> {}
