use super::*;

impl TransformResult for bool {
    fn return_value(&self) -> ScopeValue {
        ScopeValue::Bool(*self)
    }

    fn render(&self, _: RenderContext, _: &mut dyn Write) -> Result<()> {
        unimplemented!()
    }
}

impl InnerScopeValue for bool {
    fn type_name(&self) -> &'static str { "Bool" }
}

impl TryFrom<ScopeValue> for bool {
    type Error = Error;

    fn try_from(value: ScopeValue) -> Result<Self> {
        match value {
            ScopeValue::Bool(v) => Ok(v),
            _ => Err(ErrorKind::IncompatibleTypes(format!("{:?}", value), "Bool").into())
        }
    }
}
