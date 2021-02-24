pub mod type_implementations {
    use std::fmt;
    use crate::parser::types::{
        CompilationTarget,
        FileContent,
        CompilableSection
    };
    
    /// Implementation of Display Trait for a CompilationTarget
    impl fmt::Display for CompilationTarget {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "IN       : '{}'\nOUT      : '{}'\nRAW SIZE : {} characters\n", 
                self.input_path, 
                self.output_path,
                self.contents
            )
        }
    }
    
    /// Implementation of Display Trait for FileContent
    impl fmt::Display for FileContent {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", 
                match self {
                    FileContent::Raw       (    raw_code     ) => {     raw_code.len()     },
                    FileContent::Parsed    (   parsed_code   ) => {    parsed_code.size    },
                    FileContent::Transpiled( transpiled_code ) => {  transpiled_code.size  },
                }
            )
        }
    }

    /// Implementation of Display Trait for Conpilable
    impl fmt::Display for CompilableSection {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.content)
        }
    }
    
}


