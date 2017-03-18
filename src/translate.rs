use ast::mixin::Mixin;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

#[derive(Clone,Debug)]
pub struct ScopeData<'s> {
    pub parent: Option<&'s ScopeData<'s>>,
    pub vars: BTreeMap<String, ExpressionValue>,
    pub mixins: BTreeMap<String, Mixin<'s>>,
}

impl <'s> ScopeData<'s> {
    pub fn new<'a>() -> ScopeData<'a> {
        ScopeData {
            parent: None,
            vars: BTreeMap::new(),
            mixins: BTreeMap::new(),
        }
    }

    pub fn new_child_scope(&'s self) -> ScopeData<'s> {
        ScopeData {
            parent: Some(self),
            vars: BTreeMap::new(),
            mixins: BTreeMap::new(),
        }
    }
}

impl <'s> ScopeData<'s> {
    pub fn parent(&self) -> Option<&'s ScopeData<'s>> { self.parent }

    pub fn push_var(&mut self, ident:String, value: ExpressionValue) { self.vars.insert(ident, value); }
    pub fn var(&self, ident:&String) -> Option<&ExpressionValue> {
        // return from current scope if found
        if let Some(var) = self.vars.get(ident) {
            return Some(var);
        }

        // bubble up to parent scope otherwise
        if let Some(v) = self.parent() {
            return v.var(&ident);
        } else {
            return None;
        }
    }

    pub fn push_mixin(&mut self, ident:String, value:Mixin<'s>) { self.mixins.insert(ident, value); }
    pub fn mixin(&'s self, ident:&String) -> Option<&'s Mixin> {
        // return from current scope if found
        if let Some(mixin) = self.mixins.get(ident) {
            return Some(mixin);
        }

        // bubble up to parent scope otherwise
        if let Some(parent) = self.parent() {
            return parent.mixin(&ident);
        } else {
            return None;
        }
    }

    pub fn extend(&mut self, other:&ScopeData<'s>) {
        self.vars.extend(other.vars.clone());
        self.mixins.extend(other.mixins.clone());
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
    Unknown(String)
}

impl <'e> fmt::Display for TransformErr {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for TransformErr {
    fn description(&self) -> &str {
        match *self {
            TransformErr::Unknown(ref msg) => &msg
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            TransformErr::Unknown(_) => None
        }
    }
}
