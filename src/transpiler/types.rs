use crate::transpiler::implementations::{fscript_if, fscript_switch};

/// Enumeration of the types of compilation that can occur
/// Obviously, this enum will be expanded in future, after more releases
pub enum CompileType {
    If    ,                                     // compilation targeting an if, or if-else statement
    Switch,                                     // compilation targeting a switch statement
    // <new compilable element goes here>
}


pub struct CompilationInfo<'a> {
    pub fjs_block: &'a [u8],
    pub remaining: &'a [u8],
    pub comp_info: Info
}

pub enum Info {
      IfInfo   (     fscript_if::IfInfo     ),
    SwitchInfo ( fscript_switch::SwitchInfo ),
    None
}



/// # CompilationInstructions struct
///
/// Structure that describes the exact instruction to:
///     1) check for
///     2) parse from
///     3) and transpile
/// An element of fscript code, into vanilla js
///
/// # Contains:
///     `comp_type: CompileType` -- The type of fscript code that will be compiled to js
///     `check: Box<dyn Fn(&[u8], usize) -> bool>` -- pointer to a function that will 
///             check if a given index of a given string is the beginning
///             of a block of fjs code to be compiled
///     `parse: Box<dyn Fn(&[u8]) -> CompilationInfo>` -- pointer to a function that 
///             will parse and remove an fjs element to be compiled from input data
///     `transpile: Box<dyn Fn(&[u8]) -> &[u8]>` -- pointer to a function that will
///             convert an element of fjs code into vanilla js
pub struct CompilationInstructions {
    pub comp_type:      CompileType,
    pub check:          Box<dyn Fn(&[u8], usize) -> bool>,
    pub parse:          Box<dyn for<'a> Fn(&'a [u8]) -> Option<CompilationInfo<'a>>>,    
    pub transpile:      Box<dyn Fn(&[u8], Info) -> &[u8]>
}

pub type InstructionsSet = [CompilationInstructions; 2 /* <-- increment this */];

pub fn initialize_compilables () -> InstructionsSet {[
    fscript_if::implement_if(),
    fscript_switch::implement_switch(),
    // <add the new compilable here>
]}