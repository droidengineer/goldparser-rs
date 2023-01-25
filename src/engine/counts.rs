//! Table Counts Record
//! 
//! 



#[derive(Default)]
/// This is a single record that will be stored before any records containing information about 
/// symbols, sets, rules or state table information.
pub struct TableCountsRecord {
    pub symtab: u16,
    pub charset: u16,
    pub rules: u16,
    pub dfatab: u16,
    pub lalrtab: u16,
    pub lexgroups: u16,
}

impl TableCountsRecord {
//    pub const CODE: u8 = 116; //'t';   
    pub fn new(symtab: u16,
        charset: u16,
        rules: u16,
        dfatab: u16,
        lalrtab: u16,
        lexgroups: u16,
    ) -> Self {
        TableCountsRecord { 
            symtab, charset, rules, dfatab, lalrtab, lexgroups
        }
    }

}

impl std::fmt::Display for TableCountsRecord {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let disp = format!("{:9} {:3}  {:16} {:3}  {:12} {:3}\n{:9} {:3}  {:16} {:3}  {:12} {:3}",
        "Symbols", self.symtab, "Character Sets", self.charset, "DFA States",self.dfatab, 
        "Groups", self.lexgroups, "Production Rules", self.rules, "LALR States", self.lalrtab);
    write!(f,"{}", disp)
}
}