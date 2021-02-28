use regex::Regex;

use crate::transpiler::types::{
    CompilationInfo, 
    CompilationInstructions, 
    CompileType, 
    Info
};
// use crate::parser::enclosing_pairs::{
//     find_enclosing_pair, 
//     split_from_indeces,
//     EnclosingSplit, 
//     EnclosingPairs
// };
// use crate::parser::parser::parser::{skip_for_character};



pub fn implement_switch () -> CompilationInstructions {
    CompilationInstructions {
        comp_type:    CompileType::Switch,
          check:      Box::new(check_switch),
          parse:      Box::new(parse_switch),
        transpile:    Box::new(transpile_switch)
    }
}

pub struct SwitchInfo {
    pub key_parenthesis: (usize, usize),
    pub surrounding_curlies: (usize, usize),
}


fn check_switch (data: &[u8], start_index: usize) -> bool {
    // If the data 
    data.len() - start_index >= 5 && 
    &data[start_index..start_index+6] == b"switch"
}

fn parse_switch <'a> (data: &'a [u8]) -> Option<CompilationInfo <'a>> {

    // // Index from which we will start searching for the opening parenthesis for the switch
    // //      key -- starts at 6 because "switch" is 6 characters, and indexes start at 0
    // let start_index = 6usize;

    // // Skip ahead over comments and whitespace, returns Some( index ) of
    // //      the opening parenthesis -- or None, if there is an invalid
    // //      character between 'switch' and '('
    // let parenthesis = skip_for_character (
    //     data, 
    //     start_index, 
    //     '(', 
    //     pairs_lookup
    // );

    // if let Some ( p_open_index ) = parenthesis {

    //     // Set parenthesis_open
    //     let parenthesis_open = p_open_index;
        
    //     // Getting the index of the closing parenthesis
    //     let parenthesis_close = find_enclosing_pair (
    //         data, 
    //         '(', 
    //         parenthesis_open, 
    //         pairs_lookup
    //     ).0;
    
    //     if parenthesis_close == 0 {
    //         // TODO: insert error message -- no closing parenthesis to 'switch(...'
    //         return None;
    //     }

    //     // Storing opening and closing parenthesis
    //     let key_parentheses = (
    //         parenthesis_open, 
    //         parenthesis_close
    //     );
    
    //     // Now, searching for opening / closing '{' '}' brackets
    //     let start_index = parenthesis_close + 1;
    
    //     // Skip ahead over comments and whitespace, returns Some( index ) of
    //     //      the opening curly bracket -- or None, if there is an invalid
    //     //      character between ')' and '{'
    //     let curly = skip_for_character (
    //         data, 
    //         start_index, 
    //         '{', 
    //         pairs_lookup
    //     );
    
    //     if let Some( c_open_index ) = curly {

    //          // Set parenthesis_open
    //         let curly_open = c_open_index;
            
    //         // Getting the index of the closing curly
    //         let curly_close = find_enclosing_pair (
    //             data, 
    //             '{', 
    //             curly_open, 
    //             pairs_lookup
    //         ).0;
        
    //         if curly_close == 0 {
    //             // TODO: insert error message -- no closing parenthesis to 'switch(...){...'
    //             return None;
    //         }

    //         // Storing opening and closing curly
    //         let surrounding_curlies = &(
    //             curly_open, 
    //             curly_close
    //         );

    //         let curly_split = split_from_indeces(data, *surrounding_curlies);


    //         let cases = curly_split.middle;


    //         // switch:
    //         //      ((case (.*):|default:)(\{.*\}|\[.*\]|\(.*\))*)


    //         // Info::SwitchInfo(SwitchInfo{ key_parenthesis: key_parentheses, surrounding_curlies: *surrounding_curlies});


    //         Some (CompilationInfo{ fjs_block: b"", remaining: data, comp_info: Info::None })
    //     }
    //     else {
    //         // TODO: insert error message -- invalid character between 'switch(...)' and '{'
    //         None
    //     }
    // }
    // else { 
    //     // TODO: insert error message -- invalid character between 'switch' and '('
    //     None 
    // }
    None
}

fn transpile_switch (data: &[u8], compilation_info: Info) -> &[u8] {
    b""
}

#[cfg(test)]
pub mod test {
    use super::check_switch;

    #[test]
    fn test_check_switch () {
        assert_eq!(check_switch(b"switch ", 0), true);
    }
    
    #[test]
    fn test_check_switch_2 () {
        assert_eq!(check_switch(b"swit", 0), false);
    }
    
    
    #[test]
    fn test_check_switch_3 () {
        assert_eq!(check_switch(b"some arbitrary stuff before the switch switch", 39), true);
    }
    
    
    #[test]
    fn test_check_switch_4 () {
        assert_eq!(check_switch(b"some arbitrary stuff before the switch switch and some arbitrary stuff after the switch", 39), true);
    }
    
}