
use translate::TransformErr;
use translate::ScopeData;
use ast::expressions::Expression;
use translate::VariableData;

/// String value or variable reference
#[derive(Debug, Clone)]
pub enum StringBox {
    Value(String),
    Var(String)
}

impl <'a> Expression<'a> for StringBox {
    fn transform(&mut self, scope:&ScopeData) -> Result<(), TransformErr<'a>> {
        match *self {
            StringBox::Var(ref name) => {
                if let Some(value) = scope.var(name) {
                    match *value {
                        VariableData::String(ref s) => {
                            return Ok(());
                        },
                        _ => {
                            let e = TransformErr::new("Invalid type: expected string value from $".to_owned() + &name);
                            return Err(e);
                        }
                    }
                } else {
                    let e = TransformErr::new("Variable reference not found: $".to_owned() + &name);
                    return Err(e);
                }
            },
            StringBox::Value(_) => return Ok(())
        }
    }

    fn render(&self) -> Result<String, TransformErr<'a>> {
        match *self {
            StringBox::Value(ref s) => {
                return Ok(format!("\"{}\"", s));
            },
            _ => unreachable!()
        }
    }
}
