//! This is the primary interface to the GOLDParser Engine.

use std::fs;
use std::collections::HashMap;

use super::{prelude::*, 
    Token, DFAState, EnhancedGrammarTable, LALRState, Rule, SourceReader, SymbolTable
};

const GP_NAME: &str = "GOLD Parser Engine for RUST";
const GP_VERSION: &str = "5.0.3";

#[derive(Debug,Default,PartialEq)]
pub enum ParseMessage {
    TokenRead = 0,
    Reduction,
    Accept,
    NotLoadedError,
    LexicalError,
    SyntaxError,
    GroupError,
    #[default]
    InternalError,
}

pub enum ParseResult {
    Accept = 1,
    Shift,
    ReduceNormal,
    ReduceEliminated,
    SyntaxError,
    InternalError,
}

#[derive(Debug)]
pub enum GParserError {
    Format(ParseMessage),
    ParseIntError(::std::num::ParseIntError),
    ParseFloatError(::std::num::ParseFloatError),
}
impl Display for GParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParserError {:?}", self)  
    }
}

pub trait GOLDParser {
    /// Load the grammar EGT 5.0
    fn load_grammar_tables(grammar: String) -> EnhancedGrammarTable {
        let file = std::path::PathBuf::from(grammar);
        Builder::new(file.into_os_string()).to_egt()
    }
    /// Read the source code to be parsed into a string buffer
    fn load_src_as_string(source: String) -> Result<String, GParserError> {
        match fs::read_to_string(source) {
            Ok(s) => Ok(s),
            Err(e) => Err(GParserError::Format(ParseMessage::NotLoadedError))
          //.expect("Unable to read {source}")
        }
    }
    /// The tables must be loaded and source file loaded before any tokenizing and parsing
    /// can occur.
    fn is_initialized(&self) -> bool;

    /// Performs a parse action on the input source. This should continue until the grammar
    /// is accepted or an error occurs. See `parse_step()`
    /// The grammar source must be loaded before this call.
    fn parse(&mut self) -> ParseMessage {
        //if !self.is_initialized() { return GPMessage::NotLoadedError; } 

        let mut result = ParseMessage::default();

        while result != ParseMessage::Accept {
            result = self.parse_step();
        }

        result
    }

    /// If you need something custom start here and have `parse()` call it
    fn parse_step(&mut self) -> ParseMessage;

    /// Analyzes a `Token` and either:
    /// 1. Makes a single reduction and pushes a complete `Reduction` object on the stack
    /// 2. Accepts the Token and shifts
    /// 3. Errors and places the expected symbol indices in the Tokens list
    fn parse_token(&mut self, input_tokens: &mut Token) -> ParseResult;

    /// Implements the lookahead DFA for the parser's lexer. A `Token` is generated which is used by the
    /// LALR state machine. Takes into account the lexing mode of the parser.
    /// This version uses a `Stack` to manage nested group elements.
    fn next_token(&mut self) -> Token;

    /// Returns `count` characters in a `&str` from the lookahead buffer. DO NOT CONSUME
    /// These characters are used to create the text stored in a `Token`
    /// `count` should never exceed buffer length
    fn lookahead_buffer(&self, count: usize) -> &str;

    /// Return single char at the index. If not present in the lookahead buffer, the
    /// source stream will preemptively be read and added to the buffer.
    fn lookahead(&self, index: usize) -> char;



    fn grammar_version(&self) -> &str {"GOLD Parser Tables/v5.0"}
    fn about(&self) -> String {
        format!("{} - Version {}",GP_NAME, GP_VERSION)
    }
    

}