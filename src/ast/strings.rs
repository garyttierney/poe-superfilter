
use ast;
use ast::TransformedNode;
use translate::{TransformErr, ScopeData, ExpressionValue};
use ast::expressions::{Expression, TransformedExpression};
use std::rc::Rc;
use std::cell::RefCell;
use arena::TypedArena;

/// String value or variable reference
#[derive(Debug, Clone)]
pub enum StringBox {
    Value(String),
    Var(String)
}

impl <'a> TransformedExpression for String {}

impl <'a> ast::Value<'a> for StringBox {}
impl <'a> Expression<'a> for StringBox {
    fn transform<'t>(&'a self, parent_scope: Rc<RefCell<ScopeData>>, transformed_arena: &'t TypedArena<TransformedNode<'t>>)
        -> Result<&'t TransformedNode<'t>, TransformErr> {
        match self {
            &StringBox::Var(ref identifier) => {
                if let Some(value) = parent_scope.borrow().var(identifier) {
                    match value {
                        ExpressionValue::String(ref s) => {
                            return Ok(transformed_arena.alloc(
                                TransformedNode::String(s.clone())
                            ));
                        },
                        _ => {
                            let e = TransformErr::TypeError("Invalid type: expected string value from $".to_owned() + &identifier);
                            return Err(e);
                        }
                    }
                } else {
                    let e = TransformErr::Unknown("Variable reference not found: $".to_owned() + &identifier);
                    return Err(e);
                }
            },
            &StringBox::Value(ref val) => Ok(transformed_arena.alloc(TransformedNode::String(val.clone())))
        }
    }

    /*fn render(&self) -> Result<String, TransformErr> {
        match *self {
            StringBox::Value(ref s) => {
                return Ok(format!("\"{}\"", s));
            },
            _ => unreachable!()
        }
    }*/
}
