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
    

    let output_file = match args.get(2) {
        Some(path) => path.to_owned(),
        None => {
            let mut file_name = args.get(1).unwrap().clone();
            let length = file_name.len();

            file_name = file_name[..length-5].to_string().to_owned();
            file_name.push_str(".html");
            file_name
        },
    };

    // This is just to have somewhat complete code, for now
    let file_writer = Writer::new(transpiled_file.get_root_val().unwrap() );
    
    file_writer.write_to(&output_file)
        .expect(format!("File of name {} could not be written", output_file).as_str());

}


