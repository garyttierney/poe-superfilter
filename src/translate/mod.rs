use ast::mixin::Mixin;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

pub struct ScopeData<'s> {
    parent: Option<&'s ScopeData<'s>>,
    vars: BTreeMap<String, VariableData>,
    mixins: BTreeMap<String, &'s Mixin>,
}

impl <'s> ScopeData<'s> {
    pub fn from_parent(parent: &'s ScopeData) -> ScopeData<'s> {
        ScopeData {
            parent: Some(parent),
            vars: BTreeMap::new(),
            mixins: BTreeMap::new(),
        }
    }
}

impl <'s> ScopeData<'s> {
    pub fn parent(&self) -> Option<&'s ScopeData> { self.parent }

    pub fn push_var(&mut self, ident:String, value:VariableData) { self.vars.insert(ident, value); }
    pub fn var(&self, ident:&String) -> Option<&VariableData> {
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

    pub fn push_mixin(&mut self, ident:String, value:&'s Mixin) { self.mixins.insert(ident, value); }
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
}

pub enum VariableData {
    String(String),
    Int(i64),
    Decimal(f64)
}

#[derive(Debug)]
pub struct TransformErr<'e> {
    message: String,
    cause: Option<&'e Error>
}

impl <'e> TransformErr<'e> {
    pub fn new(message:String) -> TransformErr<'e> {
        TransformErr {
            message: message,
            cause: None
        }
    }
}

impl <'e> fmt::Display for TransformErr<'e> {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl <'e> Error for TransformErr<'e> {
    fn description(&self) -> &str {
        self.message.as_ref()
    }

    fn cause(&self) -> Option<&Error> {
        self.cause
    }
}
