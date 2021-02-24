use crate::transpiler::types::CompilationInstructions;

/// Structure of a .fjs file compilation target 
/// Contains the name of the raw file: file_name, a FileContent variant 
///         that file's content: contents, and the name of the destination
///         file: output_location
pub struct CompilationTarget {
    pub input_path:  String,                    // Path to the input .fjs file of this compilation target
    pub output_path: String,                    // Path to the output .js file of this compilation target
    pub contents:    FileContent,               // Variant enumeration of the content of this compilation target
}

/// Enumeration of the different two different forms of file
///         content: raw, and parsed
/// "Raw" is raw .fjs String read directly from a file
/// "Parsed" is post-processed code that has been categorized into
///         compilable, and non-compilable code
pub enum FileContent {
    Raw    (   String   ),                      // Raw code read from a file
    Parsed ( ParsedFile ),                      // Parsed file
    Transpiled ( TranspiledFile )
}

/// Structure of a file once it has been parse
/// Includes the snippets of vanilla, uncompilable .js code, and 
///         compilable .fjs expressions that need to be translated
///         into .js
/// For every section of compilable code in the raw .fjs, there is
///         are two sections of uncompilable .js, representing the
///         raw code that comes directly before it, and the raw code
///         coming directly after it.
/// So, if there are n sections of compilable code in the parsable_sections
///         vector, there will be n+1 sections of non-compilable code in the 
///        vanilla_sections vector. 
/// In the output file, the non-compilable code and fully compiled code
///         will be zippered together
pub struct ParsedFile {
    pub size:                usize,             // size of the raw file
    pub vanilla_sections:    Vec<String>,       // vector of the sections of vanilla code that will be pasted directly into the output file
    pub compilable_sections: Vec<CompilableSection>,   // vector of the .fjs expressions that will be compiled before being pasted into the output file
}

pub struct TranspiledFile {
    pub size: usize,
    pub vanilla_sections: Vec<String>,
    pub compiled_sections: Vec<String>
}

/// Structure of a transpilable section of .fjs code
/// Includes the type of compilation, and the snippet of code that the compilation
///         on which the compilation will occur
pub struct CompilableSection {
    pub comp_instructions:           // The type of compilation that will occur
        CompilationInstructions,            
    pub content: String              // The raw .fjs code that will be compiled into vanilla .js
}





