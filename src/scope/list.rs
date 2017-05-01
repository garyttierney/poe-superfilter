use super::*;

impl InnerScopeValue for Vec<ScopeValue> {
    fn type_name(&self) -> &'static str { "List" }
}

impl TransformResult for Vec<ScopeValue> {
    fn return_value(&self) -> ScopeValue {
        ScopeValue::List(self.clone())
    }

    fn render(&self, ctx: RenderContext, buf: &mut Write) -> Result<()> {
        for i in 0..(self.len() - 1) {
            self[i].render(ctx, buf)?;
            buf.write(b" ")?;
        }
        self[self.len() - 1].render(ctx, buf)
    }
}

impl TryFrom<ScopeValue> for Vec<ScopeValue> {
    type Error = Error;

    fn try_from(value: ScopeValue) -> Result<Self> {
        match value {
            ScopeValue::List(list) => Ok(list),
            _ => Err(ErrorKind::IncompatibleTypes(format!("{:?}", value), "List").into())
        }
    }
}
