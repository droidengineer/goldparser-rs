
// #[macro_use] extern crate enum_primitive;
// extern crate num_traits;

use std::{env, path::PathBuf, process};

use goldparser::{GOLDParser, engine::{Builder, EnhancedGrammarTable}};



const PROG_INFO: &str = "
egtutils v1.0.0 : Enhanced Grammar Table Utility Program
Usage: egtutils <command> <egt_file>
where <command> is:
symbols     Dump the symbol table
rules       Dump the production rules
properties  Dump the EGT properties
dfa         Dump the DFA State Table
lalr        Dump the LALR State Table
charset     Dump the character set table
group       <TBD>
interactive Run EGT REPL Shell

<egt_file> is the path to the EGT file for your grammar.
e.g. egtutils rules mygrammar.egt
";


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {println!("Wrong number of arguments.\n{}", PROG_INFO); process::exit(0);}
    let cmd = &args[1];
    let egt = gen_egt(&args[2]);
    println!("Grammar tables loaded.");

    match cmd.as_str() {
        "symbols" => {
            println!("[Symbols]\n{}",egt.symbols.to_string());
        },
        "rules" => println!("[Production Rules]\n{}",egt.productions),
        "properties" => println!("[Properties]\n{}",egt.properties_as_string()),
        "dfa" => println!("[DFA State Table]\n{}",egt.dfa_states),
        "lalr" => println!("[LALR State Table]\n{}",egt.lalr_states),
        "charset" => println!("[Character Set Table]\n{}",egt.charset),
        "counts" => println!("[Total Counts]\n{}",egt.counts),
        "group" => println!("[Group Table]\n{}","self.groups"),
        "interactive" => { interactive(&args[2]).expect("wtf");

        },

        _ => {println!("Unknown command {}.\n{}", cmd.as_str(), PROG_INFO); process::exit(0)}
    }

   // let mut parser = GOLDParser::new(FILE_NAME, SOURCE_NAME, true, true);
    
 //   parser.run();
}

use std::io;
fn interactive(egt: &String) -> io::Result<()>{
    print!("(B)rowse the Grammar Tables, (P)arse from source, or Parse (R)EPL [B/P/R/Quit]? ");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    match buf.to_uppercase().as_str() {
        "B" => todo!(),
        "P" => {
            print!("Source file: "); io::stdin().read_line(&mut buf)?;
            // TODO bring in `trim` and `case`
            let parser = GOLDParser::new(egt.as_str(), buf.as_str(), true, false);
            
        
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
    let mut bldr = Builder::new(egtfile.into_os_string());
    bldr.to_egt()
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

