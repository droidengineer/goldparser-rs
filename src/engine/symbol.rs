//! `Symbol Table Record`
//! 
//! 

use std::{fmt::Display, };

use enum_primitive::enum_from_primitive;
//use utf16string::{WString, LE};

//pub use SymbolTableRecord as Symbol;

use super::Utf16;

enum_from_primitive! {
    #[derive(Debug,Default,Clone,Copy,PartialEq,Eq)]
    #[repr(u16)]
    pub enum SymbolType {
        #[default]
        Undefined,
        NonTerminal,    // normal nonterminal
        Terminal,       // normal terminal (content passed to the parser)
        Noise,          // Noise terminal. These are ignored by the parser. Comments and whitespace are considered 'noise'.
        EndOfFile,      // End Character - End of File. This symbol is used to represent the end of the file or the end of the source input.
        GroupStart,     // Lexical group start
        GroupEnd,       // lexical group end
        Deprecated,     // Used as COMMENT_LINE in previous CGT format. Not used in EGT.
        Error           // error terminal. if the parser encounters an error reading a Token, this kind of symbol can be used to differentiate it from other terminal types
    }
}
impl SymbolType {
    pub fn format(&self) -> String {
        match self {
            SymbolType::NonTerminal => format!("<{{}}>"),
            SymbolType::Terminal => {
                format!("\'{{}}\'")
            },
            SymbolType::EndOfFile => format!("{{EOF}}"),
            _ => format!("({{}})")
        }
    }
}


/// #[derive(Debug)]
/// Each record describing a symbol in the Symbol Table is preceded by a byte containing 
/// the value 83 - the ASCII value of "S". The file will contain one of these records for 
/// each symbol in the grammar. The Table Count record, which precedes any symbol records, 
/// will contain the total number of symbols.
/* pub struct SymbolTableRecord {
    /// Index of symbol in `GOLDParser` 's `SymbolTableRecord`
    pub index: u16,
    /// Name of the symbol as character or string
    pub name: WString<LE>,
    /// Class of symbols this symbol belongs to
    pub kind: SymbolType,
}

impl SymbolTableRecord {
    pub fn new(index: u16, name: WString<LE>, kind: SymbolType) -> Self {
        SymbolTableRecord { index, name, kind }
    }
    pub fn name(&self) -> String {
        self.name.to_string()
    }

}


impl Display for SymbolTableRecord {
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
} */

#[derive(Debug,Default,Clone)]
pub struct Symbol {
    // Index into the EGT Symbol Table
    pub index: usize,
    /// Name of the symbol as character or string
    pub name: String,
    /// Class of symbols this symbol belongs to   
    pub kind: SymbolType
}

impl Symbol {
    //pub const DEFAULT: Symbol = Symbol { index: 0, name: String::from(""), kind: SymbolType::Undefined };
    const QUOTE_CHARS: &str = "|+*?()[]{}<>!";

    pub fn new(index: usize, name: String, kind: SymbolType) -> Self {
        Symbol { index, name, kind }
    }

    /// Encapsulates a string with single quotes
    /// TODO This may be accomplished thru the `String::quote`
    pub fn quote(&self, src: Utf16) -> String {
        let source = src.to_string();
        if source.contains(Self::QUOTE_CHARS) {
            format!("'{}'", source)
        } else {
            source
        }
    }

    /// Returns the text representation of the symbol as follows:
    /// * \<NonTerminal\>
    /// * 'Terminal'
    /// * (Special)
    pub fn as_handle(&self) -> String {
        match self.kind {
            SymbolType::NonTerminal =>  format!("<{}>", self.name),
            SymbolType::Terminal => format!("\'{}\'", self.name),
            _ => format!("({})", self.name)
        }
    }
}

/// Text representation of the symbol.
/// * non-terminals: <name>
/// * special terminals: (name)
/// * terminals: 'name'
impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            SymbolType::NonTerminal => write!(f, "Index: {:4} Symbol: <{}> Type: {:?}\n", self.index, self.name,self.kind),
            SymbolType::Terminal => {
                write!(f, "\'{}\'", self.name)
            },

            _ => write!(f, "Index: {:4} Symbol: ({}) Type: {:?}\n", self.index, self.name,self.kind)
        }
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.name == other.name && self.kind == other.kind
    }
}




#[cfg(test)]
pub mod test {
    use super::SymbolType;



    #[test]
    fn symbol_type() {
        let kind = SymbolType::NonTerminal;
        println!("{} {:?}",kind.format(),kind);
    }

}