use crate::transpiler::types::TranspileContents;

use super::{fscript_if, fscript_switch};

/// Enumeration of the types of compilation that can occur
/// Obviously, this enum will be expanded in future, after more releases
pub enum CompileType {
    If    ,                                     // compilation targeting an if, or if-else statement
    Switch,                                     // compilation targeting a switch statement
    // <new compilation type here>
}


pub struct CompilableStruct {
    pub comp_type: CompileType,
    pub check: Box<dyn Fn(&[u8], usize) -> bool>,
    pub parse: Box<dyn Fn(&[u8]) -> (TranspileContents, &[u8])>,    
    pub transpile: Box<dyn Fn(TranspileContents) -> TranspileContents>
}


pub fn initialize_compilables () -> [CompilableStruct;2 /* <- increment this  */] {[
    fscript_if::implement_if(),
    fscript_switch::implement_switch(),
    // <add the new compilable here>
]}