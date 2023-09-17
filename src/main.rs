use std::{env, process};
pub mod processor;
use processor::{reader::Reader, transpiler::transpile, writer::Writer, dom_tree::DOMTree};

/* 
    Idea behind this cli is to write basic html using .srth files without knowing 
    html much.
*/
fn main() {
    let args:Vec<String> = env::args().collect();
    let mut tree = DOMTree::new(30);
    
    DOMTree::traverse_and_append(&mut tree, &vec![DOMTree::new(31), DOMTree::new(40)], &30)
        .expect("Todo mal");

    DOMTree::traverse_and_append(&mut tree, &vec![DOMTree::new(2), DOMTree::new(4)], &40)
        .expect("Todo mal");

    dbg!("{}", tree);
    /* 
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
    */
}


