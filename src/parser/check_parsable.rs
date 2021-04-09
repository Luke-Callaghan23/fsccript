use crate::transpiler::types::{
    CompilationInstructions, 
    InstructionsSet
};

use crate::tokenizer::types::TokenStream;

pub fn check <'a> (
    token_stream: &'a TokenStream<'a>,
    all_compilables: &'a InstructionsSet
) -> Option<(&'a CompilationInstructions, usize)> {

    // 
    all_compilables.iter().enumerate().find_map(| (index, compilable) | { 
        if (*compilable.check)(&token_stream) { Some((compilable, index)) } 
        else { None }
    })
}