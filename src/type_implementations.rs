pub mod type_implementations {
    use std::fmt;
    use crate::types::types::{
        CompilationTarget,
        FileContent,
        CompilableSection
    };
    
    /// Implementation of Display Trait for a CompilationTarget
    impl <'a>  fmt::Display for CompilationTarget <'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "IN       : '{}'\nOUT      : '{}'\nRAW SIZE : {} characters\n", 
                self.input_path, 
                self.output_path,
                self.contents
            )
        }
    }
    
    /// Implementation of Display Trait for FileContent
    impl <'a> fmt::Display for FileContent <'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", 
                match self {
                    FileContent::Raw        (     raw_code    ) => {    raw_code.len()    },
                    FileContent::Tokenized  (  tokenized_file ) => {  tokenized_file.size },
                    FileContent::Parsed     (   parsed_file   ) => {   parsed_file.size   },
                    FileContent::Transpiled ( transpiled_file ) => { transpiled_file.size },
                }
            )
        }
    }

    /// Implementation of Display Trait for Conpilable
    impl <'a> fmt::Display for CompilableSection<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", std::str::from_utf8(self.content).unwrap())
        }
    }
    
}


