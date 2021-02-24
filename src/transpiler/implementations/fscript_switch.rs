use crate::transpiler::implementations::implementations::{CompileType, CompilableStruct};
use crate::transpiler::types::TranspileContents;
use std::str;

pub fn implement_switch () -> CompilableStruct {
    CompilableStruct {
        comp_type: CompileType::Switch,
        check: Box::new(check_switch),
        parse: Box::new(parse_switch),
        transpile: Box::new(transpile_switch)
    }
}


fn check_switch (data: &[u8], _start_index: usize) -> bool {
    String::from(str::from_utf8(data).unwrap()).starts_with("switch")
}

fn parse_switch (data: &[u8]) -> (TranspileContents, &[u8]) {
    ( TranspileContents::Original(String::from("")), b"" )
}

fn transpile_switch (data: TranspileContents) -> TranspileContents {
    TranspileContents::Transpiled(String::from(""))
}