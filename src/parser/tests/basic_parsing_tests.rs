
    #[cfg(test)]
    pub mod tests {
        // use super::next_token_of_interest;

        use crate::parser::parser::parser::next_token_of_interest;
        use crate::parser::token_type_lookup::*;

        #[test]
        fn test_only_token () {
            let res = 
                next_token_of_interest("%", &initialize_lookup());
            assert_eq!(
                res,
                Some ((
                    "",
                    TokensOfInterest::Mod
                ))
            );
        }

        #[test]
        fn test_whitespace_before_tokens () {
            let res = 
                next_token_of_interest("\t\t\t\t%", &initialize_lookup());
            assert_eq!(
                res,
                Some ((
                    "",
                    TokensOfInterest::Mod
                ))
            );
        }

        

        #[test]
        fn test_basic_first_token () {
            let res = 
                next_token_of_interest("%\t\n\t\t\t\t some arbitrary stuff after the modulo", &initialize_lookup());
            assert_eq!(
                res,
                Some ((
                    "\t\n\t\t\t\t some arbitrary stuff after the modulo",
                    TokensOfInterest::Mod
                ))
            );
        }

        #[test]
        fn test_two_tokens () {

            let table = &initialize_lookup();

            let res = 
                next_token_of_interest("% %", table);
            assert_eq!(
                res,
                Some ((
                    " %",
                    TokensOfInterest::Mod
                ))
            );

            let rem = res.unwrap().0;
            let res = next_token_of_interest(rem, table);
            assert_eq!(
                res,
                Some ((
                    "",
                    TokensOfInterest::Mod
                ))
            );

        }
    }