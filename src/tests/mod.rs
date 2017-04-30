
use std::path::Path;
use std::io::Read;
use std::fs::File;
use super::ast::transform::RenderConfig;
use std::env;
use std::sync::{Once, ONCE_INIT};

static START: Once = ONCE_INIT;

fn load_example(path : &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn test_compile(input_file : &str, expected_output_file : &str) {
    START.call_once(||{
        let mut current_path = env::current_dir().unwrap();
        current_path.push("src/tests");
        env::set_current_dir(current_path).unwrap();
    });

    let sf_file = load_example(input_file);
    let expected_result = load_example(expected_output_file);

    let mut result_vec = Vec::<u8>::new();

    let base_path = Path::new(&input_file)
        .parent()
        .unwrap()
        .to_owned();

    let render_config = RenderConfig {
        pretty: false,
        indent_str: "    ",
        base_path: base_path
    };

    super::compile(&sf_file, Path::new(input_file).to_owned(), &mut result_vec, &render_config).unwrap();

    let result_str = String::from_utf8(result_vec).unwrap();
    assert_eq!(expected_result, result_str)
}

#[test]
fn vars() {
    test_compile("vars.sf", "vars.filter")
}

#[test]
fn mixins() {
    test_compile("mixins.sf", "mixins.filter")
}

#[test]
fn simple_expressions() {
    test_compile("simple_expr.sf", "simple_expr.filter")
}

#[test]
fn import() {
    test_compile("import.sf", "import.filter")
}

#[test]
fn statement_override() {
    test_compile("statement_override.sf", "statement_override.filter")
}

#[test]
fn conditional_blocks() {
    test_compile("conditional_blocks.sf", "conditional_blocks.filter")
}
