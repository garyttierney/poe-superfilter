use ast::{Node, TransformedNode, CompileErr, AstLocation};
use ast::transform::{Transform, TransformResult, TransformContext, RenderContext};
use scope::ScopeData;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::Write;

/// Top level statements, can be Blocks of instructions
/// like Show/Hide/Mixin or single top-level statements,
/// e.g. variable definitions and imports
#[derive(Debug, Clone)]
pub struct  Block<'a> {
    pub nodes: Vec<&'a Node<'a>>,
    pub variant: BlockType,
    pub location: AstLocation
}

#[derive(Debug, Clone)]
pub enum BlockType {
    Show,
    Hide,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlainBlock<'a> {
    Show(Vec<&'a TransformedNode<'a>>),
    Hide(Vec<&'a TransformedNode<'a>>),
}

trait NodeList<'a> {
    fn push_statement(&mut self, value: &'a TransformedNode);
}

impl <'a> NodeList<'a> for Vec<&'a TransformedNode<'a>> {
    fn push_statement(&mut self, value: &'a TransformedNode) {
        if let &TransformedNode::ConditionStmt(_) = value {
            self.push(value);
        } else if let Some(index) = self.iter().position(|other| *other == value) {
            // replace existing same statement if possible
            self[index] = value;
        } else {
            self.push(value);
        }
    }
}

impl <'a> Transform<'a> for Block<'a> {
    fn transform(&self, ctx: TransformContext<'a>)
            -> Result<Option<&'a TransformedNode<'a>>, CompileErr> {
        let block_ctx = TransformContext {
            scope: Rc::new(RefCell::new(
                ScopeData::new(Some(ctx.scope.clone()))
            )),
            transform_arena: ctx.transform_arena,
            ast_arena: ctx.ast_arena,
            path: ctx.path.clone(),
        };

        // collect transformed statements from lines in this block
        let mut t_statements : Vec<&TransformedNode> = Vec::new();

        for statement in &self.nodes {
            if let Some(t_statement) = statement.transform(block_ctx.clone())? {
                match *t_statement {
                    TransformedNode::ExpandedNodes(ref resolved_stmts) => {
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

        Ok(Some(ctx.alloc_transformed(TransformedNode::Block(
            match self.variant {
                BlockType::Show => PlainBlock::Show(t_statements),
                BlockType::Hide => PlainBlock::Hide(t_statements)
            }
        ))))
    }

    fn location(&self) -> AstLocation {
        unimplemented!()
    }
}

impl <'a> TransformResult for PlainBlock<'a> {
    fn render(&self, ctx: RenderContext, buf: &mut Write) -> Result<(), CompileErr> {
        let nodes = match *self {
            PlainBlock::Show(ref nodes) => {
                buf.write("Show\n".as_ref())?;
                nodes
            },
            PlainBlock::Hide(ref nodes) => {
                buf.write("Hide\n".as_ref())?;
                nodes
            }
        };
        for n in nodes {
            n.render(ctx.increase_indent(), buf)?;
        }
        Ok(())
    }
}
