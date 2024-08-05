use scalar_types::Endian;
use std::io::{BufReader, Result, Write};
use std::env;

pub mod eff;

fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().collect();

    println!("# RE4_EFF_BLOB_RUST"); 
    println!("# Tool By Zatarita"); 
    println!("# Fork By JADERLINK"); 
    println!("# Version 1.0.0"); 

    let path = args.pop().unwrap();
    let file = args.pop().unwrap();
    let argument = args.pop().unwrap();

    if argument == "-e" { //extracao
        let file = std::fs::File::open(&file)?;
        let mut reader = BufReader::new(file);
        let mut x = eff::Eff::new(&mut reader, &Endian::Little(())).unwrap();
        x.write_to_text(std::path::Path::new(&path))?;
    }

    if argument == "-b" { //repack
        let mut effect_file = eff::Eff::read_from_text(std::path::Path::new(&file)).unwrap();
        std::fs::write(&path, effect_file.compile(&Endian::Little(())).unwrap());
    }  
    
     println!("# Finished!!!");

    Ok(())
}
