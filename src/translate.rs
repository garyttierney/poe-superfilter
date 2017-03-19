use ast::mixin::Mixin;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct ScopeData<'ast> {
    pub parent: Option<Rc<RefCell<ScopeData<'ast>>>>,
    pub vars: BTreeMap<String, ExpressionValue>,
    pub mixins: BTreeMap<String, Mixin<'ast>>,
}

impl <'ast> ScopeData<'ast> {
    pub fn new(parent: Option<Rc<RefCell<ScopeData<'ast>>>>) -> ScopeData<'ast> {
        ScopeData {
            parent: parent,
            vars: BTreeMap::new(),
            mixins: BTreeMap::new(),
        }
    }

    pub fn parent(&self) -> Option<Rc<RefCell<ScopeData<'ast>>>> { self.parent.clone() }

    pub fn push_var(&mut self, ident:String, value: ExpressionValue) { self.vars.insert(ident, value); }
    pub fn var(&self, ident:&String) -> Option<ExpressionValue> {
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

    pub fn push_mixin(&mut self, ident:String, value:Mixin<'ast>) { self.mixins.insert(ident, value); }
    pub fn mixin(&self, ident:&String) -> Option<Mixin<'ast>> {
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

#[derive(Clone,Debug)]
pub enum ExpressionValue {
    String(String),
    Int(i64),
    Decimal(f64),
    List(Vec<ExpressionValue>),
    None
}

#[derive(Debug)]
pub enum TransformErr {
    Unknown(String),
    TypeError(String)
}

impl <'e> fmt::Display for TransformErr {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for TransformErr {
    fn description(&self) -> &str {
        match *self {
            TransformErr::Unknown(ref msg) => &msg,
            TransformErr::TypeError(ref msg) => &msg
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            TransformErr::Unknown(_) => None,
            TransformErr::TypeError(_) => None,
        }
    }
}
