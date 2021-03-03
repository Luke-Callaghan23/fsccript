#[cfg(test)]
mod tests {

    use crate::tokenizer::token_types::*;
    use crate::tokenizer::*;

    #[test]
    // Most basic lookup
    fn test_single_char_lookup () {

        let lookup_table = initialize_lookup();

        let string: &str = "%";

        let res = tokenizer::lookup_token(string.as_bytes(), &lookup_table, TokenOfInterest::None);

        println!("{:?}", res);

        assert_eq!(res, TokenOfInterest::Mod);
    }
    #[test]
    // Most basic lookup
    fn test_double_char_lookup () {

        let lookup_table = initialize_lookup();

        let string: &str = "%%";

        let res = tokenizer::lookup_token(string.as_bytes(), &lookup_table, TokenOfInterest::None);

        println!("{:?}", res);

        assert_eq!(res, TokenOfInterest::Mod);
    }
    
    #[test]
    // Three-character lookup
    fn test_triple_char_lookup () {

        let lookup_table = initialize_lookup();

        let string: &str = ">>>";
        let res = tokenizer::lookup_token(string.as_bytes(), &lookup_table, TokenOfInterest::None);
        // println!("{:?}", lookup_token(string.as_bytes(), &lookup_table));

        assert_eq!(res, TokenOfInterest::ShRZeroFill);
    }

    
    #[test]
    // Four-character long lookup
    fn test_quad_char_lookup () {

        let lookup_table = initialize_lookup();

        let string: &str = ">>>=";
        let res = tokenizer::lookup_token(string.as_bytes(), &lookup_table, TokenOfInterest::None);
        // println!("{:?}", lookup_token(string.as_bytes(), &lookup_table));

        assert_eq!(res, TokenOfInterest::ShRZeroFillEq);
    }
    
    #[test]
    // Testing a string lookup -- (a longer token)
    fn test_string_lookup () {

        let lookup_table = initialize_lookup();

        let string: &str = "instanceof";
        let res = tokenizer::lookup_token(string.as_bytes(), &lookup_table, TokenOfInterest::None);
        // println!("{:?}", lookup_token(string.as_bytes(), &lookup_table));

        assert_eq!(res, TokenOfInterest::Instanceof);
    }
    
    #[test]
    // Testing another string lookup
    fn test_string_lookup_2 () {

        let lookup_table = initialize_lookup();

        let string: &str = "typeof";
        let res = tokenizer::lookup_token(string.as_bytes(), &lookup_table, TokenOfInterest::None);
        // println!("{:?}", lookup_token(string.as_bytes(), &lookup_table));

        assert_eq!(res, TokenOfInterest::Typeof);
    }
    
    
    #[test]
    // Testing a failed lookup
    fn test_failed_lookup () {

        let lookup_table = initialize_lookup();

        let string: &str = "string";
        let res = tokenizer::lookup_token(string.as_bytes(), &lookup_table, TokenOfInterest::None);
        // println!("{:?}", lookup_token(string.as_bytes(), &lookup_table));

        assert_eq!(res, TokenOfInterest::None);
    }
    
    #[test]
    // Testing a failed lookup
    fn test_failed_lookup_2 () {

        let lookup_table = initialize_lookup();

        let string: &str = "typeoff";
        let res = tokenizer::lookup_token(string.as_bytes(), &lookup_table, TokenOfInterest::None);

        assert_eq!(res, TokenOfInterest::None);
    }
    #[test]
    // Testing a failed lookup
    fn test_failed_lookup_3 () {

        let lookup_table = initialize_lookup();

        let string: &str = "a&";
        let res = tokenizer::lookup_token(string.as_bytes(), &lookup_table, TokenOfInterest::None);

        assert_eq!(res, TokenOfInterest::None);
    }
    #[test]
    // Testing a failed lookup
    fn test_failed_lookup_4 () {

        let lookup_table = initialize_lookup();

        let string: &str = "b break";
        let res = tokenizer::lookup_token(string.as_bytes(), &lookup_table, TokenOfInterest::None);

        assert_eq!(res, TokenOfInterest::None);
    }
    
    #[test]
    // Stress test -- all tokens of interest
    fn test_all_lookups () {

        let lookup_table = initialize_lookup();

        let tests: Vec<(&str, TokenOfInterest)> = vec![
            ("&", TokenOfInterest::BitAnd), 
            ("&=", TokenOfInterest::BitAndEq), 
            ("&&", TokenOfInterest::LogAnd), 
            ("|", TokenOfInterest::BitOr ), 
            ("|=", TokenOfInterest::BitOrEq ), 
            ("||", TokenOfInterest::LogOr), 
            ("^", TokenOfInterest::BitXor), 
            ("^=", TokenOfInterest::BitXorEq), 
            ("~", TokenOfInterest::BitNot), 
            ("~=", TokenOfInterest::BitNotEq), 
            ("=", TokenOfInterest::Assign), 
            ("==", TokenOfInterest::WeakEq), 
            ("===", TokenOfInterest::StrictEq),
            ("!", TokenOfInterest::Not), 
            ("!=", TokenOfInterest::NotEq), 
            ("!==", TokenOfInterest::StrictNotEq),
            (">", TokenOfInterest::Gt), 
            (">=", TokenOfInterest::Ge), 
            (">>", TokenOfInterest::ShR), 
            (">>=", TokenOfInterest::ShREq), 
            (">>>", TokenOfInterest::ShRZeroFill), 
            (">>>=", TokenOfInterest::ShRZeroFillEq),
            ("<", TokenOfInterest::Lt), 
            ("<=", TokenOfInterest::Le), 
            ("<<", TokenOfInterest::ShL), 
            ("<<=", TokenOfInterest::ShLEq),
            ("+", TokenOfInterest::Add), 
            ("+=", TokenOfInterest::AddEq), 
            ("++", TokenOfInterest::Inc), 
            ("-", TokenOfInterest::Sub), 
            ("-=", TokenOfInterest::SubEq), 
            ("--", TokenOfInterest::Dec), 
            ("*", TokenOfInterest::Mul), 
            ("*=", TokenOfInterest::MulEq), 
            ("/", TokenOfInterest::Div), 
            ("/=", TokenOfInterest::DivEq), 
            ("%", TokenOfInterest::Mod), 
            ("%=", TokenOfInterest::ModEq), 
            ("(", TokenOfInterest::OpenParen), 
            ("[", TokenOfInterest::OpenBrack),
            (":", TokenOfInterest::Colon), 
            (",", TokenOfInterest::Comma),
            ("delete", TokenOfInterest::Delete), 
            ("typeof", TokenOfInterest::Typeof), 
            ("in", TokenOfInterest::In), 
            ("instanceof", TokenOfInterest::Instanceof), 
            ("of", TokenOfInterest::Of),
            ("new", TokenOfInterest::New), 
            (".", TokenOfInterest::Dot),
            ("if", TokenOfInterest::If),
            ("switch", TokenOfInterest::Switch),
            ("]", TokenOfInterest::CloseBrack),
            (")", TokenOfInterest::CloseParen),
            ("{", TokenOfInterest::OpenCurly),
            ("}", TokenOfInterest::CloseCurly),
            ("else", TokenOfInterest::Else),
            ("case", TokenOfInterest::Case),
            ("return", TokenOfInterest::Return),
            (";", TokenOfInterest::Semicolon),
            ("default", TokenOfInterest::Default),
            ("break", TokenOfInterest::Break),
        ];

        for (string, target) in tests {
            let res = tokenizer::lookup_token(string.as_bytes(), &lookup_table, TokenOfInterest::None);
            assert_eq!(res, target);
        }

        
    }
    
    


}