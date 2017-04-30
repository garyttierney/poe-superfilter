use super::*;

#[derive(Debug,Clone)]
pub struct NoValue {}
pub static NO_VALUE : NoValue = NoValue {};

impl InnerScopeValue for &'static NoValue {
    fn try_cmp(&self, _: Self) -> CompileResult<Ordering> {
        Ok(Ordering::Equal)
    }
    fn try_eq(&self, _: Self) -> CompileResult<bool> {
        Ok(true)
    }

    fn type_name(&self) -> &'static str { "None" }
}

impl TransformResult for &'static NoValue {
    fn render(&self, _: RenderContext, _: &mut Write) -> CompileResult<()> {
        Ok(())
    }
}

impl TryFrom<ScopeValue> for &'static NoValue {
    type Error = CompileErr;

    fn try_from(value: ScopeValue) -> Result<Self, Self::Error> {
        match value {
            ScopeValue::None(v) => Ok(v),
            _ => Err(CompileErr::IncompatibleTypes(format!("{:?}", value), "None"))
        }
    }
}