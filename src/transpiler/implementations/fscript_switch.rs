use crate::transpiler::types::{CompileType, CompilationInstructions};
use std::str;

pub fn implement_switch () -> CompilationInstructions {
    CompilationInstructions {
        comp_type: CompileType::Switch,
        check: Box::new(check_switch),
        parse: Box::new(parse_switch),
        transpile: Box::new(transpile_switch)
    }
}


fn check_switch (data: &[u8], _start_index: usize) -> bool {
    String::from(str::from_utf8(data).unwrap()).starts_with("switch")
}

fn parse_switch (data: &[u8]) -> (&[u8], &[u8]) {
    ( b"", b"" )
}

fn transpile_switch (data: &[u8]) -> &[u8] {
    b""
}