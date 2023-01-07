//! Character Set Record
//! 
//! 

use std::ops::{Range, Index};

/// Manages a vector of `Range`s
pub struct CharacterSet(Vec<Range<u16>>);
impl CharacterSet {

    pub fn add(&mut self, range: (u16,u16)) {
        self.0.push(Range { start: range.0, end: range.1});
    }
    pub fn contains(&self, item: u16) -> bool {
        for range in &self.0 {
            if range.contains(&item) { return true; }
        }
        false
    }
    
    pub fn ranges(&self) -> &Vec<Range<u16>> { &self.0 }
}

impl From<CharacterSetRecord> for CharacterSet {
    fn from(rec: CharacterSetRecord) -> Self {
        let mut ranges: Vec<Range<u16>> = Vec::new();
        for r in rec.ranges {
            ranges.push(Range { start: r.0, end: r.1 });
        }
        CharacterSet(ranges)
    }
}

pub struct CharacterSetTable(Vec<CharacterSet>);
impl CharacterSetTable {
    pub fn add(&mut self, index: u16, chars: CharacterSet) {
        self.0[index as usize] = chars;
    }
}

impl Index<usize> for CharacterSetTable {
    type Output = CharacterSet;
    /// charset_table[0]
    fn index(&self, index: usize) -> &CharacterSet {
        &self.0[index]
    }
}


/// Each record describing a member in the `CharacterSetRecord` is preceded by a byte field 
/// containing the value 99 - the ASCII value of "c". This table is used by the DFA State Table 
/// to store the valid characters for each edge in the DFA state machine. The file will contain 
/// one of these records for each character set used in the table. The `TableCountsRecord`, which 
/// precedes any character set records, will contain the total number of entries.
/// 
/// | Byte | Integer | Integer | Integer | Empty | Integer1 .. Integer2 |
/// | 'c'  |  index  | unicode |numrange | rsvd  |  start        end    |
/// 
pub struct CharacterSetRecord {
    pub index: u16,
    pub unicode: u16,
    pub range_count: u16,
    pub reserved: u8,
    pub ranges: Vec<(u16,u16)>,
}
impl CharacterSetRecord {
    pub fn new(index: u16, unicode: u16, range_count: u16, ranges: Vec<(u16,u16)>) -> Self {
        CharacterSetRecord { index, unicode, range_count, reserved: 0, ranges } 
    }
}

impl std::fmt::Display for CharacterSetRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let disp = format!("@{:04X} Unicode: {:04X} Count: {} Ranges: {:?}",self.index, self.unicode, self.range_count, self.ranges);
        write!(f,"{}", disp)
    }
}

