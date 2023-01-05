//! Character Set Record
//! 
//! 





/// Each record describing a member in the `CharacterSetTable`u is preceded by a byte field 
/// containing the value 99 - the ASCII value of "c". This table is used by the DFA State Table 
/// to store the valid characters for each edge in the DFA state machine. The file will contain 
/// one of these records for each character set used in the table. The `TableCountsRecord`, which 
/// precedes any character set records, will contain the total number of entries.
pub struct CharacterSetTable {
    pub index: u16,
    pub unicode: u16,
    pub range_count: u16,
    pub reserved: u8,
    pub ranges: Vec<(u16, u16)>,
}
impl CharacterSetTable {
    pub fn new(index: u16, unicode: u16, range_count: u16, ranges: Vec<(u16,u16)>) -> Self {
        CharacterSetTable { index, unicode, range_count, reserved: 0, ranges } 
    }
}

impl std::fmt::Display for CharacterSetTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let disp = format!("@{:04X} Unicode: {:04X} Count: {} Ranges: {:?}",self.index, self.unicode, self.range_count, self.ranges);
        write!(f,"{}", disp)
    }
}

