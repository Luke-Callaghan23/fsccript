use std::env;

mod parser;

// use crate::parser::types::*;
use crate::parser::parser::parser::parse_files;

mod io;
use io::read_files::PathAndContents;
use io::read_files::process_files;
use io::write_files::write_all;

mod types;
use types::types::*;

mod type_implementations;
use type_implementations::type_implementations::*;

/// # External crates for fscript -- 
extern crate itertools;
extern crate regex;

mod transpiler;
use transpiler::transpiler::transpile_all_targets;
use transpiler::types::{
    initialize_compilables,
    CompilationInstructions,
};

mod tokenizer;
use tokenizer::tokenizer::tokenize_targets;
use tokenizer::token_types::initialize_lookup;
use tokenizer::types::TokenStream;


#[allow(unused_parens)]
fn main () {
    let mut args: Vec<String> = env::args().rev().collect();
    args.pop();
    
    // Four steps: 
    //      0. read files,
    //      1. create CompilationTargets from file contents
    //      2. tokenize compilation targets into token streams
    //      3. parse token streams, and compilation targets,
    //      4. transpile token streams, and compilation targets
    //      5. write files

    // 1. Read all files, store as Strings
    let files: Vec<PathAndContents> = process_files ((
        if args.len() > 0 {
            // If arguments were supplied, use them as the paths to compile
            args
        }
        else {
            // Default directory for comilation is ./compile/
            vec![ String::from("./compile/") ]
        }
    ));


    let targets: Vec<CompilationTarget> = files.iter().map(| file_and_path | {
        let path = &file_and_path.path;
        let contents = &file_and_path.contents;
        // Create CompilationTargets referencing String files
        // If the file was read successfully, create and return Some,
        //      compilation target struct
        CompilationTarget { 
            input_path: path.to_owned(), 
            contents: FileContent::Raw( contents.as_bytes() ), 
            output_path: path.to_owned().replace(".fjs", ".js")
        }

    }).collect();


    // for target in &targets {
    //     println!("{}", target);
    // }




    let token_lookup = initialize_lookup();
    let compilation_instructions = initialize_compilables();

    // 2. tokenize the files
    let tokenized = tokenize_targets(targets, &token_lookup);

    // 2. parse all files, store as CompilationTargets in 'parsed_targets'
    let parsed_target = parse_files(tokenized, &compilation_instructions);


    // 3. transpile all parsed files, store in transpiled in 'transpiled'
    let transpiled_targets = transpile_all_targets(parsed_target, &compilation_instructions);

    // // 4.
    // write_all(transpiled_targets);

    println!("Finished.");
}

