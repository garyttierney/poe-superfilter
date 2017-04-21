
use std::path::Path;
use std::io::Read;
use std::fs::File;
use super::ast::transform::RenderConfig;

fn load_example(file_name : &str) -> String {
    let path = "src/tests/".to_owned() + file_name;
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn test_compile(input_file : &str, expected_output_file : &str) {
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

    super::compile(&sf_file, input_file.to_owned(), &mut result_vec, &render_config).unwrap();

    let result_str = String::from_utf8(result_vec).unwrap();
    assert!(expected_result.eq(&result_str))
}

#[test]
fn test_vars() {
    test_compile("vars.sf", "vars.filter")
}

#[test]
fn test_mixins() {
    test_compile("mixins.sf", "mixins.filter")
}