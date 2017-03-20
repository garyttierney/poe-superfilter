use ast::mixin::Mixin;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;
use ast::transform::TransformResult;
use std::io::Write;
use ast::RenderErr;
use std::convert::From;

#[derive(Clone)]
pub struct ScopeData<'ast> {
    pub parent: Option<Rc<RefCell<ScopeData<'ast>>>>,
    pub vars: BTreeMap<String, ScopeValue>,
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

    pub fn push_var(&mut self, ident:String, value: ScopeValue) { self.vars.insert(ident, value); }
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

impl TransformResult for ScopeValue {
    fn return_value(&self) -> ScopeValue {
        self.clone()
    }

    fn render(&self, buf: &mut Write) -> Result<(), RenderErr> {
        match *self {
            ScopeValue::String(ref v) => { v.render(buf)?; },
            ScopeValue::Decimal(ref v) => { buf.write((v.round() as i64).to_string().as_ref())?; },
            ScopeValue::Int(ref v) => { buf.write(v.to_string().as_ref())?; },
            ScopeValue::List(ref list) => {
                for val in list {
                    val.render(buf)?;
                    buf.write(" ".as_ref())?;
                };
            },
            ScopeValue::None => ()
        };
        Ok(())
    }
}
