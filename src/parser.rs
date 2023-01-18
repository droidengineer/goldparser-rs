//! GOLD Parser
//! 
//! 


use std::collections::HashMap;
use std::fmt::{Display, write};
use std::{fs, default};
use std::path::PathBuf;
use std::str::FromStr;
use std::num::ParseIntError;

use crate::builder::Builder;
use crate::egt::EnhancedGrammarTable;
use crate::engine::states::ActionType;
use crate::engine::{LALRState, Stack, Position, Symbol, SymbolType, DFAState, Value};
use crate::engine::tables::{CharacterSetTable,DFAStateTable,SymbolTable,LRStateTable,ProductionTable, GroupTable, Table};
use crate::engine::token::{Token};
use crate::source::SourceReader;


pub trait GPParser {
    /// Load the grammar EGT 5.0
    fn load_grammar(grammar: String) -> EnhancedGrammarTable {
        let file = PathBuf::from(grammar);
        let mut bldr = Builder::new(file.into_os_string());
        bldr.init();
        bldr.to_egt()
    }

    /// Read the your source code to be parsed into a string buffer
    fn load_source(source: String) -> String {
        fs::read_to_string(source)
            .expect("Unable to read {source}")
    }

    /// Performs a parse action on the input source. This should continue until the grammar
    /// is accepted or an error occurs. See `parse_step()`
    fn parse(&mut self) -> GPMessage;

    /// If you need something custom start here and have `parse()` call it
    fn parse_step(&mut self) -> GPMessage;

    /// Analyzes a `Token` and either:
    /// 1. Makes a single reduction and pushes a complete `Reduction` object on the stack
    /// 2. Accepts the Token and shifts
    /// 3. Errors and places the expected symbol indices in the Tokens list
    fn input_tokens(&mut self, next_Token: &mut Token) -> GPParseResult;

    /// Implements the lookahead DFA for the parser's lexer. A `Token` is generated which is used by the
    /// LALR state machine. Takes into account the lexing mode of the parser.
    /// This version uses a `Stack` to manage nested group elements.
    fn next_Token(&mut self) -> Token;

    /// Returns `count` characters in a string from the lookahead buffer.
    /// These characters are used to create the text stored in a `Token`
    /// `count` should never exceed buffer length
    fn lookahead(&self, count: usize) -> String;

    /// Resets the parser to the initial state. The loaded tables are retained.
    /// After this call, the parser will bere ady to start parsing.
    fn reset(&mut self);

    /// Has the parser been initialized? Tables loaded? Source loaded?
    fn is_initialized(&self) -> bool;


    fn version(&self) -> String;


}

#[derive(Debug,Default)]
pub enum GPMessage {
    #[default]
    Empty,                   //Nothing
    TokenRead,               //A new Token is read
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
    pub source: SourceReader,

    pub properties: HashMap<String,String>,
    
    /// Symbols recognized by the system
    //pub symbols: SymbolTable, // from grammar.symbols

    /// DFA Tokenizer/scanner/lexer
    //pub dfa_states: DFAStateTable, // from grammar.dfa_states as Vec<DFAState>
    //pub charsets: CharacterSetTable, // from grammar.charset
    //lookahead_buf: String,

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
    sys_pos: Position,

}

impl GOLDParser {
    /// Move the grammar loading into new()
    pub fn new(egt: String) -> Self {
        // let egt_file = PathBuf::from(egt);
        // let mut egt_bldr = Builder::new(egt_file.into_os_string());
        // egt_bldr.init();
        // let grammar = egt_bldr.to_egt();
        let grammar = Self::load_grammar(egt);
        //let source = Self::load_source(src);

        let mut properties: HashMap<String,String> = HashMap::new();
        for rec in &grammar.properties {
            properties.insert(rec.name.to_string(), rec.value.to_string());
        }
        GOLDParser {
            grammar,
            source: Default::default(),
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
            sys_pos: Position::default(),
            //lookahead_buf: String::new(),
        }
    }

    #[inline(always)]
    pub fn col(&self) -> usize { self.curr_position.col() }
    #[inline(always)]
    pub fn line(&self) -> usize { self.curr_position.line() }

    pub fn about(&self) -> String {
        self.properties.get("About").expect("No About property").to_string()
    }

    fn get_dfa_state(&self, index: usize) -> &DFAState {
        &self.grammar.dfa_states[index]
    }
    fn get_lalr_state(&self, state: usize) -> &LALRState {
        &self.grammar.lalr_states[state]
    }

    /// Return a single character at `index`. This method will read and fill the
    /// buffer as needed from the `source` buffer.
    fn lookahead(&mut self, index: usize) -> char {
        self.source.lookahead(index)
    }

    fn load_source(&mut self, source: String)  {
        let src = <GOLDParser as GPParser>::load_source(source);
        self.source.load(src);
        self.initialized = true;
    }
    // pub fn load_source(&mut self, source: String) {
    //     self.source = fs::read_to_string(source)
    //         .expect("Unable to read {source}");
    //     self.initialized = true;
    // }

    pub fn symbol_by_name(&self, name: String) -> Option<&Symbol> {
        self.grammar.symbols.get(name)
    }
    pub fn symbol_by_type(&self, kind: SymbolType) -> Option<&Symbol> {
        self.grammar.symbols.get_by_type(kind)
    }

}

impl GPParser for GOLDParser {
    fn parse(&mut self) -> GPMessage {
        if !self.initialized { return GPMessage::NotLoadedError; }
       
        let mut done: bool = false;
        let mut result = GPMessage::Empty;

        while !done {
            // DFA lexer provides a Token
            result = self.parse_step();
            match result {
                // GPMessage::Empty => todo!(),
                // GPMessage::TokenRead => todo!(),
                // GPMessage::Reduction => todo!(),
                GPMessage::Accept => {
                    done = true;
                },
                GPMessage::NotLoadedError => todo!(),
                GPMessage::LexicalError => {
                    println!("{:?} Lexical Error",self.curr_position);
                    done = true;
                },
                GPMessage::SyntaxError => {
                    println!("{:?} Syntax error. Expected {}",self.curr_position,self.expected_symbols.to_string());
                    done = true;
                },
                GPMessage::GroupError => {
                    println!("{:?} Runaway group.",self.curr_position);
                    done = true;
                },
                GPMessage::InternalError => {
                    println!("{:?} Internal error.",self.curr_position);
                    done = true;
                },
                // all non-error events are handled by calling event procedures
                _ => { }
            }
        }
        result
    }

    fn parse_step(&mut self) -> GPMessage {
        let mut result = GPMessage::default();
        let mut done = false;

        while !done {
            if self.input_tokens.len() == 0 { // get next Token from DFA lexer
                let Token = self.next_Token();
                let kind = Token.kind().clone();
                self.input_tokens.push(Token);

                // handle case where an unterminated comment block consumes program
                if kind == SymbolType::EndOfFile && self.group.is_empty() {
                    result = GPMessage::GroupError;
                } else { // a good Token was read
                    result = GPMessage::TokenRead;
                }
                done = true;
            
            } else { // a Token is present and can be parsed
                let mut Token = self.input_tokens.peek().clone();
                let kind = Token.kind();
                self.curr_position = Token.pos;

                match kind {
                    SymbolType::Noise => {  // whitespace and other ignorables
                        self.input_tokens.pop();
                    },
                    SymbolType::EndOfFile => { 
                        if !self.group.is_empty() { // runaway group
                            result = GPMessage::GroupError;
                            done = true;
                        }
                    },
                     SymbolType::Error => {
                        result = GPMessage::LexicalError;
                        done = true;
                    },
                    _ => {  // LALR parsing of the input Token
                        let parsemsg = self.input_tokens(&mut Token);
                        match parsemsg {


                            _ => { // fallthru includes reduce/eliminated
                                   // shift, and trim-reduced
                                   // do nothing
                            },
                        }


                    },
                }

            }
        }

        result
    }

    fn input_tokens(&mut self, next_Token: &mut Token) -> GPParseResult {
        let mut result = GPParseResult::Undefined;
        self.have_reduction = false;
        let parent_symbol = &next_Token.symbol;
        let parse_action = self.get_lalr_state(self.curr_state)
                                            .find_action(parent_symbol)
                                            .expect("Problems fetching LALRAction from LALRState");
        match parse_action.action {
            ActionType::Reduce => {
                // This section of the algorithm will reduce the rule specified by action.action
                // Produce a reduction - remove as many Tokens as members in the rule and push
                // a non-terminal Token
                let rule = &self.grammar.productions[parse_action.target_idx];
                let mut head = Token::default();
                if self.trim_reductions && rule.has_only_nonterminal() {
                    // The current rule consists of a single non-terminal and can be trimmed from
                    // the parse tree
                    head = self.stack.pop();
                    head.symbol = rule.head();
                    result = GPParseResult::ReduceTrimmed;
                } else { // create a new reduction for the current rule
                    self.have_reduction = true;
                    let n = rule.symbols.len();
                    let mut reduce_tokens: Vec<Token> = vec![];
                    for i in (n-1)..0 {
                        reduce_tokens[i] = self.stack.pop();
                    }
                    head = Token::new(rule.head(), String::default());
                    head.data = Value::Reduction(reduce_tokens);
                    result = GPParseResult::Reduce;
                }
                // execute GOTO action for the rule that was just reduced
                // peek at LALR Token stack state to get its index, look the state up,
                // and find the action corresponding to the rule's head symbol
                let state_index = self.stack.peek().state();
                match self.get_lalr_state(state_index)
                                      .find_action(&rule.head())
                                      .cloned()
                {
                    Some(action) => {
                        self.curr_state = action.target_idx;
                        head.lalr_state = action.target_idx;
                        self.stack.push(head);
                    }
                    None => result = GPParseResult::InternalError,
                }

            },
            ActionType::Accept => {
                self.have_reduction = true;
                result = GPParseResult::Accept;
            },
            ActionType::Shift  => {
                // Shift to target state and push the current Token.
                self.curr_state = parse_action.target_idx;  //self.get_lalr_state(parse_action.target_idx);
                next_Token.lalr_state = self.curr_state;
                self.input_tokens.push(next_Token.clone());
                result = GPParseResult::Shift;
            },
            ActionType::Undefined |
            ActionType::Goto  => {
                // Syntax error. Generate a list of expected symbols to report
                self.expected_symbols.clear();
                let lrstate = self.get_lalr_state(self.curr_state).actions.clone();

                for action in lrstate {
                    if action.action == ActionType::Shift {
                        
                        self.expected_symbols.add(action.symbol.clone());
                    }
                }
                result = GPParseResult::SyntaxError;
            },
        }

        result
    }

    fn next_Token(&mut self) -> Token {
        let mut Token = Token::default();
        let mut curr_state = self.grammar.initial_states.dfa as usize;
        let mut length = 1;
        let mut last_accept_state: i32 = -1;
        let mut last_accept_pos: i32 = -1;
        let mut target = 0;
        let mut done = false;

        while !done {
            //if let ch = self.lookahead(length) {
            let ch = self.lookahead(length);
            // Checks whether an edge was found from the `curr_state`. If so, the state and
            // `curr_pos` advances. Else, quit main loop and report Token found. If the
            // last_accept_state is -1, then no match found and the Error Token is created.
            match self.get_dfa_state(curr_state).find_edge(ch) {
                // Checks whether the target state accepts a Token. If so, it sets the
                // appropriate variables so when the algorithm is done, it can return the
                // proper Token and number of characters
                Some(index) => { 
                    target = index as i32;
                    if self.get_dfa_state(index).accept {
                        last_accept_state = target;
                        last_accept_pos = length as i32;
                    }
                    curr_state = target as usize;
                    length += 1;
                },
                None => { // no edge found. no target state found.
                    if last_accept_state == -1 { // Lexer doesn't recognize the symbol
                        Token.symbol = self.symbol_by_type(SymbolType::Error).unwrap().clone();
                        Token.text = <GOLDParser as GPParser>::lookahead(self,1);
                    } else { // create Token and read text for Token.
                        // self.text contains the total number of accept characters
                        Token.symbol = self.get_dfa_state(last_accept_state as usize).accept_symbol.clone();
                        Token.text = <GOLDParser as GPParser>::lookahead(&self, last_accept_pos as usize);
                    }
                    done = true;
                }
            }

            
        }
        Token.pos = self.source.pos;
        Token
    }

    fn lookahead(&self, count: usize) -> String {
        let mut ahead = count;
        if ahead > self.source.buf.len() { ahead = self.source.buf.len(); }
        self.source.buf.as_str()[0..ahead].to_string()
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }
    fn reset(&mut self) {
        self.source.clear();
        //self.lookahead_buf.clear();
        //self.properties.clear();
        self.curr_state = self.grammar.initial_states.lalr as usize;
        self.stack.clear();
        self.group.clear();
        // TODO self.groups.clear()
        self.expected_symbols.clear();
        self.have_reduction = false;
        self.initialized = false;
        self.input_tokens.clear();
        self.curr_position.clear();
        self.sys_pos.clear();
    }
    fn version(&self) -> String {
        format!("{} {}",self.properties.get("Name").unwrap(), self.properties.get("Version").unwrap() )
    }

}









#[cfg(test)]
pub mod test {
    use super::GOLDParser;


#[test]
fn new() {
    let parser = GOLDParser::new(r"D:\Users\Gian\prog\repos\RUST\goldparser-rs\.ref\goldparser-test.egt".to_string());
    println!("About:\n{}",parser.about());
    assert_eq!(parser.grammar.property("Name"),"BADASS");

}



}