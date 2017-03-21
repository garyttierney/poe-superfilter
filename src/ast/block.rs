use ast::{TransformedNode,TransformErr,RenderErr};
use ast::transform::{Transform, TransformResult};
use scope::{ScopeData, ScopeValue};
use std::rc::Rc;
use std::cell::RefCell;
use arena::TypedArena;
use std::io::Write;
use ast::VarDefinition;
use ast::mixin::Mixin;
use ast::block_statements::BlockStatement;

/// Top level statements, can be Blocks of instructions
/// like Show/Hide/Mixin or single top-level statements,
/// e.g. variable definitions and imports
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Show(Vec<&'a BlockStatement<'a>>),
    Hide(Vec<&'a BlockStatement<'a>>),
    Import(String),
}

#[derive(Debug, Clone)]
pub enum PlainBlock<'a> {
    Show(Vec<&'a TransformedNode<'a>>),
    Hide(Vec<&'a TransformedNode<'a>>),
}

impl <'a> Transform<'a> for Block<'a> {
    fn transform(&'a self, parent_scope: Rc<RefCell<ScopeData<'a>>>, transformed_arena: &'a TypedArena<TransformedNode<'a>>)
            -> Result<Option<&'a TransformedNode<'a>>, TransformErr> {
        let block_scope = Rc::new(RefCell::new(ScopeData::new(Some(parent_scope))));

        // collect transformed statements from lines in this block
        let mut t_statements : Vec<&TransformedNode> = Vec::new();
        match self {
            &Block::Show(ref statements) | &Block::Hide(ref statements) => {
                for statement in statements {
                    if let Some(t_statement) = statement.transform(block_scope.clone(), transformed_arena)? {
                        t_statements.push(t_statement);
                    }
                }
            }
            node => {
                println!("{:?}", node);
                unimplemented!()
            }
        }

        Ok(Some(transformed_arena.alloc(TransformedNode::Block(
            match self {
                &Block::Show(_) => PlainBlock::Show(t_statements),
                &Block::Hide(_) => PlainBlock::Hide(t_statements),
                node => {
                    println!("{:?}", node);
                    unimplemented!()
                }
            }
        ))))
    }
}

impl <'a> TransformResult for PlainBlock<'a> {
    fn render(&self, buf: &mut Write) -> Result<(), RenderErr> {
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
            n.render(buf)?;
        }
        Ok(())
    }
}
