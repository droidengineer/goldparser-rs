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

use std::ops::{RangeInclusive};
use std::cmp;

type  CharacterRange = Vec<RangeInclusive<char>>;

#[derive(Debug,Default,Clone,PartialEq)]
/// Manages a vector of `Range`s
pub struct CharacterSet(CharacterRange);
impl CharacterSet {
    //const DEFAULT: CharacterRange = ;
    pub fn new(range: CharacterRange) -> Self { CharacterSet(Vec::from(range)) }
    pub fn add(&mut self, range: RangeInclusive<char>) {
        self.0.push(range); //Range { start: range.0, end: range.1});
    }
    pub fn contains(&self, item: char) -> bool {
        //let utfc = item.to_digit(10).expect("probs converting char->u32->u16") as u16;
        //let utfc = u32::from(item) as u16;
        for range in &self.0 {
            if range.contains(&item) { return true; }
        }
        false
    }
    pub fn as_strange(&self) -> String {
        format!("Ranges: {}\n", self.0.iter().map(|r| {
            format!("\'{}\'-\'{}\' ", r.start().escape_default(), r.end().escape_default())
        }).collect::<String>())       
    }
    pub fn ranges(&self) -> &CharacterRange { &self.0 }
    ///TODO Merges ranges and overlapping intervals
    // pub fn concat(&mut self) -> String {
    //     let mut catstr = String::new();
    //     for range in &self.0 {
    //         let (start, end) = range.into_inner();
    //         // if start == end {
    //         //     catstr.push(start);
    //         //     //return catstr;
    //         // }
    //     }
    //     // self.0.iter().map(|mut r| {
            

    //     // });
    //     catstr
    //     //let ranges = self.0.sort_by(|a,b| a.cmp(&b));
    //     //let ranges = self.0.concat().
    // }
    pub fn merge(&mut self, other: &RangeInclusive<char>) {
        todo!()
    }
}

impl std::fmt::Display for CharacterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //let disp = format!("@{:04X} Unicode: {:04X} Count: {} Ranges: {:?}",self.index, self.unicode, self.range_count, self.ranges);
        //TODO compress ranges into a string
        // write!(f,"{}",self.concat())
        let mut catstr = String::new();
        let mut range = self.0.clone();
        range.iter_mut().map(|r| {
            let mut ch = r.next();
            while ch != None {
                let c = ch.unwrap();
                catstr.push(c);
                ch = r.next();
            }
       }).count();
       write!(f,"{}",catstr)
    }
}

// use std::vec::Vec;
// impl PartialEq for CharacterSet {
//     fn eq(&self, other: &Self) -> bool {
        
//     }

//     fn ne(&self, other: &Self) -> bool {
//         !self.eq(other)
//     }
// }


#[cfg(test)]
pub mod test {
    use std::char::decode_utf16;

    use super::{CharacterSet, CharacterRange};
    static mut rs: [u16;11] = [0;11]; //[86,112,100,103,9,11,12,32,160,0];


    #[test]
    fn add() {
        let mut ranges = CharacterRange::new();
        unsafe {let rsc = decode_utf16(rs)
        .map(|r| r.map_err(|e| e.unpaired_surrogate()))
        .collect::<Vec<_>>();
        let a = rsc[0].unwrap();
        ranges.insert(0, a..=a);
        let a = rsc[1].unwrap();     
        ranges.insert(1, a..=a);
        let a = rsc[2].unwrap();
        let b = rsc[3].unwrap();
        ranges.insert(2, a..=b);
        let mut chars = CharacterSet::new(ranges);
        let a = rsc[10].unwrap();
        chars.add(a..=a);
        assert!(chars.ranges().len() == 4);}
    }
    #[test]
    fn contains() {
        let charset = gen_charset();
        //let ch = char::from_u32(50).expect("contains??");
        println!("{:?}",charset);
        assert!(charset.contains(char::from_u32(9_u32).unwrap()));
        assert!(charset.contains(char::from_u32(160_u32).unwrap()));
        assert!(charset.contains(char::from_u32(32_u32).unwrap()));        
        assert!(charset.contains(char::from_u32(180_u32).unwrap()) == false);
    }
    #[test]
    fn default() {
        let charset = gen_charset();
        let mut defset = CharacterSet::default();
        println!("{:?}",charset);
        println!("{:?}",defset);
        assert!(defset.0.is_empty());

        let c = char::from_u32(0).unwrap();
        defset.add(c..=c);
        println!("{:?}",defset);
        assert!(!defset.0.is_empty());

    }

    fn gen_charset() -> CharacterSet {
        let mut ranges = CharacterRange::new();

        unsafe {rs[0] = 86; rs[1] = 112; rs[2] = 100; rs[3] = 103; rs[4] = 9; rs[5] = 11;
        rs[6] = 12; rs[7] = 32; rs[8] = 160; rs[9] = 0; rs[10] = 33;
        let rsc = decode_utf16(rs)
            .map(|r| r.map_err(|e| e.unpaired_surrogate()))
            .collect::<Vec<_>>();
        let a = rsc[4].unwrap();
        ranges.insert(0, a..=a); println!("range added {a}..={a}");
        let a = rsc[5].unwrap();
        let b = rsc[6].unwrap();
        ranges.insert(1, a..=b);
        let a = rsc[7].unwrap();
        ranges.insert(2, a..=a);
        let a = rsc[8].unwrap();
        ranges.insert(3, a..=a);}
        CharacterSet::new(ranges)
    }

}