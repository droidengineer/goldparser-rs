use std::collections::HashMap;

use crate::engine::Value;

pub mod parser;

pub use parser::GOLDParser;

#[derive(Debug)]
pub struct CallFrame {
    pub locals: HashMap<String,Value>,
}
impl CallFrame {
    const GLOBAL_SCOPE: &str = "GLOBAL";

    pub fn new() -> Self {
         CallFrame { locals: HashMap::new() }
    }
    pub fn get_local(&self, name: &str) -> Option<&Value> {
        self.locals.get(name)
    }
    pub fn set_local(&mut self, name: &str, value: Value) {
        self.locals.insert(String::from(name), value);
    }
}