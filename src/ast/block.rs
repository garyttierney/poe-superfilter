use crate::ast::block_statements::*;
use crate::ast::expression::*;
use crate::ast::transform::{RenderContext, Transform, TransformContext, TransformResult};
use crate::ast::{AstLocation, Comment, TransformedNode};
use crate::errors::Result;
use crate::scope::{ScopeData, ScopeValue};
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

/// Top level statements, can be Blocks of instructions
/// like Show/Hide/Mixin or single top-level statements,
/// e.g. variable definitions and imports
#[derive(Debug, Clone)]
pub struct Block {
    pub nodes: Vec<BlockStatement>,
    pub variant: BlockType,
    pub location: AstLocation,
    pub condition: Option<Box<ExpressionNode>>,
    pub block_comments: Vec<Comment>,
    pub inline_comment: Option<Comment>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BlockType {
    Show,
    Hide,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlainBlock {
    pub variant: BlockType,
    pub nodes: Vec<TransformedNode>,
    pub block_comments: Vec<Comment>,
    pub inline_comment: Option<Comment>,
}

trait NodeList {
    fn push_statement(&mut self, value: TransformedNode);
}

impl NodeList for Vec<TransformedNode> {
    fn push_statement(&mut self, value: TransformedNode) {
        if let TransformedNode::ConditionStmt(_) = value {
            self.push(value);
        } else if let Some(index) = self.iter().position(|other| *other == value) {
            // replace existing same statement if possible
            self[index] = value;
        } else {
            self.push(value);
        }
    }
}

impl Transform for Block {
    fn transform(&self, ctx: TransformContext) -> Result<Option<TransformedNode>> {
        if let Some(ref condition) = self.condition {
            let condition_result = condition
                .transform(ctx.clone())?
                .and_then(|transformed_node| Some(transformed_node.return_value()));

            if let Some(ScopeValue::Bool(b)) = condition_result {
                if !b {
                    // Block condition returned false, return nothing
                    return Ok(None);
                }
            }
        }

        let block_ctx = TransformContext {
            scope: Rc::new(RefCell::new(ScopeData::new(Some(ctx.scope.clone())))),
            path: ctx.path.clone(),
        };

        // collect transformed statements from lines in this block
        let mut t_statements: Vec<TransformedNode> = Vec::new();

        for statement in &self.nodes {
            if let Some(t_statement) = statement.transform(block_ctx.clone())? {
                match t_statement {
                    TransformedNode::ExpandedNodes(resolved_stmts) => {
                        for stmt in resolved_stmts {
                            t_statements.push_statement(stmt);
                        }
                    }
                    _ => {
                        t_statements.push_statement(t_statement);
                    }
                }
            }
        }

        Ok(Some(TransformedNode::Block(PlainBlock {
            nodes: t_statements,
            variant: self.variant,
            block_comments: self.block_comments.clone(),
            inline_comment: self.inline_comment.clone(),
        })))
    }

    fn location(&self) -> AstLocation {
        unimplemented!()
    }
}

impl TransformResult for PlainBlock {
    fn render(&self, ctx: RenderContext, buf: &mut dyn Write) -> Result<()> {
        if ctx.config.pretty {
            buf.write_all(ctx.config.line_ending)?;
        }

        for comment in &self.block_comments {
            comment.render(ctx, buf)?;
        }

        buf.write_all(match self.variant {
            BlockType::Show => b"Show",
            BlockType::Hide => b"Hide",
        })?;

        self.inline_comment.render(ctx, buf)?;

        for n in &self.nodes {
            n.render(ctx.increase_indent(), buf)?;
        }

        Ok(())
    }
}
