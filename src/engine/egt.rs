use std::{fmt::Display, collections::HashMap};

use crate::{
    engine::{
        property::PropertyRecord, 
        counts::TableCountsRecord, 
        charset::CharacterSet, 
        Symbol, SymbolTable,
        group::LexicalGroup, 
        production::ProductionRule, 
        states::{InitialStatesRecord, DFAState, LALRState},
        tables::{Table, CharacterSetTable},
        builder::Builder,
    }
};

use super::tables::{ProductionTable, DFAStateTable, LALRStateTable};




pub struct EnhancedGrammarTable {
    pub header: String,
    pub properties: Vec<PropertyRecord>,
    pub counts: TableCountsRecord,
    pub charset: CharacterSetTable, //Vec<CharacterSet>,
    pub symbols: SymbolTable,
    pub groups: Vec<LexicalGroup>,
    //pub productions: Vec<ProductionRule>,
    pub productions: ProductionTable,
    pub initial_states: InitialStatesRecord,
    //pub dfa_states: Vec<DFAState>,
    pub dfa_states: DFAStateTable,
    //pub lalr_states: Vec<LALRState>,
    pub lalr_states: LALRStateTable,
}

impl EnhancedGrammarTable {
    const EGT_HEADER: &str = "GOLD Parser Tables/v5.0";

    pub fn new(header: String) -> Self {
        assert_eq!(header,Self::EGT_HEADER);

        EnhancedGrammarTable { 
            header,
            properties: Vec::new(),
            counts: TableCountsRecord::default(), //TableCountsRecord {symtab: 0, charset: 0, rules: 0, dfatab: 0, lalrtab: 0, lexgroups: 0 },
            charset: CharacterSetTable::new(),
            symbols: SymbolTable::new(),
            groups: Vec::new(),
            productions: ProductionTable::new(),
            initial_states: InitialStatesRecord { dfa: 0, lalr: 0 },
            dfa_states: DFAStateTable::new(),
            lalr_states: LALRStateTable::new(),
        }
    }
    
    /// 
    /// Searches (name,value) pairs by name and returns value
    pub fn property(&self, name: &str) -> &String {
        for rec in self.properties.as_slice() {
            if rec.name == name {
                return &rec.value;
            }
        }
        // we should return an Option<> and None here
        panic!("Parameter({}): Not Found",name)       
    }

    pub fn properties_as_string(&self) -> String {
        self.properties.iter().map(|p| {format!("{} = {}\n",p.name,p.value)}).collect::<String>()      
    }

    #[inline(always)]
    pub fn resize(&mut self) {
        self.symbols.resize(self.counts.symtab as usize);
        self.charset.resize(self.counts.charset as usize);
        self.productions.resize(self.counts.rules as usize);
        self.dfa_states.resize(self.counts.dfatab as usize);
        self.lalr_states.resize(self.counts.lalrtab as usize);
    
    }
    
    #[inline(always)]
    pub fn total_records(&self) -> usize {
        self.properties.len() + //self.counts.len() +
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
        write!(f,"[Properties]\n{}\n",self.properties.iter().map(|p| {format!("{} = {}\n",p.name,p.value)}).collect::<String>())?;
        write!(f,"[Total Counts]\n{}\n\n", self.counts)?;
        write!(f,"[Character Sets]\n{}\n\n", self.charset)?;
        write!(f,"[Symbols]\n{}\n", self.symbols.to_string())?;
        write!(f,"[Groups]\n{}\n", "self.groups")?;
        write!(f,"[Productions]\n{}\n", self.productions)?;
        write!(f,"[Initial States]\n{}\n", self.initial_states)?;
        write!(f,"[DFA States]\n{}\n", self.dfa_states)?;
        write!(f,"[LALR States]\n{}\n", self.lalr_states)?;
        write!(f,"END")
    }
}


#[cfg(test)]
mod test {
    use crate::{engine::{EnhancedGrammarTable, tables::Table, builder::test::gen_builder}};

    #[test]
    fn dfa() {
        let mut egt = gen_egt();
        println!("{}",egt.dfa_states);  
    }
    #[test]
    fn display() {
        let mut egt = gen_egt();
        println!("{}",egt);
    }

    #[test]
    fn from_builder() {
        let mut bldr = gen_builder();
        bldr.init();
        let egt = bldr.to_egt();
        assert_eq!(egt.header.to_string(),"GOLD Parser Tables/v5.0");
        println!("OK");
        let egt = EnhancedGrammarTable::from(bldr);
        assert_eq!(egt.header.to_string(),"GOLD Parser Tables/v5.0");
        println!("OK");

        println!("Header: {}", egt.header.to_string());
        println!("Properties: {}", egt.properties.len());
        println!("Table Counts: {}", egt.counts);
        println!("Character Sets: Expected: {} Read: {}", egt.counts.charset, egt.charset.len());
        println!("Symbols: Expected: {} Read: {}", egt.counts.symtab, egt.symbols.len());
        println!("Groups: Expected: {} Read: {}", egt.counts.lexgroups, egt.groups.len());
        println!("Productions: Expected: {} Read: {}", egt.counts.rules, egt.productions.len());
        println!("Initial States: DFA({}) LALR({})",egt.initial_states.dfa, egt.initial_states.lalr);
        println!("DFA States: Expected: {} Read: {}",  egt.counts.dfatab, egt.dfa_states.len());
        println!("LALR States: Expected: {} Read: {}", egt.counts.lalrtab, egt.lalr_states.len());
        println!("Total Records: {}", egt.total_records());
    }

    fn gen_egt() -> EnhancedGrammarTable {
        let mut bldr = gen_builder();
        bldr.init();
        bldr.to_egt()
    }

}