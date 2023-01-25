//! Token
//! 
//! While the `Symbol` represents an class of terminals and non-terminals, the
//! `Token` represents an individual piece of information.

//use utf16string::{WString, LE};

use crate::engine::SymbolType;

use super::{Position, Symbol, Value, reduction::Reduction};


#[derive(Debug,Default,Clone)]
/// Used to represent and organized parsed data.
/// Unlike `SymbolTableRecord`s, which are used to represent a category of terminals and
/// non-terminals, a `Token` represents instances of those symbols. For instance, the 
/// common "identifier" is a specific type of `Symbol`, but can exist in various forms such
/// as "Value1", "cat", or "Sacramento", etc.
/// 
/// Information that is read from the source text/file is stored into the `data` property
/// which can be modified by the developer.
/// Contains:
/// * a `Symbol` representing the `Token`'s parent symbol
/// * a `String` that is UTF-8 and Unicode
pub struct Token {
    /// The `Symbol` that generated this `Token`. Sometimes called parent.
    pub symbol: Symbol,
    /// String from the source file that generated this `Token`
    /// For a `Token` created by reduction, this is empty
    pub text: String,
    /// Data associated with this `Token` is a `Reduction` if present
    pub data: Option<Reduction>,
    pub lalr_state: usize,
    /// `Position` 
    pub pos: Position,
}

impl Token {
    pub fn new(symbol: Symbol, text: String) -> Self {
        
        Token {
            symbol,
            text,
            data: None,
            lalr_state: 0,
            pos: Position::default(),
        }
    }
    #[inline(always)]
    pub fn has_reduction(&self) -> bool {
        match self.data {
            Some(_) => true,
            None => false,
        }
    }
    pub fn set_data(&mut self, data: &Reduction) {
        self.data = Some(data.to_owned());
    }
    #[inline(always)]
    pub fn kind(&self) -> &SymbolType {
        &self.symbol.kind
    }
    #[inline(always)]
    pub fn name(&self) -> &String {
        &self.symbol.name
    }
    #[inline(always)]
    pub fn text(&self) -> &String {
        &self.text
    }
    #[inline(always)]
    pub fn state(&self) -> usize {
        self.lalr_state
    }
    #[inline(always)]
    /// You should always call `has_reduction` before this or risk panic
    pub fn reduction(&self) -> &Reduction {
        self.data.as_ref().unwrap()
    }

}
