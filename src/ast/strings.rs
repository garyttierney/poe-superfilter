
use ast::{TransformedNode, CompileErr};
use scope::ScopeValue;
use ast::transform::{Transform, TransformResult, TransformContext};
use std::io::Write;

/// String value or reference to a string value
#[derive(Debug, Clone)]
pub enum StringBox {
    Value(String),
    Var(String)
}

/// Implements TransformResult for any string
impl <'a> TransformResult for String {
    fn return_value(&self) -> ScopeValue {
        ScopeValue::String(self.clone())
    }

    fn render(&self, buf: &mut Write) -> Result<(), CompileErr> {
        let quotes_needed = self.contains(" ");
        if quotes_needed {
            buf.write("\"".as_ref())?;
        }
        buf.write(self.as_ref())?;
        if quotes_needed {
            buf.write("\"".as_ref())?;
        }
        Ok(())
    }
}

impl <'a> Transform<'a> for StringBox {
    fn transform(&'a self, ctx: TransformContext<'a>)
        -> Result<Option<&'a TransformedNode<'a>>, CompileErr> {
        match self {
            &StringBox::Var(ref identifier) => {
                if let Some(value) = ctx.ref_scope().var(identifier) {
                    match value {
                        ScopeValue::String(ref s) => {
                            return Ok(Some(ctx.alloc_transformed(
                                TransformedNode::Value(ScopeValue::String(s.clone()))
                            )));
                        },
                        val => Err(CompileErr::TypeMismatch("String", val.type_name(), identifier.clone()))
                    }
                } else {
                    let e = CompileErr::MissingVarRef(identifier.clone());
                    return Err(e);
                }
            },
            &StringBox::Value(ref val) => Ok(Some(ctx.alloc_transformed(
                TransformedNode::Value(ScopeValue::String(val.clone()))
            )))
        }
    }
}
