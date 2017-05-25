use ast::{TransformedNode, AstLocation, Comment};
use ast::transform::*;
use ast::var::*;
use ast::expression::*;
use ast::mixin::*;
use scope::ScopeValue;
use std::io::Write;
use std::cmp::PartialEq;
use errors::{Result, ErrorKind};

#[derive(Debug, Clone, Transform)]
pub enum BlockStatement {
    SetValue(SetValueStatement),
    Condition(ConditionStatement),
    MixinCall(MixinCall),
    VarDef(VarDefinition),
    Comment(Comment),
}

/// AST structure for a value set or other instruction statement
#[derive(Debug, Clone)]
pub struct SetValueStatement {
    pub name: String,
    pub values: Box<ExpressionNode>,
    pub location: AstLocation,
    pub comment: Option<Comment>,
}

#[derive(Debug, Clone)]
pub struct PlainSetValueStatement {
    pub name: String,
    pub values: ScopeValue,
    pub comment: Option<Comment>,
}

impl PartialEq for PlainSetValueStatement {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<'a> Transform for SetValueStatement {
    fn transform(&self, ctx: TransformContext)
                 -> Result<Option<TransformedNode>> {
        let transformed_values = self.values.transform(ctx)?;

        if transformed_values.is_none() {
            return Err(
                ErrorKind::MissingValue(format!("{:?}", self.values), self.values.location().clone()).into()
            );
        }

        Ok(Some(TransformedNode::SetValueStmt(
            PlainSetValueStatement {
                name: self.name.clone(),
                values: transformed_values.unwrap().return_value(),
                comment: self.comment.clone()
            }
        )))
    }

    fn location(&self) -> AstLocation {
        self.location.clone()
    }
}

impl TransformResult for PlainSetValueStatement {
    fn render(&self, ctx: RenderContext, buf: &mut Write) -> Result<()> {
        ctx.write_indent(buf)?;
        buf.write_all(self.name.as_ref())?;
        buf.write_all(b" ")?;
        self.values.render(ctx, buf)?;

        self.comment.render(ctx, buf)?;
        Ok(())
    }
}

/// AST structure for a condition statement
#[derive(Debug, Clone)]
pub struct ConditionStatement {
    pub name: String,
    pub condition: Condition,
    pub location: AstLocation,
    pub comment: Option<Comment>,
}

#[derive(Debug, Clone)]
pub struct PlainConditionStatement {
    pub name: String,
    pub condition: PlainCondition,
    pub comment: Option<Comment>,
}

impl PartialEq for PlainConditionStatement {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Transform for ConditionStatement {
    fn transform(&self, ctx: TransformContext)
                 -> Result<Option<TransformedNode>> {
        if let Some(t_value) = self.condition.value.transform(ctx.clone())? {
            return Ok(Some(TransformedNode::ConditionStmt(
                PlainConditionStatement {
                    name: self.name.clone(),
                    condition: PlainCondition { value: t_value.return_value(), operator: self.condition.operator },
                    comment: self.comment.clone(),
                }
            )));
        }
        Err("condition statement with no condition".into())
    }

    fn location(&self) -> AstLocation {
        self.location.clone()
    }
}

impl TransformResult for PlainConditionStatement {
    fn render(&self, ctx: RenderContext, buf: &mut Write) -> Result<()> {
        ctx.write_indent(buf)?;
        buf.write_all(self.name.as_ref())?;
        buf.write_all(b" ")?;
        self.condition.operator.render(ctx, buf)?;
        buf.write_all(b" ")?;
        self.condition.value.render(ctx, buf)?;

        self.comment.render(ctx, buf)?;
        Ok(())
    }
}

/// AST structure for a condition. This node is always embedded, so there is no
/// corresponding variant for it in the `ast::Node` enum
#[derive(Debug, Clone)]
pub struct Condition {
    pub value: Box<ExpressionNode>,
    pub operator: ComparisonOperator,
}

#[derive(Debug, Clone)]
pub struct PlainCondition {
    pub value: ScopeValue,
    pub operator: ComparisonOperator
}

/// Comparison operator AST structure
#[derive(Debug, Clone, Copy)]
pub enum ComparisonOperator {
    Eql,
    Lt,
    Lte,
    Gt,
    Gte
}

impl TransformResult for ComparisonOperator {
    #[allow(unused_variables)]
    fn render(&self, ctx: RenderContext, buf: &mut Write) -> Result<()> {
        buf.write_all(match *self {
            ComparisonOperator::Eql => "=",
            ComparisonOperator::Gt => ">",
            ComparisonOperator::Gte => ">=",
            ComparisonOperator::Lt => "<",
            ComparisonOperator::Lte => "<=",
        }.as_ref())?;
        Ok(())
    }
}
