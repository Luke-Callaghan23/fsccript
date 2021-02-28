#[derive(Debug)]
pub struct TokenLookup {
    pub lookup: Vec<Step>
}

#[derive(Debug)]
pub struct Step {
    pub next: char,
    pub step: Option<TokenLookup>,
    pub end: End
}
#[derive(Debug)]
pub enum Lookup {
    Lookup ( TokenLookup ),
    None ( regex::Regex, TokensOfInterest )
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct End {
    pub RDLP: regex::Regex,
    pub token: TokensOfInterest,
}


/// # get_token_len
///
/// Function that maps a TokensOfInterest enum to a length of that
///     enum.  For every TokensOfInterest, there is a direct mapping
///     for the length of that token.
///
/// ## Parameters -- 
/// * `token: TokensOfInterest` -- The token whose length to search for
///
/// ## Returns -- 
/// * `u8` -- length of the token
///
/// .
pub fn get_token_len (token: &TokensOfInterest) -> u8 {
    match token {
        // 0
        TokensOfInterest::EOF                    |
        TokensOfInterest::BOF                    |
        TokensOfInterest::None                   => 0,

        // 1
        TokensOfInterest::BitAnd                 |
        TokensOfInterest::BitOr                  |
        TokensOfInterest::BitXor                 |
        TokensOfInterest::BitNot                 |
        TokensOfInterest::Assign                 |
        TokensOfInterest::Not                    |
        TokensOfInterest::Add                    |
        TokensOfInterest::Sub                    |
        TokensOfInterest::Mul                    |
        TokensOfInterest::Div                    |
        TokensOfInterest::Mod                    |
        TokensOfInterest::OpenParen              |
        TokensOfInterest::OpenBrack              |
        TokensOfInterest::OpenCurly              |
        TokensOfInterest::CloseParen             |
        TokensOfInterest::CloseBrack             |
        TokensOfInterest::CloseCurly             |
        TokensOfInterest::Colon                  |
        TokensOfInterest::Semicolon              |
        TokensOfInterest::Comma                  |
        TokensOfInterest::StringSingle           |
        TokensOfInterest::StringDouble           | 
        TokensOfInterest::StringBackTick         |
        TokensOfInterest::SingleLineCommentClose |
        TokensOfInterest::Dot                    => 1,


        // 2
        TokensOfInterest::BitAndEq               | 
        TokensOfInterest::LogAnd                 | 
        TokensOfInterest::BitOrEq                | 
        TokensOfInterest::LogOr                  | 
        TokensOfInterest::BitXorEq               | 
        TokensOfInterest::BitNotEq               | 
        TokensOfInterest::WeakEq                 | 
        TokensOfInterest::ArrowFunction          | 
        TokensOfInterest::NotEq                  | 
        TokensOfInterest::Gt                     | 
        TokensOfInterest::Ge                     | 
        TokensOfInterest::ShR                    | 
        TokensOfInterest::Lt                     | 
        TokensOfInterest::Le                     | 
        TokensOfInterest::ShL                    | 
        TokensOfInterest::AddEq                  | 
        TokensOfInterest::SubEq                  | 
        TokensOfInterest::MulEq                  | 
        TokensOfInterest::DivEq                  | 
        TokensOfInterest::ModEq                  | 
        TokensOfInterest::In                     | 
        TokensOfInterest::Of                     | 
        TokensOfInterest::If                     | 
        TokensOfInterest::Inc                    | 
        TokensOfInterest::SingleLineCommentOpen  |
        TokensOfInterest::MultiLineCommentOpen   |
        TokensOfInterest::MultiLineCommentClose  |
        TokensOfInterest::Dec                    => 2,
         
         
        // 3 
        TokensOfInterest::StrictEq               |
        TokensOfInterest::ShREq                  |
        TokensOfInterest::StrictNotEq            |
        TokensOfInterest::ShRZeroFill            |
        TokensOfInterest::ShLEq                  |
        TokensOfInterest::New                    => 3,
         
        // 5
        TokensOfInterest::Break                  => 5,

        // 4 
        TokensOfInterest::Case                   |
        TokensOfInterest::Else                   |
        TokensOfInterest::ShRZeroFillEq          => 4,
         
        // 6 
        TokensOfInterest::Return                 |
        TokensOfInterest::Delete                 |
        TokensOfInterest::Typeof                 |
        TokensOfInterest::Switch                 => 6,
 
        TokensOfInterest::Default                => 7,

        // 10 
        TokensOfInterest::Instanceof             => 10,
        
    }
}


pub fn get_end_token (token: &TokensOfInterest) -> TokensOfInterest {
    match token {
        // Opening tokens
        TokensOfInterest::OpenParen              => TokensOfInterest::CloseParen,
        TokensOfInterest::OpenBrack              => TokensOfInterest::CloseBrack,
        TokensOfInterest::OpenCurly              => TokensOfInterest::CloseCurly,
        TokensOfInterest::StringSingle           => TokensOfInterest::StringSingle,  
        TokensOfInterest::StringDouble           => TokensOfInterest::StringDouble,   
        TokensOfInterest::StringBackTick         => TokensOfInterest::StringBackTick,  
        TokensOfInterest::SingleLineCommentOpen  => TokensOfInterest::SingleLineCommentClose,
        TokensOfInterest::MultiLineCommentOpen   => TokensOfInterest::MultiLineCommentClose,
        TokensOfInterest::Case                   => TokensOfInterest::Break,
        TokensOfInterest::Default                => TokensOfInterest::Break,
        
        // All other tokens --> None
        TokensOfInterest::Break                  |
        TokensOfInterest::Else                   |
        TokensOfInterest::Return                 |
        TokensOfInterest::Semicolon              |
        TokensOfInterest::BOF                    |
        TokensOfInterest::EOF                    |
        TokensOfInterest::BitAnd                 |
        TokensOfInterest::BitOr                  |
        TokensOfInterest::BitXor                 |
        TokensOfInterest::BitNot                 |
        TokensOfInterest::Assign                 |
        TokensOfInterest::Not                    |
        TokensOfInterest::Add                    |
        TokensOfInterest::Sub                    |
        TokensOfInterest::Mul                    |
        TokensOfInterest::Div                    |
        TokensOfInterest::Mod                    |
        TokensOfInterest::CloseParen             |
        TokensOfInterest::CloseBrack             |
        TokensOfInterest::CloseCurly             |
        TokensOfInterest::Colon                  |
        TokensOfInterest::Comma                  |
        TokensOfInterest::SingleLineCommentClose |
        TokensOfInterest::Dot                    |
        TokensOfInterest::None                   |
        TokensOfInterest::BitAndEq               | 
        TokensOfInterest::LogAnd                 | 
        TokensOfInterest::BitOrEq                | 
        TokensOfInterest::LogOr                  | 
        TokensOfInterest::BitXorEq               | 
        TokensOfInterest::BitNotEq               | 
        TokensOfInterest::WeakEq                 | 
        TokensOfInterest::ArrowFunction          | 
        TokensOfInterest::NotEq                  | 
        TokensOfInterest::Gt                     | 
        TokensOfInterest::Ge                     | 
        TokensOfInterest::ShR                    | 
        TokensOfInterest::Lt                     | 
        TokensOfInterest::Le                     | 
        TokensOfInterest::ShL                    | 
        TokensOfInterest::AddEq                  | 
        TokensOfInterest::SubEq                  | 
        TokensOfInterest::MulEq                  | 
        TokensOfInterest::DivEq                  | 
        TokensOfInterest::ModEq                  | 
        TokensOfInterest::In                     | 
        TokensOfInterest::Of                     | 
        TokensOfInterest::If                     | 
        TokensOfInterest::Inc                    | 
        TokensOfInterest::MultiLineCommentClose  |
        TokensOfInterest::Dec                    |
        TokensOfInterest::StrictEq               |
        TokensOfInterest::ShREq                  |
        TokensOfInterest::StrictNotEq            |
        TokensOfInterest::ShRZeroFill            |
        TokensOfInterest::ShLEq                  |
        TokensOfInterest::New                    |
        TokensOfInterest::ShRZeroFillEq          |
        TokensOfInterest::Delete                 |
        TokensOfInterest::Typeof                 |
        TokensOfInterest::Switch                 |
        TokensOfInterest::Instanceof             => TokensOfInterest::None
    }
}

pub fn token_is_comment_or_string (token: &TokensOfInterest) -> bool {
    *token == TokensOfInterest::SingleLineCommentOpen ||
    *token == TokensOfInterest::MultiLineCommentOpen  ||
    *token == TokensOfInterest::StringSingle          ||
    *token == TokensOfInterest::StringDouble          ||
    *token == TokensOfInterest::StringBackTick
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum TokensOfInterest {

    // Beginning with '&', '|', '^',
    BitAnd,  BitAndEq,   LogAnd,  
    BitOr ,  BitOrEq ,   LogOr,   
    BitXor,  BitXorEq, 
    BitNot,  BitNotEq, 

    // Beginning with '='
    Assign, WeakEq, StrictEq, ArrowFunction,
    
    // Beginning with !
    Not, NotEq, StrictNotEq,

    // Beginning with '<' or '>'
    Gt, Ge, ShR, ShREq, ShRZeroFill, ShRZeroFillEq,
    Lt, Le, ShL, ShLEq,

    // Math operators: '+', '-', '*', '/', '%'
    Add, AddEq, Inc, // +
    Sub, SubEq, Dec, // -
    Mul, MulEq,      // *
    Div, DivEq,      // /
    Mod, ModEq,      // %

    // Structural tokens: '(', '[' , ']', ')', '{', '}'
    OpenParen, OpenBrack, 
    CloseParen, CloseBrack,
    OpenCurly, CloseCurly,

    // Structural operators: ':', ','
    Colon, Comma,

    // semicolon ';'
    Semicolon,

    // String operators
    Delete, Typeof, In, Instanceof, Of,
    If, Else, Switch, Case, Return, Break,
    Default,

    // Only after operators
    New, Dot,

    // String characters
    StringSingle, StringDouble, StringBackTick,

    // Comment characters
    SingleLineCommentOpen, SingleLineCommentClose,
    MultiLineCommentOpen, MultiLineCommentClose,


    // Beginning / end of file
    BOF, EOF,

    // Not interesting :(
    None
}

use regex::Regex;

pub fn initialize_lookup () -> TokenLookup {
    let def_regex = Regex::new(".?").unwrap();

    TokenLookup {
        lookup: vec![
            Step {
                // Tokens beginning with '&' 
                next: '&',
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::BitAnd },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '&&'    
                            Step {
                                next: '&',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::LogAnd }
                            },
                            // '&='
                            Step {
                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::BitAndEq }
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '|'
            Step {
                next: '|',
                // '|'
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::BitOr },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '||'
                            Step {
                                next: '|',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::LogOr }
                            },
                            // '|='
                            Step {
                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::BitOrEq }
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '^'
            Step {
                next: '^',
                // '^'
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::BitXor },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '^='
                            Step {
                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::BitXorEq }
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '~'
            Step {
                next: '~',
                // '~'
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::BitNot },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '~='
                            Step {
                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::BitNotEq }
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '='
            Step {
                next: '=',
                // '='
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::Assign },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '=>'
                            Step {
                                next: '>',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::ArrowFunction }
                            },
                            Step {
                                next: '=',
                                // '=='
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::WeakEq },
                                step: Some (
                                    TokenLookup {
                                        lookup: vec![
                                            // '==='
                                            Step {
                                                
                                                next: '=',
                                                step: None,
                                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::StrictEq }
                                            },
                                        ]
                                    }
                                )
                            },
                        ]
                    }
                )
            },
            // Tokens beginning with '!'
            Step {
                next: '!',
                // '!'
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::Not },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            Step {
                                next: '=',
                                // '!='
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::NotEq },
                                step: Some (
                                    TokenLookup {
                                        lookup: vec![
                                            // '!=='
                                            Step {
                                                
                                                next: '=',
                                                step: None,
                                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::StrictNotEq }
                                            },
                                        ]
                                    }
                                )
                            },
                        ]
                    }
                )
            },
            // Tokens beginning with '<'
            Step {
                next: '<',
                // '<'
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::Lt },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '<='
                            Step {
                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::Le }
                            },
                            Step {
                                next: '<',
                                // '<<'
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::ShL },
                                step: Some (
                                    TokenLookup {
                                        lookup: vec![
                                            // '<<='
                                            Step {
                                                next: '=',
                                                step: None,
                                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::ShLEq }
                                            },
                                        ]
                                    }
                                )
                            },
                        ]
                    }
                )
            },
            // Tokens beginning with '>'
            Step {
                next: '>',
                // '>'
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::Gt },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '>='
                            Step {
                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::Ge }
                            },
                            Step {
                                next: '>',
                                // '>>'
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::ShR },
                                step: Some (
                                    TokenLookup {
                                        lookup: vec![
                                            // '>>='
                                            Step {
                                                
                                                next: '=',
                                                step: None,
                                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::ShREq }
                                            },
                                            Step {
                                                next: '>',
                                                // '>>>'
                                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::ShRZeroFill },
                                                step: Some (
                                                    TokenLookup {
                                                        lookup: vec![
                                                            // '>>>='
                                                            Step {
                                                                next: '=',
                                                                step: None,
                                                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::ShRZeroFillEq }
                                                            },
                                                        ]
                                                    }
                                                )
                                            },
                                        ]
                                    }
                                )
                            },
                        ]
                    }
                )
            },
            // Tokens beginning with '+'
            Step {
                next: '+',
                // '+'
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::Add },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '++'
                            Step {
                                next: '+',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::Inc }
                            },
                            // '+='
                            Step {
                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::AddEq }
                            },
                        ]
                    }
                )
            },
            // Tokens beginning with '-'
            Step {
                next: '-',
                // '-'
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::Sub },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '--'
                            Step {
                                next: '-',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::Dec }
                            },
                            // '-='
                            Step {
                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::SubEq }
                            },
                        ]
                    }
                )
            },
            // Tokens beginning with '*'
            Step {
                next: '*',
                // '*'
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::Mul },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '*='
                            Step {
                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::MulEq }
                            },
                            // '*/'
                            Step {
                                next: '/',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::MultiLineCommentClose }
                            },
                        ]
                    }
                )
            },
            // Tokens beginning with '/'
            Step {
                next: '/',
                // '/'
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::Div },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '/='
                            Step {
                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::DivEq }
                            },
                            // '/*'
                            Step {
                                next: '*',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::MultiLineCommentOpen }
                            },
                            // '//'
                            Step {
                                next: '/',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::SingleLineCommentOpen }
                            },
                        ]
                    }
                )
            },
            // Tokens beginning with '%'
            Step {
                next: '%',
                // '%'
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::Mod },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '%='
                            Step {
                                                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::ModEq }
                            },
                        ]
                    }
                )
            },
            // Tokens beginning with ';'
            Step {
                next: ';',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::Semicolon }
            },
            // Tokens beginning with '('
            Step {
                next: '(',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::OpenParen }
            },
            // Tokens beginning with '['
            Step {
                next: '[',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::OpenBrack }
            },
            // Tokens beginning with ']'
            Step {
                next: ')',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::CloseParen }
            },
            // Tokens beginning with ')'
            Step {
                next: ']',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::CloseBrack }
            },
            // Tokens beginning with '{'
            Step {
                next: '{',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::OpenCurly }
            },
            // Tokens beginning with '}'
            Step {
                next: '}',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::CloseCurly }
            },
            // Tokens beginning with ','
            Step {
                next: ',',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::Comma }
            },
            // Tokens beginning with ':'
            Step {
                next: ':',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::Colon }
            },
            // Tokens beginning with '.'
            Step {
                next: '.',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::Dot }
            },
            // Tokens beginning with '\n'
            Step {
                next: '\n',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::SingleLineCommentClose }
            },
            // Tokens beginning with '''
            Step {
                next: '\'',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::StringSingle }
            },
            // Tokens beginning with '"'
            Step {
                next: '"',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::StringDouble }
            },
            // Tokens beginning with '`'
            Step {
                next: '`',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::StringBackTick }
            },
            
            // Tokens beginning with 'in'
            Step {
                next: 'i',
                step: Some (
                    TokenLookup { 
                        lookup: vec![
                            // 'in'
                            Step {
                                next: 'n',
                                // 'in'
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::In },
                                
                                step: Some (
                                    TokenLookup {
                                        lookup: vec![
                                            // 'instanceof'
                                            Step {
                                                next: 's',
                                                step: None,
                                                end: End { RDLP: Regex::new("^tanceof([^\\d\\w].*|$)").unwrap(), token: TokensOfInterest::Instanceof  }
                                            }
                                        ]
                                    }
                                )
                            },
                            Step {
                                next: 'f',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::If }
                            },
                        ]
                    }
                ),
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::None }
            },

            // Tokens beginning with 'e'
            Step {
                next: 'e',
                step: None,
                end: End { RDLP: Regex::new("^lse([^\\d\\w].*|$)").unwrap(), token: TokensOfInterest::Else }
            },

            // Tokens beginning with 'c'
            Step {
                next: 'c',
                step: None,
                end: End { RDLP: Regex::new("^ase([^\\d\\w].*|$)").unwrap(), token: TokensOfInterest::Case }
            },
            
            // Tokens beginning with 'b'
            Step {
                next: 'b',
                step: None,
                end: End { RDLP: Regex::new("^reak([^\\d\\w].*|$)").unwrap(), token: TokensOfInterest::Break }
            },

            // Tokens beginning with 'c'
            Step {
                next: 'r',
                step: None,
                end: End { RDLP: Regex::new("^eturn([^\\d\\w].*|$)").unwrap(), token: TokensOfInterest::Return }
            },

            // Tokens beginning with 'o'
            Step {
                next: 'o',
                step: None,
                end: End { RDLP: Regex::new("^f([^\\d\\w].*|$)").unwrap(), token: TokensOfInterest::Of }
            },

            // Tokens beginning with 't'
            Step {
                next: 't',
                step: None,
                end: End { RDLP: Regex::new("^ypeof([^\\d\\w].*|$)").unwrap(), token: TokensOfInterest::Typeof }
            },

            // Tokens beginning with 's'
            Step {
                next: 's',
                step: None,
                end: End { RDLP: Regex::new("^witch([^\\d\\w].*|$)").unwrap(), token: TokensOfInterest::Switch }
            },

            // // Tokens beginning with 'd'
            // Step {
            //     next: 'd',
            //     step: None,
            //     end: End { RDLP: Regex::new("^elete([^\\d\\w].*|$)").unwrap(), token: TokensOfInterest::Delete }
            // },

            Step {
                next: 'd',
                step: Some (
                    TokenLookup { 
                        lookup: vec![
                            Step {
                                next: 'e',
                                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::None },
                                
                                step: Some (
                                    TokenLookup {
                                        lookup: vec![
                                            // 'delete'
                                            Step {
                                                next: 'l',
                                                step: None,
                                                end: End { RDLP: Regex::new("^ete([^\\d\\w].*|$)").unwrap(), token: TokensOfInterest::Delete  }
                                            },
                                            // 'default'
                                            Step {
                                                next: 'f',
                                                step: None,
                                                end: End { RDLP: Regex::new("^ault([^\\d\\w].*|$)").unwrap(), token: TokensOfInterest::Default  }
                                            }
                                        ]
                                    }
                                )
                            },
                        ]
                    }
                ),
                end: End { RDLP: def_regex.clone(), token: TokensOfInterest::None }
            },


            // Tokens beginning with 'n'
            Step {
                next: 'n',
                step: None,
                end: End { RDLP: Regex::new("^ew([^\\d\\w].*|$)").unwrap(), token: TokensOfInterest::New }
            },
        ]
    }
}

