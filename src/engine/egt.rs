use std::{fmt::Display, collections::HashMap};

use crate::engine::{ 
        charset::CharacterSet, 
        Symbol, SymbolTable,
        production::ProductionRule, 
        states::{DFAState, LALRState},
        
        
    };





pub struct EnhancedGrammarTable {
    pub header: String,
    pub properties: Vec<PropertyRecord>,
    pub counts: TableCounts,
    pub charset: Vec<CharacterSet>,
    pub symbols: SymbolTable,
    pub groups: Vec<LexicalGroup>,
    pub productions: Vec<ProductionRule>,
    //pub productions: GPTable<ProductionRule>,
    /// The initial state in the Deterministic Finite Automata table. Normally, due to how the generation
    /// algorithm is implemented, this value should be 0    
    pub dfa_init_state: u16,
    /// The initial state in the LALR state table. Like the DFA state table, this value should normally be 0
    pub lalr_init_state: u16,
    pub dfa_states: Vec<DFAState>,
    //pub dfa_states: GPTable<DFAState>,
    pub lalr_states: Vec<LALRState>,
    //pub lalr_states: GPTable<LALRState>,
}

impl EnhancedGrammarTable {
    const EGT_HEADER: &str = "GOLD Parser Tables/v5.0";

    pub fn new(header: String) -> Self {
        assert_eq!(header,Self::EGT_HEADER);

        EnhancedGrammarTable { 
            header,
            properties: vec![],
            counts: TableCounts {symtab: 0, charset: 0, rules: 0, dfatab: 0, lalrtab: 0, lexgroups: 0 },
            charset: vec![],
            symbols: SymbolTable::default(),
            groups: vec![],
            productions: vec![],
            dfa_init_state: 0, lalr_init_state: 0,
            dfa_states: vec![],
            lalr_states: vec![],
        }
    }
    
    // pub fn get_sym_by_name(&self, name: String) -> Option<&Symbol> {
    //     for sym in &self.symbols. {
    //         if 
    //     }   
    // }
    
    /// Searches (name,value) pairs by name and returns value
    pub fn property(&self, name: &str) -> &str {
        for rec in self.properties.as_slice() {
            if rec.name == name {
                return rec.value.as_str();
            }
        }
        // we should return an Option<> and None here
        panic!("Parameter({}): Not Found",name)       
    }

    // pub fn properties_as_string(&self) -> String {
    //     self.properties.iter().map(|p| {format!("{} = {}\n",p.name,p.value)}).collect::<String>()      
    // }

    #[inline(always)]
    pub fn resize(&mut self) {
        self.symbols.resize(self.counts.symtab as usize);
        self.charset.resize(self.counts.charset as usize, CharacterSet::default());
        self.productions.resize(self.counts.rules as usize, ProductionRule::default());
        self.dfa_states.resize(self.counts.dfatab as usize, DFAState::default()); //.resize(self.counts.dfatab as usize);
        self.lalr_states.resize(self.counts.lalrtab as usize, LALRState::default());
    
    }
    
    #[inline(always)]
    pub fn total_records(&self) -> usize {
        self.properties.len() + 1 + //self.counts.len() +
        self.charset.len() + self.symbols.len() +
        self.groups.len() + self.productions.len() +
        self.dfa_states.len() + self.lalr_states.len()
    }
}

impl Display for EnhancedGrammarTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"[Properties]\n{}\n",self.properties.iter().map(|p| {format!("{} = {}\n",p.name,p.value)}).collect::<String>())?;
        write!(f,"[Total Counts]\n{}\n\n", self.counts)?;
        write!(f,"[Character Sets]\n{}\n\n", self.charset.iter().map(|t| t.to_string()).collect::<String>())?;
        write!(f,"[Symbols]\n{}\n", self.symbols)?;
        write!(f,"[Groups]\n{}\n", "self.groups")?;
        write!(f,"[Productions]\n{}\n", self.productions.iter().map(|t| t.to_string()).collect::<String>())?;
        write!(f,"[Initial States]\n dfa: {} lalr: {}\n", self.dfa_init_state, self.lalr_init_state)?;
        write!(f,"[DFA States]\n{}\n", self.dfa_states.iter().map(|t| t.to_string()).collect::<String>())?;
        write!(f,"[LALR States]\n{}\n", self.lalr_states.iter().map(|t| t.to_string()).collect::<String>())?;
        write!(f,"END")
    }
}

#[derive(Default)]
/// This is a single record that stores the number of entries for each table. 
pub struct TableCounts {
    pub symtab: u16,
    pub charset: u16,
    pub rules: u16,
    pub dfatab: u16,
    pub lalrtab: u16,
    pub lexgroups: u16,
}

impl TableCounts {
//    pub const CODE: u8 = 116; //'t';   
    pub fn new(symtab: u16,
        charset: u16,
        rules: u16,
        dfatab: u16,
        lalrtab: u16,
        lexgroups: u16,
    ) -> Self {
        TableCounts { 
            symtab, charset, rules, dfatab, lalrtab, lexgroups
        }
    }

    pub fn symtab(&mut self, num: u16) -> &mut Self {
        self.symtab = num;
        self
    }
}

impl std::fmt::Display for TableCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let disp = format!("{:9} {:3}  {:16} {:3}  {:12} {:3}\n{:9} {:3}  {:16} {:3}  {:12} {:3}",
            "Symbols", self.symtab, "Character Sets", self.charset, "DFA States",self.dfatab, 
            "Groups", self.lexgroups, "Production Rules", self.rules, "LALR States", self.lalrtab);
        write!(f,"{}", disp)
    }
}


/// Property records occur at the beginning of the file and contain information
///  about the grammar as well as attributes that affect how the grammar functions. 
/// The record is preceded by a byte field that contains the value 112, the ASCII code 
/// for the letter 'p'. The record contains an index, the property name, and its 
/// associated value. The idea is to allow additional information to be added in the future. 
/// This may include more information about the grammar and/or user-defined meta-data.
#[derive(Debug)]
pub struct PropertyRecord {
    //pub index: usize,
    pub name: String,
    pub value: String,
}
impl PropertyRecord {
    pub fn new(name: String, value: String) -> Self {
        PropertyRecord {
            name, value
        }
    }
}

impl std::fmt::Display for PropertyRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //let disp = format!("{} = {}", self.name,self.value);
        write!(f,"{} = {}", self.name,self.value)
    }
}

/// Group records occur after all the Symbol Records. The record is preceded 
/// by a byte field that contains the value 103, the ASCII code for the letter 'g'
#[derive(Default,Clone,PartialEq)]
pub struct LexicalGroup {
    /// The table index of the group in the `GroupTable` Values are 0-indexed
    pub index: usize,
    /// The name of the group
    pub name: String,
    /// Index in the `SymbolTable` of the group's container symbol
    pub container_idx: usize,   
    /// Index in the `SymbolTable` of the group's start symbol
    pub start_idx: usize,
    /// Index in the `SymbolTable` of the group's end symbol
    pub end_idx: usize,
    /// `AdvanceMode` indicating how the group will advance
    pub advance_mode: AdvanceMode,
    /// `EndingMode` indicating how group will handle the end symbol
    pub ending_mode: EndingMode,

    /// How many nested group indices occur at the end
    pub nesting_count: usize,
    /// Nested 1..nesting_count
    pub nested: Vec<usize>,
}
impl LexicalGroup {
    const CODE: u8 = 103; //'g';
    
}
impl Display for LexicalGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} Container: {} Start:{} End: {} Advance by {:?} Ending {:?}, Nesting: {}",self.name,self.container_idx,self.start_idx,self.end_idx,self.advance_mode,self.ending_mode,self.nesting_count)
    }
}
/// `AdvanceMode`
#[derive(Debug,Default,Clone,PartialEq)]
pub enum AdvanceMode {
    #[default]
    /// The group will advance a Token at a time
    Token,
    /// The group will advance by one character at a time
    Character,
}

/// `EndingMode`
#[derive(Debug,Default,Clone,PartialEq)]
pub enum EndingMode {
    #[default]
    /// The ending symbol will be left on the input queue
    Open,
    /// The ending symbol will be consumed
    Closed,
}



#[cfg(test)]
mod test {
    use crate::engine::{EnhancedGrammarTable, builder::test::gen_builder};

    #[test]
    fn dfa() {
        let egt = gen_egt();
        println!("{}",egt.dfa_states.iter().map(|t| t.to_string()).collect::<String>());  
    }
    #[test]
    fn display() {
        let egt = gen_egt();
        println!("{}",egt);
    }

    #[test]
    fn from_builder() {

        let egt = gen_egt();
        assert_eq!(egt.header.to_string(),"GOLD Parser Tables/v5.0");
        println!("OK");

        println!("Header: {}", egt.header);
        println!("Properties: {}", egt.properties.len());
        println!("Table Counts: {}", egt.counts);
        println!("Character Sets: Expected: {} Read: {}", egt.counts.charset, egt.charset.len());
        println!("Symbols: Expected: {} Read: {}", egt.counts.symtab, egt.symbols.len());
        println!("Groups: Expected: {} Read: {}", egt.counts.lexgroups, egt.groups.len());
        println!("Productions: Expected: {} Read: {}", egt.counts.rules, egt.productions.len());
        println!("Initial States: DFA({}) LALR({})",egt.dfa_init_state, egt.lalr_init_state);
        println!("DFA States: Expected: {} Read: {}",  egt.counts.dfatab, egt.dfa_states.len());
        println!("LALR States: Expected: {} Read: {}", egt.counts.lalrtab, egt.lalr_states.len());
        println!("Total Records: {}", egt.total_records());
    }

    fn gen_egt() -> EnhancedGrammarTable {
        gen_builder().to_egt()
    }

}