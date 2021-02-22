use std::env;

mod parser;
use parser::*;
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


#[allow(unused_parens)]
fn main () {
    println!("Hello, world!!!");
    

    let mut args: Vec<String> = env::args().rev().collect();
    args.pop();
    
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

    for target in targets {
        println!("{}", target);
    }
}