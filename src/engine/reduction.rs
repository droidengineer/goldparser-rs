//! Reduction
//! 
//! This structure is used by the engine to hold a reduced rule. A reduction contains
//! a list of `Token`s corresponding to the `ProductionRule` it represents. The `Reduction`
//! structure is important since it is used to store the actual source program parsed by the Engine
//! 

use super::{token::Token, ProductionRule, Value};


/// This structure is used by the engine to hold a reduced rule. A reduction contains
/// a list of `Token`s corresponding to the `ProductionRule` it represents. The `Reduction`
/// structure is important since it is used to store the actual source program parsed by the Engine
pub struct Reduction {
    pub tokens: Vec<Token>, //tokenlist
    pub rule: ProductionRule,

}
impl Reduction {
    pub fn new() -> Self {

        Reduction {
            tokens: Vec::new(),
            rule: ProductionRule::default(),

        }

    }
    pub fn with_capacity(size: usize) -> Self {
        Reduction { tokens: Vec::with_capacity(size), rule: ProductionRule::default() }
    }
}

impl From<Vec<Token>> for Reduction {
    fn from(value: Vec<Token>) -> Self {
        let mut ret = Reduction::with_capacity(value.len());
        ret.tokens = value;

        ret
    }
}
