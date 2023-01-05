//! Production Record
//! 
//! | Byte | Integer | Integer | Empty | 0..n Integer |
//! |  'R' |  index  | headidx |  69   | symbol_idx   |
//! 
//! http://goldparser.org/doc/egt/record-production.htm



/// Each record describing a rule in the `RuleTable` is preceded by a byte field 
/// containing the value 82 - the ASCII code for 'R'. The file will contain one 
/// of these records for each rule in the grammar. The `TableCountsRecord`, which 
/// precedes any rule records, will contain the total number of rules.
pub struct ProductionRecord {
    /// This parameter holds the index of the rule in the `RuleTable`. The resulting rule should be stored at this Index.
    pub index: u16,
    /// Each rule derives a single nonterminal symbol. This field contains the index of the symbol in the `SymbolTable`
    pub nonterminal: u16,
    /// This field is reserved for future use
    pub reserved: u8,
    /// The remaining entries in the record will contain a series of indexes to symbols in the `SymbolTable`. These constitute the symbols, both terminals and nonterminals, that define the rule. There can be 0 or more total symbols
    pub symbol_idx: Vec<u16>,
}

impl ProductionRecord {
    const CODE: u8 = 82; //'R';
}
