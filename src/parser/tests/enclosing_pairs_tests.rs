
#[cfg(test)]
pub mod test {
    // use crate::parser::enclosing_pairs::{
    //     split_at_enclosing_pair,
    //     initialize_enclosing_pairs,
    //     try_skip_string,
    //     try_skip_comment,
    //     EnclosingSplit
    // };

    // #[test]
    // fn test_basic_enclosing () {

    //     let split = split_at_enclosing_pair (
    //         "()".as_bytes(), 
    //         '(', 
    //         0, 
    //         &initialize_enclosing_pairs()
    //     );

        
    //     assert_eq!(split, Some (EnclosingSplit {
    //         before: &"".as_bytes(),
    //         middle: &"".as_bytes(),
    //         after: &"".as_bytes(),
    //         pair: ('(', ')')
    //     }));

    // }
    // #[test]
    // fn test_nested_enclosing () {

    //     let split = split_at_enclosing_pair (
    //         "(())".as_bytes(), 
    //         '(', 
    //         0, 
    //         &initialize_enclosing_pairs()
    //     );

    //     assert_eq!(split, Some (EnclosingSplit {
    //         before: &"".as_bytes(),
    //         middle: &"()".as_bytes(),
    //         after: &"".as_bytes(),
    //         pair: ('(', ')')
    //     }));

    // }
    // #[test]
    // fn test_nested_enclosing_2 () {

    //     let split= split_at_enclosing_pair (
    //         "(())".as_bytes(), 
    //         '(', 
    //         1, 
    //         &initialize_enclosing_pairs()
    //     );

    //     assert_eq!(split, Some(EnclosingSplit {
    //         before: &"(".as_bytes(),
    //         middle: &"".as_bytes(),
    //         after: &")".as_bytes(),
    //         pair: ('(', ')')
    //     }));

    // }
    // #[test]
    // fn test_broken_enclosing () {

    //     let split = split_at_enclosing_pair (
    //         "(()".as_bytes(), 
    //         '(', 
    //         0, 
    //         &initialize_enclosing_pairs()
    //     );

    //     assert_eq!(split, None);

    // }
    // #[test]
    // fn test_enclosing_string () {

    //     let split = split_at_enclosing_pair (
    //         "\"\"".as_bytes(), 
    //         '"', 
    //         0, 
    //         &initialize_enclosing_pairs()
    //     );

    //     assert_eq!(split, Some(EnclosingSplit {
    //         before: &"".as_bytes(),
    //         middle: &"".as_bytes(),
    //         after: &"".as_bytes(),
    //         pair: ('"', '"')
    //     }));

    // }
    // #[test]
    // fn test_enclosing_string_2 () {

    //     let split = split_at_enclosing_pair (
    //         "'''".as_bytes(), 
    //         '\'', 
    //         0, 
    //         &initialize_enclosing_pairs()
    //     );

    //     assert_eq!(split, Some(EnclosingSplit {
    //         before: &"".as_bytes(),
    //         middle: &"".as_bytes(),
    //         after: &"'".as_bytes(),
    //         pair: ('\'', '\'')
    //     }));

    // }
    // #[test]
    // fn test_enclosing_string_3 () {

    //     let split = split_at_enclosing_pair (
    //         "'\\''".as_bytes(), 
    //         '\'', 
    //         0, 
    //         &initialize_enclosing_pairs()
    //     );

    //     assert_eq!(split, Some(EnclosingSplit {
    //         before: &"".as_bytes(),
    //         middle: &"\\'".as_bytes(),
    //         after: &"".as_bytes(),
    //         pair: ('\'', '\'')
    //     }));

    // }
    // #[test]
    // fn test_nexted_enclosing_3 () {

    //     let split = split_at_enclosing_pair (
    //         "([{}])".as_bytes(), 
    //         '(', 
    //         0, 
    //         &initialize_enclosing_pairs()
    //     );

    //     assert_eq!(split, Some(EnclosingSplit {
    //         before: &"".as_bytes(),
    //         middle: &"[{}]".as_bytes(),
    //         after: &"".as_bytes(),
    //         pair: ('(', ')')
    //     }));

    // }

    // #[test]
    // fn test_enclosing_in_string () {

    //     let index = split_at_enclosing_pair (
    //         "[']']".as_bytes(), 
    //         '[', 
    //         0, 
    //         &initialize_enclosing_pairs()
    //     );

    //     assert_eq!(index, Some(EnclosingSplit {
    //         before: &"".as_bytes(),
    //         middle: &"']'".as_bytes(),
    //         after: &"".as_bytes(),
    //         pair: ('[', ']')
    //     }));

    // }

    
    // #[test]
    // fn test_enclosing_stress_test () {

    //     let split = split_at_enclosing_pair (
    //         "('[{}])))``')".as_bytes(), 
    //         '(', 
    //         0, 
    //         &initialize_enclosing_pairs()
    //     );

    //     assert_eq!(split, Some(EnclosingSplit {
    //         before: &"".as_bytes(),
    //         middle: &"'[{}])))``'".as_bytes(),
    //         after: &"".as_bytes(),
    //         pair: ('(', ')')
    //     }));

    // }
    // #[test]
    // fn test_ecnlosing_stress_test_2 () {

    //     let index = split_at_enclosing_pair (
    //         "('[{}])))``'[{}]{}(((()))))".as_bytes(), 
    //         '(', 
    //         0, 
    //         &initialize_enclosing_pairs()
    //     );

    //     assert_eq!(index, Some(EnclosingSplit {
    //         before: &"".as_bytes(),
    //         middle: &"'[{}])))``'[{}]{}(((())))".as_bytes(),
    //         after: &"".as_bytes(),
    //         pair: ('(', ')')
    //     }));

    // }

    // #[test]
    // fn test_ecnlosing_stress_test_3 () {

    //     let index = split_at_enclosing_pair (
    //         "some arbitrary stuff before the split ('[{}])))``'[{}]{}(((())))) and some arbitrary stuff after the split".as_bytes(), 
    //         '(', 
    //         38, 
    //         &initialize_enclosing_pairs()
    //     );

    //     assert_eq!(index, Some(EnclosingSplit {
    //         before: &"some arbitrary stuff before the split ".as_bytes(),
    //         middle: &"'[{}])))``'[{}]{}(((())))".as_bytes(),
    //         after: &" and some arbitrary stuff after the split".as_bytes(),
    //         pair: ('(', ')')
    //     }));

    // }



    // #[test]
    // fn test_try_skip_string_1 () {
    //     let end = try_skip_string (
    //         b"\"\"",
    //         '"',
    //         0
    //     );

    //     assert_eq!(end, 1);
    // }


    // #[test]
    // fn test_try_skip_string_2 () {
    //     let end = try_skip_string (
    //         b"some arbitrary stuff before the string \"\"",
    //         '"',
    //         39
    //     );

    //     assert_eq!(end, 1);
    // }

    
    // #[test]
    // fn test_try_skip_string_3 () {
    //     let end = try_skip_string (
    //         b"some arbitrary stuff before the string \"\" and some arbitrary stuff after the string",
    //         '"',
    //         39
    //     );

    //     assert_eq!(end, 1);
    // }


    // #[test]
    // fn test_try_skip_string_4_escaped_end () {
    //     let end = try_skip_string (
    //         b"\"\\\"\"",
    //         '"',
    //         0
    //     );

    //     assert_eq!(end, 3);
    // }

    // #[test]
    // fn test_try_skip_string_5_multiple_escaped_end () {
    //     let end = try_skip_string (
    //         b"\"\\\"\\\"\\\"\"",
    //         '"',
    //         0
    //     );

    //     assert_eq!(end, 7);
    // }


    // #[test]
    // fn test_try_skip_string_6 () {
    //     let end = try_skip_string (
    //         b"\"\\\"\\\"\\\" some arbitrary stuff in the string \"",
    //         '"',
    //         0
    //     );

    //     assert_eq!(end, 43);
    // }




    // #[test]
    // fn test_try_skip_comment_1 () {
    //     let end = try_skip_comment (
    //         b"//\n",
    //         '/',
    //         0
    //     );

    //     assert_eq!(end, 2);
    // }


    // #[test]
    // fn test_try_skip_comment_2 () {
    //     let end = try_skip_comment (
    //         b"/**/",
    //         '/',
    //         0
    //     );

    //     assert_eq!(end, 3);
    // }

    // #[test]
    // fn test_try_skip_comment_3 () {
    //     let end = try_skip_comment (
    //         b"some arbitrary stuff before the comment /*some arbitrary stuff inside the comment */",
    //         '/',
    //         40
    //     );

    //     assert_eq!(end,43);
    // }

    
    // #[test]
    // fn test_try_skip_comment_4 () {
    //     let end = try_skip_comment (
    //         b"some arbitrary stuff before the comment /*some arbitrary stuff inside the comment \n\n\n\n*/",
    //         '/',
    //         40
    //     );

    //     assert_eq!(end,47);
    // }






}