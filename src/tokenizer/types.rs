#[derive(Debug, Clone)]
pub struct TokenStream <'a> {
    stream:       Stream <'a>,
    stream_index: usize,
    stream_len:   usize,
    byte_count:   usize,
    depth:        usize,
    start:        usize,
    end:          usize,
    at_start:     bool,
    // delimiters:   (&'a Token, Option<&'a Token>),
    // pub outer:        Option<&'a TokenStream <'a>>
}


#[derive(Debug, Clone)]
enum Stream <'a> {
    RefStream ( &'a [TokenOrStream <'a>] ),
    Stream ( Vec<TokenOrStream <'a>> ),
}

#[derive(Debug, Clone)]
pub enum TokenOrStream <'a> {
    Token ( Token ),
    TokenStream ( TokenStream <'a> )
}

use crate::tokenizer::token_types::TokenOfInterest;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Token {
    pub token: TokenOfInterest,
    pub start: usize,
    pub end: usize,
}

impl <'a> TokenOrStream<'a> {
    pub fn start (&self) -> usize {
        match self {
            TokenOrStream::Token( tok ) => {
                tok.start
            }
            TokenOrStream::TokenStream( stream ) => {
                stream.start
            }
        }
    }
    pub fn end (&self) -> usize {
        match self {
            TokenOrStream::Token( tok ) => {
                tok.end
            }
            TokenOrStream::TokenStream( stream ) => {
                stream.end
            }
        }
    }
}

use std::fmt;

impl <'a> std::fmt::Display for TokenStream<'a>  {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        let stream = TokenStream::stream_as_slice(&self.stream, self.stream_len);

        // Get the amount of tabs needed for this stream depth
        let tabs = (0..self.depth).fold(String::from(""), | mut acc, _i | { acc.push_str("    "); acc });

        stream.iter().for_each(| item | {
            write!(f, "{}", tabs);
            match item {
                TokenOrStream::Token(token) => {
                    write!(f, "Token ( {:?} )\n", token.token);
                }
                TokenOrStream::TokenStream(stream) => {
                    write!(f, "Stream {}\n{}{}{}\n", '{', stream, tabs, '}');
                }
            }
        });


        write!(f, "")
    }
}

impl <'a> std::fmt::Display for TokenOrStream<'a>  {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        match self {
            TokenOrStream::Token(token) => {
                write!(f, "Token ( {:?} )\n", token.token);
            }
            TokenOrStream::TokenStream(stream) => {
                write!(f, "Stream {:?}", stream.first());
            }
        }


        write!(f, "")
    }
}

// TokenStream implementations that focus on building 
impl <'a> TokenStream <'a> {

    /// # TokenStream::new
    ///
    /// Generates a new TokenStream with an initial parameter Token
    ///
    /// ## Parameters -- 
    /// * `token: Token` -- the initial
    /// * `depth: usize` -- 
    ///
    /// ## Returns --
    /// * `TokenStream <'a>` -- a new TokenStream struct
    ///
    /// .
    pub fn new (token: Token, depth: usize) -> TokenStream<'a> {
        
        let mut stream = TokenStream {
            stream:       Stream::Stream( vec![] ),
            stream_index: 0,
            stream_len:   0,
            byte_count:   0,
            depth:        depth,
            start:        token.start,
            // delimiters:   (&token, None),
            end:          0,
            at_start:     true,
        };
        stream.add_token(TokenOrStream::Token( token ));
        stream
    }

    /// # TokenStream::add_token
    ///
    /// Function to add a token to a token stream
    ///
    /// ## Returns --
    /// * `TokenStream <'a>` -- a new TokenStream struct
    ///
    /// .
    pub fn add_token (&mut self, token: TokenOrStream<'a>) -> usize {
        match &mut self.stream {
            Stream::Stream( stream ) => {
                
                // Check if the token is a comment token
                let token_type = TokenStream::get_token_type(Some( &token ));
                if token_type == TokenOfInterest::SingleLineCommentOpen
                || token_type == TokenOfInterest::MultiLineCommentOpen {
                    // If it's a comment token, we just don't add it, lol
                    return self.end;
                }
                // if let TokenOrStream::TokenStream( stream ) = &token {
                //     let token_type= stream.current_token_type();
                // }
                
                
                
                // Increment the end of self
                self.end = match &token {
                    TokenOrStream::Token( token ) => token.end,
                    TokenOrStream::TokenStream( token_stream ) => token_stream.end,
                };



                // Push the token to the end of self's stream
                stream.push(token);

                self.stream_len += 1;
                self.end
            },
            Stream::RefStream( _ ) => {
                // TODO: panic error
                0
            }
        }
    }
}

// TokenStream implementations that focus on traversing a token stream
impl <'a> TokenStream <'a> {

    
    /// # TokenStream::stream_as_slice
    ///
    /// Function to get a Stream struct as a TokenOrStream struct
    ///
    /// ## Returns -- 
    /// * `&[TokenStream<'a>]` -- input Stream as a slice
    fn stream_as_slice (stream: &'a Stream, len: usize) -> &'a [TokenOrStream <'a> ] {
        match stream {
            Stream::Stream( stream ) => { &stream[0..len] },
            Stream::RefStream( stream ) => { stream }
        }
    }

    /// # TokenStream::splice_stream
    ///
    /// Function to splice off a new token stream, starting from the current
    ///        index in self's stream
    ///
    /// ## Returns -- 
    /// * `Option<TokenStream<'a>>` -- a new stream, parallel to self's stream with
    ///         the all the same fields as self, but with a stream that only 
    ///         starts at the current index of self's stream
    pub fn splice_stream (&'a self) -> Option<TokenStream<'a>> {
        // Construct some new Token stream using almost the same
        //        exact fields as self
        // .stream is a slice of self's .stream starting at the current stream index
        // .stream_index = 0
        Some (TokenStream {
            stream: Stream::RefStream (
                if self.stream_index < self.stream_len { 
                    &TokenStream::stream_as_slice(&self.stream, self.stream_len)[self.stream_index..]
                }
                else {
                    &TokenStream::stream_as_slice(&self.stream, self.stream_len)[0..0]
                }
            ),
            stream_index: 0,
            stream_len:   self.stream_len - self.stream_index,
            byte_count:   self.byte_count,
            depth:        self.depth,
            start:        self.start,
            end:          self.end,
            at_start:     true,
            // delimiters:   self.delimiters.clone(),
            // outer:        self.outer,
        })
    }
    
    /// # TokenStream::step_into
    ///
    /// Function to step into the current item in self's TokenOrStream stream
    ///        if that item is a TokenStream    
    ///
    /// This is analagous to stepping into an enclosing scope, like {}, (), []
    ///        '', "", ``, //\n, or /**/
    ///
    /// Returns None if:
    ///    *    The current TokenStream, self, has reached the end of the stream
    ///    *    Or, if the current TokenOrStream enum is of type TokenOrStream::Token
    ///
    /// ## Returns -- 
    /// * `Option<&TokenStream <'a>>` -- The stream that we stepped into, or None if
    ///     there was no Stream to step into
    ///
    pub fn step_into (self) -> Option<TokenStream <'a>> {

        if self.stream_index < self.stream_len {
            let stream = match &self.stream {
                Stream::Stream( stream ) => { &stream[self.stream_index] },
                Stream::RefStream( stream ) => { &stream[self.stream_index] }
            };

            match stream {
                TokenOrStream::Token( _ ) => None,
                TokenOrStream::TokenStream( stream ) => Some( stream.clone() )
            }
        }
        else { None }
    }
    
    /// # TokenStream::first -- 
    ///
    /// Function get the first TokenOrStream struct from self's TokenStream
    ///     or None if the stream is empty
    ///
    /// ## Returns -- 
    /// * `Option<&TokenOrStream <'a>>` -- the first Token / Stream in self's
    ///     token stream or None if the end of the stream has been reached
    ///
    pub fn first (&'a self) -> Option<&TokenOrStream<'a>> {
        if self.stream_len > 0 {
            Some (
                &TokenStream::stream_as_slice(&self.stream, self.stream_len)[0]
            )            
        }    
        else { None }    
    }


    /// # TokenStream::first -- 
    ///
    /// Function get the first TokenOrStream struct from self's TokenStream
    ///     or None if the end of the stream is empty
    ///
    /// ## Returns -- 
    /// * `Option<&TokenOrStream <'a>>` -- the last Token / Stream in self's
    ///     token stream or None if the end of the stream has been reached
    ///
    pub fn end (&'a self) -> Option<&TokenOrStream<'a>> {
        if self.stream_len > 0 {
            Some (
                &TokenStream::stream_as_slice(&self.stream, self.stream_len)[self.stream_len - 1]
            )            
        }    
        else { None }    
    }

    pub fn is_empty(&self) -> bool {
        self.stream_index  == self.stream_len
    }


    /// # TokenStream::current -- 
    ///
    /// Function get the current TokenOrStream struct from self's TokenStream
    ///     or None if the end of the stream has been reached
    ///
    /// ## Returns -- 
    /// * `Option<&TokenOrStream <'a>>` -- the current Token / Stream in self's
    ///     token stream or None if the end of the stream has been reached
    ///
    pub fn current (&'a self) -> Option<&TokenOrStream<'a>> {
        if self.stream_index < self.stream_len {
            Some (
                &TokenStream::stream_as_slice(&self.stream, self.stream_len)[self.stream_index]
            )            
        }    
        else { None }    
    }


    /// # TokenStream::current_token_type -- 
    ///
    /// 
    ///
    /// ## Returns -- 
    ///
    pub fn get_token_type (token_or_stream: Option<&TokenOrStream>) -> TokenOfInterest {
        // let cur = self.current();
        if let Some ( token_or_stream ) = token_or_stream {
            match token_or_stream {
                TokenOrStream::Token( token ) => {
                    token.token
                },
                TokenOrStream::TokenStream( stream ) => {
                    TokenStream::get_token_type(stream.current())
                }
            }
        }
        else { TokenOfInterest::None }
    }

    
    /// # TokenStream::next -- 
    ///
    /// Function get the next TokenOrStream struct from self's TokenStream
    /// *    or None if the end of the stream has been reached
    ///
    /// ## Returns -- 
    /// *`Option<&TokenOrStream <'a>>` -- the next Token / Stream in self's
    ///     token stream, or None if there are no more
    ///
    pub fn next (&mut self) -> Option<&TokenOrStream<'a>> {
        if self.stream_index < self.stream_len {

            // Getting the stream as a slice -- either by taking the Stream::RefStream of self, or taking
            //      a 0..n slice of the Stream::Stream slice
            let stream = match &self.stream {
                Stream::Stream( stream ) => { &stream[0..self.stream_len] },
                Stream::RefStream( stream ) => { stream }
            };
            
            if !self.at_start {
                // Getting the byte-length of the current token
                let token_bytes = match &stream[ self.stream_index ] {
                    TokenOrStream::Token ( token ) => {
                        token.end - token.start
                    }
                    TokenOrStream::TokenStream ( token_stream ) => {
                        token_stream.end - token_stream.start
                    }
                };
                // Incrmenet byte count and stream index, if not at the start of the stream
                self.byte_count += token_bytes;
                self.stream_index += 1;
            }
            else {
                // .next() should return the first element element of a TokenStream,
                //      if .next() has never been called before, so a .at_start bool 
                //      is being used as a hacky workaround :) be
                self.at_start = false;
            }
            
            // Return the next item in the stream, if the end of the stream
            //      has not been reached
            if self.stream_index < self.stream_len {
                Some ( &stream[ self.stream_index ] )
            }
            else { None }
        }    
        else { None }    
    }
    
    /// # TokenStream::next -- 
    ///
    /// Function get the previous TokenOrStream struct from self's TokenStream
    ///     or None if the end of the stream has been reached
    ///
    /// ## Returns -- 
    /// * `Option<&TokenOrStream <'a>>` -- the previous Token / Stream in self's
    ///     token stream, or None if there are no more
    ///
    pub fn prev (&'a mut self) -> Option<&TokenOrStream<'a>> {
        if self.stream_index < self.stream_len {
            if self.stream_index > 0 {
                
                let stream = TokenStream::stream_as_slice(&self.stream, self.stream_len);

                // Getting the byte-length of the current token
                let token_bytes = match &stream[ self.stream_index - 1 ] {
                    TokenOrStream::Token ( token ) => {
                        token.end - token.start
                    }
                    TokenOrStream::TokenStream ( token_stream ) => {
                        token_stream.end - token_stream.start
                    }
                };
                
                // Decrement byte count and index
                self.byte_count -= token_bytes;
                self.stream_index -= 1;            

                // Return the next item in the stream, if the beginning of the stream
                //      has been reached
                if self.stream_index < stream.len() {
                    Some( &stream[ self.stream_index ] )
                }
                else { None }    
            }
            else { None }
        }    
        else { None }    
    }
    


}

