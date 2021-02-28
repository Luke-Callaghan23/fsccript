use super::parser::parser::skip_string;


pub type EnclosingPairs =  [(char, char); 6];

pub fn initialize_enclosing_pairs () -> EnclosingPairs {[
    ('(', ')'),
    ('{', '}'),
    ('[', ']'),
    ('\'', '\''),
    ('"', '"'),
    ('`', '`')
]}

/// # EnclosingSplit struct
/// 
/// Describes three sections of raw data split with data:
/// before, after, and in the middle of two enclosing
/// pairs of charactes
/// 
/// # Example -- 
/// 
/// This string: "hello (fscript) world"
/// 
/// Would be described like:
/// 
/// ``` 
/// EnclosingSplit {
///     before: "hello ",
///     middle: "fscript"
///     after:  " world",
///     pairs: ('(', ')')
/// }
/// ```
/// In an EnclosingSplit struct split on parenthesis '('
#[derive(Debug, PartialEq, Eq)]
pub struct EnclosingSplit <'a> {
    pub before: &'a [u8],
    pub middle: &'a [u8],
    pub after:  &'a [u8],
    pub pair: (char, char)
}

enum CommentType {
    Multiline,
    Singleline
}

enum StringType {
    Single,
    Double,
    BackTick
}

// fn find_end_comment (data: &[u8], comment_type: CommentType) -> usize {
//     match comment_type {
//         CommentType::Multiline => {
//             // If it's a multiline comment, search for a * followed directly
//             //      by a /
//             let mut last_was_start = false;
//             for (index, byte) in data.iter().enumerate() {
//                 let chr = *byte as char;
                
//                 if last_was_start && chr == '/' {
//                     return index;
//                 }
//                 else if chr == '*' {
//                     last_was_start = true;
//                 }
//                 else {
//                     last_was_start = false;
//                 }
//             }
//             0
//         },
//         CommentType::Singleline => {
//             // If it's a single line comment, just search for \n
//             let close = '\n';
//             for (index, byte) in data.iter().enumerate() {
//                 let chr = *byte as char;
//                 if chr == close {
//                     return index;
//                 }
//             }
//             0
//         }
//     }
// }

// /// # try_skip_string
// ///
// /// Function to skip over all a .js comment contained in a byte string, starting at a given offset, start_index,
// ///     and return the last index of string immediately after start_index
// /// 
// /// # Parameters -- 
// /// * `data:         &[u8]`     -- byte string data
// /// * `start_index:  usize`     -- beginning offset of the byte string to start skipping
// /// 
// /// # Return --  
// /// * `usize`                   -- index of the target character, if there are no invalid characters, or None, if there are
// pub fn try_skip_comment (data: &[u8], c_one: char, start_index: usize) -> usize {
//     let target = &data[start_index..];
//     if c_one == '/' && target.len() > 1 {
        
//         // If the first character is a  '/', then we can check the second character
//         let c_two = target[1] as char;

//         if c_two == '/' {
//             // single line comments -- //
//             find_end_comment(target, CommentType::Singleline)
//         }
//         else if c_two == '*' {
//             // multiline comments -- /*
//             find_end_comment(target, CommentType::Multiline)
//         }
//         // Otherwise, 0
//         else { 0 }
//     }
//     // Otherwise, 0 
//     else { 0 }
// }


// fn find_end_string (data: &[u8], string_type: char) -> usize {
//     let mut skip = false;
//     for (index, byte) in data[1..].iter().enumerate() {
//         let chr = *byte as char;
//         // println!("{}", chr);
//         // println!("{}", skip);
//         // println!("{}", chr==string_type);
//         if chr == string_type && !skip {
//             // If we find the closing pair ast character was not a backslash, we just
//             //      return the index
//             return index + 1;
//         }
//         else if chr == '\\' { skip = !skip; }       // if we find a back slash, flip skip -- (we flip, and not just set to false because two backslashes in a row cancel each other)
//         else { skip = false; };                     // any other character, just flip skip
//     }
//     0
// }

// /// # try_skip_string
// ///
// /// Function to skip over all a .js string contained in a byte string, starting at a given offset, start_index,
// ///     and return the last index of string immediately after start_index
// /// 
// /// # Parameters -- 
// /// * `data:         &[u8]`     -- byte string data
// /// * `start_index:  usize`     -- beginning offset of the byte string to start skipping
// /// 
// /// # Return --  
// /// * `usize`                   -- index of the target character, if there are no invalid characters, or None, if there are
// pub fn try_skip_string (data: &[u8], c_one: char, start_index: usize) -> usize {
//     let target = &data[start_index..];
//     if is_string_pair(c_one) {
//         find_end_string(target, c_one)
//     }
//     else { 0 }
// }

// /// # try_skip_whitespace
// ///
// /// Function to skip over all whitespace in a byte string, starting at a given offset, start_index,
// ///     and return the last index of whitespace immediately after start_index
// /// 
// /// # Parameters -- 
// /// * `data:         &[u8]`     -- byte string data
// /// * `start_index:  usize`     -- beginning offset of the byte string to start skipping
// /// 
// /// # Return --  
// /// * `usize`                   -- index of the target character, if there are no invalid characters, or None, if there are
// pub fn try_skip_whitespace (data: &[u8], c_one: char, start_index: usize) -> usize { 
//     let mut chr = c_one;
//     let mut end_index = start_index;
//     while chr.is_whitespace() && start_index < data.len() {
//         end_index += 1;
//         chr = data[start_index] as char;
//     }
//     if end_index != start_index {
//         end_index
//     }
//     else { 0 }
    
// }


/// # EnclosingSplit split_at_enclosing_pair
/// 
/// Splits an input data string on an eclosing pair of characters
/// 
/// # Parameters -- 
///
/// * data: &'a [u8] -- a string of bytes to be split by this function
/// * starting_pair: char -- the opening character of the enclosing pair being search
/// * start_index: usize -- the index of the beginning part of the string to start searching
/// * pairs_lookup: &EnclosingPairs -- lookup table of chracter pairs
/// 
/// 
/// # Returns -- 
///
/// An EnclosingSplit struct that describes the split
///
/// # Example -- 
/// 
/// This string: "hello (fscript) world"
/// 
/// Would be split like:
/// 
/// 
/// "hello ", "fscript", "world"
/// 
/// 
/// And would yield an EnclosingSplit struct:
/// 
/// ``` 
/// 
/// EnclosingSplit {
///     before: "hello ",
///     middle: "fscript"
///     after:  " world",
///     pairs: ('(', ')')
/// }
/// ```
// pub fn split_at_enclosing_pair <'a> (
//     data: &'a [u8],
//     starting_pair: char,
//     start_index: usize,
//     pairs_lookup: &EnclosingPairs
// ) -> Option<EnclosingSplit<'a>> {
    
//     let (end, closing_pair) = find_enclosing_pair (
//         data, 
//         starting_pair, 
//         start_index, 
//         pairs_lookup
//     );

//     println!("{}",end);

//     if end == 0 {
//         return None
//     }

//     Some (EnclosingSplit {
//         before: &data[0..start_index],
//         middle: &data[start_index+1..end],
//         after: &data[end+1..data.len()],
//         pair: (starting_pair, closing_pair)
//     })
// }


pub fn split_from_indeces <'a> (data: &'a [u8], pair: (usize, usize)) -> EnclosingSplit<'a> {
    // Getting starting / ending indeces
    let (start, end) = pair;
    
    // Getting pair characters
    let pair = (
        data[start] as char, 
        data[end] as char
    );

    // Creating the enclosing split
    EnclosingSplit {
        before: &data[0..start],
        middle: &data[start+1..end],
        after:  &data[end+1..data.len()],
        pair:   pair
    }
}

fn is_string_pair (chr: char) -> bool {
    chr == '"' || 
    chr == '`' || 
    chr == '\''
}


fn find_partner (chr: char, pairs_lookup: &EnclosingPairs) -> char {
    pairs_lookup.iter().find_map(| (start, end) | 
        if *start == chr { Some(*end) } 
        else { None }
    ).unwrap()
}


pub fn find_enclosing_pair (
    data: &[u8], 
    starting_pair: char, 
    start_index: usize, 
    pairs_lookup: &EnclosingPairs
) -> (usize, char) {
    
    // print!("in data: {}", str::from_utf8(data).unwrap());

    // Finding the ending pair that we're searching for
    let closing_pair = find_partner(starting_pair, pairs_lookup);

    // println!("{}", closing_pair);

    // If it's a string, pair then
    let string_end = skip_string(data, start_index);
    // println!("String end: {}", string_end);
    if string_end != 0 {
        return (string_end, starting_pair);
    }


    let mut stack = 0usize;   // stack is the stack of opening and closing pairs -- starts at 1 for strings, 0 for everything else

    // Getting a slice of the data starting from start_index
    let data = &data[start_index..];
    
    let mut index = 0;

    // If the starting index is not the correct character, then move 
    //      it up until it is
    let mut start = data[index] as char;
    while start != starting_pair && index < data.len() { 
        index += 1;
        start = data[index] as char;
    }

    while index < data.len() {
        
        // Current character in the string
        let chr = data[index] as char;

        // println!("{}: {}", index, chr);

       
        if is_string_pair(chr) {
            // If the character that we find is a string character,
            //      we have to skip over the string, so we recurse
            //      on this function with the enclosing string as
            //      target, and increment the index to the character
            //      after the closing pair for the target string
            let string_end = skip_string (
                data,
                index,
            );

            println!("string pair end: {}", string_end);

            if string_end != 0 {
                index += string_end + 1;
            }
            else {
                return (0, ' ');
            }
            continue;
        }
        else {
            // If it's the starting pair, increment the stack,
            //      and if it's the closing pair, decrement the
            //      stack
            if chr == starting_pair { stack += 1; }
            else if chr == closing_pair  { stack -= 1; }
        }
        
        
        
        if stack == 0 {
            // If the stack is 0, return the current index 
            //      plus the start index
            return (index + start_index, closing_pair);
        }

        index += 1;

    }
    (0, ' ')   // Return 0 if no pairs were found
}