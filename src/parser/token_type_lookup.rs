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
    None ( regex::Regex, TokensOfInterest )
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

use regex::Regex;

pub fn initialize_lookup () -> TokenLookup {
    let def_regex = Regex::new(".?").unwrap();

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
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::LogAnd )
                            },
                            // '&'
                            CharacterAndStep {
                                next: ' ',
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::BitAnd )
                            },
                            // '&='
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::BitAndEq )
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
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::LogOr )
                            },
                            // '|'
                            CharacterAndStep {
                                next: ' ',
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::BitOr )
                            },
                            // '|='
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::BitOrEq )
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
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::BitXor )
                            },
                            // '^='
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::BitXorEq )
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
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::BitNot )
                            },
                            // '~='
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::BitNotEq )
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
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::Assign )
                            },
                            // '=>'
                            CharacterAndStep {
                                next: '>',
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::ArrowFunction )
                            },
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::Lookup (
                                    TokenLookup {
                                        lookup: vec![
                                            // '==='
                                            CharacterAndStep {
                                                next: '=',
                                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::StrictEq )
                                            },
                                            // '=='
                                            CharacterAndStep {
                                                next: ' ',
                                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::WeakEq )
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
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::Not )
                            },
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::Lookup (
                                    TokenLookup {
                                        lookup: vec![
                                            // '!=='
                                            CharacterAndStep {
                                                next: '=',
                                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::StrictNotEq )
                                            },
                                            // '!='
                                            CharacterAndStep {
                                                next: ' ',
                                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::NotEq )
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
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::Lt )
                            },
                            // '<='
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::Le )
                            },
                            CharacterAndStep {
                                next: '<',
                                step: LookupOrEnd::Lookup (
                                    TokenLookup {
                                        lookup: vec![
                                            // '<<='
                                            CharacterAndStep {
                                                next: '=',
                                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::ShLEq )
                                            },
                                            // '<<'
                                            CharacterAndStep {
                                                next: ' ',
                                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::ShL )
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
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::Gt )
                            },
                            // '>='
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::Ge )
                            },
                            CharacterAndStep {
                                next: '>',
                                step: LookupOrEnd::Lookup (
                                    TokenLookup {
                                        lookup: vec![
                                            // '>>='
                                            CharacterAndStep {
                                                next: '=',
                                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::ShREq )
                                            },
                                            // '>>'
                                            CharacterAndStep {
                                                next: ' ',
                                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::ShR )
                                            },
                                            CharacterAndStep {
                                                next: '>',
                                                step: LookupOrEnd::Lookup (
                                                    TokenLookup {
                                                        lookup: vec![
                                                            // '>>>='
                                                            CharacterAndStep {
                                                                next: '=',
                                                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::ShRZeroFillEq )
                                                            },
                                                            // '>>>'
                                                            CharacterAndStep {
                                                                next: ' ',
                                                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::ShRZeroFill )
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
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::Inc )
                            },
                            // '+='
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::AddEq )
                            },
                            // '+'
                            CharacterAndStep {
                                next: ' ',
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::Add )
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
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::Dec )
                            },
                            // '-='
                            CharacterAndStep {
                                next: '=',
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::SubEq )
                            },
                            // '-'
                            CharacterAndStep {
                                next: ' ',
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::Sub )
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
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::MulEq )
                            },
                            // '*'
                            CharacterAndStep {
                                next: ' ',
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::Mul )
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
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::DivEq )
                            },
                            // '/'
                            CharacterAndStep {
                                next: ' ',
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::Div )
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
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::ModEq )
                            },
                            // '%'
                            CharacterAndStep {
                                next: ' ',
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::Mod )
                            }
                        ]
                    }
                )
            },
            // Tokens beginning with '('
            CharacterAndStep {
                next: '(',
                step: LookupOrEnd::None( def_regex.clone(), TokensOfInterest::OpenParen )
            },
            // Tokens beginning with '['
            CharacterAndStep {
                next: '[',
                step: LookupOrEnd::None( def_regex.clone(), TokensOfInterest::OpenBrack )
            },
            // Tokens beginning with ']'
            CharacterAndStep {
                next: ')',
                step: LookupOrEnd::None( def_regex.clone(), TokensOfInterest::CloseParen )
            },
            // Tokens beginning with ')'
            CharacterAndStep {
                next: ']',
                step: LookupOrEnd::None( def_regex.clone(), TokensOfInterest::CloseBrack )
            },
            // Tokens beginning with ','
            CharacterAndStep {
                next: ',',
                step: LookupOrEnd::None( def_regex.clone(), TokensOfInterest::Comma )
            },
            // Tokens beginning with ':'
            CharacterAndStep {
                next: ':',
                step: LookupOrEnd::None( def_regex.clone(), TokensOfInterest::Colon )
            },
            // Tokens beginning with '.'
            CharacterAndStep {
                next: '.',
                step: LookupOrEnd::None( def_regex.clone(), TokensOfInterest::Dot )
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
                                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::In )
                                            },
                                            // 'instanceof'
                                            CharacterAndStep {
                                                next: 's',
                                                step: LookupOrEnd::None ( Regex::new("tanceof\\s?$").unwrap(), TokensOfInterest::Instanceof  )
                                            }
                                        ]
                                    }
                                )
                            },
                            CharacterAndStep {
                                next: 'f',
                                step: LookupOrEnd::None ( def_regex.clone(), TokensOfInterest::If )
                            },
                        ]
                    }
                )
            },

            // Tokens beginning with 'o'
            CharacterAndStep {
                next: 'o',
                step: LookupOrEnd::None ( Regex::new("f\\s?$").unwrap(), TokensOfInterest::Of )
            },

            // Tokens beginning with 't'
            CharacterAndStep {
                next: 't',
                step: LookupOrEnd::None ( Regex::new("ypeof\\s?$").unwrap(), TokensOfInterest::Typeof )
            },

            // Tokens beginning with 's'
            CharacterAndStep {
                next: 's',
                step: LookupOrEnd::None ( Regex::new("witch\\s?$").unwrap(), TokensOfInterest::Switch )
            },

            // Tokens beginning with 'd'
            CharacterAndStep {
                next: 'd',
                step: LookupOrEnd::None ( Regex::new("elete\\s?$").unwrap(), TokensOfInterest::Delete )
            },

            // Tokens beginning with 'n'
            CharacterAndStep {
                next: 'n',
                step: LookupOrEnd::None ( Regex::new("ew\\s?$").unwrap(), TokensOfInterest::New )
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

                    if RDLP.is_match(str::from_utf8(next_token).unwrap()) {
                        return *result
                    }
                }
            }
            
        }
    }
    TokensOfInterest::None
}