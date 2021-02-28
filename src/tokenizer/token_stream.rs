#[derive(Debug)]
pub struct TokenStream <'a> {
    stream:       Stream <'a>,
    stream_index: usize,
    stream_len:   usize,
    byte_count:   usize,
    depth:        usize,
    start:        usize,
    end:          usize,
    // delimiters:   (&'a Token, Option<&'a Token>),
    // outer:        Option<&'a TokenStream <'a>>
}


#[derive(Debug)]
enum Stream <'a> {
    RefStream ( &'a [TokenOrStream <'a>] ),
    Stream ( Vec<TokenOrStream <'a>> ),
}

#[derive(Debug)]
pub enum TokenOrStream <'a> {
    Token ( Token ),
    TokenStream ( TokenStream <'a> )
}

use crate::tokenizer::token_types::TokensOfInterest;

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub token: TokensOfInterest,
    pub start: usize,
    pub end: usize,
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
            // outer:        None
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
                // Increment the end of self
                self.end = match &token {
                    TokenOrStream::Token( token ) => token.end,
                    TokenOrStream::TokenStream( token_stream ) => token_stream.end,
                };
                
                // if let TokenOrStream::TokenStream( ref mut tok  ) = token {
                //     tok.outer = Some( &self );
                // }

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
    pub fn step_into (&'a self) -> Option<&TokenStream <'a>> {
        let cur = self.current();
        if let Some ( cur ) = cur {
            match cur {
                TokenOrStream::Token ( _ ) => None,
                TokenOrStream::TokenStream ( stream ) => Some( &stream ) 
            }
        }
        else { None }
    }
    
    /*
    /// # TokenStream::step_out -- 
    ///
    /// Function to step out of the current enclosing token scope to the outer
    ///        scope, if one exists
    ///
    /// This is analagous to stepping from an enclosing scope, like {}, (), []
    ///         '', "", ``, //\n, or /**/ and into the outer scope
    ///
    /// Returns None if:
    ///    *    The current TokenStream has no outer scope
    ///
    /// ## Returns -- 
    /// * `Option<&TokenStream <'a>>` -- The stream that we stepped out into, or None if
    ///     there was no Stream to step out into
    ///
    // pub fn step_out (&'a self) -> Option<&TokenStream <'a>> { self.outer }
     */

    
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
    
    /// # TokenStream::next -- 
    ///
    /// Function get the next TokenOrStream struct from self's TokenStream
    /// *    or None if the end of the stream has been reached
    ///
    /// ## Returns -- 
    /// *`Option<&TokenOrStream <'a>>` -- the next Token / Stream in self's
    ///     token stream, or None if there are no more
    ///
    pub fn next (&'a mut self) -> Option<&TokenOrStream<'a>> {
        if self.stream_index < self.stream_len {

            let stream = TokenStream::stream_as_slice(&self.stream, self.stream_len);

            // Getting the byte-length of the current token
            let token_bytes = match &stream[ self.stream_index ] {
                TokenOrStream::Token ( token ) => {
                    token.end - token.start
                }
                TokenOrStream::TokenStream ( token_stream ) => {
                    token_stream.end - token_stream.start
                }
            };
            
            // Incrmenet byte count and index
            self.byte_count += token_bytes;
            self.stream_index += 1;
            
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

