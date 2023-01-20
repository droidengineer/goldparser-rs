//! records/mod.rs
//! 
//! Each logical record starts with a byte containing the value 77. This is the ASCII code for the letter "M", 
//! which, in turn, stands for multitype. So far, this is the only type of record stored in the file; however, 
//! it is possible, in the future, to add more types such as pictures, sounds, additional parse information, and 
//! other files.
//! 
//! Following the first byte, there is a two byte unsigned integer that contains the total number of entries in 
//! the record. The number is stored in Little Endian format, which means the least significant byte is stored 
//! first. This is the format used on the Intel family of processors and is the standard used by most file formats. 
//! Also, please note, this value is not the number of bytes to follow, but instead the number of different data
//! types are stored. You should implement a simple "for-loop" to read the entries from the file.
//! 
//! http://goldparser.org/doc/egt/structure-record.htm

//#[macro_use] extern crate enum_primitive;
extern crate num_traits;


use std::{ops::Deref, default, time::Instant};

use enum_primitive::{FromPrimitive, enum_from_primitive};
use utf16string::{WStr, LE, WString};

//pub use utf16string::WString as WString;
pub type Utf16 = WString<LE>;

pub mod stack;
pub mod tables;
pub mod property;
pub mod counts;
pub mod charset;
pub mod symbol;
pub mod group;
pub mod production;
pub mod states;
pub mod token;
pub mod reduction;
pub mod builder;
pub mod egt;
pub mod source;
pub mod parser;

pub use stack::Stack;
pub use property::PropertyRecord;
pub use counts::TableCountsRecord;
pub use charset::{CharacterSet};
pub use symbol::{Symbol, SymbolType};
pub use group::LexicalGroup;
pub use production::{ProductionRule};
pub use states::{InitialStatesRecord, DFAState, DFAEdge, LALRState, LALRAction};
pub use tables::{SymbolTable};
pub use source::SourceReader;
pub use parser::Parser;
pub use egt::EnhancedGrammarTable;
pub use builder::Builder;


use self::token::Token;
//pub use crate::records::RecordEntry;
//pub use self::Position;

enum_from_primitive! {
    #[derive(Debug,Copy,Clone, PartialEq, Eq, Hash)]
    #[repr(u8)]
    /// Each record structure consists of a series of entries which, in turn, can hold any number of data types. 
/// Preceding each entry is an identification byte which denotes the type of data which is stored. Based on 
/// this information, the appropriate number of bytes and the manner in which they are read can be deduced.
/// http://goldparser.org/doc/egt/structure-entry-overview.htm
    pub enum EntryType {
        /// The entry only consists of an identification byte containing the value 69; the ASCII value of 'E'. This type of entry is used to represent a piece of information that has not been defined for reserved for future use. It has no actual value and should be interpreted as a logical NULL.
        Empty = 69,     // 'E' u8
        /// A "byte" entry is preceded by a single byte containing the value 98; the ASCII value for 'b'. The next byte contains the actual information stored in the entry. This is a rather inefficient method for storing a mass number of bytes given that there is as much overhead as actual data. But, in the case of storing small numbers, it does save a byte over using an integer entry.
        Byte = 98,      // 'b' u8
        /// A Boolean entry is preceded by a byte containing the value 66; the ASCII value for 'B'. This entry is identical in structure to the Byte except the second byte will only contain a 1, for True, or a 0 for False.
        Boolean = 66,   // 'B' u8
        /// This is the most common entry used to store the Compiled Grammar Table information. Following the identification byte, the integer is stored using Little-Endian byte ordering. In other words, the least significant byte is stored first.
        Integer = 73, // 'I' u16
        /// A string entry starts with a byte containing the value 83, which is the ASCII value for "S". This is immediately followed by a sequence of 1 or more Unicode characters which are terminated by a null.
        String = 83, // 'S' u16..0_u16
    }
}

enum_from_primitive! {
    #[derive(Debug,Copy,Clone, PartialEq, Eq, Hash)]
    #[repr(u8)]
    pub enum RecordType {
        Multi       = 77, // 'M'
        Property    = 112, // 'p'
        Counts      = 116,   // 't'
        CharSet     = 99,   // 'c'
        Symbol      = 83,    // 'S'
        Group       = 103,    // 'g'
        Production  = 82, // 'R'
        InitState   = 73, // 'I'
        DFA         = 68,       // 'D'
        LALR        = 76,      // 'L'
    }
}

#[derive(Debug)]
pub enum RecordEntry {
    Empty,
    Byte(u8),
    Bool(u8),
    Integer(u16),
    String(WString<LE>),
}

impl RecordEntry {
    pub fn byte(&self) -> u8 {
        match self {
            RecordEntry::Byte(b) => *b,
            _ => panic!("RecordEntry::byte() {:?}", self)
        }
    }
    pub fn bool(&self) -> bool {
        match self {
            RecordEntry::Bool(b) => {
                if *b != 0u8 { true } else { false }
            },
            _ => panic!()
        }
    }
    pub fn integer(&self) -> u16 {
        match self {
            RecordEntry::Integer(i) => { /* println!("integer(): {}", *i); */ *i},
            _ => panic!()
        }
    }
    #[inline(always)]
    pub fn as_usize(&self) -> usize { self.integer() as usize }
    pub fn string(&self) -> String {
        match self {
            RecordEntry::String(i) =>  {
                //let mut wstr:  WString<LE> = WString::from(i);
                //i.clone_into(&mut &wstr);
                
                //println!("string(): {}",i.to_string()); //, wstr.to_string());
                i.to_string() //wstr
            },
            _ => panic!()
        }
    }
    pub fn wstring(&self) -> WString<LE> {
        match self {
            RecordEntry::String(i) => {
                let rets = i.deref().to_string();
                WString::from(&rets)
               // let ret = i.clone();
               // let rret = i.clone_into(&mut ret.clone());
                //ret.deref().to_string()
            },
            _ => panic!("wstring(): error")
        }
    }
} 

#[derive(Debug)]
pub struct LogicalRecord {
    pub num_entries: u16,
    pub kind: RecordType,
    pub entries: Vec<RecordEntry>,
}
impl LogicalRecord {
    pub fn new(num: u16, kind: RecordType) -> Self {
        LogicalRecord {
            num_entries: num,
            kind,
            entries: Vec::new(),
        }
    }
}

#[derive(Default,Debug,Clone,Copy)]
pub struct Position(usize,usize);
impl Position {
    /// Column number where the Token was read.
    pub fn col(&self) -> usize { self.1 }
    /// Line number where the Token was read.
    pub fn line(&self) -> usize { self.0 }
    pub fn set(&mut self, pos: Position) {
        self.0 = pos.0;
        self.1 = pos.1;
    }
    pub fn inc_col(&mut self) { self.1 += 1; }
    pub fn inc_line(&mut self) { self.0 += 1; self.1 = 1; }

    pub fn to_string(&self) -> String {
        format!(" [{}, {}]", self.line(), self.col())
    }
    pub fn clear(&mut self) {
        self.0 = 0;
        self.1 = 0;
    }

}

// #[derive(Debug,Clone)]
// pub struct Value(_);

#[derive(Debug,Clone)]
pub enum Value {
    String(String),
    Reduction(Vec<Token>),
    Bool(bool),
    Integer(u16),
    Timestamp(Instant),
}
impl Value {
    pub fn as_string(&self) -> Option<&String> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_reduction(&self) -> Option<&Vec<Token>> {
        match self {
            Value::Reduction(r) => Some(r),
            _ => None,
        }
    }
}
impl Default for Value {
    fn default() -> Self {
        Value::String(String::from(""))
    }
}
impl Into<String> for Value {
    fn into(self) -> String {
        match self {
            Value::String(s) => s,
            _ => "".to_string(),
        }
    }
}
