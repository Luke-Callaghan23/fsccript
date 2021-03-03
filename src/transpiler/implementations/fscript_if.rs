use crate::tokenizer::{token_types::TokenOfInterest, types::{TokenOrStream, TokenStream}};
use crate::transpiler::types::{
    CompilationInfo, 
    CompilationInstructions, 
    CompileType, 
    Info
};

pub fn implement_if () -> CompilationInstructions {
    CompilationInstructions {
        comp_type:   CompileType::If,
          check:     Box::new(check_if),
          parse:     Box::new(parse_if),
        transpile:   Box::new(transpile_if)
    }
}

pub struct IfInfo <'a>{
    pub stream: TokenStream<'a>

}


fn check_if <'a> (token_stream: &'a TokenStream<'a>) -> bool {
    if let Some ( cur ) = token_stream.current() {
        if let TokenOrStream::Token( tok ) = cur {
            tok.token == TokenOfInterest::If
        }
        else { false }
    }
    else { false }
}

fn parse_if <'a> (token_stream: &'a TokenStream<'a>) -> Option<CompilationInfo<'a>> {
    // Some(CompilationInfo{ fjs_block: b"", remaining: b"", comp_info: Info::None })
    None
}

fn transpile_if <'a> (data: &'a [u8], comp_info: Info) -> &'a [u8] {
    b""
}