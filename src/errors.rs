use super::ast::AstLocation;
use crate::tok::Tok;
use crate::tok::Location as TokenLocation;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Io(::std::io::Error);
        Parse(::lalrpop_util::ParseError<TokenLocation, Tok, char>);
    }

    errors {
        TypeMismatch(expected: &'static str, actual: &'static str, identifier: String, location: AstLocation) {
            description("Type mismatch")
            display("Type mismatch: Expected {} to be {}, but got {} at {}", identifier, expected, actual, location)
        }
        MissingVarRef(identifier: String, location: AstLocation) {
            description("Missing variable reference")
            display("Unresolved variable reference: {} at {}", identifier, location)
        }
        MissingValue(node: String, location: AstLocation) {
            description("Expected expression to return value, none found")
            display("Expected Expression to return a value: {:?} at {}", node, location)
        }
        WrongParameterCount(node: String, expected: usize, actual: usize, location: AstLocation) {
            description("Wrong mixin call parameter count")
            display("Wrong mixin call parameter count in {}: expected {}, got {} at {}", node, expected, actual, location)
        }
        IncompatibleTypes(value: String, type_name: &'static str) {
            description("Cannot convert value to a given type")
            display("Cannot convert {} to {}.", value, type_name)
        }
        UnsupportedOperation(value: String, op: &'static str) {
            description("Operation is unsupported by this type")
            display("{} operation cannot be performed with {} because it is not supported.", op, value)
        }
        ImportError(node: String, location: AstLocation) {
            description("Invalid import path")
            display("Invalid import expression: {:?} at {}", node, location)
        }
    }
}
