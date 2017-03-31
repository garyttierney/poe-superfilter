
use typed_arena::Arena;

use super::*;

#[test]
fn basic_vars() {
    let sfilter = indoc!(
    "Show
        Class Flask
        Rarity Magic");

    let tokens = Box::new(tok::tokenize(sfilter));
    let ast_arena = Arena::new();
    let filter = filter::parse_Filter(&ast_arena, tokens.into_iter());
}