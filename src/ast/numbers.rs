
use translate::TransformErr;
use translate::ScopeData;
use ast::expressions::Expression;

#[derive(Debug, Clone)]
pub enum NumberOperation {
    Mul,
    Div,
    Add,
    Sub
}

/// Number value or expression
#[derive(Debug, Clone)]
pub enum NumberExpression {
    Number(NumberBox),
    Op(Box<NumberExpression>, NumberOperation, Box<NumberExpression>)
}

/// Number value or variable reference
#[derive(Debug, Clone)]
pub enum NumberBox {
    IntValue(i32),
    Decimal(f32),
    Var(String)
}

impl <'a> Expression<'a> for NumberBox {
    fn transform(&mut self, scope: &ScopeData) -> Result<(), TransformErr<'a>> {
        unimplemented!()
    }

    fn render(&'a self) -> Result<String, TransformErr<'a>> {
        unimplemented!()
    }
}
