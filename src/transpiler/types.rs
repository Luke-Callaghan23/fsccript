use std::fmt;
use crate::parser::types::FileContent;

pub enum TranspileContents {
    Original   ( String ),
    Transpiled ( String )
}

/// # Display implementation for the transpilation struct
impl fmt::Display  for TranspileContents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TranspileContents::Original ( original ) => {
                write!(f, "ORIGINAL CONTENT: {}", original)
            },
            TranspileContents::Transpiled ( transpiled ) => {
                write!(f, "TRANSPILED CONTENT: {}", transpiled)
            }
        }
    }
}
