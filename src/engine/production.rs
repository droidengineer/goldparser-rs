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


use std::ops::{Index, IndexMut};

use super::{Symbol, SymbolType};

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

#[derive(Default)]
/// Represents the logical structures of the grammar. Productions consist of a head (nonterminal) followed
/// by a series of both nonterminals and terminals.
pub struct ProductionRule {
    pub index: usize,
    pub head: Symbol,
    pub symbols: Vec<Symbol>,
}

impl ProductionRule {
    pub fn new(index: usize, head: Symbol, symbols: Vec<Symbol>) -> Self {
        ProductionRule { index, head, symbols }
    }
    pub fn has_only_nonterminal(&self) -> bool {
        self.symbols.len() == 1 && self.symbols[0].kind == SymbolType::NonTerminal
    }
    pub fn head(&self) -> Symbol {
        self.head.clone()
    }
}

impl RuleHandler for ProductionRule {
    fn execute(&self) {
        todo!()
    }
}

/// Types implementing the `RuleHandler` 
/// trait will adjust this method to implement code generation or execution strategies
pub trait RuleHandler {
/// This method is called when the parsed program tree is executed. 
    fn execute(&self);
}