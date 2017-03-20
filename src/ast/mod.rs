//! Abstract syntax tree structures


pub mod transform;
pub mod mixin;
pub mod strings;
pub mod numbers;
pub mod block_statements;
pub mod var;
pub mod color;
pub mod block;

use ast::strings::*;
use ast::numbers::*;
use ast::block_statements::*;
use ast::mixin::*;
use ast::var::*;
use ast::transform::{Transform, TransformResult};
use scope::{ScopeData, ScopeValue};
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::Write;
use std::io::Error as IoError;
use std::convert::From;
use arena::TypedArena;

pub trait Value<'a> : Debug + Transform<'a> {}

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
    Block(block::Block<'ast>),
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
    Color(color::Color<'ast>),
}

#[derive(Debug)]
pub enum TransformedNode<'ast> {
    Empty,
    Root(Vec<&'ast TransformedNode<'ast>>),
    Block(block::PlainBlock<'ast>),
    SetValueStmt(PlainSetValueStatement),
    ConditionStmt(PlainConditionStatement),

    Value(ScopeValue)
}

impl <'a> Transform<'a> for Node<'a> {
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
            Node::StringBox(ref n) => n.transform(parent_scope, transformed_arena),
            ref node => {
                println!("Unimplemented node type: {:?}", node);
                unimplemented!();
            }
        }
    }
}
impl <'a> Value<'a> for Node<'a> {}
impl <'a> BlockStatement<'a> for Node<'a> {}

impl <'ast> TransformResult for TransformedNode<'ast> {
    fn return_value(&self) -> ScopeValue {
        match *self {
            // TODO: use a macro for this.
            TransformedNode::Empty => ScopeValue::None,
            TransformedNode::Root(_) => ScopeValue::None,
            TransformedNode::Block(ref node) => node.return_value(),
            TransformedNode::SetValueStmt(ref node) => node.return_value(),
            TransformedNode::ConditionStmt(ref node) => node.return_value(),
            TransformedNode::Value(ref node) => node.return_value()
        }
    }

    fn render(&self, buf: &mut Write) -> Result<(), RenderErr> {
        match *self {
            // TODO: use a macro for this.
            TransformedNode::Root(ref node) => node.render(buf)?,
            TransformedNode::Block(ref node) => node.render(buf)?,
            TransformedNode::SetValueStmt(ref node) => node.render(buf)?,
            TransformedNode::ConditionStmt(ref node) => node.render(buf)?,
            TransformedNode::Value(ref node) => node.render(buf)?,
            TransformedNode::Empty => ()
        }
        Ok(())
    }
}

impl <'ast> TransformResult for Vec<&'ast TransformedNode<'ast>> {
    fn render(&self, buf: &mut Write) -> Result<(), RenderErr> {
        for node in self {
            node.render(buf)?;
        }
        buf.write("\n".as_ref())?;
        Ok(())
    }
}

/// Variable definition
#[derive(Debug, Clone)]
pub struct VarDefinition<'a> {
    pub identifier: String,
    pub values: Vec<&'a Value<'a>>
}

impl <'a> BlockStatement<'a> for VarDefinition<'a> {}

impl <'a> Transform<'a> for VarDefinition<'a> {
    fn transform<'t>(&'a self, parent_scope: Rc<RefCell<ScopeData>>, transformed_arena: &'t TypedArena<TransformedNode<'t>>)
        -> Result<&'t TransformedNode<'t>, TransformErr> {
        for val in &self.values {
            let t_val = val.transform(parent_scope.clone(), transformed_arena)?;
            parent_scope.borrow_mut().push_var(self.identifier.clone(), t_val.return_value());
        }
        Ok(transformed_arena.alloc(TransformedNode::Empty))
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum RenderErr {
        Io(inner: IoError) {
            description("IO Error")
            display("IO Error: {}", inner)
            cause(inner)
        }
    }
}

impl From<IoError> for RenderErr {
    fn from(io_err: IoError) -> RenderErr {
        RenderErr::Io(io_err)
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum TransformErr {
        Unknown {
            description("Unknown error")
        }
        TypeMismatch(expected: &'static str, actual: &'static str, identifier: String) {
            description("Type mismatch")
            display("Type mismatch: Expected {} to be {}, but got {}", identifier, expected, actual)
        }
        MissingVarRef(identifier: String) {
            description("Missing variable reference")
            display("Unresolved variable reference: {}", identifier)
        }
    }
}