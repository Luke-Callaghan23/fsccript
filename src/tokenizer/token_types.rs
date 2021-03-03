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
    None ( regex::Regex, TokenOfInterest )
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct End {
    pub RDLP: regex::Regex,
    pub token: TokenOfInterest,
}


/// # get_token_len
///
/// Function that maps a TokenOfInterest enum to a length of that
///     enum.  For every TokenOfInterest, there is a direct mapping
///     for the length of that token.
///
/// ## Parameters -- 
/// * `token: TokenOfInterest` -- The token whose length to search for
///
/// ## Returns -- 
/// * `u8` -- length of the token
///
/// .
pub fn get_token_len (token: &TokenOfInterest) -> u8 {
    match token {
        // 0
        TokenOfInterest::EOF                    |
        TokenOfInterest::BOF                    |
        TokenOfInterest::None                   => 0,

        // 1
        TokenOfInterest::BitAnd                 |
        TokenOfInterest::BitOr                  |
        TokenOfInterest::BitXor                 |
        TokenOfInterest::BitNot                 |
        TokenOfInterest::Assign                 |
        TokenOfInterest::Not                    |
        TokenOfInterest::Add                    |
        TokenOfInterest::Sub                    |
        TokenOfInterest::Mul                    |
        TokenOfInterest::Div                    |
        TokenOfInterest::Mod                    |
        TokenOfInterest::OpenParen              |
        TokenOfInterest::OpenBrack              |
        TokenOfInterest::OpenCurly              |
        TokenOfInterest::CloseParen             |
        TokenOfInterest::CloseBrack             |
        TokenOfInterest::CloseCurly             |
        TokenOfInterest::Colon                  |
        TokenOfInterest::Semicolon              |
        TokenOfInterest::Comma                  |
        TokenOfInterest::StringSingle           |
        TokenOfInterest::StringDouble           | 
        TokenOfInterest::StringBackTick         |
        TokenOfInterest::SingleLineCommentClose |
        TokenOfInterest::Dot                    => 1,


        // 2
        TokenOfInterest::BitAndEq               | 
        TokenOfInterest::LogAnd                 | 
        TokenOfInterest::BitOrEq                | 
        TokenOfInterest::LogOr                  | 
        TokenOfInterest::BitXorEq               | 
        TokenOfInterest::BitNotEq               | 
        TokenOfInterest::WeakEq                 | 
        TokenOfInterest::ArrowFunction          | 
        TokenOfInterest::NotEq                  | 
        TokenOfInterest::Gt                     | 
        TokenOfInterest::Ge                     | 
        TokenOfInterest::ShR                    | 
        TokenOfInterest::Lt                     | 
        TokenOfInterest::Le                     | 
        TokenOfInterest::ShL                    | 
        TokenOfInterest::AddEq                  | 
        TokenOfInterest::SubEq                  | 
        TokenOfInterest::MulEq                  | 
        TokenOfInterest::DivEq                  | 
        TokenOfInterest::ModEq                  | 
        TokenOfInterest::In                     | 
        TokenOfInterest::Of                     | 
        TokenOfInterest::If                     | 
        TokenOfInterest::Inc                    | 
        TokenOfInterest::SingleLineCommentOpen  |
        TokenOfInterest::MultiLineCommentOpen   |
        TokenOfInterest::MultiLineCommentClose  |
        TokenOfInterest::Dec                    => 2,
         
         
        // 3 
        TokenOfInterest::StrictEq               |
        TokenOfInterest::ShREq                  |
        TokenOfInterest::StrictNotEq            |
        TokenOfInterest::ShRZeroFill            |
        TokenOfInterest::ShLEq                  |
        TokenOfInterest::New                    => 3,
         
        // 5
        TokenOfInterest::Break                  => 5,

        // 4 
        TokenOfInterest::Case                   |
        TokenOfInterest::Else                   |
        TokenOfInterest::ShRZeroFillEq          => 4,
         
        // 6 
        TokenOfInterest::Return                 |
        TokenOfInterest::Delete                 |
        TokenOfInterest::Typeof                 |
        TokenOfInterest::Switch                 => 6,
 
        TokenOfInterest::Default                => 7,

        // 10 
        TokenOfInterest::Instanceof             => 10,
        
    }
}


pub fn get_end_token (token: &TokenOfInterest) -> TokenOfInterest {
    match token {
        // Opening tokens
        TokenOfInterest::OpenParen              => TokenOfInterest::CloseParen,
        TokenOfInterest::OpenBrack              => TokenOfInterest::CloseBrack,
        TokenOfInterest::OpenCurly              => TokenOfInterest::CloseCurly,
        TokenOfInterest::StringSingle           => TokenOfInterest::StringSingle,  
        TokenOfInterest::StringDouble           => TokenOfInterest::StringDouble,   
        TokenOfInterest::StringBackTick         => TokenOfInterest::StringBackTick,  
        TokenOfInterest::SingleLineCommentOpen  => TokenOfInterest::SingleLineCommentClose,
        TokenOfInterest::MultiLineCommentOpen   => TokenOfInterest::MultiLineCommentClose,
        TokenOfInterest::Case                   => TokenOfInterest::Break,
        TokenOfInterest::Default                => TokenOfInterest::Break,
        
        // All other tokens --> None
        TokenOfInterest::Break                  |
        TokenOfInterest::Else                   |
        TokenOfInterest::Return                 |
        TokenOfInterest::Semicolon              |
        TokenOfInterest::BOF                    |
        TokenOfInterest::EOF                    |
        TokenOfInterest::BitAnd                 |
        TokenOfInterest::BitOr                  |
        TokenOfInterest::BitXor                 |
        TokenOfInterest::BitNot                 |
        TokenOfInterest::Assign                 |
        TokenOfInterest::Not                    |
        TokenOfInterest::Add                    |
        TokenOfInterest::Sub                    |
        TokenOfInterest::Mul                    |
        TokenOfInterest::Div                    |
        TokenOfInterest::Mod                    |
        TokenOfInterest::CloseParen             |
        TokenOfInterest::CloseBrack             |
        TokenOfInterest::CloseCurly             |
        TokenOfInterest::Colon                  |
        TokenOfInterest::Comma                  |
        TokenOfInterest::SingleLineCommentClose |
        TokenOfInterest::Dot                    |
        TokenOfInterest::None                   |
        TokenOfInterest::BitAndEq               | 
        TokenOfInterest::LogAnd                 | 
        TokenOfInterest::BitOrEq                | 
        TokenOfInterest::LogOr                  | 
        TokenOfInterest::BitXorEq               | 
        TokenOfInterest::BitNotEq               | 
        TokenOfInterest::WeakEq                 | 
        TokenOfInterest::ArrowFunction          | 
        TokenOfInterest::NotEq                  | 
        TokenOfInterest::Gt                     | 
        TokenOfInterest::Ge                     | 
        TokenOfInterest::ShR                    | 
        TokenOfInterest::Lt                     | 
        TokenOfInterest::Le                     | 
        TokenOfInterest::ShL                    | 
        TokenOfInterest::AddEq                  | 
        TokenOfInterest::SubEq                  | 
        TokenOfInterest::MulEq                  | 
        TokenOfInterest::DivEq                  | 
        TokenOfInterest::ModEq                  | 
        TokenOfInterest::In                     | 
        TokenOfInterest::Of                     | 
        TokenOfInterest::If                     | 
        TokenOfInterest::Inc                    | 
        TokenOfInterest::MultiLineCommentClose  |
        TokenOfInterest::Dec                    |
        TokenOfInterest::StrictEq               |
        TokenOfInterest::ShREq                  |
        TokenOfInterest::StrictNotEq            |
        TokenOfInterest::ShRZeroFill            |
        TokenOfInterest::ShLEq                  |
        TokenOfInterest::New                    |
        TokenOfInterest::ShRZeroFillEq          |
        TokenOfInterest::Delete                 |
        TokenOfInterest::Typeof                 |
        TokenOfInterest::Switch                 |
        TokenOfInterest::Instanceof             => TokenOfInterest::None
    }
}

pub fn token_is_comment_or_string (token: &TokenOfInterest) -> bool {
    *token == TokenOfInterest::SingleLineCommentOpen ||
    *token == TokenOfInterest::MultiLineCommentOpen  ||
    *token == TokenOfInterest::StringSingle          ||
    *token == TokenOfInterest::StringDouble          ||
    *token == TokenOfInterest::StringBackTick
}

/// token_types::TokenExpressionType
///
/// The type of a token in relation to an expression.  Any given token can be categorized into
/// 1. A token that comes before an expression
/// 2. A token that comes after an expression
/// 3. A token that can come before or after an expression
/// 4. A token that cannot come before, or after an expression
///
/// These categories are needed to identify fjs elements as expressions:
/// Example -- 
/// ```
/// function doSomeStuff () {
///      let greeting = if (a == b) {
///          return 'Hello world';
///      }
///      else {
///          return 'Goodbye world';
///      }
/// }
/// ```
/// The above fjs code shouls set the result of either branch of the if statement to "greeting", BUT:
///
/// ```
/// function doSomeStuff () {
///     if (a == b) {
///         return 'Hello world';
///     }
///     else {
///         return 'Goodbye world';
///     }
/// }
/// ```
/// The above js code should return from the function doSomeStuff  with the result of the if statement
///
/// So, all fjs elements must come before or after an expression token like the '=' in let greeting = ...
///
#[derive(PartialEq, Eq)]
pub enum TokenExpressionType {
    Expression,
    OnlyBeforeExp,
    OnlyAfterExp,
    None
}

pub fn get_token_expression_type (token: &TokenOfInterest) -> TokenExpressionType {
    match token {

        // Valid before or after an expression
        TokenOfInterest::BitAnd                 |
        TokenOfInterest::BitOr                  |
        TokenOfInterest::BitXor                 |
        TokenOfInterest::BitNot                 |
        TokenOfInterest::CloseBrack             |
        TokenOfInterest::Assign                 |
        TokenOfInterest::Add                    |
        TokenOfInterest::Sub                    |
        TokenOfInterest::Mul                    |
        TokenOfInterest::Div                    |
        TokenOfInterest::Mod                    |
        TokenOfInterest::OpenParen              |
        TokenOfInterest::OpenBrack              |
        TokenOfInterest::CloseParen             |
        TokenOfInterest::Colon                  |
        TokenOfInterest::Comma                  |
        TokenOfInterest::BitAndEq               | 
        TokenOfInterest::LogAnd                 | 
        TokenOfInterest::BitOrEq                | 
        TokenOfInterest::LogOr                  | 
        TokenOfInterest::BitXorEq               | 
        TokenOfInterest::BitNotEq               | 
        TokenOfInterest::WeakEq                 | 
        TokenOfInterest::ArrowFunction          | 
        TokenOfInterest::NotEq                  | 
        TokenOfInterest::Gt                     | 
        TokenOfInterest::Ge                     | 
        TokenOfInterest::ShR                    | 
        TokenOfInterest::Lt                     | 
        TokenOfInterest::Le                     | 
        TokenOfInterest::ShL                    | 
        TokenOfInterest::AddEq                  | 
        TokenOfInterest::SubEq                  | 
        TokenOfInterest::MulEq                  | 
        TokenOfInterest::DivEq                  | 
        TokenOfInterest::ModEq                  | 
        TokenOfInterest::In                     | 
        TokenOfInterest::Of                     | 
        TokenOfInterest::If                     | 
        TokenOfInterest::Inc                    | 
        TokenOfInterest::Dec                    |
        TokenOfInterest::StrictEq               |
        TokenOfInterest::ShREq                  |
        TokenOfInterest::StrictNotEq            |
        TokenOfInterest::ShRZeroFill            |
        TokenOfInterest::ShLEq                  |
        TokenOfInterest::Typeof                 |
        TokenOfInterest::ShRZeroFillEq          => TokenExpressionType::Expression,
        
        // Only after
        TokenOfInterest::CloseCurly             |
        TokenOfInterest::Dot                    => TokenExpressionType::OnlyAfterExp,
        
        // Only before
        TokenOfInterest::Delete                 |
        TokenOfInterest::Not                    |
        TokenOfInterest::Return                 |
        TokenOfInterest::New                    => TokenExpressionType::OnlyBeforeExp,
        
        // None
        TokenOfInterest::OpenCurly              |
        TokenOfInterest::Semicolon              |
        TokenOfInterest::StringSingle           |
        TokenOfInterest::StringDouble           | 
        TokenOfInterest::StringBackTick         |
        TokenOfInterest::SingleLineCommentClose |
        TokenOfInterest::SingleLineCommentOpen  |
        TokenOfInterest::MultiLineCommentOpen   |
        TokenOfInterest::MultiLineCommentClose  |
        TokenOfInterest::Case                   |
        TokenOfInterest::Break                  |
        TokenOfInterest::Else                   |
        TokenOfInterest::Default                |
        TokenOfInterest::Switch                 |
        TokenOfInterest::Instanceof             |
        TokenOfInterest::EOF                    |
        TokenOfInterest::BOF                    |
        TokenOfInterest::None                   => TokenExpressionType::None,
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum TokenOfInterest {

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
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::BitAnd },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '&&'    
                            Step {
                                next: '&',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::LogAnd }
                            },
                            // '&='
                            Step {
                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::BitAndEq }
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '|'
            Step {
                next: '|',
                // '|'
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::BitOr },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '||'
                            Step {
                                next: '|',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::LogOr }
                            },
                            // '|='
                            Step {
                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::BitOrEq }
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '^'
            Step {
                next: '^',
                // '^'
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::BitXor },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '^='
                            Step {
                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::BitXorEq }
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '~'
            Step {
                next: '~',
                // '~'
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::BitNot },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '~='
                            Step {
                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::BitNotEq }
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '='
            Step {
                next: '=',
                // '='
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::Assign },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '=>'
                            Step {
                                next: '>',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::ArrowFunction }
                            },
                            Step {
                                next: '=',
                                // '=='
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::WeakEq },
                                step: Some (
                                    TokenLookup {
                                        lookup: vec![
                                            // '==='
                                            Step {
                                                
                                                next: '=',
                                                step: None,
                                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::StrictEq }
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
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::Not },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            Step {
                                next: '=',
                                // '!='
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::NotEq },
                                step: Some (
                                    TokenLookup {
                                        lookup: vec![
                                            // '!=='
                                            Step {
                                                
                                                next: '=',
                                                step: None,
                                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::StrictNotEq }
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
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::Lt },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '<='
                            Step {
                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::Le }
                            },
                            Step {
                                next: '<',
                                // '<<'
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::ShL },
                                step: Some (
                                    TokenLookup {
                                        lookup: vec![
                                            // '<<='
                                            Step {
                                                next: '=',
                                                step: None,
                                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::ShLEq }
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
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::Gt },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '>='
                            Step {
                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::Ge }
                            },
                            Step {
                                next: '>',
                                // '>>'
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::ShR },
                                step: Some (
                                    TokenLookup {
                                        lookup: vec![
                                            // '>>='
                                            Step {
                                                
                                                next: '=',
                                                step: None,
                                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::ShREq }
                                            },
                                            Step {
                                                next: '>',
                                                // '>>>'
                                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::ShRZeroFill },
                                                step: Some (
                                                    TokenLookup {
                                                        lookup: vec![
                                                            // '>>>='
                                                            Step {
                                                                next: '=',
                                                                step: None,
                                                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::ShRZeroFillEq }
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
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::Add },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '++'
                            Step {
                                next: '+',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::Inc }
                            },
                            // '+='
                            Step {
                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::AddEq }
                            },
                        ]
                    }
                )
            },
            // Tokens beginning with '-'
            Step {
                next: '-',
                // '-'
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::Sub },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '--'
                            Step {
                                next: '-',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::Dec }
                            },
                            // '-='
                            Step {
                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::SubEq }
                            },
                        ]
                    }
                )
            },
            // Tokens beginning with '*'
            Step {
                next: '*',
                // '*'
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::Mul },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '*='
                            Step {
                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::MulEq }
                            },
                            // '*/'
                            Step {
                                next: '/',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::MultiLineCommentClose }
                            },
                        ]
                    }
                )
            },
            // Tokens beginning with '/'
            Step {
                next: '/',
                // '/'
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::Div },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '/='
                            Step {
                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::DivEq }
                            },
                            // '/*'
                            Step {
                                next: '*',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::MultiLineCommentOpen }
                            },
                            // '//'
                            Step {
                                next: '/',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::SingleLineCommentOpen }
                            },
                        ]
                    }
                )
            },
            // Tokens beginning with '%'
            Step {
                next: '%',
                // '%'
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::Mod },
                step: Some (
                    TokenLookup {
                        lookup: vec![
                            // '%='
                            Step {
                                                                next: '=',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::ModEq }
                            },
                        ]
                    }
                )
            },
            // Tokens beginning with ';'
            Step {
                next: ';',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::Semicolon }
            },
            // Tokens beginning with '('
            Step {
                next: '(',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::OpenParen }
            },
            // Tokens beginning with '['
            Step {
                next: '[',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::OpenBrack }
            },
            // Tokens beginning with ']'
            Step {
                next: ')',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::CloseParen }
            },
            // Tokens beginning with ')'
            Step {
                next: ']',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::CloseBrack }
            },
            // Tokens beginning with '{'
            Step {
                next: '{',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::OpenCurly }
            },
            // Tokens beginning with '}'
            Step {
                next: '}',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::CloseCurly }
            },
            // Tokens beginning with ','
            Step {
                next: ',',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::Comma }
            },
            // Tokens beginning with ':'
            Step {
                next: ':',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::Colon }
            },
            // Tokens beginning with '.'
            Step {
                next: '.',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::Dot }
            },
            // Tokens beginning with '\n'
            Step {
                next: '\n',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::SingleLineCommentClose }
            },
            // Tokens beginning with '''
            Step {
                next: '\'',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::StringSingle }
            },
            // Tokens beginning with '"'
            Step {
                next: '"',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::StringDouble }
            },
            // Tokens beginning with '`'
            Step {
                next: '`',
                step: None,
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::StringBackTick }
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
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::In },
                                
                                step: Some (
                                    TokenLookup {
                                        lookup: vec![
                                            // 'instanceof'
                                            Step {
                                                next: 's',
                                                step: None,
                                                end: End { RDLP: Regex::new("^tanceof([^\\d\\w].*|$)").unwrap(), token: TokenOfInterest::Instanceof  }
                                            }
                                        ]
                                    }
                                )
                            },
                            Step {
                                next: 'f',
                                step: None,
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::If }
                            },
                        ]
                    }
                ),
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::None }
            },

            // Tokens beginning with 'e'
            Step {
                next: 'e',
                step: None,
                end: End { RDLP: Regex::new("^lse([^\\d\\w].*|$)").unwrap(), token: TokenOfInterest::Else }
            },

            // Tokens beginning with 'c'
            Step {
                next: 'c',
                step: None,
                end: End { RDLP: Regex::new("^ase([^\\d\\w].*|$)").unwrap(), token: TokenOfInterest::Case }
            },
            
            // Tokens beginning with 'b'
            Step {
                next: 'b',
                step: None,
                end: End { RDLP: Regex::new("^reak([^\\d\\w].*|$)").unwrap(), token: TokenOfInterest::Break }
            },

            // Tokens beginning with 'c'
            Step {
                next: 'r',
                step: None,
                end: End { RDLP: Regex::new("^eturn([^\\d\\w].*|$)").unwrap(), token: TokenOfInterest::Return }
            },

            // Tokens beginning with 'o'
            Step {
                next: 'o',
                step: None,
                end: End { RDLP: Regex::new("^f([^\\d\\w].*|$)").unwrap(), token: TokenOfInterest::Of }
            },

            // Tokens beginning with 't'
            Step {
                next: 't',
                step: None,
                end: End { RDLP: Regex::new("^ypeof([^\\d\\w].*|$)").unwrap(), token: TokenOfInterest::Typeof }
            },

            // Tokens beginning with 's'
            Step {
                next: 's',
                step: None,
                end: End { RDLP: Regex::new("^witch([^\\d\\w].*|$)").unwrap(), token: TokenOfInterest::Switch }
            },

            // // Tokens beginning with 'd'
            // Step {
            //     next: 'd',
            //     step: None,
            //     end: End { RDLP: Regex::new("^elete([^\\d\\w].*|$)").unwrap(), token: TokenOfInterest::Delete }
            // },

            Step {
                next: 'd',
                step: Some (
                    TokenLookup { 
                        lookup: vec![
                            Step {
                                next: 'e',
                                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::None },
                                
                                step: Some (
                                    TokenLookup {
                                        lookup: vec![
                                            // 'delete'
                                            Step {
                                                next: 'l',
                                                step: None,
                                                end: End { RDLP: Regex::new("^ete([^\\d\\w].*|$)").unwrap(), token: TokenOfInterest::Delete  }
                                            },
                                            // 'default'
                                            Step {
                                                next: 'f',
                                                step: None,
                                                end: End { RDLP: Regex::new("^ault([^\\d\\w].*|$)").unwrap(), token: TokenOfInterest::Default  }
                                            }
                                        ]
                                    }
                                )
                            },
                        ]
                    }
                ),
                end: End { RDLP: def_regex.clone(), token: TokenOfInterest::None }
            },


            // Tokens beginning with 'n'
            Step {
                next: 'n',
                step: None,
                end: End { RDLP: Regex::new("^ew([^\\d\\w].*|$)").unwrap(), token: TokenOfInterest::New }
            },
        ]
    }
}

