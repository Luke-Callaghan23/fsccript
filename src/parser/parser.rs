pub mod parser {
    use std::collections::VecDeque;

    use crate::types::types::{
        FileContent,
        ParsedFile,
        CompilationTarget,
        CompilableSection
    };

    use crate::tokenizer::token_types::{
        TokenExpressionType, 
        TokenOfInterest, 
        get_token_expression_type
    };

    
    use crate::tokenizer::types::{
        TokenStream, 
        TokenOrStream, 
        Token
    };

    use crate::transpiler::types::{
        CompilationInstructions,
        InstructionsSet,
        CompilationInfo,
        Info
    };

        

    use crate::parser::check_parsable;
    

    pub fn parse_files <'a> (
        files: Vec<CompilationTarget<'a>>, 
        all_compilables: &'a InstructionsSet,
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
                        tokenized_file.tokenized,
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
                    // ;
                    // None
                } 
                else { None }
                
            })
        .flatten() // Flatten out the optionals
        .collect()  // collect as Vec<CompilationTarget>
    }

    fn parse_file<'a> (
        data: &'a [u8], 
        token_stream: TokenStream<'a>,
        all_compilables: &'a InstructionsSet
    ) -> ParsedFile<'a> {

        let size = data.len();
        
        let mut vanilla_sections: Vec<&'a [u8]> = Vec::new();
        let mut compilable_sections: Vec<CompilableSection<'a>> = Vec::new();

        let mut q: VecDeque<TokenStream> = VecDeque::new();
        
        let mut cur_vanilla_start = 0;

        q.push_back(token_stream);

        while !q.is_empty() {
            let mut stream = q.pop_front().unwrap();

            let mut last: Option<Token>  = None;
            while !stream.is_empty() {
                let token = stream.next();

                // println!("{}", token.unwrap());

                if let Some ( tok ) = token {
                    match &tok {
                        TokenOrStream::Token( tok ) => {
                            // Do an immutable copy of the current tok
                            let copied_token = (*tok).clone();
                            
                            // Check if the stream at this current point is a parsable fjs element
                            let check_parsable 
                                = check_parsable::check(&stream, all_compilables);
                            
                            if let Some( (compilation_instructions, index) ) = check_parsable {
                                
                                
                                // If there was some fjs element that can parse the stream at it's current part
                                //      we splice the stream, and attempt to parse the spliced stream as that fjs
                                //      element
                                
                                // Splicing the stream at its current point 
                                let mut spliced = stream.splice_stream().unwrap();
                                
                                // Attempt to parse the spliced stream
                                let info: Option<CompilationInfo> = (compilation_instructions.parse)(&mut spliced);
                                if let Some( info ) = info {
                                    
                                    let mut transpile_stream = stream.clone();
                                    for _ in 0..info.transpile_skips {
                                        transpile_stream.next();
                                    }

                                    // Skip to the end of the tokens used by the compilable section
                                    for _ in 0..info.full_skips {
                                        stream.next();
                                    }
                                    
                                    // Getting the Token immediately before and immediately after the parsed
                                    //      element
                                    let next_token 
                                        // Next token is either the next token in the stream if the next
                                        //      TokenOrStream is a token, or the first Token in the next
                                        //      stream
                                        = if let Some( tok_or_stream ) = stream.current() {
                                            match tok_or_stream {
                                                TokenOrStream::Token( tok ) => Some ( *tok ),
                                                TokenOrStream::TokenStream( ts ) => {
                                                    if let Some ( ts ) = ts.current() {
                                                        if let TokenOrStream::Token(tok) = ts {
                                                            Some(*tok)
                                                        } else { None }
                                                    } else { None }
                                                }
                                            } 
                                        } else { None };
                                    let prev_token = last;
                                    
                                    // Get the expression type of the next token
                                    let next_exp_type = 
                                        if let Some( next ) = next_token {
                                            get_token_expression_type(&next.token)
                                        }
                                        else { TokenExpressionType::None };

                                    // Get the expression type of the prev token
                                    let prev_exp_type = 
                                        if let Some( prev ) = prev_token {
                                            get_token_expression_type(&prev.token)
                                        }
                                        else { TokenExpressionType::None };
                                        

                                    // Check if next_token or prev_token are one of the tokens that denote an expression
                                    if next_exp_type == TokenExpressionType::OnlyAfterExp 
                                    || next_exp_type == TokenExpressionType::Expression 
                                    || prev_exp_type == TokenExpressionType::OnlyBeforeExp 
                                    || prev_exp_type == TokenExpressionType::Expression {
                                        
                                        // Push a new CompilableSection struct to the list of compilable sections
                                        compilable_sections.push(CompilableSection {
                                            content: &data, 
                                            stream: transpile_stream,
                                            compilation_info: info.comp_info,
                                            instruction_index: index,
                                        });

                                        // Getting the previous vanilla section, and reseting cur_vanilla_start
                                        let last_vanilla = &data[cur_vanilla_start..info.start];
                                        cur_vanilla_start = info.start;

                                        // Pushing the latest vanilla section to the end of vanilla_sections
                                        vanilla_sections.push(last_vanilla);
                                    }
                                    // Else, if the parsable object is not immediately following or followed by a token
                                    //      that denotes an expression, fall through and continue with the original
                                    //      stream
                                };  
                                // If the parse was not successful, simply fall through and continue with the originl
                                //      stream
                            }
                            last = Some(copied_token);
                        },
                        TokenOrStream::TokenStream( stream ) => {

                            // If the next token is a stream, we recurse onto it if it is any kind of stream 
                            //      but a comment, single quote string, or double quote string


                            // Finding if the stream stream begins with a comment open token
                            let skipable = 
                                if let Some( token_or_stream ) = stream.current() {
                                    match token_or_stream {
                                        TokenOrStream::Token( token ) => {
                                            token.token == TokenOfInterest::SingleLineCommentOpen ||
                                            token.token == TokenOfInterest::MultiLineCommentOpen  ||
                                            token.token == TokenOfInterest::StringDouble          ||
                                            token.token == TokenOfInterest::StringSingle          
                                        }
                                        TokenOrStream::TokenStream(_) => false
                                    }
                                } else { false };
                            
                            if !skipable {
                                // Recurse into the stream, as long as it is not a skipable stream
                                q.push_back(stream.clone());
                                last = None;
                            }
                        }
                    }
                }
            }
        }

        // Push the final section of vanilla code into vanilla_sections vec
        vanilla_sections.push(&data[cur_vanilla_start..size]);

        // Create and return a ParsedFile struct for the 
        ParsedFile {
            size,
            compilable_sections,
            vanilla_sections
        }
    }
}
