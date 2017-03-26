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
use std::fmt::Formatter;
use std::fmt::Error as FmtError;

pub struct Filter<'ast> {
    pub nodes: Vec<&'ast Node<'ast>>,
    pub transformed_arena: TypedArena<TransformedNode<'ast>>
}

impl <'ast> Debug for Filter<'ast> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.nodes.fmt(f)?;
        Ok(())
    }
}

impl <'a> Filter<'a> {
    pub fn transform(&'a self)
        -> Result<Option<&'a TransformedNode<'a>>, TransformErr> {
        let mut transformed_nodes : Vec<&TransformedNode<'a>> =  Vec::with_capacity(self.nodes.len());
        let root_scope = Rc::new(RefCell::new(ScopeData::new(None)));
        for node in &self.nodes {
            if let Some(t_node) = try!(node.transform(root_scope.clone(), &self.transformed_arena)) {
                transformed_nodes.push(t_node);
            }
        }
        return Ok(Some(
            self.transformed_arena.alloc(TransformedNode::Root(transformed_nodes))
        ));
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

    ValueExpr(ValueExpression<'ast>),

    VarRef(VarReference),
    VarDefinition(VarDefinition<'ast>),

    Mixin(Mixin<'ast>),
    MixinCall(MixinCall<'ast>),
    Color(color::Color<'ast>),
}

#[derive(Debug,Clone,PartialEq)]
pub enum TransformedNode<'ast> {
    Root(Vec<&'ast TransformedNode<'ast>>),
    Block(block::PlainBlock<'ast>),
    SetValueStmt(PlainSetValueStatement),
    ConditionStmt(PlainConditionStatement),
    Value(ScopeValue),
    ResolvedMixin(ResolvedMixin<'ast>)
}

impl <'a> Transform<'a> for Node<'a> {
    fn transform(&'a self, parent_scope: Rc<RefCell<ScopeData<'a>>>, transformed_arena: &'a TypedArena<TransformedNode<'a>>)
        -> Result<Option<&'a TransformedNode<'a>>, TransformErr> {
        // TODO: make this a macro instead of listing all the options manually
        match *self {
            Node::Block(ref n) => n.transform(parent_scope, transformed_arena),
            Node::SetValueStmt(ref n) => n.transform(parent_scope, transformed_arena),
            Node::VarDefinition(ref n) => n.transform(parent_scope, transformed_arena),
            Node::VarRef(ref n) => n.transform(parent_scope, transformed_arena),
            Node::ValueExpr(ref n) => n.transform(parent_scope, transformed_arena),
            Node::ConditionStmt(ref n) => n.transform(parent_scope, transformed_arena),
            Node::StringBox(ref n) => n.transform(parent_scope, transformed_arena),
            Node::Mixin(ref n) => n.transform(parent_scope, transformed_arena),
            Node::MixinCall(ref n) => n.transform(parent_scope, transformed_arena),
            ref node => {
                println!("Unimplemented node type: {:?}", node);
                unimplemented!();
            }
        }
    }
}

impl <'ast> TransformResult for TransformedNode<'ast> {
    fn return_value(&self) -> ScopeValue {
        match *self {
            // TODO: use a macro for this.
            TransformedNode::Root(_) => ScopeValue::None,
            TransformedNode::Block(ref node) => node.return_value(),
            TransformedNode::SetValueStmt(ref node) => node.return_value(),
            TransformedNode::ConditionStmt(ref node) => node.return_value(),
            TransformedNode::Value(ref node) => node.return_value(),
            TransformedNode::ResolvedMixin(ref node) => node.return_value()
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
            TransformedNode::ResolvedMixin(ref node) => node.render(buf)?
        }
        Ok(())
    }
}

impl <'ast> TransformResult for Vec<&'ast TransformedNode<'ast>> {
    fn render(&self, buf: &mut Write) -> Result<(), RenderErr> {
        for node in self {
            node.render(buf)?;
        }
        Ok(())
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
        MissingValue(node: String) {
            description("Expected expression to return value, none found")
            display("Expected Expression to return a value: {:?}", node)
        }
        WrongParameterCount(node: String, expected: usize, actual: usize) {
            description("Wrong mixin call parameter count")
            display("Wrong mixin call parameter count in {:?}: expected {}, got {}", node, expected, actual)
        }
    }
}