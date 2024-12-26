use crate::ast::expression::*;
use crate::ast::transform::{Transform, TransformContext};
use crate::ast::{AstLocation, TransformedNode};
use crate::errors::Result;

#[derive(Debug, Clone)]
pub struct Color {
    pub r: Box<ExpressionNode>,
    pub g: Box<ExpressionNode>,
    pub b: Box<ExpressionNode>,
    pub a: Box<ExpressionNode>,
}

impl<'a> Color {}

pub struct PlainColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Transform for Color {
    #[allow(unused_variables)]
    fn transform(&self, ctx: TransformContext) -> Result<Option<TransformedNode>> {
        unimplemented!();
    }
    fn location(&self) -> AstLocation {
        unimplemented!()
    }
}
