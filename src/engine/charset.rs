//!
//! 

use std::{ops::RangeInclusive};


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
//TODO
impl std::fmt::Display for CharacterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let mut disp_str = String::new();
        // for i in &self.0 {
        //     //let rsc = decode_utf16(iter)
        //     //write!(f,"{:?}",i)
        //     disp_str.push_str(format!("{:?}",i).as_str());
        // }
        // Err(Error)
        let mut catstr = String::new();
        let mut charset = self.0.clone();
        charset.iter_mut().map(|r| {
            let mut ch = r.next();
            while ch.is_some() {
                let c = ch.unwrap();
                catstr.push(char::from_u32(c as u32).unwrap());
                ch = r.next();
            }
       }).count();
       write!(f,"{}",catstr)

    }
}

//pub struct CharacterSetTable(Vec<CharacterSet>);
// impl Default for CharacterSet {
//     fn default() -> Self {
//         CharacterSet(vec![])
//     }
// }
//impl<T:Display+Clone+Default> Table for



#[cfg(test)]
pub mod test {
    use std::char::decode_utf16;

    use super::{CharacterSet, CharacterRange};
    static mut rs: [u16;11] = [0;11]; //[86,112,100,103,9,11,12,32,160,0];


    #[test]
    fn add() {
    }
    
    #[test]
    fn default() {
        let charset = gen_charset();

        println!("{}",charset);
    }
    
    fn gen_charset() -> CharacterSet {
        let mut ranges: Vec<CharacterRange> = vec![];

        unsafe {rs[0] = 86; rs[1] = 112; rs[2] = 100; rs[3] = 103; rs[4] = 9; rs[5] = 11;
        rs[6] = 12; rs[7] = 32; rs[8] = 160; rs[9] = 0; rs[10] = 33;
        let rsc = decode_utf16(rs)
            .map(|r| r.map_err(|e| e.unpaired_surrogate()))
            .collect::<Vec<_>>();
        println!("{:?}",rsc);
        let a = rs[4];
        ranges.insert(0, a..=a); println!("range added {a}..={a}");
        let a = rs[5];
        let b = rs[6];
        ranges.insert(1, a..=b);
        let a = rs[7];
        ranges.insert(2, a..=a);
        let a = rs[8];
        ranges.insert(3, a..=a);}
        CharacterSet::new_with(ranges,0)
    }

}