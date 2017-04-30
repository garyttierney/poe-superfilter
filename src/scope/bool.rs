use super::*;

impl TransformResult for bool {
    fn return_value(&self) -> ScopeValue {
        ScopeValue::Bool(*self)
    }

    fn render(&self, _: RenderContext, _: &mut Write) -> CompileResult<()> {
        unimplemented!()
    }
}

impl InnerScopeValue for bool {
    fn type_name(&self) -> &'static str { "Bool" }
}

impl TryFrom<ScopeValue> for bool {
    type Error = CompileErr;

    fn try_from(value: ScopeValue) -> Result<Self, Self::Error> {
        match value {
            ScopeValue::Bool(v) => Ok(v),
            _ => Err(CompileErr::IncompatibleTypes(format!("{:?}", value), "Bool"))
        }
    }
}
