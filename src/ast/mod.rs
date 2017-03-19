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

use arena::TypedArena;

pub trait Value<'a> : Debug + Expression<'a> {}

#[derive(Debug)]
pub struct Filter<'ast> {
    pub nodes: Vec<&'ast Node<'ast>>
}

impl <'ast> Filter<'ast> {
    pub fn transform<'t>(&self, transformed_arena : &'t TypedArena<TransformedNode<'t>>)
        -> &'t TransformedNode<'t> {
        //self.nodes.iter().map(|node| node.transform());
        return transformed_arena.alloc(TransformedNode::Root(vec![]));
    }
}

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

#[derive(Debug)]
pub enum TransformedNode<'ast> {
    Root(Vec<&'ast TransformedNode<'ast>>),
    Block(PlainBlock<'ast>),
    SetValueStmt(PlainSetValueStatement),

    String(String)
}

impl <'a> Expression<'a> for Node<'a> {
    fn transform<'t>(&'a self, parent_scope: Rc<RefCell<ScopeData>>, transformed_arena: &'t TypedArena<TransformedNode<'t>>)
        -> Result<&'t TransformedNode<'t>, TransformErr> {
        // TODO: make this a macro instead of
        match self {
            &Node::Block(ref n) => n.transform(parent_scope, transformed_arena),
            &Node::SetValueStmt(ref n) => n.transform(parent_scope, transformed_arena),
            _ => unimplemented!()
        }
    }
}
impl <'a> Value<'a> for Node<'a> {}
impl <'a> BlockStatement<'a> for Node<'a> {}

impl <'ast> TransformedExpression for TransformedNode<'ast> {
}

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

#[derive(Debug, Clone)]
pub enum PlainBlock<'a> {
    Show(Vec<&'a TransformedNode<'a>>),
    Hide(Vec<&'a TransformedNode<'a>>),
}

impl <'a> Expression<'a> for Block<'a> {
    fn transform<'t>(&'a self, parent_scope: Rc<RefCell<ScopeData>>, transformed_arena: &'t TypedArena<TransformedNode<'t>>)
        -> Result<&'t TransformedNode<'t>, TransformErr> {
        let block_scope = Rc::new(RefCell::new(ScopeData::new(Some(parent_scope))));

        // collect transformed statements from lines in this block
        let mut t_statements : Vec<&TransformedNode> = Vec::new();
        match self {
            &Block::Show(ref statements) | &Block::Hide(ref statements) => {
                for statement in statements {
                    t_statements.push(statement.transform(block_scope.clone(), transformed_arena)?);
                }
            }
            // TODO: handle mixin blocks, var def blocks
            _ => unimplemented!()
        }

        Ok(transformed_arena.alloc(TransformedNode::Block(
            match self {
                &Block::Show(_) => PlainBlock::Show(t_statements),
                &Block::Hide(_) => PlainBlock::Hide(t_statements),
                _ => unimplemented!()
            }
        )))
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
