
pub mod parser {

    use crate::parser::types::{
        FileContent,
        ParsedFile,
        CompilationTarget
    };

    use crate::transpiler::types::{
        initialize_compilables,
        CompilationInstructions,
        InstructionsSet
    };

    use crate::tokenizer::token_types::TokenLookup;
    use crate::tokenizer::token_stream::TokenStream;

    use crate::parser::check_parsable;
    

    pub fn parse_files <'a> (
        files: Vec<CompilationTarget<'a>>, 
        all_compilables: &'a InstructionsSet,
        token_lookup: &TokenLookup,
    ) -> Vec<CompilationTarget<'a>> {
        
        // 
        files
            .into_iter()
            .map(move | file | {

                // Extract the raw file contents from the targeted file contents
                if let FileContent::Tokenized( tokenized_file  ) = file.contents { 

                    let raw_file = tokenized_file.contents;

                    // Parse the raw file contents
                    let parsed_file = parse_file(
                        raw_file, 
                        token_lookup,
                        all_compilables
                    );
    
                    // Return a new CompilationTarget with the contents
                    //      swapped out for the parsed file
                    Some ( 
                        CompilationTarget {
                            input_path:  file.input_path,
                            output_path: file.output_path,
                            contents: FileContent::Parsed (
                                parsed_file
                            )
                        }
                    )
                } 
                else { None }
                
            })
        .flatten()
        .collect()
    }

    fn parse_file<'a> (
        data: &'a [u8], 
        token_lookup: &TokenLookup,
        // pairs_lookup: &EnclosingPairs,
        all_compilables: &InstructionsSet
    ) -> ParsedFile<'a> {

        
        // Getting the raw data of the compilation target as an array of bytes

        let mut index = 0;
        while index < data.len() {

            // let target = &data[index..];


            // Three cases to watch out for:
            //      0. new target begins with whitespace or comment -- skip over the whitespace / comments
            //      1. new target begins with a string -- skip the string
            //      2. new target begins with a parasable element --> parse, make a CompilableSection struct
            // Otherwise, just skip

            // // Skip comments / whitespace
            // let end = skip_comments_and_whitespace(data, index);
            // if end != 0 {
            //     index += end + 1;
            //     continue;
            // }

            // // Skip strings
            // // TODO: `` template strings .fjs
            // let end = skip_string(data, index);
            // if end != 0 {
            //     index += end + 1;
            //     continue;
            // }


            // // Check parsable element
            // let check_parsable 
            //     = check_parsable::check(data, index, all_compilables);
            // if let Some( parsable ) = check_parsable {

            // }
            
            index += 1;
        }


        ParsedFile {
            size: 2,
            compilable_sections: vec![],
            vanilla_sections: vec![]
        }
    }
}