use ast::mixin::PreparedMixin;
use std::collections::BTreeMap;
use std::cell::RefCell;
use std::rc::Rc;
use ast::transform::{TransformResult, RenderContext};
use std::io::Write;
use ast::{CompileErr, CompileResult};
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;

/// Symbol table structure that hold variables and mixins that are available in a scope
#[derive(Debug, Clone)]
pub struct ScopeData {
    pub parent: Option<Rc<RefCell<ScopeData>>>,
    pub vars: BTreeMap<String, ScopeValue>,
    pub mixins: BTreeMap<String, PreparedMixin>,
}

impl ScopeData {
    /// Creates a new instance with an optional reference to a parent scope
    pub fn new(parent: Option<Rc<RefCell<ScopeData>>>) -> ScopeData {
        ScopeData {
            parent: parent,
            vars: BTreeMap::new(),
            mixins: BTreeMap::new(),
        }
    }

    /// Returns the parent scope
    pub fn parent(&self) -> Option<Rc<RefCell<ScopeData>>> { self.parent.clone() }

    /// Adds a variable to this scope
    pub fn push_var(&mut self, ident:String, value: ScopeValue) { self.vars.insert(ident, value); }

    /// Resolves a variable that is available in this scope. Traverses up to parent scopes
    /// if necessary.
    pub fn var(&self, ident:&String) -> Option<ScopeValue> {
        // return from current scope if found
        if let Some(var) = self.vars.get(ident) {
            return Some(var.clone());
        }

        // bubble up to parent scope otherwise
        if let Some(v) = self.parent() {
            return v.borrow().var(&ident).clone();
        } else {
            return None;
        }
    }

    /// Adds a mixin to this scope
    pub fn push_mixin(&mut self, ident:String, value:PreparedMixin) { self.mixins.insert(ident, value); }

    /// Resolves a variable that is available in this scope. Traverses up to parent scopes
    /// if necessary.
    pub fn mixin(&self, ident:&String) -> Option<PreparedMixin> {
        // return from current scope if found
        if let Some(mixin) = self.mixins.get(ident) {
            return Some(mixin.clone());
        }

        // bubble up to parent scope otherwise
        if let Some(parent) = self.parent() {
            return parent.borrow().mixin(&ident).clone();
        } else {
            return None;
        }
    }
}

#[derive(Debug,Clone)]
pub struct NoValue {}
pub static NO_VALUE : NoValue = NoValue {};

pub trait InnerScopeValue : TransformResult + Debug + Sized {
    fn try_add(self, _: Self) -> CompileResult<ScopeValue> {
        Err(CompileErr::UnsupportedOperation(format!("{:?}", self), self.type_name(), "+"))
    }

    fn try_sub(self, _: Self) -> CompileResult<ScopeValue> {
        Err(CompileErr::UnsupportedOperation(format!("{:?}", self), self.type_name(), "-"))
    }

    fn try_mul(self, _: Self) -> CompileResult<ScopeValue> {
        Err(CompileErr::UnsupportedOperation(format!("{:?}", self), self.type_name(), "*"))
    }

    fn try_div(self, _: Self) -> CompileResult<ScopeValue> {
        Err(CompileErr::UnsupportedOperation(format!("{:?}", self), self.type_name(), "/"))
    }

    fn try_cmp(&self, _: &Self) -> CompileResult<Ordering> {
        Err(CompileErr::UnsupportedOperation(format!("{:?}", self), self.type_name(), "="))
    }

    fn try_eq(&self, _: &Self) -> CompileResult<bool>
        where Self: Sized {
        Err(CompileErr::UnsupportedOperation(format!("{:?}", self), self.type_name(), "<=>"))
    }

    fn try_gt(&self, other: &Self) -> CompileResult<bool> {
        let cmp = self.try_cmp(other)?;
        Ok(cmp == Ordering::Greater)
    }

    fn try_gte(&self, other: &Self) -> CompileResult<bool> {
        let cmp = self.try_cmp(other)?;
        Ok(cmp == Ordering::Greater || cmp == Ordering::Equal)
    }

    fn try_lt(&self, other: &Self) -> CompileResult<bool> {
        let cmp = self.try_cmp(other)?;
        Ok(cmp == Ordering::Less)
    }

    fn try_lte(&self, other: &Self) -> CompileResult<bool> {
        let cmp = self.try_cmp(other)?;
        Ok(cmp == Ordering::Less || cmp == Ordering::Equal)
    }

    fn type_name(&self) -> &'static str;
}


/// Holds the value of a variable that can be held in a ScopeData instance
#[derive(Clone,Debug,InnerScopeValue,InnerTransformResult)]
pub enum ScopeValue {
    Int(i64),
    Decimal(f64),
    Bool(bool),
    String(String),
    List(Vec<ScopeValue>),
    None(&'static NoValue)
}

impl PartialEq for ScopeValue {
    fn eq(&self, other: &ScopeValue) -> bool {
        match self.try_eq(other) {
            Ok(result) => result,
            Err(_) => false
        }
    }
}

impl InnerScopeValue for i64 {
    fn try_add(self, other: Self) -> CompileResult<ScopeValue> {
        Ok(ScopeValue::Int(self + other))
    }
    fn try_sub (self, other: Self) -> CompileResult<ScopeValue> {
        Ok(ScopeValue::Int(self - other))
    }
    fn try_mul (self, other: Self) -> CompileResult<ScopeValue> {
        Ok(ScopeValue::Int(self * other))
    }
    fn try_div(self, other: Self) -> CompileResult<ScopeValue> {
        Ok(ScopeValue::Int(self / other))
    }

    fn try_cmp(&self, other: &Self) -> CompileResult<Ordering> {
        Ok(self.cmp(other))
    }
    fn try_eq(&self, other: &Self) -> CompileResult<bool> {
        Ok(self == other)
    }

    fn type_name(&self) -> &'static str { "Int" }
}

impl TryFrom<ScopeValue> for i64 {
    type Error = CompileErr;

    fn try_from(value: ScopeValue) -> Result<Self, Self::Error> {
        match value {
            ScopeValue::Int(v) => Ok(v),
            ScopeValue::Decimal(v) => Ok(v.round() as i64),
            _ => Err(CompileErr::IncompatibleTypes(format!("{:?}", value), "Int"))
        }
    }
}

impl TransformResult for i64 {
    fn return_value(&self) -> ScopeValue {
        ScopeValue::Int(*self)
    }

    fn render(&self, _: RenderContext, buf: &mut Write) -> CompileResult<()> {
        buf.write(self.to_string().as_ref())?;
        Ok(())
    }
}

impl InnerScopeValue for f64 {
    fn try_add (self, other: Self) -> CompileResult<ScopeValue> {
        Ok(ScopeValue::Decimal(self + other))
    }
    fn try_sub (self, other: Self) -> CompileResult<ScopeValue> {
        Ok(ScopeValue::Decimal(self - other))
    }
    fn try_mul (self, other: Self) -> CompileResult<ScopeValue> {
        Ok(ScopeValue::Decimal(self * other))
    }
    fn try_div (self, other: Self) -> CompileResult<ScopeValue> {
        Ok(ScopeValue::Decimal(self / other))
    }

    fn try_cmp(&self, other: &Self) -> CompileResult<Ordering> {
        match self.partial_cmp(other) {
            Some(ordering) => Ok(ordering),
            None => panic!()
        }
    }
    fn try_eq(&self, other: &Self) -> CompileResult<bool> {
        Ok(self == other)
    }

    fn type_name(&self) -> &'static str { "Float" }
}

impl TryFrom<ScopeValue> for f64 {
    type Error = CompileErr;

    fn try_from(value: ScopeValue) -> Result<Self, Self::Error> {
        match value {
            ScopeValue::Int(v) => Ok(v as f64),
            ScopeValue::Decimal(v) => Ok(v),
            _ => Err(CompileErr::IncompatibleTypes(format!("{:?}", value), "Float"))
        }
    }
}

impl TransformResult for f64 {
    fn return_value(&self) -> ScopeValue {
        ScopeValue::Decimal(*self)
    }

    fn render(&self, _: RenderContext, buf: &mut Write) -> CompileResult<()> {
        // round float output since vanilla GGG filters only contain integers
        let rounded = self.round();
        buf.write(rounded.to_string().as_ref())?;
        Ok(())
    }
}

impl InnerScopeValue for String {
    fn try_add(self, other: Self) -> CompileResult<ScopeValue> {
        Ok(ScopeValue::String(self + other.as_ref()))
    }

    fn try_cmp(&self, other: &Self) -> CompileResult<Ordering> {
        Ok(self.cmp(other))
    }
    fn try_eq(&self, other: &Self) -> CompileResult<bool> {
        Ok(self == other)
    }

    fn type_name(&self) -> &'static str { "String" }
}

impl TryFrom<ScopeValue> for String {
    type Error = CompileErr;

    fn try_from(value: ScopeValue) -> Result<Self, Self::Error> {
        match value {
            ScopeValue::String(s) => Ok(s),
            ScopeValue::Int(v) => Ok(v.to_string()),
            ScopeValue::Decimal(v) => Ok(v.to_string()),
            ScopeValue::Bool(v) => Ok(v.to_string()),
            //ScopeValue::List(v) => Ok(v.to_string()),
            _ => Err(CompileErr::IncompatibleTypes(format!("{:?}", value), "String"))
        }
    }
}

impl InnerScopeValue for Vec<ScopeValue> {
    fn type_name(&self) -> &'static str { "List" }
}

impl TransformResult for Vec<ScopeValue> {
    fn return_value(&self) -> ScopeValue {
        ScopeValue::List(self.clone())
    }

    fn render(&self, ctx: RenderContext, buf: &mut Write) -> CompileResult<()> {
        for i in 0..(self.len() - 1) {
            self[i].render(ctx, buf)?;
            buf.write(b" ")?;
        }
        self[self.len() - 1].render(ctx, buf)
    }
}

impl TryFrom<ScopeValue> for Vec<ScopeValue> {
    type Error = CompileErr;

    fn try_from(value: ScopeValue) -> Result<Self, Self::Error> {
        match value {
            ScopeValue::List(list) => Ok(list),
            _ => Err(CompileErr::IncompatibleTypes(format!("{:?}", value), "List"))
        }
    }
}

impl InnerScopeValue for &'static NoValue {
    fn try_cmp(&self, _: &Self) -> CompileResult<Ordering> {
        Ok(Ordering::Equal)
    }
    fn try_eq(&self, _: &Self) -> CompileResult<bool> {
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

impl TransformResult for bool {
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
