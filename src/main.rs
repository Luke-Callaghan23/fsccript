use std::env;

mod parser;
use parser::parser::parser::parse_files;
use parser::types::{
    CompilationTarget
};

mod io;
use io::read_files::process_files;
use io::write_files::write_all;


/// # External crates for fscript -- 
extern crate itertools;
extern crate regex;

mod transpiler;
use transpiler::transpiler::transpile_all_targets;


#[allow(unused_parens)]
fn main () {
    let mut args: Vec<String> = env::args().rev().collect();
    args.pop();
    
    // Four steps: 
    //      read files,
    //      parse files,
    //      transpile files
    //      write files


    // 1. Read all files, store as CompilationTargets in 'targets'
    let targets: Vec<CompilationTarget> = process_files ((
        if args.len() > 0 {
            // If arguments were supplied, use them as the paths to compile
            args
        }
        else {
            // Default directory for comilation is ./compile/
            vec![ String::from("./compile/") ]
        }
    ));

    // 2. parse all files, store as CompilationTargets in 'parsed_targets'
    let parsed_target = parse_files(targets);


    // 3. transpile all parsed files, store in transpiled in 'transpiled'
    let transpiled_targets = transpile_all_targets(parsed_target);

    // 4.
    write_all(transpiled_targets);

    println!("Finished.");
}

