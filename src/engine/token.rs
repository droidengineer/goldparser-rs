//! Token
//! 
//! While the `Symbol` represents an class of terminals and non-terminals, the
//! `Token` represents an individual piece of information.

use crate::engine::SymbolTableRecord as Symbol;
use utf16string::{WString, LE};

use crate::engine::SymbolType;

#[derive(Debug)]
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
    /// The `Symbol` that generated this `Token`
    pub symbol: Symbol,
    /// String from the source file that generated this `Token`
    /// For a `Token` created by reduction, this is empty
    pub text: String,
    /// Data associated with this `Token`
    //pub data: T,
    //pub lalr_state: LALRState,

    /// Column number where the token was read.
    pub col: usize,
    /// Line number where the token was read.
    pub line: usize,
}

impl Token {
    pub fn new(symbol: Symbol, text: String) -> Self {
        
        Token {
            symbol,
            text,
            col: 0, line: 0,
        }
    }
    #[inline(always)]
    pub fn kind(&self) -> SymbolType {
        self.symbol.kind
    }
    #[inline(always)]
    pub fn name(&self) -> String {
        self.symbol.name.to_utf8()
    }
    #[inline(always)]
    pub fn text(&self) -> String {
        self.symbol.to_string()
    }


}