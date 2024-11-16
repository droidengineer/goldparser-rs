//! engine/mod.rs
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
extern crate alloc;

use std::{time::Instant, fmt::Error};
use core::{fmt::Debug};
use core::hash::Hash;

use enum_primitive::enum_from_primitive;
use utf16string::{LE, WString};

//pub use utf16string::WString as WString;
pub type Utf16 = WString<LE>;

pub mod stack;
pub mod charset;
pub mod symbol;
pub mod production;
pub mod states;
pub mod token;
pub mod reduction;
pub mod builder;
pub mod egt;
pub mod source;
pub mod parser;

pub use stack::Stack;
pub use charset::{CharacterSet};
pub use symbol::{Symbol, SymbolType, SymbolTable};
pub use production::{ProductionRule};
pub use states::{DFAState, DFAEdge, LALRState, LALRAction};
pub use source::SourceReader;
pub use parser::Parser;
pub use egt::{EnhancedGrammarTable, TableCounts, LexicalGroup, PropertyRecord};
pub use builder::Builder;


use self::token::Token;

#[derive(Default,Debug,Clone,Copy,PartialEq,Eq)]
pub struct Position(usize,usize);
impl Position {
    /// Column number where the Token was read.
    pub fn col(&self) -> usize { self.1 }
    /// Line number where the Token was read.
    pub fn line(&self) -> usize { self.0 }
    /// Span support
    pub fn start(&self) -> usize { self.0 }
    pub fn end(&self) -> usize { self.1 }

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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LineColSpan(Position, Position);
impl LineColSpan {
    pub fn new(line: Position, col: Position) -> Self {
        LineColSpan(line, col)
    }
    pub fn line_start(&self) -> usize { self.0.start() }
    pub fn line_end(&self) -> usize { self.0.end() }
    pub fn line_pos(&self) -> Position { self.0 }
    pub fn col_start(&self) -> usize { self.1.start() }
    pub fn col_end(&self) -> usize { self.1.end() }
    pub fn col_pos(&self) -> Position { self.1 }
    pub fn clear(&mut self) {
        self.0.clear();
        self.1.clear();
    }
}

/// Returns the line and column of the given `pos` and `input`
// pub(crate) fn line_col(input: &str, pos: usize, start: (usize,usize)) -> (usize,usize) {
//     let slice = &input[..pos];
//     // TODO (see pest line_col())
//   //  let prec_ln = memrchr(b'\n', slice.as_bytes());
// }

#[derive(Clone,Copy)]
pub struct Cursor<'i> {
    input: &'i str,
    pos: usize,
}
impl<'i> Cursor<'i> {
    pub(crate) unsafe fn new_unchecked(input: &str, pos: usize) -> Cursor<'_> {
        assert!(input.get(pos..).is_some());
        Cursor { input, pos }
    }
    pub fn new(input: &str, pos: usize) -> Option<Cursor<'_>> {
        input.get(pos..).map(|_| Cursor { input, pos })
    }
    /// Creates a `Cursor` at the start of a `&str`
    #[inline]
    pub fn from_start(input: &'i str) -> Cursor<'i> {
        Cursor { input, pos: 0 }
    }
    #[inline]
    pub fn pos(&self) -> usize { self.pos }
    #[inline]
    pub fn span(&self, other: &Cursor<'i>) -> Span<'i> {
        if core::ptr::eq(self.input, other.input) {
            unsafe {Span::new_unchecked(self.input, self.pos, other.pos)}
        } else {
            // TODO
            panic!("span created from positions from different inputs")
        }
    }
    // #[inline]
    // pub fn line_col(&self) -> (usize, usize) {
    //     if self.pos > self.input.len() {
    //         panic!("position out of bounds");
    //     }

    // }
}

#[derive(Clone,Copy)]
/// Represents a span within a string. input[start..end] must be valid
pub struct Span<'i> {
    input: &'i str,
    start: usize,
    end: usize,
}
impl<'i> Span<'i> {
    pub(crate) unsafe fn new_unchecked(input: &str, start: usize, end: usize) -> Span<'_> {
        Span { input, start, end }
    }
    pub fn new(input: &str, start: usize, end: usize) -> Span<'_> {
        if input.get(start..end).is_some() {
            return Span { input, start, end }
        }
        Span { input, start: usize::MAX, end: usize::MAX }
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

/// A trait which parser rules must implement
pub trait RuleType: Copy + Debug + Eq + Hash + Ord {}
impl<T: Copy + Debug + Eq + Hash + Ord> RuleType for T {}

/// A trait with a single method that parses single strings
pub trait RuleParser<R: RuleType> {
    /// Parses `&str`
    fn parse(rule: R, input: &str) -> Result<(Token, R), Error>;
}
