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
        -> Result<&'t TransformedNode<'t>, TransformErr> {
        let mut transformed_nodes : Vec<&'t TransformedNode<'t>> =  Vec::with_capacity(self.nodes.len());
        let root_scope = Rc::new(RefCell::new(ScopeData::new(None)));
        for node in &self.nodes {
            let t_node = try!(node.transform(root_scope.clone(), transformed_arena));
            transformed_nodes.push(t_node);
        }
        return Ok(transformed_arena.alloc(TransformedNode::Root(transformed_nodes)));
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
    Empty,
    Root(Vec<&'ast TransformedNode<'ast>>),
    Block(PlainBlock<'ast>),
    SetValueStmt(PlainSetValueStatement),
    ConditionStmt(PlainConditionStatement),

    Value(ExpressionValue)
}

impl <'a> Expression<'a> for Node<'a> {
    fn transform<'t>(&'a self, parent_scope: Rc<RefCell<ScopeData>>, transformed_arena: &'t TypedArena<TransformedNode<'t>>)
        -> Result<&'t TransformedNode<'t>, TransformErr> {
        // TODO: make this a macro instead of listing all the options manually
        match *self {
            Node::Block(ref n) => n.transform(parent_scope, transformed_arena),
            Node::SetValueStmt(ref n) => n.transform(parent_scope, transformed_arena),
            Node::VarDefinition(ref n) => n.transform(parent_scope, transformed_arena),
            Node::VarRef(ref n) => n.transform(parent_scope, transformed_arena),
            Node::NumExpression(ref n) => n.transform(parent_scope, transformed_arena),
            Node::ConditionStmt(ref n) => n.transform(parent_scope, transformed_arena),
            ref node => {
                println!("Unimplemented node type: {:?}", node);
                unimplemented!();
            }
        }
    }
}
impl <'a> Value<'a> for Node<'a> {}
impl <'a> BlockStatement<'a> for Node<'a> {}

impl <'ast> TransformedExpression for TransformedNode<'ast> {
    fn return_value(&self) -> ExpressionValue {
        match *self {
            TransformedNode::Empty => ExpressionValue::None,
            TransformedNode::Root(ref node) => ExpressionValue::None,
            TransformedNode::Block(ref node) => node.return_value(),
            TransformedNode::SetValueStmt(ref node) => node.return_value(),
            TransformedNode::ConditionStmt(ref node) => node.return_value(),
            TransformedNode::Value(ref node) => node.return_value()
        }
    }
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

impl <'a> TransformedExpression for PlainBlock<'a> {}

/// Variable definition
#[derive(Debug, Clone)]
pub struct VarDefinition<'a> {
    pub identifier: String,
    pub values: Vec<&'a Value<'a>>
}

impl <'a> BlockStatement<'a> for VarDefinition<'a> {}

impl <'a> Expression<'a> for VarDefinition<'a> {
    fn transform<'t>(&'a self, parent_scope: Rc<RefCell<ScopeData>>, transformed_arena: &'t TypedArena<TransformedNode<'t>>)
        -> Result<&'t TransformedNode<'t>, TransformErr> {
        for val in &self.values {
            let t_val = try!(val.transform(parent_scope.clone(), transformed_arena));
            parent_scope.borrow_mut().push_var(self.identifier.clone(), t_val.return_value());
        }
        Ok(transformed_arena.alloc(TransformedNode::Empty))
    }
}

#[derive(Debug, Clone)]
pub struct Color<'a> {
    pub r: &'a Node<'a>,
    pub g: &'a Node<'a>,
    pub b: &'a Node<'a>,
    pub a: &'a Node<'a>
}

impl <'a> Color<'a> {
}

pub struct PlainColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl <'a> Value<'a> for Color<'a> {}
impl <'a> Expression<'a> for Color<'a> {
    fn transform<'t>(&'a self, parent_scope: Rc<RefCell<ScopeData>>, transformed_arena: &'t TypedArena<TransformedNode<'t>>) -> Result<&'t TransformedNode<'t>, TransformErr> {
        unimplemented!();
    }
}
