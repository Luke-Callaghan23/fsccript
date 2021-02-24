
pub mod parser {

    use crate::parser::types::{
        FileContent,
        ParsedFile,
        CompilationTarget
    };
    use crate::transpiler::types::{
        initialize_compilables,
        CompilationInstructions,
    };

    use crate::parser::token_type_lookup::*;
    use crate::parser::check_parsable;
    use crate::parser::enclosing_pairs::{
        CommentType,
        initialize_enclosing_pairs,
        split_at_enclosing_pair,
        find_enclosing_pair,
        is_string_pair,
        find_end_comment,
        EnclosingPairs,
        EnclosingSplit
    };

    pub fn parse_files (files: Vec<CompilationTarget>) -> Vec<CompilationTarget> {
        let token_table = initialize_lookup();
        let pair_table = initialize_enclosing_pairs();
        let all_compilables = initialize_compilables();
        
        // Return files mapped on the function parse_file and &lookup_table
        files
            .into_iter()
            .map(| file | {

                // Extract the raw file contents from the targeted file contents
                let raw_file = 
                    if let FileContent::Raw( raw ) = file.contents { raw } 
                    else { String::from("") };
                
                // Parse the raw file contents
                let parsed_file = parse_file(
                    raw_file, 
                    &token_table,
                    &pair_table,
                    &all_compilables
                );

                // Return a new CompilationTarget with the contents
                //      swapped out for the parsed file
                CompilationTarget {
                    input_path:  file.input_path,
                    output_path: file.output_path,
                    contents: FileContent::Parsed (
                        parsed_file
                    )
                }
            })
        .collect()
    }

    fn parse_file (
        data: String, 
        token_lookup: &TokenLookup,
        pairs_lookup: &EnclosingPairs,
        all_compilables: &[CompilationInstructions]
    ) -> ParsedFile {

        let data = data.as_bytes();
        
        // Getting the raw data of the compilation target as an array of bytes

        let mut index = 0;
        while index < data.len() {

            let target = &data[index..];


            // Three cases to look out for:
            //      1. new target begins with a comment -- skip ahead past the comment
            //      2. new target begins with a string -- skip ahead, past the string, 
            //               keep the contents of the string if it is a template string
            //      3. new target begins with a parasable element -- parse, make a 
            //              Transpilation::Original enum
            // Otherwise, just skip

            let c_one = target[0] as char;

            // Check comment:
            if c_one == '/' && target.len() > 1 {
                let c_two = target[1] as char;

                if c_two == '/' {
                    // single line comments -- //
                    let end = find_end_comment(target, CommentType::Singleline);
                    index = end;    // move to end
                    continue;
                }
                else if c_two == '*' {
                    // multiline comments -- /*
                    let end = find_end_comment(target, CommentType::Multiline);
                    index = end;    // move to end
                    continue;
                }
            }

            // Check string
            if is_string_pair(c_one) {
                let (end, _) = find_enclosing_pair(data, c_one, index, pairs_lookup);
                if c_one == '`' {
                    let splits = EnclosingSplit {
                        before: &data[0..index],
                        middle: &data[index+1..end],
                        after: &data[end+1..data.len()],
                        pair: ('`', '`')
                    };

                    // TODO: check the inside of the template string for ${} and parsable
                    // TODO:        sections of fscript code in there

                }

                // Move the index to the end of the string section
                index = end + 1;
            }

            // Check parsable element
            let check_parsable 
                = check_parsable::check(target, index, all_compilables);
            if let Some( parsable ) = check_parsable {

            }
            
            index += 1;
        }


        ParsedFile {
            size: 2,
            compilable_sections: vec![],
            vanilla_sections: vec![]
        }
    }


    // pub fn next_token_of_interest_2 <'a> (data: &'a str, token_lookup: &TokenLookup) -> (&'a str, TokensOfInterest) {
        

    //     // let token_begin_chars = | search: char | [ '&', '|', '^', '~', '=', '!', '>', '<', '+', '-', '*', '/', '%', '(', '[', ':', ',', '.', ']', ')' ].iter().find(|x| **x == search);
    //     // let 

    //     let mut last: TokensOfInterest      // the last token found
    //         = TokensOfInterest::None;
        
    //     let mut start: usize = 0;           // 

    //     // Get rid of all leading whitespace -- that is definitely not a part
    //     //      of a token    
    //     let data = data.trim_start();
        
    //     for end in 1..data.len()+1 {
    //         // Getting the current slice to inspect as a token
    //         let slice 
    //             = &data[0..end].as_bytes();
        



    //     }


    //     ("", last)
    // }


    // /// # Function to func the next largest token of interest in the string slice
    // /// 
    // pub fn next_token_of_interest <'a> (data: &'a str, token_lookup: &TokenLookup) -> (&'a str, TokensOfInterest) {
        
    //     let mut last: TokensOfInterest      // the last token found
    //         = TokensOfInterest::None;

    //     // Get rid of all leading whitespace -- that is definitely not a part
    //     //      of a token    
    //     let data = data.trim_start();
        
    //     for end in 1..data.len()+1 {
    //         // Getting the current slice to inspect as a token
    //         let slice 
    //             = &data[0..end].as_bytes();
            
    //         // println!("{}: {}", end, &data[0..end]);

    //         // Search the current slice in the token lookup table
    //         let cur = lookup_token (
    //             slice, 
    //             token_lookup
    //         );

    //         if cur  == TokensOfInterest::None 
    //         && last != TokensOfInterest::None {

    //             match last {
    //                 TokensOfInterest::In         |
    //                 TokensOfInterest::If         |
    //                 TokensOfInterest::Of         |
    //                 TokensOfInterest::New        |
    //                 TokensOfInterest::Switch     |
    //                 TokensOfInterest::Delete     |
    //                 TokensOfInterest::Typeof     |
    //                 TokensOfInterest::Instanceof => {
    //                     if !(data.as_bytes()[end] as char).is_alphabetic() {
    //                         return (&data[end-1..], last);
    //                     }
    //                     else { 
                            
    //                     }
    //                 },
    //                 _ => {
    //                     return (&data[end-1..], last);
    //                 }
    //             };

    //             // If current slice is not a token, but the last slice
    //             //      was a valid token, then the last token is the 
    //             //      first largest token in the slice
    //             return (&data[end-1..], last);
    //         }
    //         last = cur;
    //     }
    //     // Return an empty string, and whatever token that the last
    //     //      slice evaluated to
    //     ("", last)
    // }

}