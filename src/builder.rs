//! Enhanced Grammar Table Builder
//! 
//! Use this module to build an `EGT` from a binary .egt file
//! Can be converted directly to a `EGT`

use std::{ffi::OsString, fs::File, io::Read,};

use enum_primitive::FromPrimitive;
use utf16string::{WString, LE, WStr, Utf16Error, BE};

use crate::{engine::{LogicalRecord, RecordType, EntryType, RecordEntry, property::PropertyRecord, counts::TableCountsRecord, states::{InitialStatesRecord, DFAEdge, DFAState, LALRAction, ActionType, LALRState}, charset::CharacterSetRecord, symbol::{SymbolTableRecord, SymbolType}, production::ProductionRecord}, egt::EnhancedGrammarTable};


#[derive(Debug)]
/// The `Builder`
pub struct Builder {
    bytes: Vec<u8>,
    pos: usize,
    records: Vec<LogicalRecord>,
}

impl Builder {
    pub fn new(file: OsString) -> Self {
        let mut file = File::open(file).unwrap();
        let mut buf = Vec::new(); //vec![0u8];
        match file.read_to_end(&mut buf) {
            Ok(sz) => println!("Read {sz} bytes."),
            Err(e) => panic!("Error reading file: {:?}",e),
        }
       // println!("Builder::new(): buf[0] = {} buf[1] = {}", buf[0], buf[1]);
        Builder {
            bytes: buf,
            pos: 0,
            records: Vec::new(),
        }
    }

    pub fn to_egt(&mut self) -> EnhancedGrammarTable {
        assert!(self.pos == self.bytes.len());

        let header = self.read_header();
        let mut egt = EnhancedGrammarTable::new(header);

        for record in &self.records {
            println!("{:?}", record.kind);
            match record.kind {
                RecordType::Multi => panic!(),
                RecordType::Property => {
                //for e in &record.entries  {
                    let i = &record.entries[0];
                    let n = &record.entries[1];
                    let v = &record.entries[2];
                    let r = PropertyRecord::new(
                        i.integer(),
                        n.wstring(),
                        v.wstring(),
                    );
                    println!("{}", r);
                    egt.properties.push(r);
                    //println!("");
                //}
                },
                RecordType::Counts => {
                    let s = &record.entries[0];
                    let c = &record.entries[1];
                    let r = &record.entries[2];
                    let d = &record.entries[3];
                    let l = &record.entries[4];
                    let g = &record.entries[5];
                    let rec = TableCountsRecord::new(
                        s.integer(), c.integer(), r.integer(),
                        d.integer(), l.integer(), g.integer()
                    );
                    println!("{}", rec);
                    egt.counts.push(rec);
                },
                RecordType::CharSet => {
                    let i = &record.entries[0];
                    let u = &record.entries[1];
                    let c = &record.entries[2];
                    let empty = &record.entries[3];
                    let mut r: Vec<(u16,u16)> = Vec::new();
                    let mut idx: usize = 4;
                    for _ in 0..c.integer() {
                        let a = &record.entries[idx].integer();
                        let b = &record.entries[idx+1];
                        r.push((*a, b.integer()));
                        idx += 2;
                    }
                    let rec = CharacterSetRecord::new(
                        i.integer(), u.integer(), c.integer(), r
                    );
                    println!("{}", rec);
                    egt.charset.push(rec);
                },
                RecordType::Symbol => {
                    let i = &record.entries[0];
                    let s = &record.entries[1];
                    let t = &record.entries[2];
                    if  t.integer() > SymbolType::Error as u16 { panic!("SymbolType out of range."); }
                    let k = SymbolType::from_u16(t.integer()).expect("Bad Symbol Type");

                    let rec = SymbolTableRecord::new(i.integer(),s.wstring(),k);

                    println!("{}", rec);
                    egt.symbols.push(rec);
                },
                RecordType::Group => todo!(),
                RecordType::Production => {
                    let i = &record.entries[0];
                    let h = &record.entries[1];
                    let _empty = &record.entries[2];
                    let mut r: Vec<u16> = Vec::new();
                    let mut idx = 3;
                    while idx < (record.num_entries-1) as usize {
                        let ex = &record.entries[idx];
                        r.push(ex.integer());
                        idx += 1;
                    }
                    let rec = ProductionRecord::new(
                        i.integer(), h.integer(), r
                    );
                    println!("{}", rec);
                    egt.productions.push(rec);
                },
                RecordType::InitState => {
                    let d = &record.entries[0];
                    let l = &record.entries[1];
                    let rec = InitialStatesRecord::new(d.integer(), l.integer());
                    println!("{}", rec);
                    egt.initial_states = rec;                 
                },
                RecordType::DFA => {
                    let i = &record.entries[0];
                    let s = &record.entries[1];
                    let ai = &record.entries[2];
                    let _reserved = &record.entries[3];
                    let mut edges: Vec<DFAEdge> = Vec::new();
                    let mut idx = 4;
                    println!("{} DFA[0] {:?} DFA[1] {:?} DFA[2] {:?}",record.num_entries, i, s, ai);
                    while idx < (record.num_entries - 1) as usize  {
                        let a = &record.entries[idx];
                        let b = &record.entries[idx+1];
                        let _empty = &record.entries[idx+2];
                        edges.push(DFAEdge { index: a.integer(), target_state_idx: b.integer()});
                        idx += 3;
                    }
                    let rec = DFAState::new(
                        i.integer(), s.bool(), ai.integer(), edges
                    );
                    println!("{}", rec);
                    egt.dfa_states.push(rec);        
                },
                RecordType::LALR => {
                    let i = &record.entries[0];
                    let r = &record.entries[1];
                    let mut actions: Vec<LALRAction> = Vec::new();
                    let mut idx = 2;
                    while idx < (record.num_entries - 1) as usize {
                        let a = &record.entries[idx];
                        let b = &record.entries[idx+1];
                        let c = &record.entries[idx+2];
                        let _ = &record.entries[idx+3];
                        let at = b.integer();
                        
                        actions.push(LALRAction { index: a.integer(), action: ActionType::from_u16(at).unwrap(), target: c.integer() });
                        idx += 4;
                    }
                    let rec = LALRState::new(i.integer(), actions);
                    println!("{}", rec);
                    egt.lalr_states.push(rec); 
                },
            }
        }

        egt
    }

    pub fn init(&mut self) {
        assert!(self.pos == 0);
        assert_eq!(self.read_string(), WString::from("GOLD Parser Tables/v5.0")); 
        
        let mut entries = 0;
        while self.pos < self.bytes.len() {
            // Read for 'M'
            let byte = self.read_byte();
            assert_eq!(byte, 77);
            //let entries = self.read_u16();
            let lrec = self.read_logical_record();
            entries += lrec.num_entries;
            //println!("{:?}",lrec);
            self.records.push(lrec);
        }
        println!("Total Records: {} Entries: {}", self.records.len(), entries);

    }

    /// Call after consuming byte 77 ('M') in stream.
    /// * Returns a `LogicalRecord`
    pub fn read_logical_record(&mut self) -> LogicalRecord {
        let entries = self.read_u16();
        let rectype = self.read_record_byte();
       // println!("@{} record: {:?} entries: {}", self.pos, rectype, entries);
        let mut lrec = LogicalRecord::new(entries,rectype);

        for _ in 1..lrec.num_entries {
            let byte = self.read_byte();
            let kind = EntryType::from_u8(byte).unwrap();
            let entry = self.read_entry(kind);
            // let e = self.read_record(rectype);
            lrec.entries.push(entry);
        }

        lrec
    }

    #[inline(always)]
    fn read_header(&mut self) -> WString<LE> {
        let opos = self.pos;
        self.pos = 0;
        let hdr = self.read_string();
        self.pos = opos;
        hdr
    }

    #[inline(always)]
    fn read_record_byte(&mut self) -> RecordType {
        self.read_byte();   // consume the 'b' byte
        let byte = self.read_byte();
        //println!("read_record_byte(): {:?}", RecordType::from_u8(byte).unwrap());
        RecordType::from_u8(byte).unwrap()
    }

    fn read_entry(&mut self, kind: EntryType) -> RecordEntry {
        match kind {
            EntryType::Empty => RecordEntry::Empty,
            EntryType::Byte => {
                let b = self.read_byte();
                let entry = RecordEntry::Byte(b);
                //println!("@{} => {:?}", self.pos, entry);          
                entry
            },
            EntryType::Boolean => {
                let b = self.read_byte();
                let mut entry = RecordEntry::Bool(b);
                //println!("@{} => {:?}", self.pos, entry);                      
                entry
            },
            EntryType::Integer =>  {
                let i = self.read_u16();
                let entry = RecordEntry::Integer(i);
                //println!("@{} => {:?}", self.pos, entry);          
                entry
            },
            EntryType::String => {
                let s = self.read_string();
                //let l = s.len() + 2;
                let entry = RecordEntry::String(s);
                //println!("@{} => {:?}", self.pos, entry);          
                entry
            },
        }
    }

    fn read_string(&mut self) -> WString<LE> {
        let start = self.pos;
        while self.read_u16() != 0 {

        }
        let str = WString::from_utf16le((&self.bytes[start..self.pos-2]).to_vec());
        //self.pos += 2; // adjust for 0x0000 terminal
        str.unwrap()
    }
    pub fn read_byte(&mut self) -> u8 {
        self.pos += 1;
        self.bytes[self.pos-1]
    }
    pub fn peek_byte(&self) -> u8 {
        self.bytes[self.pos]
    }
    pub fn peek_u16(&self) -> u16 {
        (self.bytes[self.pos] as u16) | (self.bytes[self.pos+1] as u16) << 8
    }
    pub fn read_u16(&mut self) -> u16 {
    //    println!("read_u16: {} {}", self.bytes[pos], self.bytes[pos+1]);
        let i = (self.bytes[self.pos] as u16) | (self.bytes[self.pos+1] as u16) << 8;
        self.pos += 2;
        i
    }

    pub fn load_egt_file(&mut self, file: OsString) -> Result<usize, std::io::Error> {
        let mut file = File::open(file).unwrap();
        file.read_to_end(&mut self.bytes)
    }

}


#[cfg(test)]
pub mod test {
    use core::panic;
    use std::{path::PathBuf, fs::File, io::Read, borrow::Borrow};

    use enum_primitive::FromPrimitive;
    use utf16string::WString;

    use crate::engine::{RecordType, LogicalRecord, EntryType, RecordEntry};

    use super::Builder;

    const FILE_NAME: &str = r"D:\Users\Gian\prog\repos\RUST\goldparser-rs\.ref\goldparser-test.egt";



    #[test]
    fn utf_test() {
        let mut file = File::open(FILE_NAME).unwrap();
        let mut buf = [0u8;2];
        // file.read_exact(&mut buf);
        // println!("{:?}",buf);

        let mut bufend = Vec::new();
        file.read_to_end(&mut bufend);
        println!("{} {}",bufend.pop().unwrap(), bufend.pop().unwrap());
        println!("{} {}",bufend[0], bufend[1]);
    }

    #[test]
    fn read_header() {
        let mut bldr = gen_builder();
        println!("Read {} bytes.", bldr.bytes.len());
        //println!("bytes[0] = {} bytes[1] = {}", bldr.bytes[0], bldr.bytes[1]);
        let header = bldr.read_string();
        //print!("header: {:?} => {:?}", header, header.as_bytes());
        assert_eq!(header, WString::from("GOLD Parser Tables/v5.0"));
    }
    #[test]
    fn init() {
        let mut bldr = gen_builder();
        bldr.init();
    }
    #[test]
    fn to_egt() {
        let mut bldr = gen_builder();
        bldr.init();

        let egt = bldr.to_egt();
    }
    #[test]
    fn read_logical_record() {
        let mut bldr = gen_builder();
        let header = bldr.read_string();
        println!("header: {}", header.to_string());
        let mut entries = 0;
        while bldr.pos < bldr.bytes.len() {
            let byte = bldr.read_byte();
            if byte == 77 {
                let lrc = bldr.read_logical_record();
                println!("{:?} : {}", lrc.kind, lrc.num_entries-1);
                entries += lrc.num_entries;
            }
        }
        println!("Total Entries: {}", entries);
    }
    #[test]
    fn read_logical_record_test() {
        let mut bldr = gen_builder();
        let hdr = bldr.read_string();
        //let mut pos = hdr.len() + 2;
        let mut byte = bldr.read_byte();
        assert_eq!(byte, 77);
        let entries = bldr.read_u16();
        println!("@{} record: {:?} entries: {}", bldr.pos, RecordType::from_u8(byte).unwrap(), entries);
        // advance position one + two bytes
        //pos += 3;
        byte = bldr.read_byte();
        byte = bldr.read_byte();
        let rectype: RecordType = RecordType::from_u8(byte).unwrap();
        println!("@{} => {:?}", bldr.pos, rectype);
        let mut lrec = LogicalRecord::new(entries,rectype);
        //pos += 2;
        match rectype {
            RecordType::Property => {  
                for n in 1..lrec.num_entries {
                    byte = bldr.read_byte();
                    let kind = EntryType::from_u8(byte).unwrap();
                    let entry = bldr.read_entry(kind);
                    lrec.entries.push(entry);
                }      
                //println!("@{} => {:?}", pos, rectype);
            },
            RecordType::Counts => {        println!("@{} => {:?}", bldr.pos, rectype);},
            RecordType::CharSet => {        println!("@{} => {:?}", bldr.pos, rectype);},
            RecordType::Symbol => {        println!("@{} => {:?}", bldr.pos, rectype);},
            RecordType::Group => {        println!("@{} => {:?}", bldr.pos, rectype);},
            RecordType::Production => {        println!("@{} => {:?}", bldr.pos, rectype);},
            RecordType::InitState => {        println!("@{} => {:?}", bldr.pos, rectype);},
            RecordType::DFA => {        println!("@{} => {:?}", bldr.pos, rectype);},
            RecordType::LALR => {        println!("@{} => {:?}", bldr.pos, rectype);},

            _ => panic!("unknown record type")
        }
        println!("Logical Record: {:#?}", lrec);
    }

    pub fn gen_builder() -> Builder {
        let file = PathBuf::from(FILE_NAME);
        let bldr = Builder::new(file.into_os_string());
        bldr
    }

}