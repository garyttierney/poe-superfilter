use super::*;

impl InnerScopeValue for String {
    fn try_add(self, other: Self) -> Result<ScopeValue> {
        Ok(ScopeValue::String(self + other.as_ref()))
    }

    fn try_cmp(&self, other: Self) -> Result<Ordering> {
        Ok(self.cmp(&other))
    }
    fn try_eq(&self, other: Self) -> Result<bool> {
        Ok(*self == other)
    }

    fn type_name(&self) -> &'static str { "String" }
}

impl TryFrom<ScopeValue> for String {
    type Error = Error;

    fn try_from(value: ScopeValue) -> Result<Self> {
        match value {
            ScopeValue::String(s) => Ok(s),
            ScopeValue::Int(v) => Ok(v.to_string()),
            ScopeValue::Decimal(v) => Ok(v.to_string()),
            ScopeValue::Bool(v) => Ok(v.to_string()),
            //ScopeValue::List(v) => Ok(v.to_string()),
            _ => Err(ErrorKind::IncompatibleTypes(format!("{:?}", value), "String").into())
        }
    }
}