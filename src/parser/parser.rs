//! Parser
//! 
//! Front-end parser generator wrapper for use by developer to read Enhanced Grammar Table file
//! and source file, parse the source for dumping or running the code, return parse tree
//! 


use crate::engine::{Parser, reduction::Reduction, Stack, ProductionRule};

use super::CallFrame;


pub struct GOLDParser {
    pub parser: Parser,
    pub root: Reduction,
    pub call_stack: Stack<CallFrame>,
    pub ignore_case: bool,
    
    // TODO Indentation support
}

impl GOLDParser {
    pub fn new(egt: &str, src: &str, trim: bool, case: bool) -> Self {
        let mut parser = Parser::new(String::from(egt));
        parser.load_source(String::from(src));
        parser.trim_reductions = trim;

        GOLDParser {
            parser,
            // TODO fix reduction
            root: Reduction { tokens: Vec::new(), rule: ProductionRule::default()},
            call_stack: Stack::new(),
            ignore_case: case
        }
    }
    pub fn clear(&mut self) {
        self.parser.clear();
        self.call_stack.clear();
        //self.root = Reduction::new(5);
    }
} 