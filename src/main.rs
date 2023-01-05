#[macro_use] extern crate enum_primitive;
extern crate num_traits;

use std::path::PathBuf;

mod records;
mod egt;
mod builder;

use builder::Builder;

const FILE_NAME: &str = r"D:\Users\Gian\prog\repos\RUST\goldparser-rs\.ref\goldparser-test.egt";
const HEADER_OFFSET: usize = 48;


fn main() {
    // let file = PathBuf::from(FILE_NAME);
    // let mut bldr = Builder::new(file.into_os_string());

 
 
 
 
    // let header = &bytes[..HEADER_OFFSET];

    // println!("Header: {:?}", header);

    // println!("Record {:?}: \'{}\'",&bytes[HEADER_OFFSET], *&bytes[HEADER_OFFSET] as char);
    // POS = HEADER_OFFSET + 1;

    // println!("- Entries: {}", egt::read_u16());
    
    // let parser = |s| { s };
    // println!("{:?}", parser(&b"\x00\x03"[..]));
}


