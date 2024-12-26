use super::*;

impl InnerScopeValue for i64 {
    fn try_add(self, other: Self) -> Result<ScopeValue> {
        Ok(ScopeValue::Int(self + other))
    }
    fn try_sub(self, other: Self) -> Result<ScopeValue> {
        Ok(ScopeValue::Int(self - other))
    }
    fn try_mul(self, other: Self) -> Result<ScopeValue> {
        Ok(ScopeValue::Int(self * other))
    }
    fn try_div(self, other: Self) -> Result<ScopeValue> {
        Ok(ScopeValue::Int(self / other))
    }

    fn try_cmp(&self, other: Self) -> Result<Ordering> {
        Ok(self.cmp(&other))
    }
    fn try_eq(&self, other: Self) -> Result<bool> {
        Ok(*self == other)
    }

    fn type_name(&self) -> &'static str { "Int" }
}

impl TryFrom<ScopeValue> for i64 {
    type Error = Error;

    fn try_from(value: ScopeValue) -> Result<Self> {
        match value {
            ScopeValue::Int(v) => Ok(v),
            ScopeValue::Decimal(v) => Ok(v.round() as i64),
            _ => Err(ErrorKind::IncompatibleTypes(format!("{:?}", value), "Int").into())
        }
    }
}

impl TransformResult for i64 {
    fn return_value(&self) -> ScopeValue {
        ScopeValue::Int(*self)
    }

    fn render(&self, _: RenderContext, buf: &mut dyn Write) -> Result<()> {
        buf.write_all(self.to_string().as_ref())?;
        Ok(())
    }
}