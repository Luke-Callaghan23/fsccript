use crate::{tokenizer::token_stream::TokenStream, transpiler::types::{CompilationInstructions, Info}};

/// Structure of a .fjs file compilation target 
/// Contains the name of the raw file: file_name, a FileContent variant 
///         that file's content: contents, and the name of the destination
///         file: output_location
pub struct CompilationTarget <'a> {
    pub input_path:  String,                    // Path to the input .fjs file of this compilation target
    pub output_path: String,                    // Path to the output .js file of this compilation target
    pub contents:    FileContent<'a>            // Variant enumeration of the content of this compilation target
}

/// Enumeration of the different two different forms of file
///         content: raw, and parsed
/// "Raw" is raw .fjs String read directly from a file
/// "Parsed" is post-processed code that has been categorized into
///         compilable, and non-compilable code
pub enum FileContent <'a> { 
    Raw        ( &'a [u8]           ),                                          // Raw code read from a file
    Tokenized  ( TokenizedFile<'a>  ),                      
    Parsed     ( ParsedFile<'a>     ),                      
    Transpiled ( TranspiledFile<'a> )
    
}

/// # struct parser::types::TokenizedFile
///
/// Structure of a file once it has been tokenized
///
/// Contains a token stream of all tokenizer::token_types::TokensOfInterest
///     of the file, and the String contents of the file
pub struct TokenizedFile <'a> {
    pub size:                usize,                    // size of the raw file
    pub tokenized:           TokenStream<'a>,
    pub contents:            &'a [u8]
}

/// # struct parser::types::ParsedFile
///
/// Structure of a file once it has been parsed
///
/// Includes the snippets of vanilla, uncompilable .js code, and 
///         compilable .fjs expressions that need to be translated
///         into .js
///
/// For every section of compilable code in the raw .fjs, there is
///         are two sections of uncompilable .js, representing the
///         raw code that comes directly before it, and the raw code
///         coming directly after it.
///
/// So, if there are n sections of compilable code in the parsable_sections
///         vector, there will be n+1 sections of non-compilable code in the 
///        vanilla_sections vector. 
///
/// In the output file, the non-compilable code and fully compiled code
///         will be zippered together
pub struct ParsedFile <'a> {
    pub size:                usize,                        // size of the raw file
    pub vanilla_sections:    Vec<&'a [u8]>,                // vector of the sections of vanilla code that will be pasted directly into the output file
    pub compilable_sections: Vec<CompilableSection<'a>>,   // vector of the .fjs expressions that will be compiled before being pasted into the output file
}


/// # struct parser::types::TranspiledFile
/// 
/// Structure of a file once it has been compiled
/// Includes the snippets of vanilla, uncompilable .js code, and 
///         compiled .fjs expressions that need to be translated
///         into .js
///
/// For every section of compilable code in the raw .fjs, there is
///         are two sections of uncompilable .js, representing the
///         raw code that comes directly before it, and the raw code
///         coming directly after it.
///
/// So, if there are n sections of compilable code in the parsable_sections
///         vector, there will be n+1 sections of non-compilable code in the 
///        vanilla_sections vector. 
///
/// In the output file, the non-compilable code and fully compiled code
///         will be zippered together
pub struct TranspiledFile <'a> {
    pub size:                usize,                    // size of the raw file 
    pub vanilla_sections:    Vec<&'a [u8]>,              // vector of the sections of vanilla code that will be pasted directly into the output file 
    pub compiled_sections:   Vec<&'a [u8]>               // vector of the .fjs expressions that will be compiled before being pasted into the output file
}

/// # struct parser::types::CompilableSection
/// 
/// Structure of a transpilable section of .fjs code
///
/// Includes the type of compilation, and the snippet of code that the compilation
///         on which the compilation will occur
pub struct CompilableSection<'a> {
    pub comp_instructions: CompilationInstructions,    // The type of compilation that will occur
    pub content: &'a [u8],                               // The raw .fjs code that will be compiled into vanilla .js
    pub compilation_info: Info                         // Information passed from the parser to the compilable about the .fjs element
}





