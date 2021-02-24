use crate::parser::types::*;
use std::fs;
use std::io::prelude::*;
use itertools::interleave;

pub fn write_all (compiled: Vec<CompilationTarget>) {
    for target in compiled {
        write_single(target);
    }
}

#[allow(unused_parens)]
fn write_single (compiled_file: CompilationTarget) {

    // Localize out_path, because it is used many times in the below function
    let out_path = &compiled_file.output_path;

    let file = make_file(out_path.as_str());
    if let Some ( file ) = file {
        println!("File at '{}' successfully created!", out_path);
        match compiled_file.contents {
            FileContent::Parsed( parsed ) => {
                let vanilla_len  = parsed.vanilla_sections.len();
                let compiled_len = parsed.compilable_sections.len();

                if vanilla_len != compiled_len + 1 {
                    // For the file write to work, the length of non-compilable js snippets must be exactly one more
                    //      than the length of the compiled snippets
                    // The parser ***should*** take care of this on its own, but I'm putting this here as an extra safeguard
                    //      against unwanted panics!
                    print! (
                        "\tError: Incorrect lengths of parsable and non-parsable material in {}\n\tVanilla snippets: {}\n\tCompiled snippets: {}", 
                        compiled_file.input_path,
                        vanilla_len,
                        compiled_len
                    );
                    return;
                }

                // Getting the final string of transpiled .js code by interleaving the 
                //      vanilla .js and the transpiled .js, and folding into a single 
                //      String
                let compiled = (
                    // Interleave the vanilla snippets with the transpiled snippets
                    interleave (
                        parsed.vanilla_sections, 
                        parsed.compilable_sections
                            .iter()
                            .map(| x | { 
                                // Map struct Compilable vector to String vector
                                x.to_string() 
                            })
                    )
                    // Fold the interleaved iterator into a single string
                    .fold(String::from(""), | mut acc, x | {
                        acc.push_str(x.as_str());
                        acc
                    })
                );

                
                // Write to the file
                write_to_file(file, compiled, out_path);
                
            },
            FileContent::Raw( raw ) => {
                // In case the .fjs file did not contain any conversions, just rewrite
                //      all raw data into the desired output location

                // Write to the file
                write_to_file(file, raw, out_path);
            }
        }
    }
    
    
}

fn make_file (path: &str) -> Option<fs::File> {
    let file = fs::File::create (path);
    match file {
        Ok( file ) => {
            Some ( file )
        },
        Err ( err ) => {
            println!("Error creating file '{}' !", path);
            println!("Error: {}", err);
            None
        }
    }
}

fn write_to_file (mut file: fs::File, write: String, out_path: &String) {
    // Make the write
    let write_status = file.write_all(write.as_bytes());
    // Report the status
    match write_status {
        Ok(()) => {
            println!("\tSuccessfuly wrote file '{}' !", out_path)
        },
        Err ( err ) => {
            println!("\tError writing to file '{}' !", out_path);
            println!("\tError: {}", err);
        }
    }
}
