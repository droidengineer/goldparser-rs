//! `Symbol Table Record`
//! 
//! 

//use enum_primitive::FromPrimitive;


enum_from_primitive! {
    #[derive(Debug)]
    #[repr(u16)]
    pub enum SymbolType {
        NonTerminal,    // normal nonterminal
        Terminal,       // normal terminal
        Noise,          // Noise terminal. These are ignored by the parser. Comments and whitespace are considered 'noise'.
        EndOfFile,      // End Character - End of File. This symbol is used to represent the end of the file or the end of the source input.
        GroupStart,     // Lexical group start
        GroupEnd,       // lexical group end
        Deprecated,     // Used in previous CGT format. Not used in EGT.
        Error           // error terminal. if the parser encounters an error reading a token, this kind of symbol can be used to differentiate it from other terminal types
    }
}

/// Each record describing a symbol in the Symbol Table is preceded by a byte containing 
/// the value 83 - the ASCII value of "S". The file will contain one of these records for 
/// each symbol in the grammar. The Table Count record, which precedes any symbol records, 
/// will contain the total number of symbols.
pub struct SymbolTableRecord {
    pub index: u16,
    pub name: String,
    pub kind: SymbolType,
}

impl SymbolTableRecord {
    pub fn new(index: u16, name: String, kind: SymbolType) -> Self {
        //let k = SymbolType::from_u16(kind);
        SymbolTableRecord { index, name, kind }
    }
}

impl std::fmt::Display for SymbolTableRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let disp = format!("@{:04X} name: {} Type: {:?}",
            self.index, self.name, self.kind);
        write!(f,"{}", disp)
    }
}