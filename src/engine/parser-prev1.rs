//! This is the primary interface to the GOLDParser Engine.

use super::{prelude::*, CharacterSet, DFAState, EnhancedGrammarTable, LALRState, Rule, SourceReader, SymbolTable};

#[derive(Debug,Default)]
pub enum ParseMessage {
    #[default]
    Empty = -1,
    TokenRead = 0,
    Reduction,
    Accept,
    NotLoadedError,
    LexicalError,
    SyntaxError,
    GroupError,
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
pub enum ParserError {
    Format(ParseMessage),
    ParseIntError(::std::num::ParseIntError),
    ParseFloatError(::std::num::ParseFloatError),
}
impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParserError {:?}", self)  
    }
}

pub struct Parser<'a> {
    pub grammar: &'a EnhancedGrammarTable,
    //pub source: SourceReader,
    pub source_text: String,

    pub symbols: &'a SymbolTable,
    pub rules: &'a Vec<Rule>,
    pub dfa: &'a Vec<DFAState>,
    pub lalr: &'a Vec<LALRState>,


}
impl<'a> Parser<'a> {
    pub fn new(grammar: &str, source: &str) -> Self {
        todo!()
    }
    pub fn new_with(grammar: &'a EnhancedGrammarTable) -> Self {

        Parser {
            grammar,
            source_text: String::new(),
            symbols: &grammar.symbols,
            rules: &grammar.productions,
            dfa: &grammar.dfa_states,
            lalr: &grammar.lalr_states,
        }
    }

    pub fn symbol_table(&self) -> &SymbolTable {
        &self.grammar.symbols
    }
    pub fn charset_table(&self) -> &Vec<CharacterSet> {
        &self.grammar.charset
    }
    pub fn production_table(&self) -> &Vec<Rule> {
        &self.grammar.productions
    }
    pub fn dfa_states(&self) -> &Vec<DFAState> {
        &self.grammar.dfa_states
    }
    pub fn lalr_states(&self) -> &Vec<LALRState> {
        &self.grammar.lalr_states
    }
}





////////////////////////////////[ TESTING ]
#[cfg(test)]
pub mod test {
    use crate::test::{GP_SIMPLE_EGT,GP_SIMPLE_SRC};
    use crate::engine::{EnhancedGrammarTable, egt::test::gen_egt};

    use super::Parser;

    // fn gen_loaded_parser<'test>() -> Parser {
    //     crate::test::init_logger();

    //     //let egt = gen_egt();
    //     let mut parser = Parser::new_with(&gen_egt());

    //     parser.into()
    // }

}