
use ast::{Node, TransformedNode, CompileErr};
use ast::transform::{Transform, TransformContext};
use scope::ScopeValue;

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
    fn transform(&self, ctx: TransformContext<'a>)
        -> Result<Option<&'a TransformedNode<'a>>, CompileErr> {
        match *self {
            ValueExpression::Number(ref num_box) => num_box.transform(ctx.clone()),
            ValueExpression::Op(ref a, ref op, ref b) => {
                // transform each operand
                let transformed_operands = (
                    a.transform(ctx.clone())?,
                    b.transform(ctx.clone())?
                );

                match transformed_operands {
                    // abort if one of the nodes returned nothing
                    (None, _) => return Err(CompileErr::MissingValue(format!("{:?}", a))),
                    (_, None) => return Err(CompileErr::MissingValue(format!("{:?}", b))),
                    (Some(a_trans), Some(b_trans)) => {
                        // get return values from transformed nodes
                        let a_val = a_trans.return_value();
                        let b_val = b_trans.return_value();

                        // Perform actual math operations
                        let result = match *op {
                            NumberOperation::Add => a_val + b_val,
                            NumberOperation::Mul => a_val * b_val,
                            NumberOperation::Div => a_val / b_val,
                            NumberOperation::Sub => a_val - b_val,
                        };
                        return Ok(Some(ctx.alloc_transformed(TransformedNode::Value(result))))
                    }
                }
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
    #[allow(unused_variables)]
    fn transform(&self, ctx: TransformContext<'a>)
        -> Result<Option<&'a TransformedNode<'a>>, CompileErr> {
        match *self {
            NumberBox::Decimal(num) => Ok(Some(ctx.alloc_transformed(
                TransformedNode::Value(ScopeValue::Decimal(num))
            ))),
            NumberBox::IntValue(num) => Ok(Some(ctx.alloc_transformed(
                TransformedNode::Value(ScopeValue::Int(num))
            ))),
            NumberBox::Var(ref identifier) => {
                if let Some(var_content) = ctx.ref_scope().var(&identifier) {
                    match var_content {
                        ScopeValue::Int(num) => Ok(Some(ctx.alloc_transformed(
                            TransformedNode::Value(ScopeValue::Int(num))
                        ))),
                        ScopeValue::Decimal(num) => Ok(Some(ctx.alloc_transformed(
                            TransformedNode::Value(ScopeValue::Decimal(num))
                        ))),
                        other => Err(CompileErr::TypeMismatch("Int or Decimal", other.type_name(), "Anonymous".to_owned()))
                    }
                } else {
                    Err(CompileErr::MissingVarRef(identifier.clone()))
                }
            }
        }
    }
}
