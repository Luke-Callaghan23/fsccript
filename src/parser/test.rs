#[cfg(test)]
mod tests {
    use crate::parser::token_type_lookup::*;

    #[test]
    // Most basic lookup
    fn single_char_lookup () {

        let lookup_table = initialize_lookup();

        let string: &str = "%";

        println!("{:?}", lookup_token(string.as_bytes(), &lookup_table));

        assert_eq!(lookup_token(string.as_bytes(), &lookup_table), TokensOfInterest::Mod);
    }
    
    #[test]
    // Three-character lookup
    fn triple_char_lookup () {

        let lookup_table = initialize_lookup();

        let string: &str = ">>>";
        let res = lookup_token(string.as_bytes(), &lookup_table);
        // println!("{:?}", lookup_token(string.as_bytes(), &lookup_table));

        assert_eq!(res, TokensOfInterest::ShRZeroFill);
    }

    
    #[test]
    // Four-character long lookup
    fn quad_char_lookup () {

        let lookup_table = initialize_lookup();

        let string: &str = ">>>=";
        let res = lookup_token(string.as_bytes(), &lookup_table);
        // println!("{:?}", lookup_token(string.as_bytes(), &lookup_table));

        assert_eq!(res, TokensOfInterest::ShRZeroFillEq);
    }
    
    #[test]
    // Testing a string lookup -- (a longer token)
    fn string_lookup () {

        let lookup_table = initialize_lookup();

        let string: &str = "instanceof";
        let res = lookup_token(string.as_bytes(), &lookup_table);
        // println!("{:?}", lookup_token(string.as_bytes(), &lookup_table));

        assert_eq!(res, TokensOfInterest::Instanceof);
    }
    
    #[test]
    // Testing another string lookup
    fn string_lookup_2 () {

        let lookup_table = initialize_lookup();

        let string: &str = "typeof";
        let res = lookup_token(string.as_bytes(), &lookup_table);
        // println!("{:?}", lookup_token(string.as_bytes(), &lookup_table));

        assert_eq!(res, TokensOfInterest::Typeof);
    }
    
    
    #[test]
    // Testing a failed lookup
    fn failed_lookup () {

        let lookup_table = initialize_lookup();

        let string: &str = "string";
        let res = lookup_token(string.as_bytes(), &lookup_table);
        // println!("{:?}", lookup_token(string.as_bytes(), &lookup_table));

        assert_eq!(res, TokensOfInterest::None);
    }
    
    #[test]
    // Stress test -- all tokens of interest
    fn all_lookups () {

        let lookup_table = initialize_lookup();

        let tests: Vec<(&str, TokensOfInterest)> = vec![
            ("&", TokensOfInterest::BitAnd), 
            ("&=", TokensOfInterest::BitAndEq), 
            ("&&", TokensOfInterest::LogAnd), 
            ("|", TokensOfInterest::BitOr ), 
            ("|=", TokensOfInterest::BitOrEq ), 
            ("||", TokensOfInterest::LogOr), 
            ("^", TokensOfInterest::BitXor), 
            ("^=", TokensOfInterest::BitXorEq), 
            ("~", TokensOfInterest::BitNot), 
            ("~=", TokensOfInterest::BitNotEq), 
            ("=", TokensOfInterest::Assign), 
            ("==", TokensOfInterest::WeakEq), 
            ("===", TokensOfInterest::StrictEq),
            ("!", TokensOfInterest::Not), 
            ("!=", TokensOfInterest::NotEq), 
            ("!==", TokensOfInterest::StrictNotEq),
            (">", TokensOfInterest::Gt), 
            (">=", TokensOfInterest::Ge), 
            (">>", TokensOfInterest::ShR), 
            (">>=", TokensOfInterest::ShREq), 
            (">>>", TokensOfInterest::ShRZeroFill), 
            (">>>=", TokensOfInterest::ShRZeroFillEq),
            ("<", TokensOfInterest::Lt), 
            ("<=", TokensOfInterest::Le), 
            ("<<", TokensOfInterest::ShL), 
            ("<<=", TokensOfInterest::ShLEq),
            ("+", TokensOfInterest::Add), 
            ("+=", TokensOfInterest::AddEq), 
            ("++", TokensOfInterest::Inc), 
            ("-", TokensOfInterest::Sub), 
            ("-=", TokensOfInterest::SubEq), 
            ("--", TokensOfInterest::Dec), 
            ("*", TokensOfInterest::Mul), 
            ("*=", TokensOfInterest::MulEq), 
            ("/", TokensOfInterest::Div), 
            ("/=", TokensOfInterest::DivEq), 
            ("%", TokensOfInterest::Mod), 
            ("%=", TokensOfInterest::ModEq), 
            ("(", TokensOfInterest::OpenParen), 
            ("[", TokensOfInterest::OpenBrack),
            (":", TokensOfInterest::Colon), 
            (",", TokensOfInterest::Comma),
            ("delete", TokensOfInterest::Delete), 
            ("typeof", TokensOfInterest::Typeof), 
            ("in", TokensOfInterest::In), 
            ("instanceof", TokensOfInterest::Instanceof), 
            ("of", TokensOfInterest::Of),
            ("new", TokensOfInterest::New), 
            (".", TokensOfInterest::Dot),
        ];

        for (string, target) in tests {
            let res = lookup_token(string.as_bytes(), &lookup_table);
            assert_eq!(res, target);
        }

        
    }
    
    


}