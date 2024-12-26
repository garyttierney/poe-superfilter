use crate::ast::transform::{Transform, TransformContext};
use crate::ast::{AstLocation, Comment, TransformedNode};
use crate::errors::{Error, ErrorKind, Result, ResultExt};
use crate::filter::FilterParser;
use crate::tok;
use std::fs;
use std::io::Read;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ImportStatement {
    pub path: String,
    pub location: AstLocation,
    pub comment: Option<Comment>,
}

impl Transform for ImportStatement {
    fn transform(&self, ctx: TransformContext) -> Result<Option<TransformedNode>> {
        let resolved_file_path = ctx.path.clone().join(self.path.clone());
        if let Some(new_base_path) = resolved_file_path.parent() {
            let mut file = fs::File::open(resolved_file_path.clone())?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let tokens = Box::new(tok::tokenize(&contents));
            let parser = FilterParser::new();
            let path_buf = Arc::new(resolved_file_path.to_owned());

            match parser.parse(&path_buf, tokens.into_iter()) {
                Ok(ref filter) => {
                    let transform_result = filter
                        .transform_begin(ctx.scope.clone(), Rc::new(new_base_path.to_owned()));

                    if let Some(TransformedNode::Root(ref nodes)) = transform_result.unwrap() {
                        return Ok(Some(TransformedNode::ExpandedNodes(nodes.to_owned())));
                    } else {
                        return Ok(None);
                    }
                }
                Err(e) => Err(e).chain_err(|| "Imported filter failed to parse"),
            }
        } else {
            Err(Error::from(ErrorKind::ImportError(
                format!("{:?}", self),
                self.location(),
            )))
        }
    }

    fn location(&self) -> AstLocation {
        self.location.clone()
    }
}
