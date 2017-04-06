use ast::{Node, TransformedNode, CompileErr, AstLocation};
use ast::var::VarReference;
use scope::ScopeValue;
use ast::transform::{Transform, TransformResult, TransformContext, RenderContext};
use std::io::Write;
use std::fmt;

#[derive(Clone)]
pub enum ExpressionValue {
    String(String),
    Int(i64),
    Decimal(f64),
    Var(VarReference)
}

impl fmt::Debug for ExpressionValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExpressionValue::String(ref v) => write!(f, "{:?}", v),
            ExpressionValue::Int(ref v) => write!(f, "{:?}", v),
            ExpressionValue::Decimal(ref v) => write!(f, "{:?}", v),
            ExpressionValue::Var(ref v) => write!(f, "{:?}", v)
        }
    }
}

impl<'a> Transform<'a> for ExpressionValue {
    fn transform(&self, ctx: TransformContext<'a>)
                 -> Result<Option<&'a TransformedNode<'a>>, CompileErr> {
        match *self {
            ExpressionValue::Var(ref node) => node.transform(ctx),
            ref val => {
                Ok(Some(ctx.alloc_transformed(TransformedNode::Value(
                    match *val {
                        ExpressionValue::String(ref s) => ScopeValue::String(s.clone()),
                        ExpressionValue::Int(i) => ScopeValue::Int(i),
                        ExpressionValue::Decimal(f) => ScopeValue::Decimal(f),
                        _ => unreachable!()
                    }
                ))))
            }
        }
    }

    fn location(&self) -> AstLocation {
        unimplemented!()
    }
}

#[derive(Clone)]
pub enum ExpressionNode<'ast> {
    Val(ExpressionValue, AstLocation),
    Op(&'ast Node<'ast>, ExpressionOperation, &'ast Node<'ast>)
}

impl <'a> fmt::Debug for ExpressionNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExpressionNode::Val(ref v, _) => write!(f, "{:?}", v),
            ExpressionNode::Op(ref a, ref op, ref b) => write!(f, "{:?} {:?} {:?}", a, op, b)
        }
    }
}

impl<'a> ExpressionNode<'a> {
    fn transform_op(&self, ctx: TransformContext<'a>, a: &'a Node<'a>, op: &ExpressionOperation, b: &'a Node<'a>)
                        -> Result<Option<&'a TransformedNode<'a>>, CompileErr> {
        // transform each operand
        let transformed_operands = (
            a.transform(ctx.clone())?,
            b.transform(ctx.clone())?
        );

        match transformed_operands {
            // abort if one of the nodes returned nothing
            (None, _) => return Err(CompileErr::MissingValue(format!("{:?}", a), a.location())),
            (_, None) => return Err(CompileErr::MissingValue(format!("{:?}", b), b.location())),
            (Some(a_trans), Some(b_trans)) => {
                // get return values from transformed nodes
                let a_val = a_trans.return_value();
                let b_val = b_trans.return_value();

                // Perform actual math operations
                let result = match *op {
                    ExpressionOperation::Add => a_val + b_val,
                    ExpressionOperation::Mul => a_val * b_val,
                    ExpressionOperation::Div => a_val / b_val,
                    ExpressionOperation::Sub => a_val - b_val,
                };
                return Ok(Some(ctx.alloc_transformed(TransformedNode::Value(result))))
            }
        }
    }
}

impl <'a> Transform<'a> for ExpressionNode<'a> {
    fn transform(&self, ctx: TransformContext<'a>)
                 -> Result<Option<&'a TransformedNode<'a>>, CompileErr> {
        match *self {
            ExpressionNode::Val(ref value, _) => value.transform(ctx),
            ExpressionNode::Op(ref a, ref op, ref b) => self.transform_op(ctx, a, op, b)
        }
    }

    fn location(&self) -> AstLocation {
        match *self {
            ExpressionNode::Val(_, ref location) => location.clone(),
            ExpressionNode::Op(ref a, _, ref b) => {
                AstLocation {
                    begin: a.location().begin,
                    end: b.location().end,
                    file: a.location().file,
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionOperation {
    Mul,
    Div,
    Add,
    Sub
}

/// Implements TransformResult for any string
impl<'a> TransformResult for String {
    fn return_value(&self) -> ScopeValue {
        ScopeValue::String(self.clone())
    }

    #[allow(unused_variables)]
    fn render(&self, ctx: RenderContext, buf: &mut Write) -> Result<(), CompileErr> {
        let quotes_needed = self.contains(" ");

        if quotes_needed { buf.write(b"\"")?; };
        buf.write(self.as_ref())?;
        if quotes_needed { buf.write(b"\"")?; };

        Ok(())
    }
}
