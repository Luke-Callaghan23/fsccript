
    #[cfg(test)]
    pub mod tests {
        // use super::next_token_of_interest;

        // use crate::parser::parser::next_token_of_interest;
        // use crate::parser::token_type_lookup::*;

        // #[test]
        // fn test_only_token () {
        //     let res = 
        //         next_token_of_interest("%", &initialize_lookup());
        //     assert_eq!( res, ("", TokensOfInterest::Mod));
        // }
        
        // #[test]
        // fn test_longer_token () {
        //     let res = 
        //         next_token_of_interest("%=", &initialize_lookup());
        //     assert_eq!( res, ("", TokensOfInterest::ModEq));
        // }

        // #[test]
        // fn test_even_longer_token () {
        //     let res = 
        //         next_token_of_interest(">>>=", &initialize_lookup());
        //     assert_eq!( res, ("", TokensOfInterest::ShRZeroFillEq));
        // }


        // #[test]
        // fn test_whitespace_before_tokens () {
        //     let res = 
        //         next_token_of_interest("\t\t\t\t%", &initialize_lookup());
        //     assert_eq!(
        //         res,
        //         ("", TokensOfInterest::Mod)
        //     );
        // }

        

        // #[test]
        // fn test_basic_first_token () {
        //     let res = 
        //         next_token_of_interest("%\t\n\t\t\t\t some arbitrary stuff after the modulo", &initialize_lookup());
        //     assert_eq!(
        //         res,
        //         ("\t\n\t\t\t\t some arbitrary stuff after the modulo",TokensOfInterest::Mod)
        //     );
        // }

        // #[test]
        // fn test_two_tokens () {

        //     let table = &initialize_lookup();

        //     let res = 
        //         next_token_of_interest("%%", table);
        //     assert_eq!(
        //         res,
        //         ("%", TokensOfInterest::Mod)
        //     );

        //     let rem = res.0;
        //     let res = next_token_of_interest(rem, table);
        //     assert_eq!(
        //         res,
        //         ("", TokensOfInterest::Mod)
        //     );

        // }

        // #[test]
        // fn test_string_token () {
        //     let res = 
        //         next_token_of_interest("in", &initialize_lookup());
        //     assert_eq!( res, ("", TokensOfInterest::In));
        // }

        
        // #[test]
        // fn test_string_token_2 () {
        //     let res = 
        //         next_token_of_interest("instanceof", &initialize_lookup());
        //     assert_eq!( res, ("", TokensOfInterest::Instanceof));
        // }

        
        // #[test]
        // fn test_string_token_3 () {
        //     let res = 
        //         next_token_of_interest("typeof", &initialize_lookup());
        //     assert_eq!( res, ("", TokensOfInterest::Typeof));
        // }
        
        
        // #[test]
        // fn test_string_token_4 () {
        //     let res = 
        //         next_token_of_interest("%typeof", &initialize_lookup());
        //     assert_eq!( res, ("typeof", TokensOfInterest::Mod));
        // }
        
        // #[test]
        // fn test_string_token_5 () {
        //     let res = 
        //         next_token_of_interest("%typeof", &initialize_lookup());
        //     assert_eq!( res, ("typeof", TokensOfInterest::Mod));

        //     let rem = res.0;
        //     let res = 
        //         next_token_of_interest(rem, &initialize_lookup());
        //     assert_eq!( res, ("", TokensOfInterest::Typeof));

        // }
        // #[test]
        // fn test_string_before_token () {
        //     let res = 
        //         next_token_of_interest("somestring%", &initialize_lookup());
        //     assert_eq!( res, ("", TokensOfInterest::Mod));

        //     let rem = res.0;
        //     let res = 
        //         next_token_of_interest(rem, &initialize_lookup());
        //     assert_eq!( res, ("", TokensOfInterest::Typeof));

        // }
        
        
        // #[test]
        // fn test_invalid_string_token () {
        //     let res = 
        //         next_token_of_interest("typeoff", &initialize_lookup());
        //     assert_eq!( res, ("", TokensOfInterest::None));
        // }

        


}