use std::{env, process};
pub mod processor;
use processor::{reader::Reader, transpiler::transpile, writer::Writer};

/* 
    Idea behind this cli is to write basic html using .txt files without knowing 
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

    let file_writer = Writer {
        file_lines: transpiled_file,   
    };
    
    file_writer.write_to("./hola.html").expect("Unable to write to file");
}


