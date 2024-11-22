//! Token
//! 
//! While the `Symbol` represents an class of terminals and non-terminals, the
//! `Token` represents an individual piece of information.
//! Tokens can represent actual data read from the file (a.k.a. terminals), but 
//! also may contain reductions (reduced nonterminals) as well.

//use utf16string::{WString, LE};

use crate::engine::SymbolType;

use super::{reduction::Reduction, FixedStack, Position, Stack, Symbol};

pub trait TokenType {}
impl<T> TokenType for T {}

pub trait TerminalToken: TokenType {
    fn lalr_state(&self) -> u16;
    fn parent(&self) -> &Symbol;
    fn text(&self) -> &str;
    fn position(&self) -> Position;
}

pub trait NonTerminalToken: TokenType {
    fn rule(&self) -> super::Rule;
    fn tokens(&self) -> Vec<impl TokenType>;
}

#[derive(Debug,Default,Clone)]
/// Used to represent and organize parsed data.
///
/// Unlike `Symbol`s, which are used to represent a category of terminals and
/// non-terminals, a `Token` represents instances of those symbols. For instance, the 
/// common "identifier" is a specific type of `Symbol`, but can exist in various forms such
/// as "Value1", "cat", or "Sacramento", etc.
/// 
/// Information that is read from the source text/file is stored into the `text` property
/// which can be modified by the developer.
/// Contains:
/// * symbol is a `Symbol` representing the `Token`'s parent symbol
/// * text is a `String` that is UTF-8 and Unicode
/// * reduction an optional *NonTerminal* `Option<Reduction>`
// pub struct TokenOld {
//     /// The `Symbol` that generated this `Token`. Sometimes called parent.
//     pub symbol: Symbol,
//     /// String from the source file that generated this `Token`
//     /// For a `Token` created by reduction, this is empty
//     /// TODO Change to &str
//     pub text: String,
//     /// reduction
//     // associated with this `Token` if a `Reduction` if present
//     pub reduction: Option<Reduction>,
//     pub lalr_state: u16,
//     /// `Position` can represent either the line/col of this token or the
//     /// start and stop of a span referencing the absolute bufpos.
//     pub pos: Position,
// }

// `T` can be a String or `Reduction`
// If terminal token -> String, if nonterminal -> Reduction
pub struct Token {
    pub symbol: Symbol,
    pub data: Option<Reduction>,
    pub lalr_state: u16,
    pub pos: Position,
    text: String,
}
impl Token {
    pub fn new(symbol: Symbol, ) -> Self {
        Self {
            symbol,
            data: None,
            lalr_state: 0,
            pos: Position(0, 0),
            text: "".to_string(),
        }
    }
    pub fn has_reduction(&self) -> bool { self.data.is_some()}
    pub fn reduction(&self) -> Option<&Reduction> { self.data.as_ref() }   
    pub fn append_text(&mut self, data: &str) { self.text.push_str(data);}

    #[inline(always)]
    pub fn is_nonterminal(&self) -> bool { self.symbol.kind == SymbolType::NonTerminal }
    #[inline(always)]
    pub fn token_type(&self) -> SymbolType {
        self.symbol.kind
    }
    /// The name of the token. Equivalent to the parent symbol's name.
    #[inline(always)]
    pub fn name(&self) -> &str {
        &self.symbol.name
    }
    
    pub fn data(&mut self, data: Reduction) -> &mut Self { self.data = Some(data); self}
    pub fn text(&mut self, data: &str) -> &mut Self { self.text = data.to_owned(); self}
}

impl TerminalToken for Token {
    #[inline(always)]
    fn lalr_state(&self) -> u16 { self.lalr_state }
    #[inline(always)]
    fn parent(&self) -> &Symbol { &self.symbol }
    #[inline(always)]
    fn text(&self) -> &str { &self.text }
    #[inline(always)]
    fn position(&self) -> Position { self.pos }
}
pub type TokenStack = Stack<Token>;
pub type StaticTokenStack<const N: usize> = FixedStack<Token,N>;

// impl TokenOld {
//     pub fn new(symbol: Symbol, text: String) -> Self {
//     #[inline(always)]
//     pub fn has_reduction(&self) -> bool {
//         self.reduction.is_some()
//     }
//     pub fn set_reduction(&mut self, reduction: &Reduction) {
//         self.reduction = Some(reduction.to_owned());
//     }
//     #[inline(always)]
//     /// You should always call `has_reduction` before this or risk panic
//     pub fn reduction(&self) -> &Reduction {
//         //match self.reduction {
//         // if let Some(r) = &self.reduction {
//         //     return r;
//         // } else {
//         //     return 
//         // }
//         self.reduction.as_ref().unwrap()
//     }
// }


// /// Create a `Token` from a `Reduction`
// impl From<Reduction> for TokenOld {
//     fn from(value: Reduction) -> Self {
//         todo!()
//     }
// }