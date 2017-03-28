use ast::{CompileErr,TransformedNode};
use ast::transform::{Transform,TransformContext};
use ast::Node;

#[derive(Debug, Clone)]
pub struct Color<'a> {
    pub r: &'a Node<'a>,
    pub g: &'a Node<'a>,
    pub b: &'a Node<'a>,
    pub a: &'a Node<'a>
}

impl <'a> Color<'a> {
}

pub struct PlainColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl <'a> Transform<'a> for Color<'a> {
    #[allow(unused_variables)]
    fn transform(&self, ctx: TransformContext<'a>)
        -> Result<Option<&'a TransformedNode<'a>>, CompileErr> {
        unimplemented!();
    }
}