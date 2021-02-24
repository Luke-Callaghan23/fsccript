
pub struct EnclosingPairs {
    pairs: [(char, char); 6]
}

pub fn initialize_enclosing_pairs () -> EnclosingPairs {
    EnclosingPairs {
        pairs: [
            ('(', ')'),
            ('{', '}'),
            ('[', ']'),
            ('\'', '\''),
            ('"', '"'),
            ('`', '`')
        ]
    }
}

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
    pub after: &'a [u8],
    pub pair: (char, char)
}

pub enum CommentType {
    Multiline,
    Singleline
}

pub fn find_end_comment (data: &[u8], comment_type: CommentType) -> usize {
    match comment_type {
        CommentType::Multiline => {
            // If it's a multiline comment, search for a * followed directly
            //      by a /
            let mut last_was_start = false;
            for (index, byte) in data.iter().enumerate() {
                let chr = *byte as char;
                
                if last_was_start && chr == '/' {
                    return index + 1;
                }
                else if chr == '*' {
                    last_was_start = true;
                }
                else {
                    last_was_start = false;
                }
            }
            0
        },
        CommentType::Singleline => {
            // If it's a single line comment, just search for \n
            let close = '\n';
            for (index, byte) in data.iter().enumerate() {
                let chr = *byte as char;
                if chr == close {
                    return index + 1;
                }
            }
            0
        }
    }
}

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
pub fn split_at_enclosing_pair <'a> (
    data: &'a [u8],
    starting_pair: char,
    start_index: usize,
    pairs_lookup: &EnclosingPairs
) -> Option<EnclosingSplit<'a>> {
    
    let (end, closing_pair) = find_enclosing_pair (
        data, 
        starting_pair, 
        start_index, 
        pairs_lookup
    );

    println!("{}",end);

    if end == 0 {
        return None
    }

    Some (EnclosingSplit {
        before: &data[0..start_index],
        middle: &data[start_index+1..end],
        after: &data[end+1..data.len()],
        pair: (starting_pair, closing_pair)
    })
}


pub fn is_string_pair (chr: char) -> bool {
    chr == '"' || 
    chr == '`' || 
    chr == '\''
}


fn find_closing_pair (chr: char, pairs_lookup: &EnclosingPairs) -> char {
    pairs_lookup.pairs.iter().find_map(| (start, end) | 
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
    let closing_pair = find_closing_pair(starting_pair, pairs_lookup);

    // println!("{}", closing_pair);

    // This function behaves differently if the pair we're looking for
    //      if a string delimiter, or not, so we first get that 
    let is_str = is_string_pair(starting_pair);
    
    let mut stack = if is_str { 1 } else { 0 };   // stack is the stack of opening and closing pairs -- starts at 1 for strings, 0 for everything else
    let mut skip  = false;                       // boolean to indicate if the last character in a string was a backslash (meaning, skip the next character)
    
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

        if !is_str {

            if is_string_pair(chr) {
                // If the character that we find is a string character,
                //      we have to skip over the string, so we recurse
                //      on this function with the enclosing string as
                //      target, and increment the index to the character
                //      after the closing pair for the target string
                index += 1 + find_enclosing_pair (
                    &data[index..],
                    chr,
                    0,
                    pairs_lookup
                ).0;
                continue;
            }
            else {
                // If it's the starting pair, increment the stack,
                //      and if it's the closing pair, decrement the
                //      stack
                if chr == starting_pair { stack += 1; }
                else if chr == closing_pair  { stack -= 1; }
            }
        }
        else {
            skip = 
                if chr == closing_pair  {
                    // If the pair we're looking for is a string, we only need one enclosing,
                    //      so if we find it and last character was not a backslash, we just
                    //      set the stack to 0
                    if !skip && index != 0 { stack = 0; }
                    false
                }
                else if chr == '\\' { !skip }       // if we find a back slash, flip skip -- (we flip, and not just set to false because two backslashes in a row cancel each other)
                else { false };                     // any other character, just flip skip
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