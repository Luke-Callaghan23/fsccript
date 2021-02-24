
#[cfg(test)]
pub mod test {
    use crate::parser::enclosing_pairs::{split_at_enclosing_pair, initialize_enclosing_pairs, EnclosingSplit};

    #[test]
    fn test_basic_enclosing () {

        let split = split_at_enclosing_pair (
            "()".as_bytes(), 
            '(', 
            0, 
            &initialize_enclosing_pairs()
        );

        
        assert_eq!(split, Some (EnclosingSplit {
            before: &"".as_bytes(),
            middle: &"".as_bytes(),
            after: &"".as_bytes(),
            pair: ('(', ')')
        }));

    }
    #[test]
    fn test_nested_enclosing () {

        let split = split_at_enclosing_pair (
            "(())".as_bytes(), 
            '(', 
            0, 
            &initialize_enclosing_pairs()
        );

        assert_eq!(split, Some (EnclosingSplit {
            before: &"".as_bytes(),
            middle: &"()".as_bytes(),
            after: &"".as_bytes(),
            pair: ('(', ')')
        }));

    }
    #[test]
    fn test_nested_enclosing_2 () {

        let split= split_at_enclosing_pair (
            "(())".as_bytes(), 
            '(', 
            1, 
            &initialize_enclosing_pairs()
        );

        assert_eq!(split, Some(EnclosingSplit {
            before: &"(".as_bytes(),
            middle: &"".as_bytes(),
            after: &")".as_bytes(),
            pair: ('(', ')')
        }));

    }
    #[test]
    fn test_broken_enclosing () {

        let split = split_at_enclosing_pair (
            "(()".as_bytes(), 
            '(', 
            0, 
            &initialize_enclosing_pairs()
        );

        assert_eq!(split, None);

    }
    #[test]
    fn test_enclosing_string () {

        let split = split_at_enclosing_pair (
            "\"\"".as_bytes(), 
            '"', 
            0, 
            &initialize_enclosing_pairs()
        );

        assert_eq!(split, Some(EnclosingSplit {
            before: &"".as_bytes(),
            middle: &"".as_bytes(),
            after: &"".as_bytes(),
            pair: ('"', '"')
        }));

    }
    #[test]
    fn test_enclosing_string_2 () {

        let split = split_at_enclosing_pair (
            "'''".as_bytes(), 
            '\'', 
            0, 
            &initialize_enclosing_pairs()
        );

        assert_eq!(split, Some(EnclosingSplit {
            before: &"".as_bytes(),
            middle: &"".as_bytes(),
            after: &"'".as_bytes(),
            pair: ('\'', '\'')
        }));

    }
    #[test]
    fn test_enclosing_string_3 () {

        let split = split_at_enclosing_pair (
            "'\\''".as_bytes(), 
            '\'', 
            0, 
            &initialize_enclosing_pairs()
        );

        assert_eq!(split, Some(EnclosingSplit {
            before: &"".as_bytes(),
            middle: &"\\'".as_bytes(),
            after: &"".as_bytes(),
            pair: ('\'', '\'')
        }));

    }
    #[test]
    fn test_nexted_enclosing_3 () {

        let split = split_at_enclosing_pair (
            "([{}])".as_bytes(), 
            '(', 
            0, 
            &initialize_enclosing_pairs()
        );

        assert_eq!(split, Some(EnclosingSplit {
            before: &"".as_bytes(),
            middle: &"[{}]".as_bytes(),
            after: &"".as_bytes(),
            pair: ('(', ')')
        }));

    }

    #[test]
    fn test_enclosing_in_string () {

        let index = split_at_enclosing_pair (
            "[']']".as_bytes(), 
            '[', 
            0, 
            &initialize_enclosing_pairs()
        );

        assert_eq!(index, Some(EnclosingSplit {
            before: &"".as_bytes(),
            middle: &"']'".as_bytes(),
            after: &"".as_bytes(),
            pair: ('[', ']')
        }));

    }

    
    #[test]
    fn test_enclosing_stress_test () {

        let split = split_at_enclosing_pair (
            "('[{}])))``')".as_bytes(), 
            '(', 
            0, 
            &initialize_enclosing_pairs()
        );

        assert_eq!(split, Some(EnclosingSplit {
            before: &"".as_bytes(),
            middle: &"'[{}])))``'".as_bytes(),
            after: &"".as_bytes(),
            pair: ('(', ')')
        }));

    }
    #[test]
    fn test_ecnlosing_stress_test_2 () {

        let index = split_at_enclosing_pair (
            "('[{}])))``'[{}]{}(((()))))".as_bytes(), 
            '(', 
            0, 
            &initialize_enclosing_pairs()
        );

        assert_eq!(index, Some(EnclosingSplit {
            before: &"".as_bytes(),
            middle: &"'[{}])))``'[{}]{}(((())))".as_bytes(),
            after: &"".as_bytes(),
            pair: ('(', ')')
        }));

    }

    #[test]
    fn test_ecnlosing_stress_test_3 () {

        let index = split_at_enclosing_pair (
            "some arbitrary stuff before the split ('[{}])))``'[{}]{}(((())))) and some arbitrary stuff after the split".as_bytes(), 
            '(', 
            38, 
            &initialize_enclosing_pairs()
        );

        assert_eq!(index, Some(EnclosingSplit {
            before: &"some arbitrary stuff before the split ".as_bytes(),
            middle: &"'[{}])))``'[{}]{}(((())))".as_bytes(),
            after: &" and some arbitrary stuff after the split".as_bytes(),
            pair: ('(', ')')
        }));

    }
}