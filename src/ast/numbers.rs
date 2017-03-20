
use ast;
use ast::{Node, TransformedNode, TransformErr};
use ast::transform::Transform;
use scope::{ScopeData, ScopeValue};
use arena::TypedArena;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum NumberOperation {
    Mul,
    Div,
    Add,
    Sub
}

/// Number value or expression
#[derive(Debug, Clone)]
pub enum NumberExpression<'ast> {
    Number(NumberBox),
    Op(&'ast Node<'ast>, NumberOperation, &'ast Node<'ast>)
}

impl <'a> ast::Value<'a> for NumberExpression<'a> {}
impl <'a> Transform<'a> for NumberExpression<'a> {
    fn transform<'t>(&'a self, parent_scope: Rc<RefCell<ScopeData>>, transformed_arena: &'t TypedArena<TransformedNode<'t>>)
        -> Result<&'t TransformedNode<'t>, TransformErr> {
        match *self {
            NumberExpression::Number(ref num_box) => num_box.transform(parent_scope.clone(), transformed_arena),
            _ => unimplemented!()
        }
    }
}

/// Number value or variable reference
#[derive(Debug, Clone)]
pub enum NumberBox {
    IntValue(i64),
    Decimal(f64),
    Var(String)
}

impl <'a> Transform<'a> for NumberBox {
    fn transform<'t>(&'a self, parent_scope: Rc<RefCell<ScopeData>>, transformed_arena: &'t TypedArena<TransformedNode<'t>>)
        -> Result<&'t TransformedNode<'t>, TransformErr> {
        match *self {
            NumberBox::Decimal(num) => Ok(transformed_arena.alloc(
                TransformedNode::Value(ScopeValue::Decimal(num))
            )),
            NumberBox::IntValue(num) => Ok(transformed_arena.alloc(
                TransformedNode::Value(ScopeValue::Int(num))
            )),
            NumberBox::Var(ref identifier) => {
                if let Some(var_content) = parent_scope.borrow().var(&identifier) {
                    match var_content {
                        ScopeValue::Int(num) => Ok(transformed_arena.alloc(
                            TransformedNode::Value(ScopeValue::Int(num))
                        )),
                        ScopeValue::Decimal(num) => Ok(transformed_arena.alloc(
                            TransformedNode::Value(ScopeValue::Decimal(num))
                        )),
                        other => Err(TransformErr::TypeMismatch("Int or Decimal", other.type_name(), "Anonymous".to_owned()))
                    }
                } else {
                    Err(TransformErr::MissingVarRef(identifier.clone()))
                }
            }
        }
    }
}
