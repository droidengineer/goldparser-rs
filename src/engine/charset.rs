//! Character Set Record
//! 
//! Each record describing a member in the `CharacterSetRecord` is preceded by a byte field 
//! containing the value 99 - the ASCII value of "c". This table is used by the DFA State Table 
//! to store the valid characters for each edge in the DFA state machine. The file will contain 
//! one of these records for each character set used in the table. The `TableCountsRecord`, which 
//! precedes any character set records, will contain the total number of entries.
//! 
//! | Byte | Integer | Integer | Integer | Empty | Integer1 .. Integer2 |
//! | 'c'  |  index  | unicode |numrange | rsvd  |  start        end    |

use std::ops::{Range, RangeInclusive};

type  CharacterRange = Vec<RangeInclusive<u16>>;

#[derive(Debug,Default,Clone)]
/// Manages a vector of `Range`s
pub struct CharacterSet(CharacterRange);
impl CharacterSet {
    //const DEFAULT: CharacterRange = ;
    pub fn new(range: CharacterRange) -> Self { CharacterSet(Vec::from(range)) }
    pub fn add(&mut self, range: RangeInclusive<u16>) {
        self.0.push(range); //Range { start: range.0, end: range.1});
    }
    pub fn contains(&self, item: u16) -> bool {
        //let utfc = item.to_digit(10).expect("probs converting char->u32->u16") as u16;
        //let utfc = u32::from(item) as u16;
        for range in &self.0 {
            if range.contains(&item) { return true; }
        }
        false
    }
    
    pub fn ranges(&self) -> &CharacterRange { &self.0 }
}

impl std::fmt::Display for CharacterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //let disp = format!("@{:04X} Unicode: {:04X} Count: {} Ranges: {:?}",self.index, self.unicode, self.range_count, self.ranges);
        let mut disp = String::from("Range ");
        for range in &self.0 {
            let fmt = format!("{}-{} ", range.start(), range.end());
            disp.push_str(fmt.as_str());
        }
        write!(f,"{}", disp)
    }
}



#[cfg(test)]
pub mod test {
    use super::{CharacterSet, CharacterRange};


    #[test]
    fn add() {
        let mut ranges = CharacterRange::new();
        ranges.insert(0, 86..=86);
        ranges.insert(1, 112..=112);
        ranges.insert(2, 100..=103);
        let mut chars = CharacterSet::new(ranges);
        chars.add(33..=33);
        assert!(chars.ranges().len() == 4);
    }
    #[test]
    fn contains() {
        let charset = gen_charset();
        //let ch = char::from_u32(50).expect("contains??");
        println!("{:?}",charset);
        assert!(charset.contains(9));
        assert!(charset.contains(180) == false);
    }
    #[test]
    fn default() {
        let charset = gen_charset();
        let mut defset = CharacterSet::default();
        println!("{:?}",charset);
        println!("{:?}",defset);
        assert!(defset.0.is_empty());
        defset.add(0..=0);
        println!("{:?}",defset);
        assert!(!defset.0.is_empty());

    }

    fn gen_charset() -> CharacterSet {
        let mut ranges = CharacterRange::new();
        ranges.insert(0, 9..=9);
        ranges.insert(1, 11..=12);
        ranges.insert(2, 32..=32);
        ranges.insert(3, 160..=160);
        CharacterSet::new(ranges)
    }

}