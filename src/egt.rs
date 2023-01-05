use utf16string::{WString, LE};

use crate::{builder::Builder, 
    records::{
        property::PropertyRecord, 
        counts::TableCountsRecord, 
        charset::CharacterSetTable, 
        symbol::SymbolTableRecord, 
        group::GroupRecord, 
        production::ProductionRecord, 
        states::{InitialStatesRecord, DFAStateRecord, LALRSateRecord}
    }
};




pub struct EnhancedGrammarTable {
    pub header: WString<LE>,
    pub properties: Vec<PropertyRecord>,
    pub counts: Vec<TableCountsRecord>,
    pub charset: Vec<CharacterSetTable>,
    pub symbols: Vec<SymbolTableRecord>,
    pub groups: Vec<GroupRecord>,
    pub productions: Vec<ProductionRecord>,
    pub initial_states: InitialStatesRecord,
    pub dfa_states: Vec<DFAStateRecord>,
    pub lalr_states: Vec<LALRSateRecord>,
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
}

impl From<Builder> for EnhancedGrammarTable {
    fn from(value: Builder) -> Self {
        todo!()
    }
}


#[cfg(test)]
mod test {


    #[test]
    fn from_builder() {
        
    }

}