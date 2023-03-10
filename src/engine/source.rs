//! Source Reader
//! 

use crate::engine::Position;


/// Responsible for storage and access of the source buffer, both as unicode `char`
/// and string represting same. It is naive and has no knowlege about the source.
#[derive(Default)]
pub struct SourceReader {
    pub src: Vec<char>,
    buf: String,
    pub pos: Position,  // line,col position
    bufpos: usize,      // absolute position
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
            debug!("Pre-emptively reading {} chars",count-self.buf.len());
            for _ in 0..count-self.buf.len() {
                match self.read() {
                    Some((_,c)) => self.buf.push(c),
                    None => return '', //panic!("Reading past src vector"),
                }
            }
            debug!("buf: \'{}\'",self.buf);
        }
        self.buf.chars().nth(count-1).expect(format!("Problem indexing lookahead buf: {}",count).as_str())
    }
    pub fn get_abs_pos(&self) -> usize { self.bufpos }
    pub fn get_buf_len(&self) -> usize { self.buf.len() }
    pub fn get_buf_slice_to(&self, end: usize) -> &str {
        &self.buf[0..end]
    }
    /// Looks into `src` data. Does not change bufpos
    pub fn peek(&mut self, count: usize) -> char {

        if self.bufpos+count <= self.src.len() {
            return self.src[self.bufpos+count-1];        
        } else {
            return char::MAX;
        }
    }
    /// Works on lookahead buffer `buf`. Consumes characters from lookahead `buf`
    /// Adjusts `Position` pos to reflect this.
    pub fn consume_buf(&mut self, count: usize) {
        trace!("consume_buf({count})");
        if count > 0 && count <= self.buf.len() {
            // adjust position
            self.buf.chars().for_each(|c| {
                if c == '\n' {  //0x0A {
                    if self.pos.col() > 1 {
                        self.pos.inc_line();
                    }
                } else if c == '\r' { //0x0D {
                    self.pos.inc_line();
                } else {
                    self.pos.inc_col();
                }
            });
            debug!("Pre-crop: \'{}\'",self.buf);
            // remove the characters
            let cropped = match self.buf.char_indices().skip(count).next() {
                Some((pos,_)) => self.buf.split_off(pos), // = &self.buf[pos..],
                None => String::new(),
            };
            self.buf = cropped;
            debug!("Post-crop: \'{}\'",self.buf);
        } else {
            error!("Buf len is {} but count is {count}",self.buf.len());
        }
    }
    /// Mutable read takes next and returns a `char` and the index it was found
    pub fn read(&mut self) -> Option<(usize,char)> {
        trace!("read()");
        if self.bufpos >= self.src.len() { return None; }

        let oldpos = self.bufpos;
        self.inc_bufpos();
        debug!("=> \'{}\'", self.src[oldpos]);
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
    //fn bufpos(&self) -> usize {self.bufpos}

}

impl From<String> for SourceReader {
    fn from(value: String) -> Self {
       Self::new(value)
    }
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