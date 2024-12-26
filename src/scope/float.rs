use super::*;

impl InnerScopeValue for f64 {
    fn try_add(self, other: Self) -> Result<ScopeValue> {
        Ok(ScopeValue::Decimal(self + other))
    }
    fn try_sub(self, other: Self) -> Result<ScopeValue> {
        Ok(ScopeValue::Decimal(self - other))
    }
    fn try_mul(self, other: Self) -> Result<ScopeValue> {
        Ok(ScopeValue::Decimal(self * other))
    }
    fn try_div(self, other: Self) -> Result<ScopeValue> {
        Ok(ScopeValue::Decimal(self / other))
    }

    fn try_cmp(&self, other: Self) -> Result<Ordering> {
        match self.partial_cmp(&other) {
            Some(ordering) => Ok(ordering),
            None => panic!(),
        }
    }
    fn try_eq(&self, other: Self) -> Result<bool> {
        Ok(*self == other)
    }

    fn type_name(&self) -> &'static str {
        "Float"
    }
}

impl TryFrom<ScopeValue> for f64 {
    type Error = Error;

    fn try_from(value: ScopeValue) -> Result<Self> {
        match value {
            ScopeValue::Int(v) => Ok(v as f64),
            ScopeValue::Decimal(v) => Ok(v),
            _ => Err(ErrorKind::IncompatibleTypes(format!("{:?}", value), "Float").into()),
        }
    }
}

impl TransformResult for f64 {
    fn return_value(&self) -> ScopeValue {
        ScopeValue::Decimal(*self)
    }

    fn render(&self, _: RenderContext, buf: &mut dyn Write) -> Result<()> {
        // round float output since vanilla GGG filters only contain integers
        let rounded = self.round();
        buf.write_all(rounded.to_string().as_ref())?;
        Ok(())
    }
}
