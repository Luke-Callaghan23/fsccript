/*use regex::Regex;


#[derive(Debug)]
pub struct TokenLookup {
    lookup: Vec<CharacterAndStep>
}

#[derive(Debug)]
pub struct CharacterAndStep {
    next: regex::Regex,
    step: LookupOrEnd,
}
#[derive(Debug)]
pub enum LookupOrEnd {
    Lookup ( TokenLookup ),
    None ( &'static str, TokensOfInterest )
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
    Assign, WeakEq, StrictEq,
    
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

    // Structural tokens: '(', '[' , ']', ')'
    OpenParen, OpenBrack, CloseParen, CloseBrack,

    // Structural operators: ':', ','
    Colon, Comma,

    // String operators
    Delete, Typeof, In, Instanceof, Of,
    If, Switch,

    // Only after operators
    New, Dot,

    // Not interesting :(
    None
}


pub fn initialize_lookup () -> TokenLookup {
    TokenLookup {
        lookup: vec![
            CharacterAndStep {
                // Tokens beginning with '&' 
                next: Regex::new("&").unwrap(),
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '&&'    
                            CharacterAndStep {
                                next: Regex::new("&").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::LogAnd )
                            },
                            // '&'
                            CharacterAndStep {
                                next: Regex::new("\\s").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::BitAnd )
                            },
                            // '&='
                            CharacterAndStep {
                                next: Regex::new("=").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::BitAndEq )
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '|'
            CharacterAndStep {
                next: Regex::new("\\|").unwrap(),
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '||'
                            CharacterAndStep {
                                next: Regex::new("\\|").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::LogOr )
                            },
                            // '|'
                            CharacterAndStep {
                                next: Regex::new("\\s").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::BitOr )
                            },
                            // '|='
                            CharacterAndStep {
                                next: Regex::new("=").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::BitOrEq )
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '^'
            CharacterAndStep {
                next: Regex::new("\\^").unwrap(),
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '^'
                            CharacterAndStep {
                                next: Regex::new("\\s").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::BitXor )
                            },
                            // '^='
                            CharacterAndStep {
                                next: Regex::new("=").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::BitXorEq )
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '~'
            CharacterAndStep {
                next: Regex::new("~").unwrap(),
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '~'
                            CharacterAndStep {
                                next: Regex::new("\\s").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::BitNot )
                            },
                            // '~='
                            CharacterAndStep {
                                next: Regex::new("=").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::BitNotEq )
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '='
            CharacterAndStep {
                next: Regex::new("=").unwrap(),
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '='
                            CharacterAndStep {
                                next: Regex::new("\\s").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::Assign )
                            },
                            CharacterAndStep {
                                next: Regex::new("=").unwrap(),
                                step: LookupOrEnd::Lookup (
                                    TokenLookup {
                                        lookup: vec![
                                            // '==='
                                            CharacterAndStep {
                                                next: Regex::new("=").unwrap(),
                                                step: LookupOrEnd::None ( "", TokensOfInterest::StrictEq )
                                            },
                                            // '=='
                                            CharacterAndStep {
                                                next: Regex::new("\\s").unwrap(),
                                                step: LookupOrEnd::None ( "", TokensOfInterest::WeakEq )
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
            CharacterAndStep {
                next: Regex::new("!").unwrap(),
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '!'
                            CharacterAndStep {
                                next: Regex::new("\\s").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::Not )
                            },
                            CharacterAndStep {
                                next: Regex::new("=").unwrap(),
                                step: LookupOrEnd::Lookup (
                                    TokenLookup {
                                        lookup: vec![
                                            // '!=='
                                            CharacterAndStep {
                                                next: Regex::new("=").unwrap(),
                                                step: LookupOrEnd::None ( "", TokensOfInterest::StrictNotEq )
                                            },
                                            // '!='
                                            CharacterAndStep {
                                                next: Regex::new("\\s").unwrap(),
                                                step: LookupOrEnd::None ( "", TokensOfInterest::NotEq )
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
            CharacterAndStep {
                next: Regex::new("<").unwrap(),
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '<'
                            CharacterAndStep {
                                next: Regex::new("\\s").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::Lt )
                            },
                            // '<='
                            CharacterAndStep {
                                next: Regex::new("=").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::Le )
                            },
                            CharacterAndStep {
                                next: Regex::new("<").unwrap(),
                                step: LookupOrEnd::Lookup (
                                    TokenLookup {
                                        lookup: vec![
                                            // '<<='
                                            CharacterAndStep {
                                                next: Regex::new("=").unwrap(),
                                                step: LookupOrEnd::None ( "", TokensOfInterest::ShLEq )
                                            },
                                            // '<<'
                                            CharacterAndStep {
                                                next: Regex::new("\\s").unwrap(),
                                                step: LookupOrEnd::None ( "", TokensOfInterest::ShL )
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
            CharacterAndStep {
                next: Regex::new(">").unwrap(),
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '>'
                            CharacterAndStep {
                                next: Regex::new("\\s").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::Gt )
                            },
                            // '>='
                            CharacterAndStep {
                                next: Regex::new("=").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::Ge )
                            },
                            CharacterAndStep {
                                next: Regex::new(">").unwrap(),
                                step: LookupOrEnd::Lookup (
                                    TokenLookup {
                                        lookup: vec![
                                            // '>>='
                                            CharacterAndStep {
                                                next: Regex::new("=").unwrap(),
                                                step: LookupOrEnd::None ( "", TokensOfInterest::ShREq )
                                            },
                                            // '>>'
                                            CharacterAndStep {
                                                next: Regex::new("\\s").unwrap(),
                                                step: LookupOrEnd::None ( "", TokensOfInterest::ShR )
                                            },
                                            CharacterAndStep {
                                                next: Regex::new(">").unwrap(),
                                                step: LookupOrEnd::Lookup (
                                                    TokenLookup {
                                                        lookup: vec![
                                                            // '>>>='
                                                            CharacterAndStep {
                                                                next: Regex::new("=").unwrap(),
                                                                step: LookupOrEnd::None ( "", TokensOfInterest::ShRZeroFillEq )
                                                            },
                                                            // '>>>'
                                                            CharacterAndStep {
                                                                next: Regex::new("\\s").unwrap(),
                                                                step: LookupOrEnd::None ( "", TokensOfInterest::ShRZeroFill )
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
            CharacterAndStep {
                next: Regex::new("\\+").unwrap(),
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '++'
                            CharacterAndStep {
                                next: Regex::new("\\+").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::Inc )
                            },
                            // '+='
                            CharacterAndStep {
                                next: Regex::new("=").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::AddEq )
                            },
                            // '+'
                            CharacterAndStep {
                                next: Regex::new("\\s").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::Add )
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '-'
            CharacterAndStep {
                next: Regex::new("-").unwrap(),
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '--'
                            CharacterAndStep {
                                next: Regex::new("-").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::Dec )
                            },
                            // '-='
                            CharacterAndStep {
                                next: Regex::new("=").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::SubEq )
                            },
                            // '-'
                            CharacterAndStep {
                                next: Regex::new("\\s").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::Sub )
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '*'
            CharacterAndStep {
                next: Regex::new("\\*").unwrap(),
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '*='
                            CharacterAndStep {
                                next: Regex::new("=").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::MulEq )
                            },
                            // '*'
                            CharacterAndStep {
                                next: Regex::new("\\s").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::Mul )
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '/'
            CharacterAndStep {
                next: Regex::new("/").unwrap(),
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '/='
                            CharacterAndStep {
                                next: Regex::new("=").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::DivEq )
                            },
                            // '/'
                            CharacterAndStep {
                                next: Regex::new("\\s").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::Div )
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '%'
            CharacterAndStep {
                next: Regex::new("%").unwrap(),
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '%='
                            CharacterAndStep {
                                next: Regex::new("=").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::ModEq )
                            },
                            // '%'
                            CharacterAndStep {
                                next: Regex::new("\\s").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::Mod )
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '('
            CharacterAndStep {
                next: Regex::new("\\(").unwrap(),
                step: LookupOrEnd::None( "", TokensOfInterest::OpenParen )
            },
            // Tokens beginning with '['
            CharacterAndStep {
                next: Regex::new("\\[").unwrap(),
                step: LookupOrEnd::None( "", TokensOfInterest::OpenBrack )
            },
            // Tokens beginning with ']'
            CharacterAndStep {
                next: Regex::new("\\)").unwrap(),
                step: LookupOrEnd::None( "", TokensOfInterest::CloseParen )
            },
            // Tokens beginning with ')'
            CharacterAndStep {
                next: Regex::new("\\]").unwrap(),
                step: LookupOrEnd::None( "", TokensOfInterest::CloseBrack )
            },
            // Tokens beginning with ','
            CharacterAndStep {
                next: Regex::new(",").unwrap(),
                step: LookupOrEnd::None( "", TokensOfInterest::Comma )
            },
            // Tokens beginning with ':'
            CharacterAndStep {
                next: Regex::new(":").unwrap(),
                step: LookupOrEnd::None( "", TokensOfInterest::Colon )
            },
            // Tokens beginning with '.'
            CharacterAndStep {
                next: Regex::new(".").unwrap(),
                step: LookupOrEnd::None( "", TokensOfInterest::Dot )
            },
            
            // Tokens beginning with 'in'
            CharacterAndStep {
                next: Regex::new("i").unwrap(),
                step: LookupOrEnd::Lookup (
                    TokenLookup { 
                        lookup: vec![
                            // 'in'
                            CharacterAndStep {
                                next: Regex::new("n").unwrap(),
                                step: LookupOrEnd::Lookup (
                                    TokenLookup {
                                        lookup: vec![
                                            // 'in'
                                            CharacterAndStep {
                                                next: Regex::new("\\s").unwrap(),
                                                step: LookupOrEnd::None ( "", TokensOfInterest::In )
                                            },
                                            // 'instanceof'
                                            CharacterAndStep {
                                                next: Regex::new("s").unwrap(),
                                                step: LookupOrEnd::None ( "tanceof", TokensOfInterest::Instanceof  )
                                            }
                                        ]
                                    }
                                )
                            },
                            CharacterAndStep {
                                next: Regex::new("f").unwrap(),
                                step: LookupOrEnd::None ( "", TokensOfInterest::If )
                            },
                        ]
                    }
                )
            },

            // Tokens beginning with 'o'
            CharacterAndStep {
                next: Regex::new("o").unwrap(),
                step: LookupOrEnd::None ( "f", TokensOfInterest::Of )
            },

            // Tokens beginning with 't'
            CharacterAndStep {
                next: Regex::new("t").unwrap(),
                step: LookupOrEnd::None ( "ypeof", TokensOfInterest::Typeof )
            },

            // Tokens beginning with 's'
            CharacterAndStep {
                next: Regex::new("s").unwrap(),
                step: LookupOrEnd::None ( "witch", TokensOfInterest::Switch )
            },

            // Tokens beginning with 'd'
            CharacterAndStep {
                next: Regex::new("d").unwrap(),
                step: LookupOrEnd::None ( "elete", TokensOfInterest::Delete )
            },

            // Tokens beginning with 'n'
            CharacterAndStep {
                next: Regex::new("n").unwrap(),
                step: LookupOrEnd::None ( "ew", TokensOfInterest::New )
            },
        ]
    }
}



use std::str;


#[allow(non_snake_case)]
pub fn lookup_token (token: &[u8], lookup_table: &TokenLookup) -> TokensOfInterest {
    println!("{:?}", token);
    for char_step in &lookup_table.lookup {
        
        // Getting the next character of the token, and the token to test against
        let (next_char, next_token) = 
            if token.len() > 0 { (token[ 0usize ], &token[1..]) }
            else { (' ' as u8, token) };

        if !char_step.next.is_match(str::from_utf8(&[ next_char; 1 ]).unwrap()) {
            // If the first character of the token slice is not the 
            //      next char for this CharAndStep branch, then 
            //      just continue
            continue;
        }
        else {
            println!("{:?}", char_step.step);
            // If the character does match the character, then we check if the 
            //      step property of the CharacterAndStep struct is another 
            //      lookup, or a token of interest
            match &char_step.step {
                LookupOrEnd::Lookup( lookup ) => return lookup_token(next_token, &lookup),
                LookupOrEnd::None( RDLP, result ) => {
                    
                    // println!("Rem str: {:?}", next_token);
                    // println!("RDLP: {:?}", RDLP);

                    if str::from_utf8(next_token).unwrap() == *RDLP {
                        return *result;
                    }
                }
            }
            
        }
    }
    TokensOfInterest::None
}



*/
#[derive(Debug)]
pub struct TokenLookup {
    lookup: Vec<CharacterAndStep>
}

#[derive(Debug)]
pub struct CharacterAndStep {
    next: char,
    step: LookupOrEnd,
}
#[derive(Debug)]
pub enum LookupOrEnd {
    Lookup ( TokenLookup ),
    None ( &'static str, TokensOfInterest )
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
    Assign, WeakEq, StrictEq,
    
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

    // Structural tokens: '(', '[' , ']', ')'
    OpenParen, OpenBrack, CloseParen, CloseBrack,

    // Structural operators: ':', ','
    Colon, Comma,

    // String operators
    Delete, Typeof, In, Instanceof, Of,
    If, Switch,

    // Only after operators
    New, Dot,

    // Not interesting :(
    None
}


pub fn initialize_lookup () -> TokenLookup {
    TokenLookup {
        lookup: vec![
            CharacterAndStep {
                // Tokens beginning with '&' 
                next: '&',
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '&&'    
                            CharacterAndStep {
                                next: '&',
                                step: LookupOrEnd::None ( "", TokensOfInterest::LogAnd )
                            },
                            // '&'
                            CharacterAndStep {
                                next: ' ',
                                step: LookupOrEnd::None ( "", TokensOfInterest::BitAnd )
                            },
                            // '&='
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::None ( "", TokensOfInterest::BitAndEq )
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '|'
            CharacterAndStep {
                next: '|',
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '||'
                            CharacterAndStep {
                                next: '|',
                                step: LookupOrEnd::None ( "", TokensOfInterest::LogOr )
                            },
                            // '|'
                            CharacterAndStep {
                                next: ' ',
                                step: LookupOrEnd::None ( "", TokensOfInterest::BitOr )
                            },
                            // '|='
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::None ( "", TokensOfInterest::BitOrEq )
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '^'
            CharacterAndStep {
                next: '^',
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '^'
                            CharacterAndStep {
                                next: ' ',
                                step: LookupOrEnd::None ( "", TokensOfInterest::BitXor )
                            },
                            // '^='
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::None ( "", TokensOfInterest::BitXorEq )
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '~'
            CharacterAndStep {
                next: '~',
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '~'
                            CharacterAndStep {
                                next: ' ',
                                step: LookupOrEnd::None ( "", TokensOfInterest::BitNot )
                            },
                            // '~='
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::None ( "", TokensOfInterest::BitNotEq )
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '='
            CharacterAndStep {
                next: '=',
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '='
                            CharacterAndStep {
                                next: ' ',
                                step: LookupOrEnd::None ( "", TokensOfInterest::Assign )
                            },
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::Lookup (
                                    TokenLookup {
                                        lookup: vec![
                                            // '==='
                                            CharacterAndStep {
                                                next: '=',
                                                step: LookupOrEnd::None ( "", TokensOfInterest::StrictEq )
                                            },
                                            // '=='
                                            CharacterAndStep {
                                                next: ' ',
                                                step: LookupOrEnd::None ( "", TokensOfInterest::WeakEq )
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
            CharacterAndStep {
                next: '!',
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '!'
                            CharacterAndStep {
                                next: ' ',
                                step: LookupOrEnd::None ( "", TokensOfInterest::Not )
                            },
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::Lookup (
                                    TokenLookup {
                                        lookup: vec![
                                            // '!=='
                                            CharacterAndStep {
                                                next: '=',
                                                step: LookupOrEnd::None ( "", TokensOfInterest::StrictNotEq )
                                            },
                                            // '!='
                                            CharacterAndStep {
                                                next: ' ',
                                                step: LookupOrEnd::None ( "", TokensOfInterest::NotEq )
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
            CharacterAndStep {
                next: '<',
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '<'
                            CharacterAndStep {
                                next: ' ',
                                step: LookupOrEnd::None ( "", TokensOfInterest::Lt )
                            },
                            // '<='
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::None ( "", TokensOfInterest::Le )
                            },
                            CharacterAndStep {
                                next: '<',
                                step: LookupOrEnd::Lookup (
                                    TokenLookup {
                                        lookup: vec![
                                            // '<<='
                                            CharacterAndStep {
                                                next: '=',
                                                step: LookupOrEnd::None ( "", TokensOfInterest::ShLEq )
                                            },
                                            // '<<'
                                            CharacterAndStep {
                                                next: ' ',
                                                step: LookupOrEnd::None ( "", TokensOfInterest::ShL )
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
            CharacterAndStep {
                next: '>',
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '>'
                            CharacterAndStep {
                                next: ' ',
                                step: LookupOrEnd::None ( "", TokensOfInterest::Gt )
                            },
                            // '>='
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::None ( "", TokensOfInterest::Ge )
                            },
                            CharacterAndStep {
                                next: '>',
                                step: LookupOrEnd::Lookup (
                                    TokenLookup {
                                        lookup: vec![
                                            // '>>='
                                            CharacterAndStep {
                                                next: '=',
                                                step: LookupOrEnd::None ( "", TokensOfInterest::ShREq )
                                            },
                                            // '>>'
                                            CharacterAndStep {
                                                next: ' ',
                                                step: LookupOrEnd::None ( "", TokensOfInterest::ShR )
                                            },
                                            CharacterAndStep {
                                                next: '>',
                                                step: LookupOrEnd::Lookup (
                                                    TokenLookup {
                                                        lookup: vec![
                                                            // '>>>='
                                                            CharacterAndStep {
                                                                next: '=',
                                                                step: LookupOrEnd::None ( "", TokensOfInterest::ShRZeroFillEq )
                                                            },
                                                            // '>>>'
                                                            CharacterAndStep {
                                                                next: ' ',
                                                                step: LookupOrEnd::None ( "", TokensOfInterest::ShRZeroFill )
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
            CharacterAndStep {
                next: '+',
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '++'
                            CharacterAndStep {
                                next: '+',
                                step: LookupOrEnd::None ( "", TokensOfInterest::Inc )
                            },
                            // '+='
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::None ( "", TokensOfInterest::AddEq )
                            },
                            // '+'
                            CharacterAndStep {
                                next: ' ',
                                step: LookupOrEnd::None ( "", TokensOfInterest::Add )
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '-'
            CharacterAndStep {
                next: '-',
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '--'
                            CharacterAndStep {
                                next: '-',
                                step: LookupOrEnd::None ( "", TokensOfInterest::Dec )
                            },
                            // '-='
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::None ( "", TokensOfInterest::SubEq )
                            },
                            // '-'
                            CharacterAndStep {
                                next: ' ',
                                step: LookupOrEnd::None ( "", TokensOfInterest::Sub )
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '*'
            CharacterAndStep {
                next: '*',
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '*='
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::None ( "", TokensOfInterest::MulEq )
                            },
                            // '*'
                            CharacterAndStep {
                                next: ' ',
                                step: LookupOrEnd::None ( "", TokensOfInterest::Mul )
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '/'
            CharacterAndStep {
                next: '/',
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '/='
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::None ( "", TokensOfInterest::DivEq )
                            },
                            // '/'
                            CharacterAndStep {
                                next: ' ',
                                step: LookupOrEnd::None ( "", TokensOfInterest::Div )
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '%'
            CharacterAndStep {
                next: '%',
                step: LookupOrEnd::Lookup (
                    TokenLookup {
                        lookup: vec![
                            // '%='
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::None ( "", TokensOfInterest::ModEq )
                            },
                            // '%'
                            CharacterAndStep {
                                next: ' ',
                                step: LookupOrEnd::None ( "", TokensOfInterest::Mod )
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '('
            CharacterAndStep {
                next: '(',
                step: LookupOrEnd::None( "", TokensOfInterest::OpenParen )
            },
            // Tokens beginning with '['
            CharacterAndStep {
                next: '[',
                step: LookupOrEnd::None( "", TokensOfInterest::OpenBrack )
            },
            // Tokens beginning with ']'
            CharacterAndStep {
                next: ')',
                step: LookupOrEnd::None( "", TokensOfInterest::CloseParen )
            },
            // Tokens beginning with ')'
            CharacterAndStep {
                next: ']',
                step: LookupOrEnd::None( "", TokensOfInterest::CloseBrack )
            },
            // Tokens beginning with ','
            CharacterAndStep {
                next: ',',
                step: LookupOrEnd::None( "", TokensOfInterest::Comma )
            },
            // Tokens beginning with ':'
            CharacterAndStep {
                next: ':',
                step: LookupOrEnd::None( "", TokensOfInterest::Colon )
            },
            // Tokens beginning with '.'
            CharacterAndStep {
                next: '.',
                step: LookupOrEnd::None( "", TokensOfInterest::Dot )
            },
            
            // Tokens beginning with 'in'
            CharacterAndStep {
                next: 'i',
                step: LookupOrEnd::Lookup (
                    TokenLookup { 
                        lookup: vec![
                            // 'in'
                            CharacterAndStep {
                                next: 'n',
                                step: LookupOrEnd::Lookup (
                                    TokenLookup {
                                        lookup: vec![
                                            // 'in'
                                            CharacterAndStep {
                                                next: ' ',
                                                step: LookupOrEnd::None ( "", TokensOfInterest::In )
                                            },
                                            // 'instanceof'
                                            CharacterAndStep {
                                                next: 's',
                                                step: LookupOrEnd::None ( "tanceof", TokensOfInterest::Instanceof  )
                                            }
                                        ]
                                    }
                                )
                            },
                            CharacterAndStep {
                                next: 'f',
                                step: LookupOrEnd::None ( "", TokensOfInterest::If )
                            },
                        ]
                    }
                )
            },

            // Tokens beginning with 'o'
            CharacterAndStep {
                next: 'o',
                step: LookupOrEnd::None ( "f", TokensOfInterest::Of )
            },

            // Tokens beginning with 't'
            CharacterAndStep {
                next: 't',
                step: LookupOrEnd::None ( "ypeof", TokensOfInterest::Typeof )
            },

            // Tokens beginning with 's'
            CharacterAndStep {
                next: 's',
                step: LookupOrEnd::None ( "witch", TokensOfInterest::Switch )
            },

            // Tokens beginning with 'd'
            CharacterAndStep {
                next: 'd',
                step: LookupOrEnd::None ( "elete", TokensOfInterest::Delete )
            },

            // Tokens beginning with 'n'
            CharacterAndStep {
                next: 'n',
                step: LookupOrEnd::None ( "ew", TokensOfInterest::New )
            },
        ]
    }
}



use std::str;


#[allow(non_snake_case)]
pub fn lookup_token (token: &[u8], lookup_table: &TokenLookup) -> TokensOfInterest {
    println!("{:?}", token);
    for char_step in &lookup_table.lookup {
        
        // Getting the next character of the token, and the token to test against
        let (next_char, next_token) = 
            if token.len() == 0usize { (' ', token) }              // if there are no more characters, we've reached the end, so the last character is whitespace
            else { (token[ 0usize ] as char, &token[1..]) };       // otherwise, it is just the first character of the token
        
        if char_step.next != next_char {
            // If the first character of the token slice is not the 
            //      next char for this CharAndStep branch, then 
            //      just continue
            continue;
        }
        else {
            // println!("{:?}", char_step.step);
            // If the character does match the character, then we check if the 
            //      step property of the CharacterAndStep struct is another 
            //      lookup, or a token of interest
            match &char_step.step {
                LookupOrEnd::Lookup( lookup ) => return lookup_token(&token[1..], &lookup),
                LookupOrEnd::None( RDLP, result ) => {
                    
                    println!("Rem str: {:?}", next_token);
                    println!("RDLP: {:?}", RDLP);

                    if str::from_utf8(next_token).unwrap() == *RDLP {
                        return *result
                    }
                }
            }
            
        }
    }
    TokensOfInterest::None
}