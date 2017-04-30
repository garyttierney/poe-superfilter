use ast::{Node, TransformedNode, CompileErr, AstLocation};
use ast::transform::{Transform, TransformResult, TransformContext, RenderContext};
use scope::{ScopeData, ScopeValue};
use std::rc::Rc;
use std::cell::RefCell;
use std::io::Write;

/// Top level statements, can be Blocks of instructions
/// like Show/Hide/Mixin or single top-level statements,
/// e.g. variable definitions and imports
#[derive(Debug, Clone)]
pub struct  Block {
    pub nodes: Vec<Node>,
    pub variant: BlockType,
    pub location: AstLocation,
    pub condition: Option<Box<Node>>
}

#[derive(Debug, Clone)]
pub enum BlockType {
    Show,
    Hide,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlainBlock {
    Show(Vec<TransformedNode>),
    Hide(Vec<TransformedNode>),
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
    fn transform(&self, ctx: TransformContext)
            -> Result<Option<TransformedNode>, CompileErr> {
        if let Some(ref condition) = self.condition {
            let condition_result = condition
                .transform(ctx.clone())?
                .and_then(|transformed_node| Some(transformed_node.return_value()));

            if let Some(ScopeValue::Bool(b)) = condition_result {
                if !b {
                    return Ok(None)
                }
            }
        }

        let block_ctx = TransformContext {
            scope: Rc::new(RefCell::new(
                ScopeData::new(Some(ctx.scope.clone()))
            )),
            path: ctx.path.clone(),
        };

        // collect transformed statements from lines in this block
        let mut t_statements : Vec<TransformedNode> = Vec::new();

        for statement in &self.nodes {
            if let Some(t_statement) = statement.transform(block_ctx.clone())? {
                match t_statement {
                    TransformedNode::ExpandedNodes(resolved_stmts) => {
                        for stmt in resolved_stmts {
                            t_statements.push_statement(stmt);
                        }
                    },
                    _ => {
                        t_statements.push_statement(t_statement);
                    }
                }
            }
        }

        Ok(Some(TransformedNode::Block(
            match self.variant {
                BlockType::Show => PlainBlock::Show(t_statements),
                BlockType::Hide => PlainBlock::Hide(t_statements)
            }
        )))
    }

    fn location(&self) -> AstLocation {
        unimplemented!()
    }
}

impl TransformResult for PlainBlock {
    fn render(&self, ctx: RenderContext, buf: &mut Write) -> Result<(), CompileErr> {
        let nodes = match *self {
            PlainBlock::Show(ref nodes) => {
                buf.write(b"Show")?;
                buf.write(ctx.config.line_ending)?;
                nodes
            },
            PlainBlock::Hide(ref nodes) => {
                buf.write(b"Hide")?;
                buf.write(ctx.config.line_ending)?;
                nodes
            }
        };
        for n in nodes {
            n.render(ctx.increase_indent(), buf)?;
        }
        Ok(())
    }
}
