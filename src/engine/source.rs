//! Source Reader
//! 

use crate::engine::Position;


#[derive(Default)]
pub struct SourceReader {
    pub src: Vec<char>,
    pub buf: String,
    pub pos: Position,
    bufpos: usize,
}


impl SourceReader {
    pub fn new(source: String) -> Self {
        let src = source.chars().collect();
        SourceReader {src, buf: String::new(), pos: Position::default(), bufpos: 0 }
    }
    /// Operates on the lookahead buffer `buf`. Non-consuming.
    /// Will autoload buffer from source `src` if needed
    pub fn lookahead(&mut self, count: usize) -> char {
        if self.src.len() == 0 { panic!("Attemped lookahead on unloaded source. Load source file before calling lookahead()"); }
        // Autoload from src into buffer
        if count > self.buf.len() {
            println!("Pre-emptively reading {} chars",count-self.buf.len());
            for n in 0..count-self.buf.len() {
                match self.read() {
                    Some((i,c)) => self.buf.push(c),
                    None => panic!("Reading past src vector"),
                }
            }
            println!("buf: {}",self.buf);
        }
        self.buf.chars().nth(count-1).expect(format!("Problem indexing lookahead buf: {}",count).as_str())
    }
    /// Looks into `src` data. Does not change bufpos
    pub fn peek(&mut self, count: usize) -> char {

        if self.bufpos+count <= self.src.len() {
            return self.src[self.bufpos+count-1];        
        } else {
            return char::MAX;
        }
    }
    /// Works on lookahead buffer `buf`. Consumes characters from `buf`
    /// Adjusts `Position` pos to reflect this.
    fn consume_buf(&mut self, count: usize) {

    }
    /// Mutable read takes next and returns a `char` and the index it was found
    pub fn read(&mut self) -> Option<(usize,char)> {
        if self.bufpos >= self.src.len() { return None; }

        let oldpos = self.bufpos;
        self.inc_bufpos();
        Some((oldpos, self.src[oldpos]))
    }
    pub fn clear(&mut self) {
        self.src.clear();
        self.buf.clear();
        self.pos.clear();
        self.bufpos = 0;
    }

    pub fn load(&mut self, source: String) {
        self.clear();
        self.src = source.chars().collect();
        self.buf = String::new();
        self.pos = Position::default();
        self.bufpos = 0;
    }

    pub fn len(&self) -> usize {
        self.src.len()
    }
    pub fn to_string(&self) -> String {
        let ret = self.src.iter().collect::<String>();
        ret
    }

    #[inline(always)]
    fn inc_bufpos(&mut self) { self.bufpos += 1; }
    fn col(&self) -> usize {self.pos.col()}
    fn line(&self) -> usize {self.pos.line()}
    fn bufpos(&self) -> usize {self.bufpos}

}

// impl Default for SourceReader {
//     fn default() -> Self {
       
//     }
// }





#[cfg(test)]
mod test {
    use super::SourceReader;

    const SRC_TEST: &str = "LDI R0,23\nLDI R1,10\nMOV R0, R1";
    #[test]
    fn look() {
        let mut src = SourceReader::new(String::from(SRC_TEST));
        println!("{SRC_TEST}");
        println!("{:?}",src.src);
        println!("src[5] = \'{}\'", src.peek(5));
        println!("src[4..=5] = \'{:?}\'",src.src[4..=5].to_vec());
        println!("src[3] = \'{}\'", src.peek(3));
        println!("lookahead: 3 = {}", src.lookahead(3));
        println!("bufpos: {}",src.bufpos);
    }
    #[test]
    fn read() {
        let mut src = SourceReader::new(String::from(SRC_TEST));
        loop {
            match src.read() {
                Some((idx,ch)) => print!("{idx}:{ch} "),
                None => break,
            }
            print!("\n");
        }
    }
}