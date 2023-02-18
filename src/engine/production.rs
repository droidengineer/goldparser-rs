//! Production Record
//! 
//! | Byte | Integer | Integer | Empty | 0..n Integer |
//! |  'R' |  index  | headidx |  69   | symbol_idx   |
//! 
//! Each record describing a rule in the `RuleTable` is preceded by a byte field 
//! containing the value 82 - the ASCII code for 'R'. The file will contain one 
//! of these records for each rule in the grammar. The `TableCountsRecord`, which 
//! precedes any rule records, will contain the total number of rules.
//! http://goldparser.org/doc/egt/record-production.htm


use std::{fmt::Display};

use super::{Symbol, SymbolType, SymbolTable, tables::Table};

pub use ProductionRule as Rule;

#[derive(Debug,Default,Clone)]
/// Represents the  
/// 
/// Each rule consists of a series of `Symbol`s, both terminals and nonterminals,
/// and the single nonterminal (head) that the rule defines. Rules are not
/// creatable during runtime but are instead accessed through the `GOLDParser`'s
/// `RuleTable` that was built from reading the grammar's **EGT**.
/// * `head` is the nonterminal `Symbol` that the rule defines
/// * `symbols` is a `SymbolTable` of terminals and nonterminals
/// * `index` is this rule's index in the `GOLDParser.RuleTable` 
/// 
/// Symbols of a rule e.g. 'Identifier' '=' 'Expression' | 
/// Tokens of a rule e.g. variable1 = (variable1*0.025)
pub struct ProductionRule {
    pub index: usize,
    pub head: Symbol,
    pub symbols: SymbolTable, //Vec<Symbol>,
}

impl ProductionRule {
    //pub const DEFAULT: ProductionRule = ProductionRule { index: 0, head: Symbol::default(), symbols: SymbolTable::new()};
    pub fn new(index: usize, head: Symbol, symbols: SymbolTable) -> Self {
        ProductionRule { index, head, symbols }
    }
    pub fn has_only_nonterminal(&self) -> bool {
        self.symbols.len() == 1 && self.symbols[0].kind == SymbolType::NonTerminal
    }
    pub fn head(&self) -> &Symbol {
        &self.head
    }
    /// Prints the RHS of the rule
    pub fn handle(&self) -> String {
        //self.symbols.to_string()
        self.symbols.as_handle()
    }
    /// Prints the *Backus-Naur* representation of the rule
    pub fn to_string(&self) -> String {
        format!("{:16} ::= {}",self.head.name, self.handle())
    }
}

impl Display for ProductionRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.to_string())
    }
}


