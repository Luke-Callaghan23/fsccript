use std::str;
use crate::{parser::types::{CompilationTarget, FileContent, TokenizedFile}, tokenizer::token_types::*};
use super::token_stream::{Token, TokenStream, TokenOrStream};



/// # lookup_token
/// 
/// Token lookup function that returns the starting TokensOfInterest of an input byte string
///
/// # Parameters -- 
/// 
/// * `token: &[u8]` -- byte string on which the token lookup will be perfomed
/// * `lookup_table: &TokenLookup` -- the complete token lookup table found in 
///         ./src/tokenizer/token_type_lookup returned by initialize_lookup
///
/// # Returns -- 
/// * `TokensOfInterest` -- the beginning token of the `token` byte string
///
///
/// ## Examples -- 
/// ```
///     let lookup_table = initialize_lookup();
///
///     let string: &str = ">>>";
///     let res = tokenizer::lookup_token(string.as_bytes(), &lookup_table, TokensOfInterest::None);
///     assert_eq!(res, TokensOfInterest::ShRZeroFill);
/// 
///     let string: &str = ">>>=";
///     let res = tokenizer::lookup_token(string.as_bytes(), &lookup_table, TokensOfInterest::None);
///     assert_eq!(res, TokensOfInterest::ShRZeroFillEq);
///
///     let string: &str = "instanceof";
///     let res = tokenizer::lookup_token(string.as_bytes(), &lookup_table, TokensOfInterest::None);
///     assert_eq!(res, TokensOfInterest::Instanceof);
///
///     let string: &str = "a&";
///     let res = tokenizer::lookup_token(string.as_bytes(), &lookup_table, TokensOfInterest::None);
///     assert_eq!(res, TokensOfInterest::None);
///
/// ```
///
#[allow(non_snake_case)]
pub(crate) fn lookup_token (
    token: &[u8], 
    lookup_table: &TokenLookup, 
    last_token: TokensOfInterest, 
) -> TokensOfInterest {
    println!("{:?}", token);
    for char_step in &lookup_table.lookup {
        

        // Getting the next character of the token, and the token to test against
        let (next_char, next_token) = 
            if token.len() == 0usize { ('\0', token) }              // if there are no more characters, we've reached the end, so the last character is whitespace
            else { (token[ 0usize ] as char, &token[1..]) };        // otherwise, it is just the first character of the token
        
        if char_step.next != next_char {
            // If the first character of the token slice is not the 
            //      next char for this CharAndStep branch, then 
            //      just continue
            continue;
        }
        else {

            // If the current first character of the string does match the .next field of the 
            //      the current Step struct, continue the algorithm recursively

            // Finding the "last_token" for the next call of lookup token
            let RDLP = &char_step.end.RDLP;

            let string = str::from_utf8(next_token).unwrap();

            let last_token  = 
                if RDLP.is_match(string) {
                    // If the current string matches the RDLP field of the current step
                    //      then the last_token is the token of this step
                    char_step.end.token
                }
                else {
                    // If it does not match, set last_token to None
                    TokensOfInterest::None
                };
            
            match &char_step.step {
                Some ( lookup ) => {
                    // If there is another lookup table to search through, call lookup_token again 
                    //      with the new lookup and the last_token found above
                    return lookup_token (
                        &token[1..], 
                        &lookup, 
                        last_token,
                    );
                },
                None => {
                    // If there isn't another lookup table, just return
                    //      last_token
                    return last_token;
                }
            }

        }
    }
    // Return the last_token from the previous recursion -- (or None, if this is the first call)
    last_token
}

/// # tokenizer::find_next_token
///
/// Function to find the next token in an input byte string, starting from a given input
/// 
/// Returns None if start_index has reached the end of the non-whitespace character of a the byte
///     byte string
///
/// ## Parameters -- 
/// * `data: &'a [u8]` -- byte string to tokenize
/// * `token_start: &[char; 30]` -- array of characters that denote the start of all non-string TokensOfInterest
/// * `start_index: usize` -- starting index to tokenize the stream
/// * `lookup_table: &TokenLookup` -- lookup table for all TokensOfInterest
///
/// ## Returns --
/// * `Option<Token>` -- The next Token in the byte string, or None if the end of the bytestring has been reached
/// 
/// .
fn find_next_token (
    data:         &[u8], 
    start_index:  usize, 
    token_start:  &[char; 30], 
    lookup_table: &TokenLookup
) -> Option<Token> { 
    
    // Short loop to skip over all white space at the beginning of the 
    //      slice of data, data[start_index..], and sets local variable
    //      "index" will be the index of the first non-whitespace character
    //      after start_index
    let mut index = start_index;
    let mut chr = ' ';
    while index < data.len() {
        chr = data[index] as char;
        if !chr.is_whitespace() {
            break;
        }
        index = index + 1;
    }

    // println!("{}", start_index);

    // If index past the length of the data string, return None
    if index >= data.len() { None }
    else {

        // Do a lookup for a TokensOfInterest enum in the data array
        //      starting at index
        let next = lookup_token (
            &data[index..], 
            lookup_table, 
            TokensOfInterest::None
        );

        match next {
            TokensOfInterest::None => {

                // If the token we found was not an interesting token ( :( ), search for the end 
                //      of the non-interesting token by advancing the index of the byte-string
                //      and searching for each new character in the token_start array
                let start = index;
                while !token_start.iter().any(| ts | *ts == chr) && index < data.len() {        // "while chr is not in start_index, and index is still in range of the input array"
                    index += 1;                     // increment offset
                    chr = data[index] as char;      // get new character
                }
                // Once the end of the non-interesting token is found, return Some ( not interesting token )
                Some ( 
                    Token {
                        start: start,
                        end: index,
                        token: TokensOfInterest::None,
                    }
                )
            },
            _ => {
                // If the token was an interesting token, first find the 
                //      length of that token with get_token_len
                //      then return Some ( interesting token )
                let token_len = get_token_len(&next);
                Some (
                    Token {
                        start: index,
                        end: index + token_len as usize,
                        token: next
                    }
                )
            }
        }
    }
}

/// # tokenizer::tokenize_comment_or_String
///
/// Function to tokenize a comment or string and store the contents into a parameter stream
///
/// ## Parameters -- 
/// * `data: &'a [u8]` -- byte string to tokenize
/// * `start_index: usize` -- starting index to tokenize the stream
/// * `lookup_table: &TokenLookup` -- lookup table for all TokensOfInterest
/// * `end_token: TokensOfInterest` -- the ending token of this stream
/// * `mut stream: TokenStream<'a>` -- the TokenStream that this function will fill in 
///
/// ## Returns --
/// * `TokenStream<'a>` -- parameter `mut stream: TokenStream<'a>` stream with commend / stream
///     tokens added
/// .
fn tokenize_comment_or_String <'a> (
    data: &'a [u8], 
    start_index: usize, 
    lookup_table: &TokenLookup,
    end_token: TokensOfInterest, 
    mut stream: TokenStream<'a>
) -> TokenStream<'a> {

    let mut index = start_index;
    let start = start_index;
    let end_len =  get_token_len(&end_token) as usize;
    while index < data.len() {

        // Get the next token at the current index
        let next = lookup_token(&data[index..], lookup_table, TokensOfInterest::None);

        if next == end_token {

            // Add a None token for the entire string / comment text
            stream.add_token(
                TokenOrStream::Token(
                    Token {
                        start: start,
                        end: index,
                        token: TokensOfInterest::None,
                    }
                )
            );

            // Add a the end comment / string token
            stream.add_token(
                TokenOrStream::Token(
                    Token {
                        start: index,
                        end: index + end_len,
                        token: end_token
                    }
                )
            );

            // Return the stream
            return stream;
        }

        // increment the index
        index += 1;
    }

    // If no end token was found, add a big ole' None Token to the stream
    stream.add_token (
        TokenOrStream::Token (
            Token {
                start: start,
                end: data.len(),
                token: TokensOfInterest::None
            }
        )
    );

    // Then, add an EOF token to the end of this stream
    stream.add_token (
        TokenOrStream::Token (
            Token {
                start: data.len(),
                end: data.len(),
                token: TokensOfInterest::EOF
            }
        )
    );

    // Then, return the stream
    stream
}


/// # tokenizer::tokenize_stream
///
/// Function to tokenize an enclosing stream of TokensOfInterest enums, starting from a given Token, start_token
///     and ending at an ending TokensOfInterest, end_token.
/// 
/// If no ending token is found in the byte string, then the stream will be finished by an EOF Token
///
/// ## Parameters -- 
/// * `data: &'a [u8]` -- byte string to tokenize
/// * `start_index: usize` -- starting index to tokenize the stream
/// * `token_start: &[char; 30]` -- array of characters that denote the start of all non-string TokensOfInterest
/// * `lookup_table: &TokenLookup` -- lookup table for all TokensOfInterest
/// * `depth: usize` -- the count of all TokenStreams that are parent to this one
/// * `start_token: Token` -- starting token of the stream to tokenize
/// * `end_token: TokensOfInterest` -- the ending token of this stream
///
/// ## Returns --
/// * `TokenStream<'a>` -- a token stream of all TokensOfInterest in this stream, between the 
///     start_token that was anready found, and the end_token that this function is searching for
/// .
fn tokenize_stream <'a> (
    data:         &'a [u8], 
    start_index:  usize, 
    token_start:  &[char; 30], 
    stream_start: &[TokensOfInterest; 8],
    lookup_table: &TokenLookup,
    depth:        usize,
    start_token:  Token,
    end_token:    TokensOfInterest,
) -> TokenStream<'a> {
    
    let is_comment_or_string = token_is_comment_or_string(&start_token.token);
    let token_len = get_token_len(&start_token.token);
    let mut stream = TokenStream::new(start_token, depth);
    
    if is_comment_or_string {
        // If the current token is the opening token for a comment or string,
        //      simply call skip_to_end_comment_string, and return the result
        tokenize_comment_or_String(
            data, 
            start_index, 
            lookup_table,
            end_token, 
            stream
        )
    }
    else {

        let mut index = start_index;
        loop {
            let next = find_next_token(data, index, token_start, lookup_table);
    
            match next {
                Some ( token ) => {
                    let token_type = token.token.clone();
                    if stream_start.iter().any(| tok | *tok == token_type) {
                        // If the next token is a stream_start token, then recurse on this function
                        //      with the new token as the starting token
                        // And add the returned stream to this stream
                        index = stream.add_token (
                            TokenOrStream::TokenStream (
                                // Create the new token stream by recursing onto the function
                                tokenize_stream (
                                    data,
                                    token.end,                    // start_index is the end index of the retrieved token
                                    token_start,
                                    stream_start,
                                    lookup_table,                           
                                    depth + 1,                         // incremented depth
                                    token,                        // starting token is the token retrieved from find_next_token
                                    get_end_token(&token_type)      // end token is the get_end_token pair of the starting token
                                )
                            )
                        );
                        println!("{}", index);
                    }
                    else {
                        println!("{:?}", token);
                        // Add the new token to the current token stream
                        index = stream.add_token (TokenOrStream::Token ( token ));
                        if token_type  == end_token {
                            // And, if the current token is the end_token for this
                            //      stream, we break the while true loop, and 
                            //      return the stream
                            break;
                        }
                    }
                },
                None => {
                    // If no token was returned we've reached EOF,
                    //      add an EOF token and return
                    stream.add_token (
                        TokenOrStream::Token (
                            Token {
                                token: TokensOfInterest::EOF,
                                start: index,
                                end: data.len()
                            }
                        )
                    );
                    if end_token == TokensOfInterest::EOF && depth == 0 {
                        break;
                    }
                    else {
                        // TODO: error -- broken .js
                        break;
                    }
                }
            }
        }
        stream
    }
}

/// # tokenizer::tokenize
///
/// Function to tokenize a byte string into a stream of TokensOfInterest and sub-streams
///         of Tokens for enclosing scopes
///
/// ## Parameters -- 
/// * `data: &'a [u8]` -- byte string to tokenize
/// * `lookup_table: &TokenLookup` -- lookup table for all TokensOfInterest
///
/// ## Returns --
/// * `TokenStream<'a>` -- a token stream of all TokensOfInterest in the input data array,
///     starting at the beginning of the stream
///
/// ## Examples -- 
/// 
/// ```
///
/// let token_stream = tokenize(
///     b"
///     switch(key) {
///         case 0: 
///             a = b;
///             break;
///     }
///     ",
///     &initialize_lookup()
/// );
/// 
/// assert_eq!(format!("{}", token_stream),         
/// "Token ( BOF )
/// Token ( Switch )
/// Stream {
///     Token ( OpenParen )
///     Token ( None )
///     Token ( CloseParen )
/// }
/// Stream {
///     Token ( OpenCurly )
///     Stream {
///         Token ( Case )
///         Token ( None )
///         Token ( Colon )
///         Token ( None )
///         Token ( Assign )
///         Token ( None )
///         Token ( Semicolon )
///         Token ( Break )
///     }
///     Token ( Semicolon )
///     Token ( CloseCurly )
/// }
/// Token ( EOF )
/// ");
///
///
/// let token_stream = tokenize(
///     b"
///     switch(key) {
///         case 0: 
///             a = b;
///             break;
///         case 1:
///             b = a;
///             break;
///         case 4:
///             console.log(b);
///             break;
///         default:
///             c = a;
///             break
///     }
///     ",
///     &initialize_lookup()
/// );
/// 
/// assert_eq!(format!("{}", token_stream),         
/// "Token ( BOF )
/// Token ( Switch )
/// Stream {
///     Token ( OpenParen )
///     Token ( None )
///     Token ( CloseParen )
/// }
/// Stream {
/// Token ( OpenCurly )
/// Stream {
///     Token ( Case )
///     Token ( None )
///     Token ( Colon )
///     Token ( None )
///     Token ( Assign )
///     Token ( None )
///     Token ( Semicolon )
///     Token ( Break )
/// }
/// Token ( Semicolon )
/// Stream {
///     Token ( Case )
///     Token ( None )
///     Token ( Colon )
///     Token ( None )
///     Token ( Assign )
///     Token ( None )
///     Token ( Semicolon )
///     Token ( Break )
/// }
/// Token ( Semicolon )
/// Stream {
///     Token ( Case )
///     Token ( None )
///     Token ( Colon )
///     Token ( None )
///     Token ( Dot )
///     Token ( None )
///     Stream {
///         Token ( OpenParen )
///         Token ( None )
///         Token ( CloseParen )
///     }
///     Token ( Semicolon )
///     Token ( Break )
/// }
/// Token ( Semicolon )
/// Stream {
///     Token ( Default )
///     Token ( Colon )
///     Token ( None )
///     Token ( Assign )
///     Token ( None )
///     Token ( Semicolon )
///     Token ( Break )
///     }
///     Token ( CloseCurly )
/// }
/// Token ( EOF )
/// ");
/// 
/// ```
pub fn tokenize <'a> (data: &'a [u8], lookup_table: &TokenLookup) -> TokenStream<'a> {
    
    // Array of all characters that begin a new token
    let token_start = [
        '&', '|', '^', '~', '=', 
        '!', '<', '>', '+', '-', 
        '*', '/', '%', '(', '[', 
        ')', ']', ',', ':', '.', 
        '\n', '\'',  '\t', '\r', 
        ' ', '"', '`', '{', '}',
        ';',
    ];

    // Array of all the tokens that should begin a sub-stream inside of a token stream
    let stream_start = [
        TokensOfInterest::OpenCurly,             TokensOfInterest::OpenParen,             TokensOfInterest::OpenBrack, 
        TokensOfInterest::StringSingle,          TokensOfInterest::StringDouble,          TokensOfInterest::StringBackTick,
        TokensOfInterest::SingleLineCommentOpen, TokensOfInterest::MultiLineCommentOpen,  
        // TokensOfInterest::Case,
        // TokensOfInterest::Default, 
    ];

    // Call tokenize stream with base variables, and returns resulting stream
    tokenize_stream (
        data,                
        0,                              // start at the beginning of the string
        &token_start,                              
        &stream_start,
        &lookup_table,
        0,                                   // depth = 0
        Token {                         // Starting token is BOF (beginning of file), start=0, end=0
            start:0,
            end:0,
            token: TokensOfInterest::BOF,
        },
        TokensOfInterest::EOF             // ending token is EOF (end of file)
    )
}

pub fn tokenize_targets <'a> (targets: Vec<CompilationTarget<'a>>,  lookup_table: &TokenLookup) -> Vec<CompilationTarget<'a>> {
    targets.into_iter().map(| target | {
        tokenize_target(target, lookup_table)
    })
    .flatten()
    .collect()
}

fn tokenize_target <'a> (target: CompilationTarget<'a>, lookup_table: &TokenLookup) -> Option<CompilationTarget<'a>> {
    
    if let FileContent::Raw( raw_file ) = target.contents {
        Some (
            CompilationTarget {
                contents: FileContent::Tokenized(
                    TokenizedFile {
                        size: raw_file.len(),
                        tokenized: tokenize(raw_file, lookup_table),
                        contents: raw_file
                    }
                ),
                input_path: target.input_path,
                output_path: target.output_path,
            }
        )    
    }
    else { None }
}