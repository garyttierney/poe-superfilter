
use ast;
use ast::{Node, TransformedNode};
use ast::expressions::Expression;
use translate::{TransformErr, ScopeData, ExpressionValue};
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
impl <'a> Expression<'a> for NumberExpression<'a> {
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

impl <'a> Expression<'a> for NumberBox {
    fn transform<'t>(&'a self, parent_scope: Rc<RefCell<ScopeData>>, transformed_arena: &'t TypedArena<TransformedNode<'t>>)
        -> Result<&'t TransformedNode<'t>, TransformErr> {
        match *self {
            NumberBox::Decimal(num) => Ok(transformed_arena.alloc(
                TransformedNode::Value(ExpressionValue::Decimal(num))
            )),
            NumberBox::IntValue(num) => Ok(transformed_arena.alloc(
                TransformedNode::Value(ExpressionValue::Int(num))
            )),
            NumberBox::Var(ref identifier) => {
                if let Some(var_content) = parent_scope.borrow().var(&identifier) {
                    match var_content {
                        ExpressionValue::Int(num) => Ok(transformed_arena.alloc(
                            TransformedNode::Value(ExpressionValue::Int(num))
                        )),
                        ExpressionValue::Decimal(num) => Ok(transformed_arena.alloc(
                            TransformedNode::Value(ExpressionValue::Decimal(num))
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
