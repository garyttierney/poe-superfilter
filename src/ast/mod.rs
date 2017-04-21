//! Abstract syntax tree structures


pub mod transform;
pub mod mixin;
pub mod block_statements;
pub mod var;
pub mod color;
pub mod block;
pub mod import;
pub mod expression;

use ast::block_statements::*;
use ast::mixin::*;
use ast::var::*;
use ast::import::ImportStatement;
use ast::transform::{Transform, TransformResult, TransformContext, RenderContext};
use scope::{ScopeData, ScopeValue};
use ast::expression::{ExpressionNode, ExpressionValue};

use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::Write;
use std::io::Error as IoError;
use std::convert::From;

use std::fmt;
use std::fmt::Formatter;
use std::fmt::Error as FmtError;

use lalrpop_util::ParseError;
use tok::Tok;
use tok::Location as TokenLocation;
use std::ops::Deref;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Filter {
    pub nodes: Vec<Node>,
    pub location: AstLocation
}

impl Debug for Filter {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.nodes.fmt(f)?;
        Ok(())
    }
}

impl Filter {
    pub fn transform_begin(&self,
                           root_scope: Rc<RefCell<ScopeData>>,
                           base_path: Rc<PathBuf>)
                           -> Result<Option<TransformedNode>, CompileErr> {
        let root_ctx = TransformContext {
            scope: root_scope,
            path: base_path
        };

        self.transform(root_ctx)
    }
}

impl Transform for Filter {
    fn transform(&self, ctx: TransformContext)
                 -> Result<Option<TransformedNode>, CompileErr> {
        let mut transformed_nodes: Vec<TransformedNode> = Vec::with_capacity(self.nodes.len());

        for node in &self.nodes {
            if let Some(t_node) = node.transform(ctx.clone())? {
                transformed_nodes.push(t_node);
            }
        }
        return Ok(Some(
            TransformedNode::Root(transformed_nodes)
        ));
    }

    fn location(&self) -> AstLocation {
        self.location.clone()
    }
}

/// Data structure to holds all node types that can occur in the acstract syntax tree.
/// The InnerTransform derivation implements the Deref trait for this enum so the transform
/// method can be called directly on the node and the inner value will receive the method call.
#[derive(Debug, InnerTransform, Clone)]
pub enum Node {
    Filter(Filter),
    Import(ImportStatement),
    Block(block::Block),
    SetValueStmt(SetValueStatement),
    ConditionStmt(ConditionStatement),

    Expression(ExpressionNode),
    Value(ExpressionValue),

    VarRef(VarReference),
    VarDefinition(VarDefinition),
    Mixin(Mixin),
    MixinCall(MixinCall),
    Color(color::Color)
}

/// Holds fully transformed nodes that can be rendered.
/// Calls to methods of the TransformResult trait on the inner values can be performed with no
/// explicit conversion because this struct implements Deref into &TransformResult.
#[derive(Debug, Clone, PartialEq, InnerTransformResult)]
pub enum TransformedNode {
    Root(Vec<TransformedNode>),
    Block(block::PlainBlock),
    SetValueStmt(PlainSetValueStatement),
    ConditionStmt(PlainConditionStatement),
    Value(ScopeValue),
    ExpandedNodes(Vec<TransformedNode>)
}

/// Implements rendering for lists of nodes by rendering each item in the list
impl TransformResult for Vec<TransformedNode> {
    fn render(&self, ctx: RenderContext, buf: &mut Write) -> Result<(), CompileErr> {
        for node in self {
            node.render(ctx, buf)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct AstLocation {
    pub begin: TokenLocation,
    pub end: TokenLocation,
    pub file: String
}

impl AstLocation {
    pub fn new(l: TokenLocation, r: TokenLocation, file: String)
               -> AstLocation {
        AstLocation {
            begin: l,
            end: r,
            file: file
        }
    }
}

impl fmt::Display for AstLocation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: {}:{} - {}:{}", self.file, self.begin.line, self.begin.pos, self.end.line, self.end.pos)
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum CompileErr {
        Unknown {
            description("Unknown error")
        }
        TypeMismatch(expected: &'static str, actual: &'static str, identifier: String, location: AstLocation) {
            description("Type mismatch")
            display("Type mismatch: Expected {} to be {}, but got {} at {}", identifier, expected, actual, location)
        }
        MissingVarRef(identifier: String, location: AstLocation) {
            description("Missing variable reference")
            display("Unresolved variable reference: {} at {}", identifier, location)
        }
        MissingValue(node: String, location: AstLocation) {
            description("Expected expression to return value, none found")
            display("Expected Expression to return a value: {:?} at {}", node, location)
        }
        WrongParameterCount(node: String, expected: usize, actual: usize, location: AstLocation) {
            description("Wrong mixin call parameter count")
            display("Wrong mixin call parameter count in {}: expected {}, got {} at {}", node, expected, actual, location)
        }
        ImportError(node: String, location: AstLocation) {
            description("Invalid import path")
            display("Invalid import expression: {:?} at {}", node, location)
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
