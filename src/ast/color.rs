use arena::TypedArena;
use std::rc::Rc;
use std::cell::RefCell;
use ast::{RenderErr,TransformErr,TransformedNode};
use ast::transform::Transform;
use scope::ScopeData;
use ast::{Value, Node};

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

impl <'a> Value<'a> for Color<'a> {}
impl <'a> Transform<'a> for Color<'a> {
    fn transform<'t>(&'a self, parent_scope: Rc<RefCell<ScopeData>>, transformed_arena: &'t TypedArena<TransformedNode<'t>>) -> Result<&'t TransformedNode<'t>, TransformErr> {
        unimplemented!();
    }
}