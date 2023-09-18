use std::{env, process};
pub mod processor;
use processor::{reader::Reader, transpiler::transpile, writer::Writer};

/* 
    Idea behind this cli is to write basic html using .srth files without knowing 
    html much.
*/
fn main() {
    
    let args:Vec<String> = env::args().collect();


    let mut file_reader = match Reader::instantiate(String::from(&args[1])) {
        Ok(rdr) => rdr,
        Err(msg) => panic!("{}", msg),
    }; 


    if let Err(msg) = file_reader.build() {
        println!("{}", msg);
        process::exit(-1);
    };


    let transpiled_file = transpile(file_reader.get_lines());
    
    dbg!("DOMTree: \n {}", transpiled_file);
    //let file_writer = Writer::new(transpiled_file, "<html>".to_string());
    
    
}


