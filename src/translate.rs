use ast::Mixin;
use std::collections::BTreeMap;
use std::error::Error;
use ast::StringBox;
use std::fmt;
use std::str;
use regex::Regex;

pub struct ScopeData<'s> {
    parent: Option<&'s ScopeData<'s>>,
    vars: BTreeMap<String, VariableData>,
    mixins: BTreeMap<String, &'s Mixin>,
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
        if let Some(p) = self.parent() {
            return p.mixin(&ident);
        } else {
            return None;
        }
    }
}

/// Content of a variable
pub enum VariableData {
    String(String),
    Int(i64),
    Decimal(f64)
}

/// Represents any expression
pub trait Expression<'a, T: Sized> {
    /// Transform this expression's syntax tree into its fully resolved form
    /// should resolve all variable and other references contained in it
    fn transform(self, scope:&ScopeData) -> Result<T, TransformErr<'a>>;

    /// Render the transformed expression into text
    fn render(&'a self) -> Result<String, TransformErr<'a>>;
}

impl <'a> Expression<'a, StringBox> for StringBox {
    fn transform(self, scope:&ScopeData) -> Result<StringBox, TransformErr<'a>> {
        match self {
            StringBox::Var(ref name) => {
                if let Some(value) = scope.var(name) {
                    match *value {
                        VariableData::String(ref s) => {
                            return Ok(StringBox::Value(s.clone()));
                        },
                        _ => {
                            let e = TransformErr::new("Invalid type: expected string value from $".to_owned() + &name);
                            return Err(e);
                        }
                    }
                } else {
                    let e = TransformErr::new("Variable reference not found: $".to_owned() + &name);
                    return Err(e);
                }
            },
            StringBox::Value(_) => return Ok(self)
        }
    }

    fn render(&self) -> Result<String, TransformErr<'a>> {
        match *self {
            StringBox::Value(ref s) => {
                lazy_static! {
                    static ref NO_QUOTES_REQUIRED:Regex = Regex::new("^[:alnum:]+$").unwrap();
                }
                if NO_QUOTES_REQUIRED.is_match(s) {
                    return Ok(s.clone())
                } else {
                    return Ok(format!("\"{}\"", str::replace(s, r#"""#, r#"\""#)));
                }
            },
            _ => unreachable!()
        }
    }
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
