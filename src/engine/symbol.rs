//! `Symbol`
//! 
//! 

use std::fmt::Display;
use std::ops::{Index, IndexMut};
//use enum_primitive::enum_from_primitive;
//use utf16string::{WString, LE};

use super::Utf16;

enum_from_primitive! {
    #[derive(Debug,Default,Clone,Copy,PartialEq,Eq)]
    #[repr(u16)]
    pub enum SymbolType {
        
        //Undefined,
        NonTerminal,    // normal nonterminal
        Terminal,       // normal terminal (content passed to the parser)
        Noise,          // Noise terminal. These are ignored by the parser. Comments and whitespace are considered 'noise'.
        EndOfFile,      // End Character - End of File. This symbol is used to represent the end of the file or the end of the source input.
        GroupStart,     // Lexical group start
        GroupEnd,       // lexical group end
        Deprecated,     // Used as COMMENT_LINE in previous CGT format. Not used in EGT.
        #[default]
        Error           // error terminal. if the parser encounters an error reading a Token, this kind of symbol can be used to differentiate it from other terminal types
    }
}
impl SymbolType {

}
impl Display for SymbolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            SymbolType::NonTerminal => "NonTerminal",
            SymbolType::Terminal => "Terminal",
            SymbolType::Noise => "Noise",
            SymbolType::EndOfFile => "EOF",
            SymbolType::GroupStart => "GroupStart",
            SymbolType::GroupEnd => "GroupEnd",
            SymbolType::Deprecated => "Deprecated",
            SymbolType::Error => "Error",
        };
        write!(f,"{res}")
    }
}

#[derive(Debug,Default,Clone)]
pub struct Symbol {
    /// Index into the EGT Symbol Table
    index: u16,
    /// Name of the symbol as character or string
    pub name: String,
    /// Class of symbols this symbol belongs to   
    pub kind: SymbolType
}

impl Symbol {
    const QUOTE_CHARS: &'static str = "|+*?()[]{}<>!._-";

    pub fn new(index: u16, name: &str, kind: SymbolType) -> Self {
        Symbol { index, name: name.to_owned(), kind }
    }

    pub fn index(&self) -> usize { self.index as usize }

    /// Encapsulates a `Utf16` with single quotes
    pub fn quote_utf16(&self, src: Utf16, delimit_terminals: bool) -> String {
        let source = src.to_string();
        if source.contains(Self::QUOTE_CHARS) || delimit_terminals {
            format!("'{}'", source)
        } else {
            source
        }
    }

    /// Returns the text BNF representation of the symbol as follows:
    /// * \<NonTerminal\>
    /// * 'Terminal'
    /// * (Special)
    pub fn as_bnf(&self) -> String {
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
        //writeln!(f,"Idx: {:<4}  {:<16} Type: {}",self.index, self.as_bnf(), self.kind)
        writeln!(f,"{}",self.as_bnf())
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.name == other.name && self.kind == other.kind
    }
}


// impl<'a> Default for &'a Symbol {
//     fn default() -> &'a Self {
//         Symbol::default().
//     }
// }

#[derive(Debug,Default,Clone)]
pub struct SymbolTable(Vec::<Symbol>);
//pub struct SymbolTable(HashMap<String, Symbol>);
impl SymbolTable {

    pub fn new() -> Self {
        SymbolTable(Vec::new())
    }

    pub fn as_handle(&self) -> String {
        format!("{}\n", self.0.iter().map(|s| s.as_bnf() + " ").collect::<String>())
    }
    pub fn get(&self, name: String) -> Option<&Symbol> {
        for sym in &self.0 {
            if sym.name == name { return Some(sym) }
        }
        None
    }
    // gets 1st occurance of `SymbolType` in the table
    pub fn get_by_type(&self, kind: SymbolType) -> Option<&Symbol> {
        for sym in &self.0 {
            if sym.kind == kind { return Some(sym) }
        }
        None
    }
    pub fn with_capacity(size: usize) -> Self {
        SymbolTable(Vec::with_capacity(size))
    }
    pub fn push(&mut self, item: Symbol) {
        self.0.push(item);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn resize(&mut self, sz: usize) {
        self.0.resize(sz, Symbol::default());//Self::DEFAULT);
    }    
    
    pub fn clear(&mut self) {
        self.0.clear();
    }
    
}

impl Display for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"{}", self.0.iter().map(|s| format!("Idx: {:<4}  {:<16} Type: {}\n",s.index(), s.as_bnf(), s.kind)).collect::<String>())//.collect::<String>())
    }
}

impl From<Vec::<Symbol>> for SymbolTable {
    fn from(value: Vec::<Symbol>) -> Self {
        SymbolTable(value)
    }
}
impl Index<usize> for SymbolTable {
    type Output = Symbol;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
        //panic!("No symbol found at index {}",index)
    }
}
impl IndexMut<usize> for SymbolTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}


// pub trait TableItem: Default+Clone+Display+PartialEq {}
// impl<T:Default+Display+Clone+PartialEq> TableItem for T {}
// pub trait SymbolItem : TableItem {
//     type Item;

//     fn get_by_name(&self, item: &str) -> Option<&Symbol>;
//     fn get_by_type(&self, kind: SymbolType) -> Option<&Self::Item>;
//     fn as_handle(&self) -> String;
// }









#[cfg(test)]
pub mod test {
    use super::SymbolType;



    #[test]
    fn symbol_type() {
        let kind = SymbolType::NonTerminal;
        println!("{} {:?}",kind,kind);
    }

}