
use ast::{TransformedNode, CompileErr, Node};
use scope::{ScopeData, ScopeValue};
use ast::transform::{Transform, TransformResult};
use std::rc::Rc;
use std::cell::RefCell;
use arena::TypedArena;
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
    #[allow(unused_variables)]
    fn transform(&'a self, parent_scope: Rc<RefCell<ScopeData<'a>>>, transformed_arena: &'a TypedArena<TransformedNode<'a>>, ast_arena: &'a TypedArena<Node<'a>> )
        -> Result<Option<&'a TransformedNode<'a>>, CompileErr> {
        match self {
            &StringBox::Var(ref identifier) => {
                if let Some(value) = parent_scope.borrow().var(identifier) {
                    match value {
                        ScopeValue::String(ref s) => {
                            return Ok(Some(transformed_arena.alloc(
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
            &StringBox::Value(ref val) => Ok(Some(transformed_arena.alloc(
                TransformedNode::Value(ScopeValue::String(val.clone()))
            )))
        }
    }
}
