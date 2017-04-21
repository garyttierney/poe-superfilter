use ast::{CompileErr,TransformedNode,AstLocation};
use ast::transform::{Transform,TransformContext};
use ast::Node;

#[derive(Debug, Clone)]
pub struct Color {
    pub r: Box<Node>,
    pub g: Box<Node>,
    pub b: Box<Node>,
    pub a: Box<Node>
}

impl <'a> Color {
}

pub struct PlainColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Transform for Color {
    #[allow(unused_variables)]
    fn transform(&self, ctx: TransformContext)
        -> Result<Option<TransformedNode>, CompileErr> {
        unimplemented!();
    }
    fn location(&self) -> AstLocation {
        unimplemented!()
    }
}