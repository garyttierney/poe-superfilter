use scope::{ScopeData, ScopeValue};
use arena::TypedArena;
use std::rc::Rc;
use std::cell::Ref;
use std::cell::RefMut;
use std::cell::RefCell;
use std::io::Write;
use ast::{CompileErr,TransformedNode,Node};
use std::path::PathBuf;

/// This trait needs to be implemented for any abstract syntax tree structure, it contains the
/// functions to transform the structure's representation into the final structure before it gets
/// rendered into plain GGG syntax tree output
pub trait Transform<'a> {
    /// Perform any transformations that need to be done before rendering this structure into
    /// plain GGG loot filter syntax
    fn transform(&self, ctx: TransformContext<'a>)
        -> Result<Option<&'a TransformedNode<'a>>, CompileErr>;
}

#[derive(Clone)]
pub struct TransformContext<'a> {
    pub scope: Rc<RefCell<ScopeData<'a>>>,
    pub transform_arena: &'a TypedArena<TransformedNode<'a>>,
    pub ast_arena: &'a TypedArena<Node<'a>>,
    pub path: Rc<PathBuf>
}

impl <'a> TransformContext<'a> {
    pub fn alloc_transformed(&self, node: TransformedNode<'a>) -> &'a TransformedNode<'a> {
        self.transform_arena.alloc(node)
    }

    pub fn alloc_ast(&self, node: Node<'a>) -> &'a Node<'a> {
        self.ast_arena.alloc(node)
    }

    pub fn mut_scope(&self) -> RefMut<ScopeData<'a>> {
        self.scope.borrow_mut()
    }

    pub fn ref_scope(&self) -> Ref<ScopeData<'a>> {
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
        ScopeValue::None
    }

    /// Renders the output for this node into a writable stream.
    fn render(&self, ctx: RenderContext, buf: &mut Write) -> Result<(), CompileErr>;
}

/// Holds configuration values for the render output
#[derive(Debug)]
pub struct RenderConfig {
    pub pretty: bool,
    pub indent_str: &'static str,
    pub base_path: PathBuf
}

impl RenderConfig {
    pub fn indent(&self) -> bool {
        self.pretty
    }
}

/// Holds any contextual information needed to render a node
#[derive(Clone,Copy)]
pub struct RenderContext<'a> {
    pub config: &'a RenderConfig,
    pub indent_level: isize,
}

impl <'a> RenderContext<'a> {
    pub fn write_indent(&self, buf: &'a mut Write) -> ::std::io::Result<usize> {
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
