use std::fmt;
pub enum Transpilation {
    Original   ( String ),
    Transpiled ( String )
}

/// # Display implementation for the transpilation struct
impl fmt::Display for Transpilation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Transpilation::Original ( original ) => {
                write!(f, "ORIGINAL CONTENT: {}", original)
            },
            Transpilation::Transpiled ( transpiled ) => {
                write!(f, "TRANSPILED CONTENT: {}", transpiled)
            }
        }
    }
}
