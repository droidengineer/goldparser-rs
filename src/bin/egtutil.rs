
// #[macro_use] extern crate enum_primitive;
// extern crate num_traits;
#[macro_use] extern crate log;

use std::{env, path::{PathBuf,Path}, process};

use goldparser::{engine::{Builder, EnhancedGrammarTable}};

pub fn curr_exe_location() -> PathBuf {
    env::current_exe().unwrap_or_else(|err| {
        error!("Problem getting current executable location: {}", err);
        panic!();
    })
}
// pub fn get_all_files_in_location(path: &Path) -> ReadDir {
//     fs::read_dir(path).unwrap_or_else(|err| {
//         error!("Problem reading directory {:?}, error: {}", path, err);
//         process::exit(1);
//     })
// }

const PROG_INFO: &str = "
egtutils v1.0.0 : Enhanced Grammar Table Utility Program
Usage: egtutils <command> <egt_file>
where <command> is:
  symbols     Dump the symbol table
  rules       Dump the production rules
  properties  Dump the grammar\'s properties
  dfa         Dump the DFA State Table
  lalr        Dump the LALR State Table
  charset     Dump the character set table
  group       <TBD>
  interactive Run EGT REPL Shell
  about       About and Credits

<egt_file> is the path to the EGT file for your grammar.
e.g. egtutils rules mygrammar.egt
";


fn main() {
    env_logger::init();
    info!("Starting...");
    
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {println!("Wrong number of arguments.\n{}", PROG_INFO); process::exit(0);}
    let cmd = &args[1];
    let egt = gen_egt(&args[2]);
    println!("Grammar tables loaded.");

    match cmd.as_str() {
        "symbols" => println!("[Symbols]\n{}",egt.symbols),
        "rules" => println!("[Production Rules]\n{}",egt.productions.iter().map(|t| t.to_string()).collect::<String>()),
        "properties" => println!("[Properties]\n{}",egt.properties.iter().fold(String::new(),|mut acc,p| { acc.push_str(format!("{} = {}\n",p.name,p.value).as_str()); acc})),
        "dfa" => println!("[DFA State Table]\n{}",egt.dfa_states.iter().map(|t| t.to_string()).collect::<String>()),
        "lalr" => println!("[LALR State Table]\n{}",egt.lalr_states.iter().map(|t| t.to_string()).collect::<String>()),
        "charset" => println!("[Character Set Table]\n{}",egt.charset.iter().map(|t| t.to_string()).collect::<String>()),
        "counts" => println!("[Total Counts]\n{}",egt.counts),
        "group" => println!("[Group Table]\n{}","self.groups"),
        "interactive" => { interactive(&egt).expect("wtf");
        },

        _ => {println!("Unknown command {}.\n{}", cmd.as_str(), PROG_INFO); process::exit(0)}
    }

   // let mut parser = GOLDParser::new(FILE_NAME, SOURCE_NAME, true, true);
    
 //   parser.run();
}

// fn print_charsets(sets: &Vec<crate::CharacterSet>) -> String {

// }
use std::io;
fn interactive(egt: &EnhancedGrammarTable) -> io::Result<()>{
    print!("(B)rowse the Grammar Tables, (P)arse from source, or Parse (R)EPL [B/P/R/Quit]? ");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    match buf.to_uppercase().as_str() {
        "B" => todo!(),
        "P" => {
            print!("Source file: "); io::stdin().read_line(&mut buf)?;
            // TODO bring in `trim` and `case`
            //let parser = GOLDParser::new(egt, buf.as_str(), true, false);
            
        
        },
        "R" => {
            todo!()
        },
        "Q" => return Ok(()),

        _   => { println!("Unknown choice."); return Ok(()); }
    }





    Ok(())
}

fn gen_egt(file: &String) -> EnhancedGrammarTable {
    let egtfile = PathBuf::from(file);
    Builder::new(egtfile.into_os_string()).to_egt()
}


#[cfg(test)]
mod test {
    //use utf16string::LE;

    #[test]
    fn parse_test() {
        // let parser = |wc: &[u8]| { wc };
        // println!("{:?}", parser(&b"\x00\x03"[..])); 
    }
}

