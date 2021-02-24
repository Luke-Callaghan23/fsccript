use crate::transpiler::implementations::implementations::{CompileType, CompilableStruct};
use crate::transpiler::types::TranspileContents;
use std::str;

pub fn implement_if () -> CompilableStruct {
    CompilableStruct {
        comp_type: CompileType::If,
        check: Box::new(check_if),
        parse: Box::new(parse_if),
        transpile: Box::new(transpile_if)
    }
}


fn check_if (data: &[u8], _start_index: usize) -> bool {
    String::from(str::from_utf8(data).unwrap()).starts_with("if")
}

fn parse_if (data: &[u8]) -> (TranspileContents, &[u8]) {
    ( TranspileContents::Original(String::from("")), b"" )
}

fn transpile_if (data: TranspileContents) -> TranspileContents {
    TranspileContents::Transpiled(String::from(""))
}