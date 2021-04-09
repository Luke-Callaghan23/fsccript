use std::{borrow::Borrow, collections::VecDeque};

use crate::{tokenizer::{token_types::TokenOfInterest, types::{TokenOrStream, TokenStream}}, transpiler::types::{
    CompilationInfo, 
    CompilationInstructions, 
    CompileType, 
    Info
}, types::types::CompilableSection};



use crate::transpiler::transpiler;

pub fn implement_switch () -> CompilationInstructions {
    CompilationInstructions {
        comp_type:    CompileType::Switch,
          check:      Box::new(check_switch),
          parse:      Box::new(parse_switch),
        transpile:    Box::new(transpile_switch)
    }
}

pub struct SwitchInfo {
    parens: (usize, usize),
    curlies: (usize, usize)
}


fn check_switch <'a> (token_stream: &'a TokenStream<'a>) -> bool {
    if let Some ( cur ) = token_stream.current() {
        if let TokenOrStream::Token( tok ) = cur {
            tok.token == TokenOfInterest::Switch
        }
        else { false }
    }
    else { false }
}


fn parse_switch<'a> (token_stream: &'a mut TokenStream<'a>) -> Option<CompilationInfo> {
    // We can assume that since this function has been called, the current token in the parameter token_stream
    //      is a TokenOfInterest::Switch token
    // The next token should be a parenthesis token stream, and the token after that should be a curly brackets
    //      theme

    let switch = token_stream.current().unwrap();
    let start = switch.start();

    token_stream.next();

    let parens_token = token_stream.next();
    let parens_type = TokenStream::get_token_type(parens_token);
    
    if parens_type  == TokenOfInterest::OpenParen {

        
        if let Some( parens_token ) = parens_token {
            
            let parens_start = parens_token.start();
            let parens_end   = parens_token.end();
            
            let curlies_token = token_stream.next();
            let curlies_type = TokenStream::get_token_type(curlies_token);
            
            if curlies_type  == TokenOfInterest::OpenCurly {
                
                if let Some( curlies_token ) = curlies_token {
                    
                    let curlies_start = curlies_token.start();
                    let curlies_end   = curlies_token.end();
                    
                    Some (CompilationInfo {
                        transpile_skips: 2,
                        full_skips: 3,
                        start: start,
                        end: curlies_end,
                        comp_info: Info::SwitchInfo (
                            SwitchInfo {
                                parens:  (parens_start, parens_end),
                                curlies: (curlies_start, curlies_end),
                            }
                        )
                    })
                }
                else { None }
            } 
            else { None }
        }
        else { None }
    }
    else { None }
}

fn transpile_switch<'a> (comp_section: CompilableSection<'a>) -> String {
    let data = comp_section.content;
    let switch_info = comp_section.compilation_info;
    if let Info::SwitchInfo( switch_info ) = switch_info {
        match comp_section.stream.step_into() {
            Some ( mut stream ) => {

                let parens  = switch_info.parens;
                let curlies = switch_info.curlies;
    
                println!("{}", std::str::from_utf8(&data[parens.0+1..parens.1-1]).unwrap());
                println!("{}", std::str::from_utf8(&data[curlies.0+1..curlies.1-1]).unwrap());
    
    
                let paren_data = &data[parens.0+1..parens.1-1];
                let curly_data = &data[curlies.0+1..curlies.1-1];

                let mut insert_return: VecDeque<usize> = VecDeque::new();

                while !stream.is_empty() {

                    let tok = stream.current().unwrap();

                    println!("{}", stream.current().unwrap());

                    // todo find places to add 'return'

                    


                    stream.next();
                }

    
                match std::str::from_utf8(data) {
                    Ok( str_data ) => {
                        // Wrapping the return string as a js expression IIFIE
                        transpiler::wrap_as_expression(
                            // insert "return " strings in every specified area
                            transpiler::insert_returns(
                                str_data, 
                                insert_return
                            )
                        )
                    },    
                    Err( err ) => {
                        String::from("oopsies!")
                    }    
                }    
            }
            None => String::from("oopsies!"),
        }
    }
    else { String::from("oopsies!") }
    
    /*
    // Getting the parenthesis as a spliced token stream
    let parens = 
        if let TokenOrStream::TokenStream( stream ) = parens_token {
            stream.splice_stream()
        } else { None }
    .unwrap();
    // Getting the curlies stream  as a spliced token stream
    let mut curlies = 
        if let TokenOrStream::TokenStream( stream ) = curlies_token {
            stream.splice_stream()
        } else { None }
    .unwrap();
    */
    
    // let mut token_index: usize = 1;          // index of the current token in the curlies stream
    // let mut cases: Vec<usize> = Vec::new();   // indeces of all cases or defaults in the curlies token stream
    
    // let _open = curlies.next();
    
    // // Gathering the case statemesnts in the curly brackets 
    // while !curlies.is_empty() {
        
    //     let cur = curlies.next();
    
    //     let tok_type = TokenStream::get_token_type(cur);
    //     if tok_type == TokenOfInterest::Case 
    //     || tok_type == TokenOfInterest::Default {
    //         cases.push(token_index);
    //     }
    //     // curlies.next();
    //     token_index += 1
    // }
    
    // if cases.len() >= 1 {
    // }
    // else { None }
}
