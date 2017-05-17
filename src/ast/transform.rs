use scope::{ScopeData, ScopeValue, NO_VALUE};
use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};
use std::io::Write;
use ast::{TransformedNode, AstLocation};
use std::path::PathBuf;
use std::fmt::Debug;
use errors::Result;

/// This trait needs to be implemented for any abstract syntax tree structure, it contains the
/// functions to transform the structure's representation into the final structure before it gets
/// rendered into plain GGG syntax tree output
pub trait Transform: Debug {
    /// Perform any transformations that need to be done before rendering this structure into
    /// plain GGG loot filter syntax
    fn transform(&self, ctx: TransformContext)
                 -> Result<Option<TransformedNode>>;

    fn location(&self) -> AstLocation;
}

#[derive(Clone)]
pub struct TransformContext {
    pub scope: Rc<RefCell<ScopeData>>,
    pub path: Rc<PathBuf>
}

impl TransformContext {
    pub fn mut_scope(&self) -> RefMut<ScopeData> {
        self.scope.borrow_mut()
    }

    pub fn ref_scope(&self) -> Ref<ScopeData> {
        self.scope.borrow()
    }
}

/// Fully transformed AST structures should implement this trait in order to be renderable and make
/// their result available for use in parent nodes
pub trait TransformResult {
    /// Returns the return value of this AST structure if it has one. The return value of this
    /// function can be used in parent structures that base their transformation on the result
    /// of child nodes.
    fn return_value(&self) -> ScopeValue {
        ScopeValue::None(&NO_VALUE)
    }

    /// Renders the output for this node into a writable stream.
    fn render(&self, ctx: RenderContext, buf: &mut Write) -> Result<()>;
}

/// Holds configuration values for the render output
#[derive(Debug)]
pub struct RenderConfig {
    pub pretty: bool,
    pub indent_str: &'static str,
    pub base_path: PathBuf,
    pub line_ending: &'static [u8]
}

impl RenderConfig {
    pub fn indent(&self) -> bool {
        self.pretty
    }
}

/// Holds any contextual information needed to render a node
#[derive(Clone, Copy)]
pub struct RenderContext<'a> {
    pub config: &'a RenderConfig,
    pub indent_level: isize,
}

impl<'a> RenderContext<'a> {
    pub fn write_indent(&self, buf: &mut Write) -> ::std::io::Result<usize> {
        if self.config.indent() {
            let mut written = 0;

            #[allow(unused_variables)]
            for i in 0..self.indent_level {
                written += buf.write(self.config.indent_str.as_ref())?;
            }
            return Ok(written)
        }
        Ok(0)
    }

    pub fn increase_indent(&self) -> Self {
        RenderContext {
            config: self.config,
            indent_level: self.indent_level + 1,
        }
    }
}
