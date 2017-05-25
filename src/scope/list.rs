use super::*;

impl InnerScopeValue for Vec<ScopeValue> {
    fn type_name(&self) -> &'static str { "List" }
}

impl TransformResult for Vec<ScopeValue> {
    fn return_value(&self) -> ScopeValue {
        ScopeValue::List(self.clone())
    }

    fn render(&self, ctx: RenderContext, buf: &mut Write) -> Result<()> {
        for item in self.iter().take(self.len() - 1) {
            item.render(ctx, buf)?;
            buf.write_all(b" ")?;
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
