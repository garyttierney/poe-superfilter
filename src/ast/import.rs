use ast::{AstLocation, TransformedNode, Node, CompileErr};
use ast::transform::{Transform, TransformContext};
use filter;
use tok;
use std::io::Read;
use std::fs;
use std::rc::Rc;

#[derive(Debug,Clone)]
pub struct ImportStatement {
    pub path: String,
    pub location: AstLocation
}

impl<'a> Transform<'a> for ImportStatement {
    fn transform(&self, ctx: TransformContext<'a>) -> Result<Option<&'a TransformedNode<'a>>, CompileErr> {
        let resolved_file_path = ctx.path.clone().join(self.path.clone());
        let new_base_path = resolved_file_path
            .parent()
            .ok_or(CompileErr::ImportError(format!("{:?}", self), self.location()))?
            .to_owned();

        let mut file = fs::File::open(resolved_file_path.clone())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        {
            let tokens = Box::new(tok::tokenize(&contents));
            let path_str = resolved_file_path.to_str().unwrap();
            match filter::parse_Filter(ctx.ast_arena, path_str, tokens.into_iter()) {
                Ok(&Node::Filter(ref filter)) => {
                    let transform_result = filter.transform_begin(ctx.ast_arena,
                                                                  ctx.scope.clone(),
                                                                  Rc::new(new_base_path));
                    if let Some(&TransformedNode::Root(ref nodes)) = transform_result.unwrap() {
                        return Ok(Some(ctx.alloc_transformed(
                            TransformedNode::ExpandedNodes(
                                nodes.to_owned()
                            )
                        )));
                    } else {
                        return Ok(None);
                    }
                }
                _ => return Err(CompileErr::Unknown)
            }
        }
    }

    fn location(&self) -> AstLocation {
        self.location.clone()
    }
}
