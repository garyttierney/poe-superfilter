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
use ast::import::*;
use ast::transform::*;
use ast::block::*;
use scope::*;
use errors::Result;

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
    pub nodes: Vec<BlockLevelNode>,
    pub location: AstLocation
}

#[derive(Debug, Clone, Transform)]
pub enum BlockLevelNode {
    Block(Block),
    Mixin(Mixin),
    Import(ImportStatement),
    VarDef(VarDefinition),
    Comment(Comment)
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
        Ok(Some(
            TransformedNode::Root(transformed_nodes)
        ))
    }

    fn location(&self) -> AstLocation {
        self.location.clone()
    }
}

/// Holds fully transformed nodes that can be rendered.
/// Calls to methods of the `TransformResult` trait on the inner values can be performed with no
/// explicit conversion because this struct implements Deref into `&TransformResult`.
#[derive(Debug, Clone, PartialEq, TransformResult)]
pub enum TransformedNode {
    Root(Vec<TransformedNode>),
    Block(PlainBlock),
    SetValueStmt(PlainSetValueStatement),
    ConditionStmt(PlainConditionStatement),
    Value(ScopeValue),
    ExpandedNodes(Vec<TransformedNode>),
    Comment(Comment)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    pub content: String,
    pub inline: bool
}

impl TransformResult for Comment {
    fn render(&self, ctx: RenderContext, buf: &mut Write) -> Result<()> {
        if ctx.config.comments {
            if !self.inline { ctx.write_indent(buf)?; }
            buf.write_all(b"#")?;
            buf.write_all(self.content.as_ref())?;
        }
        buf.write_all(ctx.config.line_ending)?;
        Ok(())
    }
}

impl Transform for Comment {
    #[allow(unused_variables)]
    fn transform(&self, ctx: TransformContext) -> Result<Option<TransformedNode>> {
        Ok(Some(
            TransformedNode::Comment(self.clone())
        ))
    }

    fn location(&self) -> AstLocation {
        unimplemented!()
    }
}

impl TransformResult for Option<Comment> {
    fn render(&self, ctx: RenderContext, buf: &mut Write) -> Result<()> {
        match *self {
            Some(ref comment) => {
                if ctx.config.comments {
                    buf.write_all(b" ")?;
                    comment.render(ctx, buf)
                } else {
                    buf.write_all(ctx.config.line_ending)?;
                    Ok(())
                }
            },
            None => {
                buf.write_all(ctx.config.line_ending)?;
                Ok(())
            }
        }
    }
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
