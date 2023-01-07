//! `Symbol Table Record`
//! 
//! 

//use enum_primitive::FromPrimitive;

use regex::Regex;
use utf16string::{WString, LE};

pub use SymbolTableRecord as Symbol;

enum_from_primitive! {
    #[derive(Debug,Clone,Copy)]
    #[repr(u16)]
    pub enum SymbolType {
        Undefined,
        NonTerminal,    // normal nonterminal
        Terminal,       // normal terminal
        Noise,          // Noise terminal. These are ignored by the parser. Comments and whitespace are considered 'noise'.
        EndOfFile,      // End Character - End of File. This symbol is used to represent the end of the file or the end of the source input.
        GroupStart,     // Lexical group start
        GroupEnd,       // lexical group end
        Deprecated,     // Used as COMMENT_LINE in previous CGT format. Not used in EGT.
        Error           // error terminal. if the parser encounters an error reading a token, this kind of symbol can be used to differentiate it from other terminal types
    }
}
impl SymbolType {
    pub fn to_wstring(&self) -> String {
        match self {
            SymbolType::NonTerminal => format!("<{{}}>"),
            SymbolType::Terminal => {
                format!("\'{{}}\'")
            },
            _ => format!("({{}})")
        }
    }
}

#[derive(Debug)]
/// Each record describing a symbol in the Symbol Table is preceded by a byte containing 
/// the value 83 - the ASCII value of "S". The file will contain one of these records for 
/// each symbol in the grammar. The Table Count record, which precedes any symbol records, 
/// will contain the total number of symbols.
pub struct SymbolTableRecord {
    /// Index of symbol in `GOLDParser` 's `SymbolTableRecord`
    pub index: u16,
    /// Name of the symbol as character or string
    pub name: WString<LE>,
    /// Class of symbols this symbol belongs to
    pub kind: SymbolType,
}

impl SymbolTableRecord {
    pub fn new(index: u16, name: WString<LE>, kind: SymbolType) -> Self {
        //let k = SymbolType::from_u16(kind);
        SymbolTableRecord { index, name, kind }
    }

    
}

/// Text representation of the symbol.
/// * non-terminals: <name>
/// * special terminals: (name)
/// * terminals: 'name'
impl std::fmt::Display for SymbolTableRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            SymbolType::NonTerminal => write!(f, "<{}>", self.name.to_string()),
            SymbolType::Terminal => {
                //let n = self.name.to_string().as_str();
                //let re = Regex::new(n).unwrap();
                write!(f, "\'{}\'", self.name.to_string())
            },

            _ => write!(f, "({})", self.name.to_string())
        }
    }
}