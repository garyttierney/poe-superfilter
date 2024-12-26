use crate::ast::mixin::PreparedMixin;
use std::collections::BTreeMap;
use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::transform::{TransformResult, RenderContext};
use std::io::Write;
use std::cmp::{Ordering, PartialEq};
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use crate::errors::{Result, ErrorKind, Error};

mod int;
mod float;
mod bool;
mod no_value;
mod list;
mod string;

pub use self::int::*;
pub use self::float::*;
pub use self::bool::*;
pub use self::no_value::*;
pub use self::list::*;
pub use self::string::*;

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
    pub fn push_var(&mut self, ident: String, value: ScopeValue) { self.vars.insert(ident, value); }

    /// Resolves a variable that is available in this scope. Traverses up to parent scopes
    /// if necessary.
    pub fn var(&self, ident: &str) -> Option<ScopeValue> {
        // return from current scope if found
        if let Some(var) = self.vars.get(ident) {
            return Some(var.clone());
        }

        // bubble up to parent scope otherwise
        if let Some(v) = self.parent() {
            return v.borrow().var(ident).clone();
        } else {
            return None;
        }
    }

    /// Adds a mixin to this scope
    pub fn push_mixin(&mut self, ident: String, value: PreparedMixin) { self.mixins.insert(ident, value); }

    /// Resolves a variable that is available in this scope. Traverses up to parent scopes
    /// if necessary.
    pub fn mixin(&self, ident: &str) -> Option<PreparedMixin> {
        // return from current scope if found
        if let Some(mixin) = self.mixins.get(ident) {
            return Some(mixin.clone());
        }

        // bubble up to parent scope otherwise
        if let Some(parent) = self.parent() {
            return parent.borrow().mixin(ident).clone();
        } else {
            return None;
        }
    }
}

pub trait InnerScopeValue: TransformResult + Debug + Sized {
    fn try_add(self, _: Self) -> Result<ScopeValue> {
        Err(ErrorKind::UnsupportedOperation(format!("{:?}", self), "+").into())
    }

    fn try_sub(self, _: Self) -> Result<ScopeValue> {
        Err(ErrorKind::UnsupportedOperation(format!("{:?}", self), "-").into())
    }

    fn try_mul(self, _: Self) -> Result<ScopeValue> {
        Err(ErrorKind::UnsupportedOperation(format!("{:?}", self), "*").into())
    }

    fn try_div(self, _: Self) -> Result<ScopeValue> {
        Err(ErrorKind::UnsupportedOperation(format!("{:?}", self), "/").into())
    }

    fn try_cmp(&self, _: Self) -> Result<Ordering> {
        Err(ErrorKind::UnsupportedOperation(format!("{:?}", self), "=").into())
    }

    fn try_eq(&self, _: Self) -> Result<bool>
        where Self: Sized {
        Err(ErrorKind::UnsupportedOperation(format!("{:?}", self), "<=>").into())
    }

    fn try_gt(&self, other: Self) -> Result<bool> {
        let cmp = self.try_cmp(other)?;
        Ok(cmp == Ordering::Greater)
    }

    fn try_gte(&self, other: Self) -> Result<bool> {
        let cmp = self.try_cmp(other)?;
        Ok(cmp == Ordering::Greater || cmp == Ordering::Equal)
    }

    fn try_lt(&self, other: Self) -> Result<bool> {
        let cmp = self.try_cmp(other)?;
        Ok(cmp == Ordering::Less)
    }

    fn try_lte(&self, other: Self) -> Result<bool> {
        let cmp = self.try_cmp(other)?;
        Ok(cmp == Ordering::Less || cmp == Ordering::Equal)
    }

    fn type_name(&self) -> &'static str;
}


/// Holds the value of a variable that can be held in a `ScopeData` instance
#[derive(Clone, Debug, InnerScopeValue, TransformResult)]
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
        self.try_eq(other.clone()).unwrap_or(false)
    }
}
