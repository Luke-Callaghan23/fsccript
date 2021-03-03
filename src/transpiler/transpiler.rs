use std::ops::Deref;

use crate::types::types::{
    FileContent,
    CompilationTarget,
    TranspiledFile
};

pub fn transpile_all_targets (uncompiled: Vec<CompilationTarget>) -> Vec<CompilationTarget> {
    // Just iterate over each compilation target, and compile it, return the collected list
    uncompiled.into_iter().map(| target | {
        compile_target(target)
    }).collect()
}

fn compile_target (target: CompilationTarget) -> CompilationTarget {
    return if let FileContent::Parsed( parsed ) = target.contents {
        // Rebuilding the compilation target struct:
        CompilationTarget {
            // Same input / output -- copy and paste them into the new Compilation targer
            input_path: target.input_path,
            output_path: target.output_path,
            contents: FileContent::Transpiled (
                // Rebuilding the contents as TranspiledFile struct:
                TranspiledFile {
                    // Same size and vanilla sections -- copy and paste them into the TranspiledFile
                    size: parsed.size,
                    vanilla_sections: parsed.vanilla_sections,
                    compiled_sections: 
                        // Iterating over the compilable sections of the parsed file, and executing the campile instructions on
                        //      each section of compilable code
                        parsed.compilable_sections.into_iter().map(| compilable | {

                            // Getting the compilation instructions function from the compilable obect's comp_type
                            let compilation_instructions 
                                = compilable.comp_instructions.transpile.deref();

                            // Getting the compilation information given from the parser
                            let info = compilable.compilation_info;

                            // Getting the content to be compiled
                            let content = compilable.content;

                            // Executing the compilation instruction(
                            compilation_instructions(content, info)
                            
                        }).collect()
                }
            )
        }
    } 
    else { target }
}
