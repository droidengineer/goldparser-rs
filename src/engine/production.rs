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


use super::{Symbol, SymbolType, SymbolTable, prelude::*};

#[derive(Debug,Default,Clone)]
/// Represents the rules of the grammar. `Rule`s consist of a head containing
/// a nonterminal followed by a series of both nonterminals and terminals.
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
pub struct Rule {
    pub index: u16,
    pub head: Symbol,
    pub symbols: SymbolTable,
}

impl Rule {
    pub fn new_with(index: u16, head: Symbol, symbols: SymbolTable) -> Self {
        Rule { index, head, symbols }
    }
    pub fn new(index: u16, head: Symbol) -> Self {
        Rule {
            index,
            head,
            symbols: SymbolTable::new(),
        }
    }
    pub fn contains_one_nonterminal(&self) -> bool {
        self.symbols.len() == 1 && self.symbols[0].kind == SymbolType::NonTerminal
    }
    pub fn head(&self) -> &Symbol {
        &self.head
    }
    /// The non-terminal symbol to which its following symbols may be reduced
    pub fn lhs(&self) -> String { self.head.as_bnf() } 
    /// Prints the RHS of the rule   
    pub fn rhs(&self) -> String { self.symbols.as_bnf() }

    pub fn index(&self) -> usize {self.index as usize}
    pub fn count(&self) -> usize {self.symbols.len()}
    pub fn symbols(&self,idx: usize) -> Option<&Symbol> {
        if idx <= self.symbols.len() {
            Some(&self.symbols[idx])
        } else {
            None
        }
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:16} ::= {}",self.lhs(), self.rhs())
    }
}


#[cfg(test)]
mod test {

    #[test]
    fn test_rule() {
        
    }
}