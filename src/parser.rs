//! GOLD Parser
//! 
//! 


use std::collections::HashMap;
use std::fmt::{Display, write};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::num::ParseIntError;

use crate::builder::Builder;
use crate::egt::EnhancedGrammarTable;
use crate::engine::{LALRState, Stack, Position, Symbol};
use crate::engine::tables::{CharacterSetTable,DFAStateTable,SymbolTable,LRStateTable,ProductionTable, GroupTable, Table};
use crate::engine::token::Token;


pub trait GPParser {
    /// Performs a parse action on the input source. This should continue until the grammar
    /// is accepted or an error occurs. See `parse_step()`
    fn parse(&mut self) -> GPMessage;

    /// If you need something custom start here and have `parse()` call it
    fn parse_step(&mut self) -> GPParseResult;

    /// Analyzes a `Token` and either:
    /// 1. Makes a single reduction and pushes a complete `Reduction` object on the stack
    /// 2. Accepts the token and shifts
    /// 3. Errors and places the expected symbol indices in the tokens list
    fn parseToken(&mut self, next_token: Token) -> GPParseResult;

    /// Implements the DFA for the parser's lexer. A `Token` is generated which is used by the
    /// LALR state machine. Takes into account the lexing mode of the parser.
    fn nextToken(&mut self) -> Token;

    ///
    fn lookahead(&self) -> Token;
}

#[derive(Debug)]
pub enum GPMessage {
    Empty,                   //Nothing
    TokenRead,               //A new token is read
    Reduction,               //A rule is reduced
    Accept,                  //Grammar complete
    NotLoadedError,          //No grammar is loaded
    LexicalError,            //Token not recognized
    SyntaxError,             //Token is not expected
    GroupError,              //Reached the end of the file - mostly due to being stuck in comment mode
    InternalError,           //Something is wrong, very wrong
}

#[derive(Debug)]
pub enum GPParseResult {
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
        write!(f, "ParserError {:?}", self)  
        //::std::fmt::Debug::fmt(&self, f)
    }
}

//#[derive(Debug)]
/// This is the main class in the GOLD Parser Engine and is used to perform
/// all duties required to the parsing of a source text string. This class
/// contains the LALR(1) State Machine code, the DFA State Machine code,
/// character table (used by the DFA algorithm) and all other structures and
/// methods needed to interact with the developer.
pub struct GOLDParser {
    /// The grammar the source is written against
    pub grammar: EnhancedGrammarTable,
    pub source: String,

    pub properties: HashMap<String,String>,
    
    /// Symbols recognized by the system
    //pub symbols: SymbolTable, // from grammar.symbols

    /// DFA tokenizer/scanner/lexer
    //pub dfa_states: DFAStateTable, // from grammar.dfa_states as Vec<DFAState>
    //pub charsets: CharacterSetTable, // from grammar.charset
    lookahead_buf: String,

    // Rules
    //pub rules: ProductionTable, // from grammar.rules

    // LALR parser
    //pub lalr_states: LRStateTable, // from grammar.lalr_states as Vec<LALRState>
    pub curr_state: usize,
    //curr_lalr_state: LALRState,    
    pub stack: Stack<Token>,

    // Lexical groups
    group: Stack<Token>,
    groups: GroupTable,

    // Reductions
    expected_symbols: SymbolTable,
    pub have_reduction: bool,
    trim_reductions: bool,

    // Housekeeping
    initialized: bool,
    /// Tokens to be analyzed
    input_tokens: Stack<Token>,
    /// Location of last read terminal
    curr_position: Position,

    comment_level: usize,
    current_line: usize,

}

impl GOLDParser {
    /// Move the grammar loading into new()
    pub fn new(egt: String) -> Self {
        let egt_file = PathBuf::from(egt);
        let mut egt_bldr = Builder::new(egt_file.into_os_string());
        egt_bldr.init();
        let grammar = egt_bldr.to_egt();

        let mut properties: HashMap<String,String> = HashMap::new();
        for rec in &grammar.properties {
            properties.insert(rec.name.to_string(), rec.value.to_string());
        }
        GOLDParser {
            grammar,
            source: String::new(),
            properties,
            curr_state: 0,
            stack: Stack::new(),
            group: Stack::new(),
            groups: GroupTable::new(),
            expected_symbols: SymbolTable::new(),
            have_reduction: false,
            trim_reductions: false,
            initialized: false,
            input_tokens: Stack::new(),
            curr_position: Position::default(),
            lookahead_buf: String::new(),
            comment_level: 0,
            current_line: 0
        }
    }

    #[inline(always)]
    pub fn col(&self) -> usize { self.curr_position.col() }
    #[inline(always)]
    pub fn line(&self) -> usize { self.curr_position.line() }

    /// Return a single character at `index`. This method will read and fill the
    /// buffer as needed from the `source` buffer.
    fn lookahead(&mut self, index: usize) -> String {
        assert!(index >= 0);


        todo!()
    }

    pub fn load_source(&mut self, source: String) {
        self.source = fs::read_to_string(source)
            .expect("Unable to read {source}");
        self.initialized = true;
    }

    pub fn symbol_by_name(&self, name: String) -> Option<&Symbol> {
        self.grammar.symbols.get(name)
    }
}



#[cfg(test)]
pub mod test {
    use super::GOLDParser;


#[test]
fn new() {
    let parser = GOLDParser::new(r"D:\Users\Gian\prog\repos\RUST\goldparser-rs\.ref\goldparser-test.egt".to_string());

    assert_eq!(parser.grammar.property("Name"),"BADASS");

}



}