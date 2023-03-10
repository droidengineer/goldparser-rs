//! Parser
//! 
//! Defines `GPParser` trait, as well as the messages that are passed during lexing and parsing.


use std::collections::HashMap;
use std::fmt::{Display};
use std::{fs};
use std::path::PathBuf;

use super::egt::EnhancedGrammarTable;
use super::reduction::Reduction;
use crate::engine::states::ActionType;
use crate::engine::{LALRState, Stack, Position, Symbol, SymbolType, DFAState, reduction};
use crate::engine::tables::{GroupTable, Table};
use crate::engine::token::{Token};
use super::source::SourceReader;
use super::{Builder, SymbolTable};

/// Trait for exposing granular parsing methods
pub trait GPParser {
    /// Load the grammar EGT 5.0
    fn load_grammar(grammar: String) -> EnhancedGrammarTable {
        let file = PathBuf::from(grammar);
        let mut bldr = Builder::new(file.into_os_string());
        bldr.init();
        bldr.to_egt()
    }

    /// Read the your source code to be parsed into a string buffer
    fn load_source(source: String) -> Result<String, ParserError> {
        match fs::read_to_string(source) {
            Ok(s) => Ok(s),
            Err(e) => Err(ParserError::Format(GPMessage::NotLoadedError))
          //  .expect("Unable to read {source}")
        }
    }

    /// Performs a parse action on the input source. This should continue until the grammar
    /// is accepted or an error occurs. See `parse_step()`
    /// The grammar source must be loaded before this call.
    fn parse(&mut self) -> GPMessage;

    /// If you need something custom start here and have `parse()` call it
    fn parse_step(&mut self) -> GPMessage;

    /// Analyzes a `Token` and either:
    /// 1. Makes a single reduction and pushes a complete `Reduction` object on the stack
    /// 2. Accepts the Token and shifts
    /// 3. Errors and places the expected symbol indices in the Tokens list
    fn parse_token(&mut self, input_tokens: &mut Token) -> GPParseResult;

    /// Implements the lookahead DFA for the parser's lexer. A `Token` is generated which is used by the
    /// LALR state machine. Takes into account the lexing mode of the parser.
    /// This version uses a `Stack` to manage nested group elements.
    fn input_token(&mut self) -> Token;

    /// Returns `count` characters in a `&str` from the lookahead buffer.
    /// These characters are used to create the text stored in a `Token`
    /// `count` should never exceed buffer length
    fn lookahead(&self, count: usize) -> &str;

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
pub enum ParserError {
    Format(GPMessage),
    ParseIntError(::std::num::ParseIntError),
    ParseFloatError(::std::num::ParseFloatError),
}
impl Display for ParserError {
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
pub struct Parser {
    /// The grammar the source is written against stored in a compiled
    /// binary called the *Enhanced Grammar Table*
    pub grammar: EnhancedGrammarTable,
    /// `SourceReader` responsible for opening the source file and maintaining
    /// the lookahead buffer.
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
    /// The index into current LALR State from `grammar.lalr_states`
    pub curr_state: usize,
    //curr_lalr_state: LALRState,
    /// *LALR Parser Stack* | Will be used during reduction
    pub stack: Stack<Token>,

    /// Lexical groups
    group: Stack<Token>,
    groups: GroupTable,

    // Reductions
    /// **TODO** For *Reductions*
    expected_symbols: SymbolTable,
    
    pub have_reduction: bool,
    /// Controls whether reduced rules should be trimmed
    pub trim_reductions: bool,

    // Housekeeping
    initialized: bool,
    /// Tokens to be analyzed
    pub input_tokens: Stack<Token>,
    /// Location of last read terminal
    curr_position: Position,
    sys_pos: Position,

}

impl Parser {
    pub const PARSER_NAME: &str = "GOLD Parser Engine";
    pub const PARSER_VERSION: &str = "5.0.3";

    pub fn new(egt: String) -> Self {
        let grammar = Self::load_grammar(egt);
        //let source = Self::load_source(src);

        let mut properties: HashMap<String,String> = HashMap::new();
        for rec in &grammar.properties {
            properties.insert(rec.name.to_string(), rec.value.to_string());
        }
        Parser {
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
    fn lookahead(&mut self, index: usize) -> Option<char> {
        match self.source.lookahead(index) {
            '???' => None,
            ch@_ => Some(ch)
        }
    }

    /// Loads the parse tables from the specified `source` as `String`
    pub fn load_source(&mut self, source: String)  -> Result<(), ParserError> {
        let src = <Parser as GPParser>::load_source(source)?;
        self.source.load(src);
        self.initialized = true;
        Ok(())
    }
    pub fn clear(&mut self) {
        self.reset();
    }

    pub fn symbol_by_name(&self, name: &str) -> Option<&Symbol> {
        self.grammar.symbols.get(name.to_string())
    }
    pub fn symbol_by_type(&self, kind: SymbolType) -> Option<&Symbol> {
        self.grammar.symbols.get_by_type(kind)
    }
    pub fn get_current_token(&self) -> Option<&Token> {
        self.input_tokens.peek()
    }
    pub fn get_current_reduction(&self) -> Option<&Reduction> {
        //if self.have_reduction {
        self.stack.peek()?.reduction.as_ref()

    }
    pub fn set_current_reduction(&mut self, reduction: &Reduction) {
        // if self.have_reduction {
        //     // self.stack.peek_mut()?.set_data(&reduction);
        // }
        if let Some(peek) = self.stack.peek_mut() {
            peek.set_reduction(reduction);
        }
    }

    /// Wraps `<GPParser>::input_token`, manages group blocks, consumes
    /// the lookahead buffer and //pushes the token onto the `input_tokens` stack.
    pub fn produce_token(&mut self) -> Token {
        trace!("produce_token");
        let mut nested_group = false;
        let mut tok = self.input_token();
        debug!("Token: \'{}\'",&tok.text);

        

        if nested_group {

        } else {
            let len = tok.text.len();
            self.source.consume_buf(len);
        }
        //self.input_tokens.push(tok.to_owned());
        //debug!("Pushed {} onto input token queue.",tok.text());
        tok       
    }

}

impl GPParser for Parser {
    fn parse(&mut self) -> GPMessage {
        if !self.initialized { return GPMessage::NotLoadedError; }
       
        let mut done: bool = false;
        let mut result = GPMessage::Empty;

        while !done {
            // DFA lexer provides a Token amd :ALR parses Token
            result = self.parse_step();
            match result {
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
                // GPMessage::Empty => todo!(),
                // GPMessage::TokenRead => todo!(),
                // GPMessage::Reduction => todo!(),                
                // all non-error events are handled by calling event procedures
                _ => { }
            }
        }
        result
    }

    fn parse_step(&mut self) -> GPMessage {
        let mut result = GPMessage::Empty;
        let mut done = false;
        trace!("parse_step()");

        while !done {
            if self.input_tokens.len() == 0 { // get next Token from DFA lexer
                trace!("Getting token from DFA");
                let token = self.produce_token();
                let kind = token.kind();
                //self.input_tokens.push(token);

                // handle case where an unterminated comment block consumes program
                if *kind == SymbolType::EndOfFile && self.group.is_empty() {
                    result = GPMessage::Empty;
                } else { // a good Token was read
                    result = GPMessage::TokenRead;
                }
                done = true;
            
            } else { // a Token is present and can be parsed
                trace!("Token is present");
                let mut token = self.input_tokens.peek().expect("peek with input tokens").clone();
                let kind = token.kind();
                self.curr_position = token.pos;

                match kind {
                    SymbolType::Noise => {  // whitespace and other ignorables
                        trace!("SymbolType::Noise");
                        self.input_tokens.pop();
                    },
                    SymbolType::EndOfFile => { 
                        if !self.group.is_empty() { // runaway group
                            result = GPMessage::GroupError;
                            done = true;
                        } else  {
                            result = GPMessage::Empty;
                        }
                    },
                     SymbolType::Error => {
                        result = GPMessage::LexicalError;
                        done = true;
                    },
                    _ => {  // LALR parsing of the input Token
                        trace!("Parsing input token");
                        let parsemsg = self.parse_token(&mut token);
                        match parsemsg {
                            // TODO I think I have to do something here
                            GPParseResult::Shift => {
                                self.input_tokens.pop();
                            },
                            GPParseResult::SyntaxError => {
                                panic!("Syntax error on parse_token()");
                            },
                            // GPParseResult::Reduce => {
                            //     println!("LALR parser return Reduce");
                            // },
                            _ => { // fallthru includes reduce/eliminated
                                   // shift, and trim-reduced
                                   // do nothing
                                   //println!("{:?}",parsemsg);
                            },
                        }
                    },
                }
            }
        }

        result
    }

    fn parse_token(&mut self, input_token: &mut Token) -> GPParseResult {
        trace!("parse_token({})",&input_token.text);
        let mut result = GPParseResult::Undefined;
        self.have_reduction = false;
        let parent_symbol = &input_token.symbol;
        let parse_action = self.get_lalr_state(self.curr_state)
                                            .find_action(parent_symbol)
                                            .expect("Problems fetching LALRAction from LALRState");
        match parse_action.action {
            // Creates a new reduction. Pops all the terminals and non-terminals for
            // this rule and push the most left non-terminal.
            ActionType::Reduce => {
                trace!("ActionType::Reduce");
                // This section of the algorithm will reduce the rule specified by action.action
                // Produce a reduction - remove as many Tokens as members in the rule and push
                // a non-terminal Token
                let rule = &self.grammar.productions[parse_action.target_idx];
                // Create a new non-terminal to represent the reduction
                let mut head = Token::default();
                
                // If the rule has only a non-terminal then we don't create a reduction
                // node for this rule in the tree since its not useful. If the user enabled 
                // trimming it is used here.
                if self.trim_reductions && rule.has_only_nonterminal() {
                    // The current rule consists of a single non-terminal and can be trimmed from
                    // the parse tree
                    head = self.stack.pop().expect("empty stack");
                    head.symbol = rule.head.to_owned();
                    result = GPParseResult::ReduceTrimmed;
                } else { // create a new reduction for the current rule
                    self.have_reduction = true;
                    let n = rule.symbols.len(); // n - 1;
                    let mut reduce_tokens: Vec<Token> = vec![];
                    // pop the tokens off the stack for the reduced rule
                    for i in n..0 {
                        reduce_tokens[i] = self.stack.pop().expect("empty stack");
                    }
                    head = Token::new(rule.head.to_owned(), String::from(""));
                    //head.reduction = Some(Reduction::new(rule, reduce_tokens));
                    head.reduction = Some(reduction::reduce(rule, reduce_tokens));
                    result = GPParseResult::Reduce;
                }
                // execute GOTO action for the rule that was just reduced
                // peek at LALR Token stack state to get its index, look the state up,
                // and find the action corresponding to the rule's head symbol
                let state_index = self.stack.peek().expect("Invalid peek").state();
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
                trace!("ActionType::Accept");
                self.have_reduction = true;
                result = GPParseResult::Accept;
            },
            // Pushes current token onto the stack
            ActionType::Shift  => {
                trace!("ActionType::Shift");
                // Shift to target state and push the current Token.
                self.curr_state = parse_action.target_idx;  //self.get_lalr_state(parse_action.target_idx);
                input_token.lalr_state = self.curr_state;
                self.input_tokens.push(input_token.clone());
                result = GPParseResult::Shift;
                debug!("Pushed {} onto input stack and Parser shifted to state {}",input_token.text,input_token.lalr_state)
            },
            ActionType::Undefined |
            ActionType::Goto  => {
                trace!("ActionType::Undefined|Goto");
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

    fn input_token(&mut self) -> Token {
        trace!("input_token()");
        let mut token = Token::default();
        let mut curr_state = self.grammar.initial_states.dfa as usize;
        let mut length = 1;
        let mut last_accept_state: i32 = -1;
        let mut last_accept_pos: i32 = -1;
        //let mut target = 0;
        let mut done = false;

        while !done {
            let mut ch = '???';
            // Search all the branches of the current DFA state for the next
            // character in the input stream. If found, the target state is returned.
            //let ch = self.lookahead(length);
            if let Some(c) = self.lookahead(length) {
                ch = c;
            } else {
                //done = true;
                token.symbol.kind = SymbolType::EndOfFile;
                break;
            }
            // Checks whether an edge was found from the `curr_state`. If so, the state and
            // `curr_pos` advances. Else, quit main loop and report Token found. If the
            // last_accept_state is -1, then no match found and the Error Token is created.
            match self.get_dfa_state(curr_state).find_edge(ch) {
                // Checks whether the target state accepts a Token. If so, it sets the
                // appropriate variables so when the algorithm is done, it can return the
                // proper Token and number of characters
                Some(index) => { 
                    let target = index as i32;
                    if self.get_dfa_state(index).accept {
                        last_accept_state = target;
                        last_accept_pos = length as i32;
                        debug!("target state {index} accepts a token");
                    }
                    curr_state = target as usize;
                    length += 1;
                    debug!("curr_state = {target}");
                },
                None => { // no edge found. no target state found.
                    if last_accept_state == -1 { // Lexer doesn't recognize the symbol
                        token.symbol = self.symbol_by_type(SymbolType::Error).unwrap().to_owned();
                        token.text = <Parser as GPParser>::lookahead(self,1).to_string();
                    } else { // create Token and read text for Token.
                        // self.text contains the total number of accept characters
                        token.symbol = self.get_dfa_state(last_accept_state as usize).accept_symbol.to_owned();
                        token.text = <Parser as GPParser>::lookahead(&self, last_accept_pos as usize).to_string();
                    }
                    done = true;
                    debug!("done.");
                }
            }
            println!("Current DFA State: {curr_state} Edge: \'{ch}\'\nLast accept DFA State: {last_accept_state} Position: {last_accept_pos}");

            
        }
        token.pos = self.source.pos;
        println!("Span: {}:{}  {length} {:?}",self.source.pos.line(),self.source.pos.col(),token);
        token
    }

    fn lookahead(&self, count: usize) -> &str {
        trace!("<GPParser>::lookahead({count})");
        let mut ahead = count;
        if ahead > self.source.get_buf_len() { ahead = self.source.get_buf_len(); }
        let str = self.source.get_buf_slice_to(ahead);
        debug!("\'{}\'",str);
        str
       // self.source.buf.as_str()[0..ahead].to_string()
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
        let ver = format!("{} - Version {}",Self::PARSER_NAME, Self::PARSER_VERSION);
        format!("{}\n{} {}",ver, self.properties.get("Name").unwrap(), self.properties.get("Version").unwrap() )
    }

}







#[cfg(test)]
pub mod test {
    use crate::engine::{parser::GPParser, SymbolType};

    use super::{Parser, GPMessage};

    #[test]
    fn parse_step() {
        let mut parser = gen_loaded_parser();
        let mut done = false;

        while !done {
            match parser.parse_step() {
                GPMessage::TokenRead => debug!("TokenRead"),
                GPMessage::Reduction => {
                    debug!("Reduction");
                },
                GPMessage::Empty => {
                    debug!("Reached end of source file.");
                    done = true;
                },
                GPMessage::Accept => {
                    debug!("Parsed grammar accepted.");
                    done = true;
                },

                msg@_ => {    
                    done = true;
                    debug!("{:?}",msg);
                },
            }

        }

    }
    #[test]
    /// Uses input_token and nested group logic
    fn test_produce_token() {
        let mut parser = gen_loaded_parser();
        let mut done = false;
        let mut result = GPMessage::Empty;

        //while !done {
            let tok = produce_token(&mut parser); info!("{:?}",tok.kind());
            let tok = produce_token(&mut parser); info!("{:?}",tok.kind());
            let tok = produce_token(&mut parser); info!("{:?}",tok.kind());
            
        //}
            // while !done {
        //     if parser.input_tokens.len() == 0 {
        //         let tok = produce_token(&mut parser);
        //         //parser.curr_position = tok.pos;
        //         if *tok.kind() == SymbolType::EndOfFile && parser.group.is_empty() {
        //             result = GPMessage::GroupError;
        //         } else { // a good Token was read
        //             result = GPMessage::TokenRead;
        //         }
        //         //done = true;
        //     } else {
        //         let mut tok = parser.input_tokens.peek().expect("peek with input tokens").clone();
        //         let kind = tok.kind();
        //         parser.curr_position = tok.pos;

        //         debug!("{:?}",kind);
        //         match kind {
        //             SymbolType::Terminal |
        //             SymbolType::Noise => {
        //                 trace!("SymbolType::Terminal|Noise");
        //                 parser.input_tokens.pop();
        //             },
        //             SymbolType::Error => {
        //                 result = GPMessage::LexicalError;
        //                 done = true;
        //             },
        //             _ => match parser.parse_token(&mut tok) {
        //                 super::GPParseResult::Shift => {
        //                     parser.input_tokens.clear();
        //                 }    
        //                 super::GPParseResult::Accept => done = true,
        //                 msg@_ => { debug!("{:?}",msg);}
        //             }
        //         }
        //     }
        // }
    }

    #[test]
    fn parse_token() {
        let mut parser = gen_loaded_parser();
        let mut tok = parser.input_token();
        parser.input_tokens.push(tok);
        tok = parser.input_tokens.peek().expect("peek").clone();

        debug!("Parsing Token({})", tok.text);
        match parser.parse_token(&mut tok) {
            i@_ => println!("{:?}",i)
        }
    }
    #[test]
    fn input_token() {
        let mut parser = gen_loaded_parser();
        //assert!(parser.get_current_token());
        //let mut tokens: Vec<Token> = vec![];

        //println!("{:?}",parser.get_current_token());
        info!("Starting...");
     //   loop {
            let mut tok = parser.input_token();
            debug!("Token found: {} {}",tok.name(),tok.pos.to_string());
            if tok.kind() == &SymbolType::EndOfFile ||
                tok.kind() == &SymbolType::Error {
                //break;
            }
            parser.input_tokens.push(tok.to_owned());
            //parser.input_tokens.push(tok.to_owned());
            //tok = parser.input_tokens.pop();
            //let result = parser.parse_tokens(&mut tok);
            //debug!("Parse results: {:?}", result);
    //    }
        debug!("Pushed {} tokens. {:?}",parser.input_tokens.len(),parser.input_tokens);
        assert_eq!(&tok.text, "assign");
    }
    #[test]
    fn new() {
        let parser = Parser::new(crate::test::GP_TEST_EGT.to_string());
        println!("About:\n{}",parser.about());
        assert_eq!(parser.grammar.property("Name"),"BADASS");
    }
    #[test]
    fn load_source() {
        let mut parser = Parser::new(crate::test::GP_TEST_EGT.to_string());
        if let Ok(_) = parser.load_source(crate::test::GP_TEST_SRC.to_string()) {
            println!("{}",parser.version());
            println!("Source length: {}", parser.source.len());
            println!("Source:\n{}",parser.source.to_string());
        } else {
            println!("Error loading source");
        }
    }

    fn produce_token(parser: &mut Parser) -> crate::engine::token::Token {
        trace!("<test>::produce_token");
        let mut nested_group = false;
        let mut tok = parser.input_token();
        debug!("Token: \'{}\'",tok.text);
        if nested_group {

        } else {
            let len = tok.text.len();
            parser.source.consume_buf(len);
        }
        parser.input_tokens.push(tok.to_owned());
        debug!("Pushed {} onto input token queue.",tok.text);
        tok
    }

    fn gen_loaded_parser<'test>() -> Parser {
        crate::test::init_logger();

        let mut parser = Parser::new(crate::test::GP_SIMPLE_EGT.to_string());
        if let Ok(_) = parser.load_source(crate::test::GP_SIMPLE_SRC.to_string()) {
            // println!("{}",parser.version());
            // println!("Source length: {}", parser.source.len());
            // println!("Source:\n{}",parser.source.to_string());
        } else {
            println!("Error loading source");
        }
        parser       
    }

}