use crate::transpiler::types::{CompilationInstructions, InstructionsSet};

pub fn check <'a> (
    data: &[u8], 
    start_index: usize,
    all_compilables: &'a InstructionsSet
) -> Option<&'a CompilationInstructions> {
    all_compilables.iter().find_map(| compilable | { 
        if (*compilable.check)(data, start_index) { Some(compilable) } 
        else { None }
    })
}