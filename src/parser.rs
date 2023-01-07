//! GOLD Parser
//! 
//! 


use std::fmt::{Display, write};
use std::str::FromStr;
use std::num::ParseIntError;

use crate::egt::EnhancedGrammarTable;
use crate::engine::LALRSateRecord;

// named!(int8 <&str, Result<i8, ParseIntError>>,
//     map!(number, FromStr::from_str));

pub fn get_header() {

}

#[derive(Debug)]
pub enum GPMessage {
    Empty,                   //Nothing
    TokenRead,               //A new token is read
    Reduction,               //A rule is reduced
    Accept,                  //Grammar complete
    NotLoadedError,          //Now grammar is loaded
    LexicalError,            //Token not recognized
    SyntaxError,             //Token is not expected
    GroupError,            //Reached the end of the file - mostly due to being stuck in comment mode
    InternalError,           //Something is wrong, very wrong
}

#[derive(Debug)]
enum GPParseResult {
    Undefined,
    Shift,
    Reduce,
    ReduceTrimmed,
    Accept,
    SyntaxError,
    InternalError,
}

#[derive(Debug)]
pub enum GOLDParserError {
    Format(GPMessage),
    ParseIntError(::std::num::ParseIntError),
    ParseFloatError(::std::num::ParseFloatError),
}
impl Display for GOLDParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write(f, "Line {}:{}", self.col, self.line);   
        ::std::fmt::Debug::fmt(&self, f)
    }
}

//#[derive(Debug)]
pub struct GOLDParser {
    grammar: EnhancedGrammarTable,

    input_stack: Vec<u16>,
    comment_level: usize,
    current_line: usize,

    curr_lalr_state: LALRSateRecord,
    trim_reductions: bool,
}


