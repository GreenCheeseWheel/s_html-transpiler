use std::fs;
use super::dom_tree::DOMTree;

pub struct Writer
{
    pub file_lines:String,
}

impl Writer
{
    pub fn new(lines:String) -> Self
    {
        Writer { 
            file_lines: lines, 
        }
    }

    pub fn compile_dom(&self)
    {

    }

    pub fn write_to(&self, file_name:&str) -> Result<bool, &str>
    {
        match fs::write(file_name, self.file_lines.as_str()){
            Ok(_) => return Ok(true),
            Err(_) => return Err("Unable to write file"),
        };
    }

}