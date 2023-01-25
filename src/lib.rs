#[macro_use] extern crate enum_primitive;
extern crate num_traits;


pub mod engine;
pub mod parser;

pub use engine::Parser;
pub use parser::GOLDParser;
