//! Reduction
//! 
//! This structure is used by the engine to hold a reduced rule. A reduction contains
//! a list of `Token`s corresponding to the `ProductionRule` it represents. The `Reduction`
//! structure is important since it is used to store the actual source program parsed by the 
//! Engine.
//! 
//! When the parsing engine has read enough tokens to conclude a rule in the grammar is 
//! complete, it is 'reduced' and passed to the developer. Basically a 'reduction'  will 
//! contain the tokens which correspond to the symbols of the rule. Tokens can represent 
//! actual data read from the file (a.k.a. terminals), but also may contain objects as well. 
//! Since a reduction contains the terminals of a rule as well as the nonterminals (reductions 
//! made earlier), the parser engine creates a 'parse tree'   which contains a break down of 
//! the source text along the grammar's rules.
//! 
//! Essentially, when a reduction takes place, the GOLDParser object will respond with the 
//! `GPMessage` message, create a new `Reduction`, and then store it in the `current_reduction` 
//! property. This property can be read and then reassigned to another object if you decide 
//! to create your own custom reduction. However, this is not required since the parse tree will be c
//! reated using the produced Reduction.
//! 
//! The `Reduction` is designed to be general purpose - able to store any rule, but at 
//! a cost. Since it is general purpose, there is additional overhead required that would 
//! not be needed in a specialized object. Invariably there will be numerous instances of 
//! the Reduction object, so it was was designed to use as little memory as possible while 
//! maintaining basic functionality.
//! 
//! 
//! 
//! > From http://goldparser.org/engine/1/vb6/doc/object-reduction.htm
//! 
//! Devin Cook (http://www.DevinCook.com/GOLDParser)
//! Ralph Iden (http://www.creativewidgetworks.com), port to Java
//! Gian James (https://www.convolutedsystems.com), port to Rust

use super::{token::{StaticTokenStack, TokenStack}, Rule};
use crate::parser::RuleHandler;

pub trait Reducible {
    type Item;

    fn reduce(&mut self) -> Reduction;
}

// pub fn reduce<T:Default+Clone>(rule: &Production, tokens: ReductionStack) -> Reduction {
// //pub fn reduce<R: RuleHandler>(rule: &'static ProductionRule, tokens: TokenStack) -> Reduction {

//     Reduction { tokens, rule, tag: 0 }
// }

type ReductionStack = StaticTokenStack<1024>;

#[derive(Debug,Clone)]
/// Basically, a 'reduction' will contain the tokens which correspond to the
/// symbols of the rule.
/// This structure is used by the engine to hold a reduced rule. A reduction contains
/// a list of `Token`s corresponding to the `ProductionRule` it represents. The `Reduction`
/// structure is important since it is used to store the actual parsed source program.
pub struct Reduction {
    pub tokens: TokenStack, // 
    pub rule: &'static Rule,
    tag: u16,
}
impl Reduction {
    pub fn new(rule: &'static Rule, tokens: TokenStack) -> Self {
        //Reduction::with_capacity(tokens.len(), rule, tokens)
        Reduction {
            tokens,
            rule,
            tag: 0,
        }
    }
    // pub fn with_capacity(size: usize, rule: &Production, tokens: TokenStack) -> Self {
    //     let mut tok = Vec::with_capacity(size);
    //     tok.clone_from(&tokens);
    //     Reduction { tokens: tok, rule: rule.to_owned(), tag: 0 }
    // }
    pub fn reduce(&mut self) -> Reduction {
        todo!()
    }
    /// Returns the number of tokens that make up the body of the `Reduction`.
    /// This will always be equal to the number of symbols in the rule.
    pub fn token_count(&self) -> usize {
        self.tokens.len()
    }
}

// impl From<TokenStack> for Reduction<'_> {
//     fn from(value: TokenStack) -> Self {
//         let mut ret = Reduction::with_capacity(value.len(),&ProductionRule::default(),value);
//         ret.tokens = value;

//         ret
//     }
// }
