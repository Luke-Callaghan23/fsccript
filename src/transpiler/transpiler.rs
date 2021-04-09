use std::{collections::VecDeque, ops::Deref};

use itertools::interleave;

use crate::types::types::{
    FileContent,
    CompilationTarget,
    TranspiledFile
};

use super::types::{InstructionsSet};

pub fn transpile_all_targets<'a> (
    uncompiled: Vec<CompilationTarget<'a>>,
    instructions: &InstructionsSet
) -> Vec<CompilationTarget<'a>> {
    // Just iterate over each compilation target, and compile it, return the collected list
    uncompiled.into_iter().map(| target | {
        compile_target(target, instructions)
    }).collect()
}

fn compile_target<'a> (
    target: CompilationTarget<'a>,
    instructions: &InstructionsSet
) -> CompilationTarget<'a> {
    if let FileContent::Parsed( parsed ) = target.contents {
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
                            // Getting the compilation instructions function from compilable struct's 
                            //      instruction_index field
                            let compilation_instructions = 
                                instructions[ compilable.instruction_index ]
                                    .transpile
                                    .deref();

                            // Executing the compilation instruction(
                            compilation_instructions(compilable)
                            
                        }).collect()
                }
            )
        }
    } 
    else { target }
    // target
}


pub fn construct_file (transpiled: TranspiledFile) -> String {
    let poop: Vec<String> 
        = transpiled.vanilla_sections
            .into_iter()
            .map(|section| {
                String::from(std::str::from_utf8(section).unwrap())
            }).collect();
    // Getting the final string of transpiled .js code by interleaving the 
    //      vanilla .js and the transpiled .js, and folding into a single 
    //      String
    // Interleave the vanilla snippets with the transpiled snippets
    interleave (
        poop, 
        transpiled.compiled_sections
    )
    // Fold the interleaved iterator into a single string
    .fold(String::from(""), | mut acc, x | {
        acc.push_str(&x);
        acc
    })
}


pub fn wrap_as_expression (wrapping: String) -> String {
    let mut before = String::from("(() => { return ");
    let after  = String::from("})();");

    before.push_str(&wrapping);
    before.push_str(&after);
    before
}


pub fn insert_returns (source: &str, mut return_indices: VecDeque<usize>) -> String {
    let return_str = "\nreturn ";
    let mut as_str = String::from(source);
    let mut start: usize = 0;
    while !return_indices.is_empty() {
        start += return_indices.pop_front().unwrap();
        as_str.insert_str(start, return_str);
    }
    as_str
}