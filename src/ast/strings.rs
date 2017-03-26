
use ast::{TransformedNode, TransformErr};
use scope::{ScopeData, ScopeValue};
use ast::transform::{Transform, TransformResult};
use std::rc::Rc;
use std::cell::RefCell;
use arena::TypedArena;
use std::io::Write;
use ast::RenderErr;

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

    fn render(&self, buf: &mut Write) -> Result<(), RenderErr> {
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
    fn transform(&'a self, parent_scope: Rc<RefCell<ScopeData<'a>>>, transformed_arena: &'a TypedArena<TransformedNode<'a>>)
        -> Result<Option<&'a TransformedNode<'a>>, TransformErr> {
        match self {
            &StringBox::Var(ref identifier) => {
                if let Some(value) = parent_scope.borrow().var(identifier) {
                    match value {
                        ScopeValue::String(ref s) => {
                            return Ok(Some(transformed_arena.alloc(
                                TransformedNode::Value(ScopeValue::String(s.clone()))
                            )));
                        },
                        val => Err(TransformErr::TypeMismatch("String", val.type_name(), identifier.clone()))
                    }
                } else {
                    let e = TransformErr::MissingVarRef(identifier.clone());
                    return Err(e);
                }
            },
            &StringBox::Value(ref val) => Ok(Some(transformed_arena.alloc(
                TransformedNode::Value(ScopeValue::String(val.clone()))
            )))
        }
    }
}
