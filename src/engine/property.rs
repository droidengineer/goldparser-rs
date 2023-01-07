//! Property Record
//! 
//! 

use utf16string::{WString, LE};

use super::Utf16;



/// Property records occur at the beginning of the file and contain information
///  about the grammar as well as attributes that affect how the grammar functions. 
/// The record is preceded by a byte field that contains the value 112, the ASCII code 
/// for the letter 'p'. The record contains an index, the property name, and its 
/// associated value. The idea is to allow additional information to be added in the future. 
/// This may include more information about the grammar and/or user-defined meta-data.
#[derive(Debug)]
pub struct PropertyRecord {
    pub index: u16,
    pub name: Utf16,
    pub value: Utf16,
}
impl PropertyRecord {
    pub fn new(index: u16, name: Utf16, value: Utf16) -> Self {
        PropertyRecord {
            index, name, value
        }
    }

}

impl std::fmt::Display for PropertyRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let disp = format!("@{:04X} {} = {}",self.index, self.name.to_utf8(),self.value.to_utf8());
        write!(f,"{}", disp)
    }
}