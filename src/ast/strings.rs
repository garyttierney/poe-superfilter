
use ast;
use translate::{TransformErr, ScopeData, ExpressionValue};
use ast::expressions::{Expression, TransformedExpression};

/// String value or variable reference
#[derive(Debug, Clone)]
pub enum StringBox {
    Value(String),
    Var(String)
}

type TransformedStringBox = String;
impl <'a> TransformedExpression<'a> for TransformedStringBox {}

impl <'a> ast::Value<'a> for StringBox {}
impl <'a> Expression<'a> for StringBox {
    /*fn transform(self, parent_scope: &ScopeData) -> Result<Box<TransformedExpression<'a>>, TransformErr> {
        match *self {
            StringBox::Var(ref name) => {
                if let Some(value) = parent_scope.var(name) {
                    match *value {
                        ExpressionValue::String(ref s) => {
                            return Ok(s.clone());
                        },
                        _ => {
                            let e = TransformErr::Unknown("Invalid type: expected string value from $".to_owned() + &name);
                            return Err(e);
                        }
                    }
                } else {
                    let e = TransformErr::Unknown("Variable reference not found: $".to_owned() + &name);
                    return Err(e);
                }
            },
            StringBox::Value(val) => return Ok(TransformedStringBox(val))
        }
    }*/

    /*fn render(&self) -> Result<String, TransformErr> {
        match *self {
            StringBox::Value(ref s) => {
                return Ok(format!("\"{}\"", s));
            },
            _ => unreachable!()
        }
    }*/
}
