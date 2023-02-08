//! Parser
//! 
//! Front-end parser generator wrapper for use by developer to read Enhanced Grammar Table file
//! and source file, parse the source for dumping or running the code, return parse tree
//! 


use std::collections::HashMap;

use crate::engine::*;
use crate::engine::{Parser, reduction::Reduction, parser::{GPParser, ParserError}, Value, SymbolType, token::Token};
use super::Scope;


pub struct GOLDParser {
    pub parser: Parser,
    pub root: Option<Reduction>,
    pub scopes: HashMap<String,Scope>,
    pub curr_scope: Scope,
    pub ignore_case: bool,
    pub generate_tree: bool,
    
    // TODO Indentation support
    pub ignore_indent: bool,
}

impl GOLDParser {
    const VT_INDENT_INC: &str = "IndentIncrease";
    const VT_INDENT_DEC: &str = "IndentDecrease";

    pub fn new(egt: &str, src: &str, trim: bool, case: bool) -> Self {
        let mut parser = Parser::new(String::from(egt));
        let mut ignore_indent = true;
        //println!("Parser tables loaded.");
        if let Some(_) = parser.symbol_by_name(Self::VT_INDENT_INC) {
            ignore_indent = false;
        }
        parser.load_source(String::from(src)).expect(src);
        parser.trim_reductions = trim;
        let mut scopes = HashMap::new();
        scopes.insert(Scope::GLOBAL_SCOPE.to_string(),Scope::default());

        GOLDParser {
            parser,
            // TODO fix reduction
            root: None,
            scopes,
            curr_scope: Scope::default(),
            ignore_case: case,
            ignore_indent,
            generate_tree: false,
        }
    }
    /// Top-level method to begin parsing. If something very custom is needed, the
    /// `GOLDParser` can use an overridden method.
    /// This is where high-level error processing and text is done.
    pub fn parse_source(&mut self) -> bool {
        let accept = false;

        if !self.is_initialized() { return false; }
        match self.parser.parse() {
            crate::engine::parser::GPMessage::TokenRead => {
                true; // return true;
            },
            crate::engine::parser::GPMessage::Reduction => {
                return self.process_reduction()
            },
            crate::engine::parser::GPMessage::Empty => todo!(),
            crate::engine::parser::GPMessage::Accept => {
                self.root = self.parser.get_current_reduction().cloned();
                false;
            },
            crate::engine::parser::GPMessage::NotLoadedError => todo!(),
            crate::engine::parser::GPMessage::LexicalError => todo!(),
            crate::engine::parser::GPMessage::SyntaxError => {
                let tokstr = self.get_current_token().text();
                // TODO add error message info
                
                false;
            },
            crate::engine::parser::GPMessage::GroupError => todo!(),
            crate::engine::parser::GPMessage::InternalError => todo!(),
        }
        true
    }

    /// The `GOLDParser` builds a tree of `Reduction` objects
    /// This method can be overridden or changed to process the reductions
    /// Returns `bool` to indicate whether processing should stop (false) or continue (true)
    pub fn process_reduction(&mut self) -> bool {
        todo!()
        // let reduction = self.get_current_reduction();
        // match reduction {
        //     Some(r) => {
        //         self.set_current_reduction(r);
        //         return true;
        //     },
        //     None => return false,
        // }

        // match self.get_current_reduction() {
        //     Some(reduction) => {
        //         &mut self.set_current_reduction(reduction);
        //         return true;
        //     },
        //     None => return false,

        // }



        // if let Some(reduction) = &self.parser.get_current_reduction() {
            
        //     &self.set_current_reduction(reduction);
        //     return true;
        // } else {
        //     return false;
        // }
    }
    fn set_current_reduction(&mut self, reduction: &Reduction) {
        self.parser.set_current_reduction(reduction);
    }
    fn get_current_reduction(&self) -> Option<&Reduction> {
        self.parser.get_current_reduction()
    }
    pub fn get_parse_tree(&self) -> String {
        let mut tree = String::new();
        match &self.root {
            Some(r) => {
                let f = format!("+-{}\rn", r.rule);
                tree.push_str(&f);
                self.draw_reduction(&mut tree, r, 1);
            },
            None => { return format!("Error: Parse Tree Not Available."); }
        }
        tree
    }
    pub fn draw_reduction(&self, tree: &mut String, reduction: &Reduction, indent: usize) {
        let mut indent_str = String::new();
        for _ in 0..indent {  indent_str.push_str("| "); }
        for token in &reduction.tokens {
            match token.kind() {
                SymbolType::NonTerminal => {
                    let redref = token.reduction.as_ref().unwrap();
                    let f = format!("{}+-{}\r\n",indent_str,redref.rule);
                    tree.push_str(f.as_str());
                    self.draw_reduction(tree, redref, indent + 1);
                },
                _ => {
                    let f = format!("{}+-{}\r\n",indent_str,token.name());
                    tree.push_str(f.as_str());
                }
            }
        }
    }
    pub fn run(&mut self) {
        let ret = self.parser.parse();
        println!("{:?}",ret);
    }
    pub fn set_curr_scope(&mut self, scope: Scope) -> Scope {
        let old_scope = self.curr_scope.clone();
        self.curr_scope = scope.clone();

        if !self.scopes.contains_key(&scope.name) {
            self.scopes.insert(scope.name.clone(), scope);
        }

        old_scope
    }
    pub fn get_variable(&self, name: &str) -> Value {
        todo!()
    }
    pub fn set_variable(&mut self, name: &str, value: Value) {
        self.curr_scope.locals.insert(name.to_string(), value);
    }
    pub fn clear_variable(&mut self, name: &str) {
        let mut key = name.to_string();
        if self.ignore_case { 
            key = key.to_uppercase();
        }
        let mut scope = &mut self.curr_scope;
        loop {
            if scope.contains(&key) {
                scope.clear_local(&key);
            }
            if scope.parent == "".to_string() { break; }
            else {
        //TODO        scope = self.scopes.get_mut(&scope.parent).expect("Can't find {scope.parent}");
            }
        }
    }
    pub fn load_source(&mut self, file: String) -> Result<(), ParserError> {
        self.parser.load_source(file)
    }
    pub fn is_initialized(&self) -> bool {
        self.parser.is_initialized() && self.parser.source.len() > 1
    }
    pub fn clear(&mut self) {
        self.parser.clear();
        self.scopes.clear();
        self.curr_scope = Scope::default();
        self.root = None;
    }
    pub fn get_current_token(&self) -> &Token {
        self.parser.input_tokens.peek().expect("current token from input tokens empty")
    }
    pub fn parser(&mut self) -> &mut Parser {
        &mut self.parser
    }
    pub fn source_size(&self) -> usize { self.parser.source.src.len() }
    pub fn source_pos(&self) -> Position { self.parser.source.pos }
    pub fn source_abs_pos(&self) -> usize { self.parser.source.get_abs_pos() }
    
}

// impl super::RuleHandler for ProductionRule {
//     type Item = GOLDParser;

//     fn execute(&self) {
//         todo!()
//     }

//     fn rule(&self) -> &str {
//         todo!()
//     }
// }

#[cfg(test)]
mod test {


    #[test]
    fn new() {

    }
}