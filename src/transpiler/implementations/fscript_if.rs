use crate::transpiler::types::{CompilationInfo, CompilationInstructions, CompileType, Info};
use std::str;

pub fn implement_if () -> CompilationInstructions {
    CompilationInstructions {
        comp_type:   CompileType::If,
          check:     Box::new(check_if),
          parse:     Box::new(parse_if),
        transpile:   Box::new(transpile_if)
    }
}

pub struct IfInfo {

}



fn check_if (data: &[u8], _start_index: usize) -> bool {
    String::from(str::from_utf8(data).unwrap()).starts_with("if")
}

fn parse_if <'a> (data: &'a [u8]) -> Option<CompilationInfo<'a>> {
    Some(CompilationInfo{ fjs_block: b"", remaining: b"", comp_info: Info::None })
}

fn transpile_if (data: &[u8], comp_info: Info) -> &[u8] {
    b""
}