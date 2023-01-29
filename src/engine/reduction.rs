//! Reduction
//! 
//! This structure is used by the engine to hold a reduced rule. A reduction contains
//! a list of `Token`s corresponding to the `ProductionRule` it represents. The `Reduction`
//! structure is important since it is used to store the actual source program parsed by the Engine
//! 
//! Devin Cook (http://www.DevinCook.com/GOLDParser)
//! Ralph Iden (http://www.creativewidgetworks.com), port to Java
//! Gian James (https://www.convolutedsystems.com), port to Rust

use super::{token::Token, ProductionRule};
use crate::parser::RuleHandler;

pub trait Reducible {
    type Item;

    fn reduce(&mut self) -> Reduction;
}

pub fn reduce<R: RuleHandler>(rule: ProductionRule, tokens: Vec<Token>) -> Reduction {

    Reduction { tokens, rule, }
}

#[derive(Debug,Clone)]
/// This structure is used by the engine to hold a reduced rule. A reduction contains
/// a list of `Token`s corresponding to the `ProductionRule` it represents. The `Reduction`
/// structure is important since it is used to store the actual source program parsed by the Engine
pub struct Reduction {
    pub tokens: Vec<Token>, //tokenlist
    pub rule: ProductionRule,

}
impl Reduction {
    pub fn new(rule: ProductionRule, tokens: Vec<Token>) -> Self {
        Reduction {
            tokens,
            rule,
        }
    }
    pub fn with_capacity(size: usize, rule: ProductionRule, tokens: Vec<Token>) -> Self {
        let mut tok = Vec::with_capacity(size);
        tok.clone_from(&tokens);
        Reduction { tokens: tok, rule, }
    }
    pub fn reduce(&mut self) -> Reduction {
        todo!()
    }
}

// impl From<Vec<Token>> for Reduction<'_> {
//     fn from(value: Vec<Token>) -> Self {
//         let mut ret = Reduction::with_capacity(value.len(),&ProductionRule::default(),value);
//         ret.tokens = value;

//         ret
//     }
// }
