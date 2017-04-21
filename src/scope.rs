use ast::mixin::PreparedMixin;
use std::collections::BTreeMap;
use std::cell::RefCell;
use std::rc::Rc;
use ast::transform::{TransformResult, RenderContext};
use std::io::Write;
use ast::CompileErr;
use std::ops::{Add, Sub, Mul, Div};

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

/// Holds the value of a variable that can be held in a ScopeData instance
#[derive(Clone,Debug,PartialEq)]
pub enum ScopeValue {
    String(String),
    Int(i64),
    Decimal(f64),
    List(Vec<ScopeValue>),
    None
}

impl ScopeValue {
    pub fn type_name(&self) -> &'static str {
        match *self {
            ScopeValue::String(_) => "String",
            ScopeValue::Int(_) => "Integer",
            ScopeValue::Decimal(_) => "Decimal",
            ScopeValue::List(_) => "List",
            ScopeValue::None => "None",
        }
    }
}

impl Add for ScopeValue {
    type Output = ScopeValue;

    fn add(self, other: ScopeValue) -> ScopeValue {
        match self {
            ScopeValue::Int(a) => {
                match other {
                    ScopeValue::Int(b) => ScopeValue::Int(a + b),
                    ScopeValue::Decimal(b) => ScopeValue::Decimal(a as f64 + b),
                    ScopeValue::String(b) => ScopeValue::String(format!("{}{}", a, b)),
                    _ => unimplemented!()
                }
            },
            ScopeValue::Decimal(a) => {
                match other {
                    ScopeValue::Int(b) => ScopeValue::Decimal(a + b as f64),
                    ScopeValue::Decimal(b) => ScopeValue::Decimal(a + b),
                    _ => unimplemented!()
                }
            },
            _ => unimplemented!()
        }
    }
}

impl Sub for ScopeValue {
    type Output = ScopeValue;

    fn sub(self, other: ScopeValue) -> ScopeValue {
        match self {
            ScopeValue::Int(a) => {
                match other {
                    ScopeValue::Int(b) => ScopeValue::Int(a - b),
                    ScopeValue::Decimal(b) => ScopeValue::Decimal(a as f64 - b),
                    _ => unimplemented!()
                }
            },
            ScopeValue::Decimal(a) => {
                match other {
                    ScopeValue::Int(b) => ScopeValue::Decimal(a - b as f64),
                    ScopeValue::Decimal(b) => ScopeValue::Decimal(a - b),
                    _ => unimplemented!()
                }
            },
            _ => unimplemented!()
        }
    }
}


impl Mul for ScopeValue {
    type Output = ScopeValue;

    fn mul(self, other: ScopeValue) -> ScopeValue {
        match self {
            ScopeValue::Int(a) => {
                match other {
                    ScopeValue::Int(b) => ScopeValue::Int(a * b),
                    ScopeValue::Decimal(b) => ScopeValue::Decimal(a as f64 * b),
                    _ => unimplemented!()
                }
            },
            ScopeValue::Decimal(a) => {
                match other {
                    ScopeValue::Int(b) => ScopeValue::Decimal(a * b as f64),
                    ScopeValue::Decimal(b) => ScopeValue::Decimal(a * b),
                    _ => unimplemented!()
                }
            },
            _ => unimplemented!()
        }
    }
}

impl Div for ScopeValue {
    type Output = ScopeValue;

    fn div(self, other: ScopeValue) -> ScopeValue {
        match self {
            ScopeValue::Int(a) => {
                match other {
                    ScopeValue::Int(b) => ScopeValue::Int(a / b),
                    ScopeValue::Decimal(b) => ScopeValue::Decimal(a as f64 / b),
                    _ => unimplemented!()
                }
            },
            ScopeValue::Decimal(a) => {
                match other {
                    ScopeValue::Int(b) => ScopeValue::Decimal(a / b as f64),
                    ScopeValue::Decimal(b) => ScopeValue::Decimal(a / b),
                    _ => unimplemented!()
                }
            },
            _ => unimplemented!()
        }
    }
}

impl TransformResult for ScopeValue {
    fn return_value(&self) -> ScopeValue {
        self.clone()
    }

    fn render(&self, ctx: RenderContext, buf: &mut Write) -> Result<(), CompileErr> {
        match *self {
            ScopeValue::String(ref v) => { v.render(ctx, buf)?; },
            ScopeValue::Decimal(ref v) => {
                // round float output since vanilla GGG filters only contain integers
                let rounded = v.round();
                buf.write(rounded.to_string().as_ref())?;
            },
            ScopeValue::Int(ref v) => { buf.write(v.to_string().as_ref())?; },
            ScopeValue::List(ref list) => {
                for i in 0..(list.len() - 1) {
                    list[i].render(ctx, buf)?;
                    buf.write(b" ")?;
                }
                list[list.len() - 1].render(ctx, buf)?;
            },
            ScopeValue::None => ()
        };
        Ok(())
    }
}
