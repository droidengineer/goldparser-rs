//! CharacterSet
//! Ranges of valid characters for a DFA edge

use std::ops::RangeInclusive;


pub type CharacterRange = RangeInclusive<u16>;

#[derive(Debug,Clone,Default,PartialEq)]
pub struct CharacterSet(Vec<CharacterRange>,u16);
impl CharacterSet {
    pub fn new() -> CharacterSet {
        CharacterSet(vec![],0)
    }
    pub fn new_with(set: Vec<CharacterRange>, idx: u16) -> CharacterSet {
        CharacterSet(set,idx)
    }
    pub fn add(mut self, range: CharacterRange) {
        self.0.push(range);
    }
    pub fn contains(&self, char_code: u16) -> bool {
        for range in &self.0 {
            if range.contains(&char_code) {
                return true;
            }
        }
        false
    }
    pub fn ranges(&self) -> &Vec<CharacterRange> {&self.0}
    pub fn index(&self) -> u16 {self.1}
}
impl std::fmt::Display for CharacterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ranges = self.0.clone();
        let catstr = ranges.into_iter().map(|r| {
            let result: String = r
                .filter_map(|re| char::from_u32(re as u32))
                .collect();
            result
        }).collect::<String>();
        
        writeln!(f,"{:?}",catstr)
    }
}


#[cfg(test)]
pub mod test {
    use std::char::decode_utf16;

    use super::{CharacterSet, CharacterRange};
    static mut RS: [u16;11] = [86,112,100,103,9,11,12,32,160,0,33]; //[86,112,100,103,9,11,12,32,160,0];


    #[test]
    fn add() {
    }
    
    #[test]
    fn default() {
        let charset = gen_charset();

        println!("Index: {} Ranges: {}\n{}",charset.index(),charset.ranges().len(),charset);
    }
    
    fn gen_charset() -> CharacterSet {
        let mut ranges: Vec<CharacterRange> = vec![];

        unsafe {
        let rsc = decode_utf16(RS)
            .map(|r| r.map_err(|e| e.unpaired_surrogate()))
            .map(|r| r.unwrap())
            .collect::<Vec<_>>();
        println!("{:?}",rsc);

        let a = RS[4];
        ranges.insert(0, a..=a); println!("range added {a}..={a}");
        let a = RS[5];
        let b = RS[6];
        ranges.insert(1, a..=b);println!("range added {a}..={b}");
        let a = RS[7];
        ranges.insert(2, a..=a);println!("range added {a}..={a}");
        let a = RS[8];
        ranges.insert(3, a..=a);println!("range added {a}..={a}");}
        CharacterSet::new_with(ranges,0)
    }

}