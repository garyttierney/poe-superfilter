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
use std::ops::Deref;

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
        let root_ctx = TransformContext {
            scope: root_scope,
            transform_arena: &self.transformed_arena,
            ast_arena: ast_arena,
        };

        self.transform(root_ctx)
    }
}

impl <'a> Transform<'a> for Filter<'a> {
    fn transform(&self, ctx: TransformContext<'a>)
                 -> Result<Option<&'a TransformedNode<'a>>, CompileErr> {
        let mut transformed_nodes : Vec<&TransformedNode<'a>> =  Vec::with_capacity(self.nodes.len());

        for node in &self.nodes {
            if let Some(t_node) = node.transform(ctx.clone())? {
                transformed_nodes.push(t_node);
            }
        }
        return Ok(Some(
            ctx.alloc_transformed(TransformedNode::Root(transformed_nodes))
        ));
    }
}

macro_rules! as_ref {
    ( #[$meta:meta] pub enum $name:ident<'ast> as $trai:ty { $( $variant:ident($arg:ty) ),* } ) => (
        #[$meta]
        pub enum $name<'ast> { $( $variant ( $arg ) ),* }
        impl <'ast> $name<'ast> {
            pub fn as_ref(&self) -> &($trai) {
                match *self {
                    $( $name::$variant ( ref x ) => x as &$trai ),*
                }
            }
        }
    )
}

/// Data structure to holds all node types that can occur in the acstract syntax tree.
/// The InnerTransform derivation implements the Deref trait for this enum so the transform
/// method can be called directly on the node and the inner value will receive the method call.
#[derive(Debug,InnerTransform)]
pub enum Node<'ast> {
    Filter(Filter<'ast>),
    Block(block::Block<'ast>),
    SetValueStmt(SetValueStatement<'ast>),
    ConditionStmt(ConditionStatement<'ast>),
    StringBox(StringBox),
    ValueExpr(ValueExpression<'ast>),
    VarRef(VarReference),
    VarDefinition(VarDefinition<'ast>),
    Mixin(Mixin<'ast>),
    MixinCall(MixinCall<'ast>),
    Color(color::Color<'ast>)
}

/// Holds fully transformed nodes that can be rendered.
/// Calls to methods of the TransformResult trait on the inner values can be performed with no
/// explicit conversion because this struct implements Deref into &TransformResult.
#[derive(Debug,Clone,PartialEq,InnerTransformResult)]
pub enum TransformedNode<'ast> {
    Root(Vec<&'ast TransformedNode<'ast>>),
    Block(block::PlainBlock<'ast>),
    SetValueStmt(PlainSetValueStatement),
    ConditionStmt(PlainConditionStatement),
    Value(ScopeValue),
    ExpandedNodes(Vec<&'ast TransformedNode<'ast>>)
}

/// Implements rendering for lists of nodes by rendering each item in the list
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
