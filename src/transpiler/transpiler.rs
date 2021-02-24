use crate::parser::types::{
    FileContent,
    CompilationTarget,
    ParsedFile,
    Compilable
};

use crate::transpiler::types::TranspileContents;

pub fn transpile_all_targets (uncompiled: Vec<CompilationTarget>) -> Vec<CompilationTarget> {
    uncompiled.into_iter().map(| target | {
        compile_target(target)
    }).collect()
}

fn compile_target (target: CompilationTarget) -> CompilationTarget {
    return if let FileContent::Parsed( parsed ) = target.contents {
        // Rebuilding the compilation target 
        CompilationTarget {
            input_path: target.input_path,
            output_path: target.output_path,
            contents: FileContent::Parsed (
                ParsedFile {
                    size: parsed.size,
                    vanilla_sections: parsed.vanilla_sections,
                    compilable_sections: 
                    parsed.compilable_sections.into_iter().map(| compilable | {
                        Compilable {
                            comp_type: compilable.comp_type,
                            content: if let TranspileContents::Original ( content ) = compilable.content {
                                TranspileContents::Transpiled(content)
                            }
                            else {
                                TranspileContents::Transpiled(String::from(""))
                            }
                        }
                    }).collect()
                }
            )
        }
    } 
    else { target }
}
