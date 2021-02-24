use std::ops::Deref;

use crate::parser::types::{
    FileContent,
    CompilationTarget,
    TranspiledFile,
    CompilableSection
};

pub fn transpile_all_targets (uncompiled: Vec<CompilationTarget>) -> Vec<CompilationTarget> {
    uncompiled.into_iter().map(| target | {
        compile_target(target)
    }).collect()
}

fn compile_target (target: CompilationTarget) -> CompilationTarget {
    return if let FileContent::Parsed( parsed ) = target.contents {
        // Rebuilding the compilation target struct:
        CompilationTarget {
            // Same input / output
            input_path: target.input_path,
            output_path: target.output_path,
            contents: FileContent::Transpiled (
                // Rebuilding the contents as TranspiledFile struct:
                TranspiledFile {
                    // Same size and vanilla sections
                    size: parsed.size,
                    vanilla_sections: parsed.vanilla_sections,
                    compiled_sections: 
                        // 
                        parsed.compilable_sections.into_iter().map(| compilable | {

                            // Getting the compilation instructions function from the compilable obect's comp_type
                            let compilation_instructions = compilable.comp_instructions.transpile.deref();

                            // Executing the compilation instruction, and wrapping that u8 array into
                            //      a &str, and then a String
                            String::from (
                                std::str::from_utf8 (
                                    compilation_instructions(compilable.content.as_bytes())
                                ).unwrap()
                            )
                        }).collect()
                }
            )
        }
    } 
    else { target }
}
