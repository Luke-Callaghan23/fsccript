#[cfg(test)]
pub mod test_tokenizer {
    
    use crate::tokenizer::tokenizer::tokenize;
    use crate::tokenizer::token_types::initialize_lookup;

    #[test]
    fn test_tokenizer_very_basic () {

        let token_stream = tokenize(
            b"%&",
            &initialize_lookup()
        );

        assert_eq!(format!("{}", token_stream), "Token ( BOF )
Token ( Mod )
Token ( BitAnd )
Token ( EOF )
");
    }
    #[test]
    fn test_tokenizer_basic_stream () {

        let token_stream = tokenize(
            b"% hello &",
            &initialize_lookup()
        );

        println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream), "Token ( BOF )
Token ( Mod )
Token ( None )
Token ( BitAnd )
Token ( EOF )
");
    }
    #[test]
    fn test_tokenizer_enclosing () {

        let token_stream = tokenize(
            b"{}",
            &initialize_lookup()
        );

        println!("{}", token_stream);

        assert_eq!(0, 0);
    }
    #[test]
    fn test_tokenizer_enclosing_and_token () {

        let token_stream = tokenize(
            b"{&=}",
            &initialize_lookup()
        );

        println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream), 
"Token ( BOF )
Stream {
    Token ( OpenCurly )
    Token ( BitAndEq )
    Token ( CloseCurly )
}
Token ( EOF )
"
        );
    }
    #[test]
    fn test_tokenizer_double_enclosing () {

        let token_stream = tokenize(
            b"{{}}",
            &initialize_lookup()
        );

        // println!("{:?}", token_stream);
        // println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream), "Token ( BOF )
Stream {
    Token ( OpenCurly )
    Stream {
        Token ( OpenCurly )
        Token ( CloseCurly )
    }
    Token ( CloseCurly )
}
Token ( EOF )
");

    }
    #[test]
    fn test_tokenizer_more_enclosing () {

        let token_stream = tokenize(
            b"{{{{}}}}",
            &initialize_lookup()
        );

        println!("{:?}", token_stream);
        println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream),         
"Token ( BOF )
Stream {
    Token ( OpenCurly )
    Stream {
        Token ( OpenCurly )
        Stream {
            Token ( OpenCurly )
            Stream {
                Token ( OpenCurly )
                Token ( CloseCurly )
            }
            Token ( CloseCurly )
        }
        Token ( CloseCurly )
    }
    Token ( CloseCurly )
}
Token ( EOF )
");

    }
    #[test]
    fn test_tokenizer_some_actual_code () {

        let token_stream = tokenize(
            b"
            if (a == b) 
            {
                return a;
            }
            ",
            &initialize_lookup()
        );

        println!("{:?}", token_stream);
        println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream),         
"Token ( BOF )
Token ( If )
Stream {
    Token ( OpenParen )
    Token ( None )
    Token ( WeakEq )
    Token ( None )
    Token ( CloseParen )
}
Stream {
    Token ( OpenCurly )
    Token ( Return )
    Token ( None )
    Token ( Semicolon )
    Token ( CloseCurly )
}
Token ( EOF )
");

    }
    
    #[test]
    fn test_tokenizer_some_actual_code_2 () {

        let token_stream = tokenize(
            b"
            let b = 0;
            for (let i = 0; i < 20; i++)
            {
                if (i == b) 
                {
                    return a;
                }
                else if (i == 100)
                {
                    return 100;
                }
            }
            ",
            &initialize_lookup()
        );

        println!("{:?}", token_stream);
        println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream),         
"Token ( BOF )
Token ( None )
Token ( None )
Token ( Assign )
Token ( None )
Token ( Semicolon )
Token ( None )
Stream {
    Token ( OpenParen )
    Token ( None )
    Token ( None )
    Token ( Assign )
    Token ( None )
    Token ( Semicolon )
    Token ( None )
    Token ( Lt )
    Token ( None )
    Token ( Semicolon )
    Token ( None )
    Token ( Inc )
    Token ( CloseParen )
}
Stream {
    Token ( OpenCurly )
    Token ( If )
    Stream {
        Token ( OpenParen )
        Token ( None )
        Token ( WeakEq )
        Token ( None )
        Token ( CloseParen )
    }
    Stream {
        Token ( OpenCurly )
        Token ( Return )
        Token ( None )
        Token ( Semicolon )
        Token ( CloseCurly )
    }
    Token ( Else )
    Token ( If )
    Stream {
        Token ( OpenParen )
        Token ( None )
        Token ( WeakEq )
        Token ( None )
        Token ( CloseParen )
    }
    Stream {
        Token ( OpenCurly )
        Token ( Return )
        Token ( None )
        Token ( Semicolon )
        Token ( CloseCurly )
    }
    Token ( CloseCurly )
}
Token ( EOF )
");

    }

    #[test]
    fn test_tokenizer_some_actual_code_switch () {

        let token_stream = tokenize(
            b"
            switch(key) {
                case 0: 
                    a = b;
                    break;
            }
            ",
            &initialize_lookup()
        );

        println!("{:?}", token_stream);
        println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream),         
"Token ( BOF )
Token ( Switch )
Stream {
    Token ( OpenParen )
    Token ( None )
    Token ( CloseParen )
}
Stream {
    Token ( OpenCurly )
    Token ( Case )
    Token ( None )
    Token ( Colon )
    Token ( None )
    Token ( Assign )
    Token ( None )
    Token ( Semicolon )
    Token ( Break )
    Token ( Semicolon )
    Token ( CloseCurly )
}
Token ( EOF )
");
    }
    
    
    #[test]
    fn test_tokenizer_some_actual_code_switch_2 () {

        let token_stream = tokenize(
            b"
            switch(key) {
                case 0: 
                    a = b;
                    break;
                case 1:
                    b = a;
                    break;
                case 4:
                    console.log(b);
                    break;
                default:
                    c = a;
                    break
            }
            ",
            &initialize_lookup()
        );

        println!("{:?}", token_stream);
        println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream),         
"Token ( BOF )
Token ( Switch )
Stream {
    Token ( OpenParen )
    Token ( None )
    Token ( CloseParen )
}
Stream {
    Token ( OpenCurly )
    Token ( Case )
    Token ( None )
    Token ( Colon )
    Token ( None )
    Token ( Assign )
    Token ( None )
    Token ( Semicolon )
    Token ( Break )
    Token ( Semicolon )
    Token ( Case )
    Token ( None )
    Token ( Colon )
    Token ( None )
    Token ( Assign )
    Token ( None )
    Token ( Semicolon )
    Token ( Break )
    Token ( Semicolon )
    Token ( Case )
    Token ( None )
    Token ( Colon )
    Token ( None )
    Token ( Dot )
    Token ( None )
    Stream {
        Token ( OpenParen )
        Token ( None )
        Token ( CloseParen )
    }
    Token ( Semicolon )
    Token ( Break )
    Token ( Semicolon )
    Token ( Default )
    Token ( Colon )
    Token ( None )
    Token ( Assign )
    Token ( None )
    Token ( Semicolon )
    Token ( Break )
    Token ( CloseCurly )
}
Token ( EOF )
");

    }

    
    #[test]
    fn test_tokenizer_comment () {

        let token_stream = tokenize(
            b"/* text inside of the comment */",
            &initialize_lookup()
        );

        // println!("{:?}", token_stream);
        println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream), 
"Token ( BOF )
Stream {
    Token ( MultiLineCommentOpen )
    Token ( None )
    Token ( MultiLineCommentClose )
}
Token ( EOF )
");
    }
    
    #[test]
    fn test_tokenizer_comment_2 () {

        let token_stream = tokenize(
            b"// text inside of the comment \n",
            &initialize_lookup()
        );

        // println!("{:?}", token_stream);
        println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream), 
"Token ( BOF )
Stream {
    Token ( SingleLineCommentOpen )
    Token ( None )
    Token ( SingleLineCommentClose )
}
Token ( EOF )
");
    }
    
    #[test]
    fn test_tokenizer_comment_3 () {

        let token_stream = tokenize(
            b"// text inside of the comment \n// text inside of the comment \n// text inside of the comment \n// text inside of the comment \n",
            &initialize_lookup()
        );

        // println!("{:?}", token_stream);
        println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream), 
"Token ( BOF )
Stream {
    Token ( SingleLineCommentOpen )
    Token ( None )
    Token ( SingleLineCommentClose )
}
Stream {
    Token ( SingleLineCommentOpen )
    Token ( None )
    Token ( SingleLineCommentClose )
}
Stream {
    Token ( SingleLineCommentOpen )
    Token ( None )
    Token ( SingleLineCommentClose )
}
Stream {
    Token ( SingleLineCommentOpen )
    Token ( None )
    Token ( SingleLineCommentClose )
}
Token ( EOF )
");
    }

    #[test]
    fn test_tokenizer_comment_4 () {

        let token_stream = tokenize(
            b"/*// text inside of the comment \n// text inside of the comment \n// text inside of the comment \n// text inside of the comment \n*/",
            &initialize_lookup()
        );

        // println!("{:?}", token_stream);
        println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream), 
"Token ( BOF )
Stream {
    Token ( MultiLineCommentOpen )
    Token ( None )
    Token ( MultiLineCommentClose )
}
Token ( EOF )
");
    }
    #[test]
    fn test_tokenizer_comment_5 () {

        let token_stream = tokenize(
            b"// text inside of the comment \n// text inside of the comment \n/*// text inside of the comment \n// text inside of the comment \n*/",
            &initialize_lookup()
        );

        // println!("{:?}", token_stream);
        println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream), 
"Token ( BOF )
Stream {
    Token ( SingleLineCommentOpen )
    Token ( None )
    Token ( SingleLineCommentClose )
}
Stream {
    Token ( SingleLineCommentOpen )
    Token ( None )
    Token ( SingleLineCommentClose )
}
Stream {
    Token ( MultiLineCommentOpen )
    Token ( None )
    Token ( MultiLineCommentClose )
}
Token ( EOF )
");
    }
    
    #[test]
    fn test_tokenizer_comment_6 () {

        let token_stream = tokenize(
            b"//\n",
            &initialize_lookup()
        );

        // println!("{:?}", token_stream);
        println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream), 
"Token ( BOF )
Stream {
    Token ( SingleLineCommentOpen )
    Token ( None )
    Token ( SingleLineCommentClose )
}
Token ( EOF )
");
    }
    
    #[test]
    fn test_tokenizer_string () {

        let token_stream = tokenize(
            b"''",
            &initialize_lookup()
        );

        // println!("{:?}", token_stream);
        println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream), 
"Token ( BOF )
Stream {
    Token ( StringSingle )
    Token ( None )
    Token ( StringSingle )
}
Token ( EOF )
");
    }
    #[test]
    fn test_tokenizer_string_2 () {

        let token_stream = tokenize(
            b"'``'",
            &initialize_lookup()
        );

        // println!("{:?}", token_stream);
        println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream), 
"Token ( BOF )
Stream {
    Token ( StringSingle )
    Token ( None )
    Token ( StringSingle )
}
Token ( EOF )
");
    }
    #[test]
    fn test_tokenizer_string_3 () {

        let token_stream = tokenize(
            b"''``",
            &initialize_lookup()
        );

        // println!("{:?}", token_stream);
        println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream), 
"Token ( BOF )
Stream {
    Token ( StringSingle )
    Token ( None )
    Token ( StringSingle )
}
Stream {
    Token ( StringBackTick )
    Token ( None )
    Token ( StringBackTick )
}
Token ( EOF )
");
    }
    #[test]
    fn test_tokenizer_string_4 () {

        let token_stream = tokenize(
            b"''``\"\"",
            &initialize_lookup()
        );

        // println!("{:?}", token_stream);
        println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream), 
"Token ( BOF )
Stream {
    Token ( StringSingle )
    Token ( None )
    Token ( StringSingle )
}
Stream {
    Token ( StringBackTick )
    Token ( None )
    Token ( StringBackTick )
}
Stream {
    Token ( StringDouble )
    Token ( None )
    Token ( StringDouble )
}
Token ( EOF )
");
    }
    #[test]
    fn test_tokenizer_string_5 () {

        let token_stream = tokenize(
            b"'some stuff in the single quotes'`some other stuff in the double quotes`\"some more stuff in the back ticks\"",
            &initialize_lookup()
        );

        // println!("{:?}", token_stream);
        println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream), 
"Token ( BOF )
Stream {
    Token ( StringSingle )
    Token ( None )
    Token ( StringSingle )
}
Stream {
    Token ( StringBackTick )
    Token ( None )
    Token ( StringBackTick )
}
Stream {
    Token ( StringDouble )
    Token ( None )
    Token ( StringDouble )
}
Token ( EOF )
");
    }

    #[test]
    fn test_tokenizer_string_broken_string () {

        let token_stream = tokenize(
            b"'some stuff in the single quotes",
            &initialize_lookup()
        );

        // println!("{:?}", token_stream);
        println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream), 
"Token ( BOF )
Stream {
    Token ( StringSingle )
    Token ( None )
    Token ( EOF )
}
Token ( EOF )
");
    }
    
    #[test]
    fn test_tokenizer_string_broken_string_2 () {

        let token_stream = tokenize(
            b"\"some stuff in the double quotes",
            &initialize_lookup()
        );

        // println!("{:?}", token_stream);
        println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream), 
"Token ( BOF )
Stream {
    Token ( StringDouble )
    Token ( None )
    Token ( EOF )
}
Token ( EOF )
");
    }
    
    #[test]
    fn test_tokenizer_string_broken_string_3 () {

        let token_stream = tokenize(
            b"`some stuff in the back ticks",
            &initialize_lookup()
        );

        // println!("{:?}", token_stream);
        println!("{}", token_stream);

        assert_eq!(format!("{}", token_stream), 
"Token ( BOF )
Stream {
    Token ( StringBackTick )
    Token ( None )
    Token ( EOF )
}
Token ( EOF )
");
    }
}
