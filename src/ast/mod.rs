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
use errors::*;

use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::io::Write;

use std::fmt;
use std::fmt::Formatter;
use std::fmt::Error as FmtError;

use tok::Location as TokenLocation;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Filter {
    pub nodes: Vec<Node>,
    pub location: AstLocation
}

impl Debug for Filter {
    fn fmt(&self, f: &mut Formatter) -> ::std::result::Result<(), FmtError> {
        self.nodes.fmt(f)?;
        Ok(())
    }
}

impl Filter {
    pub fn transform_begin(&self,
                           root_scope: Rc<RefCell<ScopeData>>,
                           base_path: Rc<PathBuf>)
                           -> Result<Option<TransformedNode>> {
        let root_ctx = TransformContext {
            scope: root_scope,
            path: base_path
        };

        self.transform(root_ctx)
    }
}

impl Transform for Filter {
    fn transform(&self, ctx: TransformContext)
                 -> Result<Option<TransformedNode>> {
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
#[derive(Debug, Transform, Clone)]
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
#[derive(Debug, Clone, PartialEq, TransformResult)]
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
    fn render(&self, ctx: RenderContext, buf: &mut Write) -> Result<()> {
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
    pub file: Arc<PathBuf>
}

impl AstLocation {
    pub fn new(begin: TokenLocation, end: TokenLocation, file: Arc<PathBuf>)
               -> AstLocation {
        AstLocation { begin, end, file: file.clone() }
    }
}

impl fmt::Display for AstLocation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}: {}:{} - {}:{}", self.file, self.begin.line, self.begin.pos, self.end.line, self.end.pos)
    }
}
