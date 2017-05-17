use ast::{Node, TransformedNode, AstLocation};
use ast::var::VarReference;
use scope::ScopeValue;
use ast::transform::{Transform, TransformResult, TransformContext, RenderContext};
use ast::block_statements::ComparisonOperator;
use std::io::Write;
use std::fmt;
use scope::InnerScopeValue;
use errors::{Result, ErrorKind};

#[derive(Clone)]
pub enum ExpressionValue {
    String(String),
    Int(i64),
    Decimal(f64),
    Bool(bool),
    Var(VarReference),
    List(Vec<Node>)
}

impl fmt::Debug for ExpressionValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExpressionValue::String(ref v) => write!(f, "{:?}", v),
            ExpressionValue::Int(ref v) => write!(f, "{:?}", v),
            ExpressionValue::Decimal(ref v) => write!(f, "{:?}", v),
            ExpressionValue::Var(ref v) => write!(f, "{:?}", v),
            ExpressionValue::Bool(ref v) => write!(f, "{:?}", v),
            ExpressionValue::List(ref v) => {
                for item in v {
                    write!(f, "{:?}", item)?;
                }
                Ok(())
            }
        }
    }
}

impl Transform for ExpressionValue {
    fn transform(&self, ctx: TransformContext)
                 -> Result<Option<TransformedNode>> {
        match *self {
            ExpressionValue::Var(ref node) => node.transform(ctx),
            ExpressionValue::List(ref list) => {
                let mut transformed_values: Vec<ScopeValue> = vec![];
                for value in list {
                    if let Some(t_value) = value.transform(ctx.clone())? {
                        transformed_values.push(t_value.return_value());
                    }/*else {
                        return Err(CompileErr::MissingValue(format!("{:?}", value), self.location.clone()));
                    }*/
                }
                Ok(Some(TransformedNode::Value(ScopeValue::List(transformed_values))))
            }
            ref val => {
                Ok(Some(TransformedNode::Value(
                    match *val {
                        ExpressionValue::String(ref s) => ScopeValue::String(s.clone()),
                        ExpressionValue::Int(i) => ScopeValue::Int(i),
                        ExpressionValue::Decimal(f) => ScopeValue::Decimal(f),
                        ExpressionValue::Bool(b) => ScopeValue::Bool(b),
                        _ => unreachable!()
                    }
                )))
            }
        }
    }

    fn location(&self) -> AstLocation {
        unimplemented!()
    }
}

#[derive(Clone)]
pub enum ExpressionNode {
    Val(ExpressionValue, AstLocation),
    Op(Box<Node>, ExpressionOperation, Box<Node>)
}

impl fmt::Debug for ExpressionNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExpressionNode::Val(ref v, _) => write!(f, "{:?}", v),
            ExpressionNode::Op(ref a, ref op, ref b) => write!(f, "{:?} {:?} {:?}", a, op, b)
        }
    }
}

impl ExpressionNode {
    fn transform_op(&self, ctx: TransformContext, a: &Node, op: &ExpressionOperation, b: &Node)
                    -> Result<Option<TransformedNode>> {
        // transform each operand
        let transformed_operands = (
            a.transform(ctx.clone())?,
            b.transform(ctx.clone())?
        );

        match transformed_operands {
            // abort if one of the nodes returned nothing
            (None, _) => return Err(ErrorKind::MissingValue(format!("{:?}", a), a.location()).into()),
            (_, None) => return Err(ErrorKind::MissingValue(format!("{:?}", b), b.location()).into()),
            (Some(a_trans), Some(b_trans)) => {
                // get return values from transformed nodes
                let a_val = a_trans.return_value();
                let b_val = b_trans.return_value();

                // Perform actual math operations
                let result = match *op {
                    ExpressionOperation::Add => a_val.try_add(b_val)?,
                    ExpressionOperation::Mul => a_val.try_mul(b_val)?,
                    ExpressionOperation::Div => a_val.try_div(b_val)?,
                    ExpressionOperation::Sub => a_val.try_sub(b_val)?,
                    ExpressionOperation::Eql => ScopeValue::Bool(a_val.try_eq(b_val)?),
                    ExpressionOperation::Lt => ScopeValue::Bool(a_val.try_lt(b_val)?),
                    ExpressionOperation::Lte => ScopeValue::Bool(a_val.try_lte(b_val)?),
                    ExpressionOperation::Gt => ScopeValue::Bool(a_val.try_gt(b_val)?),
                    ExpressionOperation::Gte => ScopeValue::Bool(a_val.try_gte(b_val)?)
                };
                return Ok(Some(TransformedNode::Value(result)))
            }
        }
    }
}

impl Transform for ExpressionNode {
    fn transform(&self, ctx: TransformContext)
                 -> Result<Option<TransformedNode>> {
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
    Sub,

    Eql,
    Lt,
    Lte,
    Gt,
    Gte
}

impl From<ComparisonOperator> for ExpressionOperation {
    fn from(op: ComparisonOperator) -> Self {
        match op {
            ComparisonOperator::Eql => ExpressionOperation::Eql,
            ComparisonOperator::Lt => ExpressionOperation::Lt,
            ComparisonOperator::Lte => ExpressionOperation::Lte,
            ComparisonOperator::Gt => ExpressionOperation::Gt,
            ComparisonOperator::Gte => ExpressionOperation::Gte
        }
    }
}

/// Implements TransformResult for any string
impl TransformResult for String {
    fn return_value(&self) -> ScopeValue {
        ScopeValue::String(self.clone())
    }

    #[allow(unused_variables)]
    fn render(&self, ctx: RenderContext, buf: &mut Write) -> Result<()> {
        let quotes_needed = self.contains(" ");

        if quotes_needed { buf.write(b"\"")?; };
        buf.write(self.as_ref())?;
        if quotes_needed { buf.write(b"\"")?; };

        Ok(())
    }
}
