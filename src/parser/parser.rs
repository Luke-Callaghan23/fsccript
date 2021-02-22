
pub mod parser {
    use std::borrow::Borrow;

    use crate::types::{
        FileContent::Parsed,
        FileContent::Raw,
        ParsedFile,
        CompilationTarget
    };

    use crate::token_type_lookup::*;


    pub fn parse_files (files: Vec<CompilationTarget>) -> Vec<CompilationTarget> {
        let lookup_table = initialize_lookup();
        
        // Return files mapped on the function parse_file and &lookup_table
        files
            .into_iter()
            .map(| file | {
                parse_file(file, &lookup_table)
            })
        .collect()
    }

    fn parse_file (target: CompilationTarget, token_lookup: &TokenLookup) -> CompilationTarget {
        
        // Getting the raw data of the compilation target as an array of bytes
        let raw_file = 
            if let Raw( raw ) = target.contents { raw } 
            else { String::from("") };



        CompilationTarget {
            contents: Parsed (
                ParsedFile {
                    size: 2,
                    compilable_sections: vec![],
                    vanilla_sections: vec![]
                }
            ),
            input_path: target.input_path,
            output_path: target.output_path
        }
    }

    use regex::Regex;

    pub fn next_token_of_interest <'a> (
        data: &'a str, 
        token_lookup: &TokenLookup
    ) -> Option<(&'a str, TokensOfInterest)> {

        // Regex that captures the all characters before a whitespace character
        let re: regex::Regex = Regex::new("[^\\s]*").unwrap();
        
        // Finding the next token of interest, by:
        re
        .find_iter(data)   // find_iter creates an iterator over matches of the "re" regex
        .find_map(|m| {           // find_map returns the first Some<> result of the below closure on the matches of the "re" regex
            
            // Search the token 
            let token = lookup_token (
                m.as_str().as_bytes(), 
                token_lookup
            );

            // Match the token to some interesting token, or TokensOfInterest::None
            match token {
                 // If the token is None, return none
                TokensOfInterest::None => None,   
                _ => {
                    // Otherwise, return a slice of the data string, and the found
                    //      token
                    let end = m.end();
                    Some ((
                        &data[end..],
                        token
                    ))
                }
            }
        })
    }

}