use std::collections::HashMap;

use crate::engine::Value;

pub mod parser;

pub use parser::GOLDParser;

#[derive(Debug,Clone)]
pub struct Scope {
    pub parent: String,
    pub name: String,
    pub locals: HashMap<String,Value>,
}
impl Scope {
    pub const GLOBAL_SCOPE: &str = "GLOBAL";

    pub fn new(name: &str, parent: &str) -> Self {
         Scope { parent: parent.to_string(), name: String::from(name), locals: HashMap::new() }
    }
    pub fn get_local(&self, name: &str) -> Option<&Value> {
        self.locals.get(name)
    }
    pub fn set_local(&mut self, name: &str, value: Value) {
        self.locals.insert(String::from(name), value);
    }
    pub fn clear_local(&mut self, name: &str) {
        self.locals.remove(name).expect("Couldn't remove {name}");
    }
    pub fn clear(&mut self) {
        self.locals.clear();
    }
    pub fn contains(&self, name: &str) -> bool {
        self.locals.contains_key(name)
    }
}
impl Default for Scope {
    fn default() -> Self {
        Self { 
            parent: "".to_string(), 
            name: String::from(Self::GLOBAL_SCOPE), 
            locals: HashMap::new() 
        }
    }
}