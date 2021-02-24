use crate::transpiler::implementations::implementations::CompilableStruct;

pub fn check <'a> (
    data: &[u8], 
    start_index: usize,
    all_compilables: &'a [CompilableStruct]
) -> Option<&'a CompilableStruct> {
    all_compilables.iter().find_map(| compilable | { 
        if (*compilable.check)(data, start_index) { Some(compilable) } 
        else { None }
    })
}