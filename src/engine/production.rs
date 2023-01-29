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

// pub struct ProductionRecord {
//     /// This parameter holds the index of the rule in the `RuleTable`. The resulting rule should be stored at this Index.
//     pub index: u16,
//     /// Each rule derives a single nonterminal symbol (Head). This field contains the index of the symbol in the `SymbolTable`
//     pub nonterminal: u16,
//     /// The remaining entries in the record will contain a series of indexes to symbols in the `SymbolTable`. 
//     /// These constitute the symbols, both terminals and nonterminals, that define the rule. 
//     /// There can be 0 or more total symbols. Also known as a `Handle`
//     pub symbols: Vec<u16>,
// }

// impl std::fmt::Display for ProductionRecord {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let disp = format!("@{:04X} Head Index: {} Extra: {:?}",
//             self.index, self.nonterminal, self.symbols);
//         write!(f,"{}", disp)
//     }
// }


// impl ProductionRecord {
//     pub fn new(index: u16, nonterminal: u16, symbols: Vec<u16>) -> Self {
//         ProductionRecord { index, nonterminal, symbols }
//     }

// }

#[derive(Debug,Default,Clone)]
/// Represents the logical structures of the grammar. Productions consist of a head (nonterminal) followed
/// by a series of both nonterminals and terminals.
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
    pub fn head(&self) -> Symbol {
        self.head.clone()
    }
    pub fn handle(&self) -> String {
        //self.symbols.to_string()
        self.symbols.as_handle()
    }
    pub fn to_string(&self) -> String {
        format!("{:16} ::= {}",self.head.name, self.handle())
    }
}

impl Display for ProductionRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.to_string())
    }
}


