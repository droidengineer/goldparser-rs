//! Enhanced Grammar Table Builder
//! 
//! Use this module to build an `EGT` for use in a grammar parser from a binary .egt file.
//! Can be converted directly to a `EGT`

use std::{fs, ffi::OsString, fs::{File,ReadDir}, path::Path, io::Read, ops::Deref,};

use enum_primitive::FromPrimitive;
use utf16string::{WString, LE};

use crate::engine::{ 
        charset::{CharacterRange, CharacterSet}, 
        //egt::{EnhancedGrammarTable, PropertyRecord, LexicalGroup, TableCounts}, 
        production::Rule, 
        states::{ActionType, DFAEdge, DFAState, LALRAction, LALRState}, 
        symbol::{Symbol, SymbolType}, 
        SymbolTable
};
use super::{EnhancedGrammarTable,TableCounts,PropertyRecord};

pub fn get_all_files_in_location(path: &Path) -> ReadDir {
    fs::read_dir(path).unwrap_or_else(|err| {
        error!("Problem reading directory {:?}, error: {}", path, err);
        panic!();
    })
}

#[derive(Debug)]
/// The `Builder`
/// reads binary grammar file (*.egt),creates logical records, builds EGT.
/// 
/// Ex: `let egt = Builder::new(file_name).to_egt();`
pub struct Builder {
    /// The raw bytes from the EGT file
    bytes: Vec<u8>,
    pos: usize,
    /// Collection of `LogicalRecord`s decoded from `bytes`
    records: Vec<LogicalRecord>,
    initialized: bool,
}

impl Builder {
    pub fn new(flname: OsString) -> Self {
        //let mut file = File::open(file).unwrap();
        let mut file = match File::open(flname) {
            Ok(f) => f,
            Err(e) => { // we shouldn't panic in new()
                panic!("{e}")
            }
        };
        
        let mut buf = Vec::new(); //vec![0u8];
        match file.read_to_end(&mut buf) {
            Ok(sz) => println!("Read {sz} bytes."),
            Err(e) => panic!("Error reading file: {:?}",e),
        }
       // println!("Builder::new(): buf[0] = {} buf[1] = {}", buf[0], buf[1]);
        let mut reval = Builder {
            bytes: buf,
            pos: 0,
            records: vec![],
            initialized: false,
        };
        reval.init();
        reval
    }

    /// turns `LogicalRecord`s in `self.records` into an `EnhancedGrammarTable`
    pub fn to_egt(&mut self) -> EnhancedGrammarTable {
        let header = self.read_header();
        let mut egt = EnhancedGrammarTable::new(header.to_string());
        //let records = self.records;
        for record in &self.records { //self.records.as_slice() {
            match record.kind {
                //RecordType::Multi => panic!(),
                RecordType::Property => {
                    let idx = record.entries[0].as_usize();
                    let name = record.entries[1].string();
                    let value = record.entries[2].string();
                    egt.properties.insert(idx,PropertyRecord::new(name,value));
                },
                RecordType::Counts => {
                    egt.counts = TableCounts::new(
                    record.entries[0].integer(), 
                    record.entries[1].integer(), 
                    record.entries[2].integer(), 
                    record.entries[3].integer(), 
                    record.entries[4].integer(), 
                    record.entries[5].integer());

                    // sets up our random access through array indexing here
                    egt.resize();
                },
                RecordType::CharSet => {
                    let i = record.entries[0].as_usize();  // index of this charset in CharacterSetTable
                    let _u = record.entries[1].integer();    // unicode plane
                    let c = record.entries[2].as_usize();   // number of ranges in this charset
                    let _empty = &record.entries[3];
                    
                    //let rec = CharacterSet::new();
                    let mut ranges: Vec<CharacterRange> = Vec::new();
                    let mut idx: usize = 4;
                    for _ in 0..c {
                        let a = record.entries[idx].integer();
                        let b = record.entries[idx+1].integer();

                        // let v = decode_utf16([a,b])
                        //     .map(|r| r.map_err(|e| e.unpaired_surrogate()))
                        //     .collect::<Vec<_>>();
                        // let v0 = v[0].unwrap();
                        // let v1 = v[1].unwrap();

                        ranges.push(CharacterRange::new(a,b));
                        idx += 2;
                    }
                    
                    // let rec = CharacterSet::new(r);
                    //DEBUG println!("{:?}", rec);
                    //egt.charset[i] = rec;
                    egt.charset[i] = CharacterSet::new_with(ranges,i as u16);
                    //egt.charset.insert(i,CharacterSet::new_with(r,i as u16));
                },
                RecordType::Symbol => {
                    let index = record.entries[0].as_usize();
                    let s = record.entries[1].string();
                    let t = record.entries[2].integer();
                    //if  index > SymbolType::Error as usize { panic!("SymbolType out of range."); }
                    let k = SymbolType::try_from(t).expect("Bad Symbol Type");

                    //let rec = Symbol::new(index,s,k);

                    //DEBUG println!("{}", rec);c
                    egt.symbols[index] = Symbol::new(index as u16,s.as_str(),k);
                },
                RecordType::Group => {},
                RecordType::Production => {
                    let index = record.entries[0].integer();
                    let h = record.entries[1].as_usize();
                    let _empty = &record.entries[2];
                    let mut symbols: Vec<Symbol> = vec![]; //Vec::with_capacity(record.num_entries as usize);
                    let mut idx = 3;
                    while idx < (record.num_entries-1) as usize {
                        let ex = record.entries[idx].as_usize();
                        let sym = egt.symbols[ex].clone();
                        //symbols[ex] = sym;
                        //symbols.insert(ex, sym);
                        symbols.push(sym);
                        idx += 1;
                    }

                    let head = egt.symbols[h].clone();
                    let rec = Rule::new_with(index,head,SymbolTable::from(symbols));
                    //println!("{:?}", rec);
                    egt.productions[index as usize] = rec;
                },
                RecordType::InitState => {
                    egt.dfa_init_state = record.entries[0].integer();
                    egt.lalr_init_state = record.entries[1].integer();
                    // let dfa = record.entries[0].integer();
                    // let lalr = record.entries[1].integer();
                    // let rec = InitialStatesRecord::new(dfa,lalr);
                    //DEGBUG println!("{}", rec);
                    //egt.initial_states = rec;                 
                },
                RecordType::DFA => {
                    let state_idx = record.entries[0].integer(); // index of this DFAState in DFAStateTable
                    let accepts_symbol = record.entries[1].bool(); // accept state
                    let ai = record.entries[2].as_usize(); // index into symbol table for accept symbol
                    let _reserved = &record.entries[3];
                    let mut edges: Vec<DFAEdge> = Vec::new();
                    let mut idx = 4;
                    //println!("{} DFA[0] {:?} DFA[1] {:?} DFA[2] {:?}",record.num_entries, i, s, ai);
                    while idx < (record.num_entries - 1) as usize  {
                        let a = record.entries[idx].as_usize();   // this edge's characterset index in CharacterSetTable
                        let b = record.entries[idx+1].integer(); // index of target state symbol
                        let _empty = &record.entries[idx+2];
                        let chars = egt.charset[a].clone();
                        edges.push(DFAEdge { chars, target_state: b});
                        idx += 3;
                    }
                    let mut sym: Symbol = Symbol::default();
                    if accepts_symbol {
                        sym = egt.symbols[ai].clone();
                    }
                    let rec = DFAState::new(
                        state_idx, accepts_symbol, sym, edges
                    );
                    // DEBUG println!("{}", rec);
                    //egt.dfa_states.insert(state_idx, rec);  
                    egt.dfa_states[state_idx as usize] = rec;      
                },
                RecordType::LALR => {
                    // Let's make an LALRState
                    // index into LALRStateTable for this state
                    let index = record.entries[0].integer(); 
                    let _empty = &record.entries[1];
                    let mut actions: Vec<LALRAction> = Vec::new();
                    let mut idx = 2;
                    // Add any actions associated with this state
                    while idx < (record.num_entries - 1) as usize {
                        let a = record.entries[idx].as_usize(); // symbol index
                        let b = record.entries[idx+1].integer();   // action
                        let c = record.entries[idx+2].integer();  // target index
                        let _ = &record.entries[idx+3]; // empty
                        let symbol = egt.symbols[a].clone();
                        let action = ActionType::from_u16(b).unwrap();
                        actions.push(LALRAction { symbol, action, target_idx: c });
                        idx += 4;
                    }
                    let rec = LALRState::new(index, actions);
                    // DEBUG println!("{}", rec);
                    //egt.lalr_states.insert(index, rec); 
                    egt.lalr_states[index as usize] = rec;
                },
            }
        }
        // validate egt (table counts, etc)

        egt
    }

    /// parses byte buffer and populates `Builder.records`
    fn init(&mut self) {
        assert!(self.pos == 0);
        assert_eq!(self.read_string(), WString::from("GOLD Parser Tables/v5.0")); 
        
        while self.pos < self.bytes.len() {
            // Read for 'M'
            let byte = self.read_byte();
            assert_eq!(byte, 77);
            let lrec = self.read_logical_record();
            
            self.records.push(lrec);
        }
        self.initialized = true;
    }

    /// Call after consuming byte 77 ('M') in stream.
    /// * Returns a `LogicalRecord`
    fn read_logical_record(&mut self) -> LogicalRecord {
        let entries = self.read_u16();
        let rectype = self.read_record_byte();
       // println!("@{} record: {:?} entries: {}", self.pos, rectype, entries);
        let mut lrec = LogicalRecord::new(entries,rectype);

        for _ in 1..lrec.num_entries {
            let byte = self.read_byte();
            let kind = EntryType::from_u8(byte).unwrap();
            let entry = self.read_entry(kind);
            
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
                RecordEntry::Byte(b)
            },
            EntryType::Boolean => {
                let b = self.read_byte();
                RecordEntry::Bool(b)
            },
            EntryType::Integer =>  {
                let i = self.read_u16();
                RecordEntry::Integer(i)
            },
            EntryType::String => {
                let s = self.read_string();
                //let l = s.len() + 2;
                RecordEntry::String(s)
            },
        }
    }

    fn read_string(&mut self) -> WString<LE> {
        let start = self.pos;
        while self.read_u16() != 0 {

        }
        let str = WString::from_utf16le((self.bytes[start..self.pos-2]).to_vec());
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


enum_from_primitive! {
    #[derive(Debug,Copy,Clone, PartialEq, Eq, Hash)]
    #[repr(u8)]
    /// Each record structure consists of a series of entries which, in turn, can hold any number of data types. 
    /// Preceding each entry is an identification byte which denotes the type of data which is stored. Based on 
    /// this information, the appropriate number of bytes and the manner in which they are read can be deduced.
    /// http://goldparser.org/doc/egt/structure-entry-overview.htm
    pub enum EntryType {
        /// The entry only consists of an identification byte containing the value 69; the ASCII value of 'E'. 
        /// This type of entry is used to represent a piece of information that has not been defined for reserved 
        /// for future use. It has no actual value and should be interpreted as a logical NULL.
        Empty = 69,     // 'E' u8
        /// A "byte" entry is preceded by a single byte containing the value 98; the ASCII value for 'b'. The next byte contains the actual information stored in the entry. This is a rather inefficient method for storing a mass number of bytes given that there is as much overhead as actual data. But, in the case of storing small numbers, it does save a byte over using an integer entry.
        Byte = 98,      // 'b' u8
        /// A Boolean entry is preceded by a byte containing the value 66; the ASCII value for 'B'. This entry is identical in structure to the Byte except the second byte will only contain a 1, for True, or a 0 for False.
        Boolean = 66,   // 'B' u8
        /// This is the most common entry used to store the Compiled Grammar Table information. Following the identification byte, the integer is stored using Little-Endian byte ordering. In other words, the least significant byte is stored first.
        Integer = 73, // 'I' u16
        /// A string entry starts with a byte containing the value 83, which is the ASCII value for "S". This is immediately followed by a sequence of 1 or more Unicode characters which are terminated by a null.
        String = 83, // 'S' u16..0_u16
    }
}
enum_from_primitive! {
    #[derive(Debug,Copy,Clone, PartialEq, Eq, Hash)]
    #[repr(u8)]
    pub enum RecordType {
        //Multi       = 77, // 'M'
        Property    = 112, // 'p'
        Counts      = 116,   // 't'
        CharSet     = 99,   // 'c'
        Symbol      = 83,    // 'S'
        Group       = 103,    // 'g'
        Production  = 82, // 'R'
        /// The `InitialStateRecord` only occurs once in the `EnhancedGrammarTable` file. 
        /// It will contain the initial states for both the DFA and LALR algorithms.  
        /// The record is preceded by a byte field contains the value 73, the ASCII code for the letter 'I'.
        InitState   = 73, // 'I'
        DFA         = 68,       // 'D'
        LALR        = 76,      // 'L'
    }
}

#[derive(Debug)]
pub enum RecordEntry {
    Empty,
    Byte(u8),
    Bool(u8),
    Integer(u16),
    String(WString<LE>),
}

impl RecordEntry {
    pub fn byte(&self) -> u8 {
        match self {
            RecordEntry::Byte(b) => *b,
            _ => panic!("RecordEntry::byte() {:?}", self)
        }
    }
    pub fn bool(&self) -> bool {
        match self {
            RecordEntry::Bool(b) => {
                *b != 0u8
            },
            _ => panic!()
        }
    }
    pub fn integer(&self) -> u16 {
        match self {
            RecordEntry::Integer(i) => { /* println!("integer(): {}", *i); */ *i},
            _ => panic!()
        }
    }
    #[inline(always)]
    pub fn as_usize(&self) -> usize { self.integer() as usize }
    pub fn string(&self) -> String {
        match self {
            RecordEntry::String(i) =>  {
                //let mut wstr:  WString<LE> = WString::from(i);
                //i.clone_into(&mut &wstr);
                
                //println!("string(): {}",i.to_string()); //, wstr.to_string());
                i.to_string() //wstr
            },
            _ => panic!()
        }
    }
    pub fn wstring(&self) -> WString<LE> {
        match self {
            RecordEntry::String(i) => {
                let rets = i.deref().to_string();
                WString::from(&rets)
               // let ret = i.clone();
               // let rret = i.clone_into(&mut ret.clone());
                //ret.deref().to_string()
            },
            _ => panic!("wstring(): error")
        }
    }
}

#[derive(Debug)]
pub struct LogicalRecord {
    pub num_entries: u16,
    pub kind: RecordType,
    pub entries: Vec<RecordEntry>,
}
impl LogicalRecord {
    pub fn new(num: u16, kind: RecordType) -> Self {
        LogicalRecord {
            num_entries: num,
            kind,
            entries: Vec::new(),
        }
    }
}




#[cfg(test)]
pub mod test {
    use core::panic;
    use std::{path::PathBuf, fs::File, io::Read, };

    use enum_primitive::FromPrimitive;
    use utf16string::WString;

    use crate::test::GP_SIMPLE_EGT;

    use super::{RecordType, LogicalRecord, EntryType, };
    use super::Builder;

    const FILE_NAME: &str = GP_SIMPLE_EGT;

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
    fn to_egt() {
        let egt = gen_builder().to_egt();
        assert_eq!(egt.header,"GOLD Parser Tables/v5.0");
        println!("--------------------------------------------");
        println!("{}",egt);
    }
    #[test]
    fn read_logical_record() {
        let mut bldr = gen_builder();
        let header = bldr.read_string();
        println!("header: {}", header.as_wstr());
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
        let _hdr = bldr.read_string();
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
                for _n in 1..lrec.num_entries {
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
        
        Builder::new(file.into_os_string())
    }

}