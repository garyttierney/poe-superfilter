use super::*;

#[derive(Debug, Clone)]
pub struct NoValue {}

pub static NO_VALUE: NoValue = NoValue {};

impl InnerScopeValue for &'static NoValue {
    fn try_cmp(&self, _: Self) -> Result<Ordering> {
        Ok(Ordering::Equal)
    }
    fn try_eq(&self, _: Self) -> Result<bool> {
        Ok(true)
    }

    fn type_name(&self) -> &'static str { "None" }
}

impl TransformResult for &'static NoValue {
    fn render(&self, _: RenderContext, _: &mut dyn Write) -> Result<()> {
        Ok(())
    }
}

impl TryFrom<ScopeValue> for &'static NoValue {
    type Error = Error;

    fn try_from(value: ScopeValue) -> Result<Self> {
        match value {
            ScopeValue::None(v) => Ok(v),
            _ => Err(ErrorKind::IncompatibleTypes(format!("{:?}", value), "None").into())
        }
    }
}