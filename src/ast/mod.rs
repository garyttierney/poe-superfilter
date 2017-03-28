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
use ast::transform::{Transform, TransformResult, TransformContext, RenderContext};
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
use lalrpop_util::ParseError;
use tok::Tok;
use tok::Location as TokenLocation;

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
    pub fn transform_begin(&'a self, ast_arena: &'a TypedArena<Node<'a>>, root_scope: Rc<RefCell<ScopeData<'a>>>)
                           -> Result<Option<&'a TransformedNode<'a>>, CompileErr> {
        let mut transformed_nodes : Vec<&TransformedNode<'a>> =  Vec::with_capacity(self.nodes.len());

        let root_ctx = TransformContext {
            scope: root_scope,
            transform_arena: &self.transformed_arena,
            ast_arena: ast_arena,
        };

        for node in &self.nodes {
            if let Some(t_node) = node.transform(root_ctx.clone())? {
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
    Filter(Filter<'ast>),
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
    ExpandedNodes(Vec<&'ast TransformedNode<'ast>>)
}

impl <'a> Transform<'a> for Node<'a> {
    fn transform(&'a self, ctx: TransformContext<'a>)
        -> Result<Option<&'a TransformedNode<'a>>, CompileErr> {
        // TODO: make this a macro instead of listing all the options manually
        match *self {
            Node::Block(ref n) => n.transform(ctx),
            Node::SetValueStmt(ref n) => n.transform(ctx),
            Node::VarDefinition(ref n) => n.transform(ctx),
            Node::VarRef(ref n) => n.transform(ctx),
            Node::ValueExpr(ref n) => n.transform(ctx),
            Node::ConditionStmt(ref n) => n.transform(ctx),
            Node::StringBox(ref n) => n.transform(ctx),
            Node::Mixin(ref n) => n.transform(ctx),
            Node::MixinCall(ref n) => n.transform(ctx),
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
            TransformedNode::ExpandedNodes(ref node) => node.return_value()
        }
    }

    fn render(&self, ctx: RenderContext, buf: &mut Write) -> Result<(), CompileErr> {
        match *self {
            // TODO: use a macro for this.
            TransformedNode::Root(ref node) => node.render(ctx, buf)?,
            TransformedNode::Block(ref node) => node.render(ctx, buf)?,
            TransformedNode::SetValueStmt(ref node) => node.render(ctx, buf)?,
            TransformedNode::ConditionStmt(ref node) => node.render(ctx, buf)?,
            TransformedNode::Value(ref node) => node.render(ctx, buf)?,
            TransformedNode::ExpandedNodes(ref node) => node.render(ctx, buf)?
        }
        Ok(())
    }
}

impl <'ast> TransformResult for Vec<&'ast TransformedNode<'ast>> {
    fn render(&self, ctx: RenderContext, buf: &mut Write) -> Result<(), CompileErr> {
        for node in self {
            node.render(ctx, buf)?;
        }
        Ok(())
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum CompileErr {
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
        Io(inner: IoError) {
            description("IO Error")
            display("IO Error: {}", inner)
            cause(inner)
        }
        ParseError(inner: ParseError<TokenLocation, Tok, char>) {
            description("Parse Error")
            display("Parse Error: {}", inner)
            cause(inner)
        }
    }
}

impl From<IoError> for CompileErr {
    fn from(io_err: IoError) -> CompileErr {
        CompileErr::Io(io_err)
    }
}

impl From<ParseError<TokenLocation, Tok, char>> for CompileErr {
    fn from(err: ParseError<TokenLocation, Tok, char>) -> CompileErr {
        CompileErr::ParseError(err)
    }
}
