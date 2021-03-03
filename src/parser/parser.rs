
pub mod parser {

    use std::{collections::VecDeque, ops::Deref};

    use crate::{tokenizer::{token_types::{TokenExpressionType, TokenOfInterest, get_token_expression_type}, types::Token}, types::types::{
        FileContent,
        ParsedFile,
        CompilationTarget
    }};

    use crate::transpiler::types::{
        initialize_compilables,
        CompilationInstructions,
        InstructionsSet,
        CompilationInfo
    };

    use crate::tokenizer::token_types::TokenLookup;
    use crate::tokenizer::types::{TokenStream, TokenOrStream};

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
                        tokenized_file.tokenized,
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
                    // ;
                    // None
                } 
                else { None }
                
            })
        .flatten()
        .collect()
    }

    fn parse_file<'a, 'stream> (
        data: &'a [u8], 
        mut token_stream: TokenStream<'stream>,
        token_lookup: &TokenLookup,
        // pairs_lookup: &EnclosingPairs,
        all_compilables: &InstructionsSet
    ) -> ParsedFile<'a> {

        
        fn recurse (mut stream: TokenStream, all_compilables: &InstructionsSet) {
            let mut last: Option<Token>  = None;
            while !stream.is_empty() {
                let token = stream.next();

                if let Some ( tok ) = token {
                    match &tok {
                        TokenOrStream::Token( tok ) => {
                            // Do an immutable copy of the current tok
                            let copied_token = (*tok).clone();
                            
                            // Check if the stream at this current point is a parsable fjs element
                            let check_parsable 
                            = check_parsable::check(&stream, all_compilables);
                            
                            if let Some( parsable ) = check_parsable {
                                
                                // If there was some fjs element that can parse the stream at it's current part
                                //      we splice the stream, and attempt to parse the spliced stream as that fjs
                                //      element
                                
                                // Splicing the stream at its current point 
                                let spliced = stream.splice_stream().unwrap();

                                // Attempt to parse the spliced stream
                                let info: Option<CompilationInfo> = (parsable.parse)(&spliced);
                                if let Some( info) = info {
                                    
                                    // If the parse was successful, the remaining stream of tokens will be found in
                                    //      info.rem_stream
                                    let rem = info.rem_stream;
                                    
                                    // Getting the Token immediately before and immediately after the parsed
                                    //      element
                                    let next_token 
                                        // Next token is either the next token in the stream if the next
                                        //      TokenOrStream is a token, or the first Token in the next
                                        //      stream
                                        = if let Some( tok_or_stream ) = rem.current() {
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


                                        // TODO: make the compilable object thingy-ma-do



                                        // Recurse onto the "remaining stream" that the parsing function
                                        //      has passed to us
                                        recurse(rem, all_compilables);
                                        return;
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
                            // Finding if the stream stream begins with a comment open token
                            let is_comment = 
                                if let Some( token_or_stream ) = stream.current() {
                                    match token_or_stream {
                                        TokenOrStream::Token( token ) => {
                                            token.token == TokenOfInterest::SingleLineCommentOpen ||
                                            token.token == TokenOfInterest::MultiLineCommentOpen
                                        }
                                        TokenOrStream::TokenStream(_) => false
                                    }
                                } else { false };
                            
                            // If the stream is a comment, we can  skip over recursing into it
                            if !is_comment {
                                let spliced = stream.splice_stream().unwrap();
                                recurse(spliced, all_compilables);
                                last = None;
                            }
                        }
                    }
                }
            }
        }

        recurse(token_stream, all_compilables);

            
        ParsedFile {
            size: 2,
            compilable_sections: vec![],
            vanilla_sections: vec![]
        }
    }
}