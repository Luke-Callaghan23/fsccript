#[cfg(test)]
pub mod tests {
    use crate::parser::parser::parser::skip_string;

    use super::super::super::parser::parser::skip_comments_and_whitespace;


    #[test]
    fn test_skip_whitepsace_and_comments_invalid () {
        let i = skip_comments_and_whitespace("p//                       \n/*\n\n\n\n\n\n*/                                  \n\n\n\nhello//\nsup".as_bytes(), 0);
        assert_eq!(i, 0);
    }

    #[test]
    fn test_skip_whitepsace_and_comments () {
        let i = skip_comments_and_whitespace("//                       \n/*\n\n\n\n\n\n*/                                  \n\n\n\nhello//\nsup".as_bytes(), 0);
        assert_eq!(i, 74);
    }
    #[test]
    fn test_skip_whitespace_and_comments_2 () {
        let i = skip_comments_and_whitespace("\t\t\t\t\t\t\t\t\thello//\nsup".as_bytes(), 0);
        assert_eq!(i, 9);
    }
    #[test]
    fn test_skip_whitespace_and_comments_multiple() {
        let i = skip_comments_and_whitespace("\t\t\t\t\t\t\t\t\thello//\nsup".as_bytes(), 0);
        assert_eq!(i, 9);
        let i = skip_comments_and_whitespace("\t\t\t\t\t\t\t\t\thello//\nsup".as_bytes(), 14);
        assert_eq!(i, 3);
    }
    #[test]
    fn test_skip_whitespace_and_comments_mix() {
        let i = skip_comments_and_whitespace("\t\t\t\t\t\t\t\t\t/*\n\n\n*/hello//\nsup".as_bytes(), 0);
        assert_eq!(i, 16);
    }

    #[test]
    fn test_skip_string () {
        let i = skip_string(b"'hello world';", 0);
        assert_eq!(i, 13);
    }
    #[test]
    fn test_skip_string_2 () {
        let i = skip_string(b"\"hello world\";", 0);
        assert_eq!(i, 13);
    }

    #[test]
    fn test_skip_string_3 () {
        let i = skip_string(b"`hello world`;", 0);
        assert_eq!(i, 13);
    }

    #[test]
    fn test_skip_string_invalid () {
        let i = skip_string(b"f`hello world`;", 0);
        assert_eq!(i, 0);
    }
    #[test]
    fn test_skip_string_longer () {
        let i = skip_string(b"some arbitrary stuff before the string `hello world`;", 39);
        assert_eq!(i, 13);
    }

    #[test]
    fn test_skip_string_escaped () {
        let i = skip_string(b"`\\`  `;", 0);
        assert_eq!(i, 6);
    }

       

}