mod engine;
mod builder;
mod egt;
mod parser;
mod token;

#[macro_use] extern crate enum_primitive;
extern crate num_traits;

use std::path::PathBuf;




const FILE_NAME: &str = r"D:\Users\Gian\prog\repos\RUST\goldparser-rs\.ref\goldparser-test.egt";
const HEADER_OFFSET: usize = 48;


fn main() {
    // let file = PathBuf::from(FILE_NAME);
    // let mut bldr = Builder::new(file.into_os_string());

    //let bldr = 
 
 
 
    // let header = &bytes[..HEADER_OFFSET];

    // println!("Header: {:?}", header);

    // println!("Record {:?}: \'{}\'",&bytes[HEADER_OFFSET], *&bytes[HEADER_OFFSET] as char);
    // POS = HEADER_OFFSET + 1;

    // println!("- Entries: {}", egt::read_u16());
    
    // let parser = |s| { s };
    // println!("{:?}", parser(&b"\x00\x03"[..]));
}

#[cfg(test)]
mod test {
    use utf16string::LE;

    #[test]
    fn parse_test() {
        // let parser = |wc: &[u8]| { wc };
        // println!("{:?}", parser(&b"\x00\x03"[..])); 
    }
}

