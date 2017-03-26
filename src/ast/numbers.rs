
use ast::{Node, TransformedNode, TransformErr};
use ast::transform::{Transform, TransformResult};
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
pub enum ValueExpression<'ast> {
    Number(NumberBox),
    Op(&'ast Node<'ast>, NumberOperation, &'ast Node<'ast>)
}

impl <'a> Transform<'a> for ValueExpression<'a> {
    fn transform(&'a self, parent_scope: Rc<RefCell<ScopeData<'a>>>, transformed_arena: &'a TypedArena<TransformedNode<'a>>)
        -> Result<Option<&'a TransformedNode<'a>>, TransformErr> {
        match *self {
            ValueExpression::Number(ref num_box) => num_box.transform(parent_scope.clone(), transformed_arena),
            ValueExpression::Op(ref a, ref op, ref b) => {
                // transform each operand
                let t_a = a.transform(parent_scope.clone(), transformed_arena)?;
                if let None = t_a {
                    return Err(TransformErr::MissingValue(format!("{:?}", t_a)))
                }

                let t_b = b.transform(parent_scope.clone(), transformed_arena)?;
                if let None = t_b {
                    return Err(TransformErr::MissingValue(format!("{:?}", t_b)))
                }

                let a_val = t_a.unwrap().return_value();
                let b_val = t_b.unwrap().return_value();

                let result = match *op {
                    NumberOperation::Add => a_val + b_val,
                    NumberOperation::Mul => a_val * b_val,
                    NumberOperation::Div => a_val / b_val,
                    NumberOperation::Sub => a_val - b_val,
                };
                Ok(Some(transformed_arena.alloc(TransformedNode::Value(result))))
            }
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
    fn transform(&'a self, parent_scope: Rc<RefCell<ScopeData<'a>>>, transformed_arena: &'a TypedArena<TransformedNode<'a>>)
        -> Result<Option<&'a TransformedNode<'a>>, TransformErr> {
        match *self {
            NumberBox::Decimal(num) => Ok(Some(transformed_arena.alloc(
                TransformedNode::Value(ScopeValue::Decimal(num))
            ))),
            NumberBox::IntValue(num) => Ok(Some(transformed_arena.alloc(
                TransformedNode::Value(ScopeValue::Int(num))
            ))),
            NumberBox::Var(ref identifier) => {
                if let Some(var_content) = parent_scope.borrow().var(&identifier) {
                    match var_content {
                        ScopeValue::Int(num) => Ok(Some(transformed_arena.alloc(
                            TransformedNode::Value(ScopeValue::Int(num))
                        ))),
                        ScopeValue::Decimal(num) => Ok(Some(transformed_arena.alloc(
                            TransformedNode::Value(ScopeValue::Decimal(num))
                        ))),
                        other => Err(TransformErr::TypeMismatch("Int or Decimal", other.type_name(), "Anonymous".to_owned()))
                    }
                } else {
                    Err(TransformErr::MissingVarRef(identifier.clone()))
                }
            }
        }
    }
}
