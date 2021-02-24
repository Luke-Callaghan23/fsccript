use crate::transpiler::types::{CompileType, CompilationInstructions};
use std::str;

pub fn implement_if () -> CompilationInstructions {
    CompilationInstructions {
        comp_type: CompileType::If,
        check: Box::new(check_if),
        parse: Box::new(parse_if),
        transpile: Box::new(transpile_if)
    }
}


fn check_if (data: &[u8], _start_index: usize) -> bool {
    String::from(str::from_utf8(data).unwrap()).starts_with("if")
}

fn parse_if (data: &[u8]) -> (&[u8], &[u8]) {
    ( b"", b"" )
}

fn transpile_if (data: &[u8]) -> &[u8] {
    b""
}