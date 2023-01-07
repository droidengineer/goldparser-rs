//! Group Record
//! 
//! http://goldparser.org/doc/egt/record-group.htm
//! 


/// Group records occur after all the Symbol Records. The record is preceded 
/// by a byte field that contains the value 103, the ASCII code for the letter 'g'
pub struct GroupRecord {
    /// The table index of the group in the `GroupTable` Values are 0-indexed
    pub index: u16,
    /// The name of the group
    pub name: String,
    /// Index in the `SymbolTable` of the group's container symbol
    pub container_idx: u16,   
    /// Index in the `SymbolTable` of the group's start symbol
    pub start_idx: u16,
    /// Index in the `SymbolTable` of the group's end symbol
    pub end_idx: u16,
    /// `AdvanceMode` indicating how the group will advance
    pub advance_mode: AdvanceMode,
    /// `EndingMode` indicating how group will handle the end symbol
    pub ending_mode: EndingMode,
    /// Empty
    pub reserved: u8,
    /// How many nested group indices occur at the end
    pub nesting_count: u16,
    /// Nested 1..nesting_count
    pub nested: Vec<u16>,
}
impl GroupRecord {
    const CODE: u8 = 103; //'g';
    
}

/// `AdvanceMode`
pub enum AdvanceMode {
    /// The group will advance a token at a time
    Token,
    /// The group will advance by one character at a time
    Character,
}

/// `EndingMode`
pub enum EndingMode {
    /// The ending symbol will be left on the input queue
    Open,
    /// The ending symbol will be consumed
    Closed,
}