use crate::transpiler::types::{
    CompilationInstructions, 
    InstructionsSet
};

use crate::tokenizer::types::TokenStream;

pub fn check <'a> (
    token_stream: &'a TokenStream<'a>,
    all_compilables: &'a InstructionsSet
) -> Option<&'a CompilationInstructions> {

    // 
    all_compilables.iter().find_map(| compilable | { 
        if (*compilable.check)(&token_stream) { Some(compilable) } 
        else { None }
    })
}