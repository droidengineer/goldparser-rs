use std::{fmt::Display, collections::HashMap};

use utf16string::{WString, LE};

use crate::{builder::Builder, 
    engine::{
        property::PropertyRecord, 
        counts::TableCountsRecord, 
        charset::CharacterSetRecord, 
        Symbol, 
        group::GroupRecord, 
        production::ProductionRecord, 
        states::{InitialStatesRecord, DFAState, LALRState}, Utf16
    }
};




pub struct EnhancedGrammarTable {
    pub header: Utf16,
    pub properties: Vec<PropertyRecord>,
    pub counts: Vec<TableCountsRecord>,
    pub charset: Vec<CharacterSetRecord>,
    pub symbols: Vec<Symbol>,
    pub groups: Vec<GroupRecord>,
    pub productions: Vec<ProductionRecord>,
    pub initial_states: InitialStatesRecord,
    pub dfa_states: Vec<DFAState>,
    pub lalr_states: Vec<LALRState>,
}

impl EnhancedGrammarTable {
    pub fn new(header: WString<LE>) -> Self {

        EnhancedGrammarTable { 
            header,
            properties: Vec::new(),
            counts: Vec::new(),
            charset: Vec::new(),
            symbols: Vec::new(),
            groups: Vec::new(),
            productions: Vec::new(),
            initial_states: InitialStatesRecord { dfa: 0, lalr: 0 },
            dfa_states: Vec::new(),
            lalr_states: Vec::new(),
        }
    }
    

    /// Searches (name,value) pairs by name and returns value
    pub fn property(&self, name: &str) -> &Utf16 {
        for rec in self.properties.as_slice() {
            if rec.name.to_string() == name {
                return &rec.value
            }
        }
        panic!("Parameter(Name): Not Found")       
    }
    
    #[inline(always)]
    pub fn total_records(&self) -> usize {
        self.properties.len() + self.counts.len() +
        self.charset.len() + self.symbols.len() +
        self.groups.len() + self.productions.len() +
        self.dfa_states.len() + self.lalr_states.len()
    }
}

/// The `Builder` must have already called `Builder::init()`
impl From<Builder> for EnhancedGrammarTable {
    fn from(mut builder: Builder) -> Self {
        builder.to_egt()
    }
}

impl Display for EnhancedGrammarTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


#[cfg(test)]
mod test {
    use crate::egt::EnhancedGrammarTable;



    #[test]
    fn from_builder() {
        let mut bldr = crate::builder::test::gen_builder();
        bldr.init();
        let egt = bldr.to_egt();
        assert_eq!(egt.header.to_string(),"GOLD Parser Tables/v5.0");
        println!("OK");
        let egt = EnhancedGrammarTable::from(bldr);
        assert_eq!(egt.header.to_string(),"GOLD Parser Tables/v5.0");
        println!("OK");

        println!("Header: {}", egt.header.to_string());
        println!("Properties: {}", egt.properties.len());
        println!("Table Counts: {}", egt.counts.len());
        println!("Character Sets: {}", egt.charset.len());
        println!("Symbols: {}", egt.symbols.len());
        println!("Groups: {}", egt.groups.len());
        println!("Productions: {}", egt.productions.len());
        println!("Initial States: DFA({}) LALR({})", egt.initial_states.dfa, egt.initial_states.lalr);
        println!("DFA States: {}", egt.dfa_states.len());
        println!("LALR States: {}", egt.lalr_states.len());
        println!("Total Records: {}", egt.total_records());
    }

}