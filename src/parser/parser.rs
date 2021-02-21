#![crate_name = "Parser"]


pub mod parser {
    use crate::types::{
        ParsedFile,
        CompilationTarget
    };


    pub fn parse_file (target: CompilationTarget) -> ParsedFile {
        ParsedFile {
            size: 2,
            compilable_sections: vec![],
            vanilla_sections: vec![]
        }
    }
}