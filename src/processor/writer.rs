use std::fs;

pub struct Writer
{
    pub file_lines:String,
}

impl Writer
{
    pub fn write_to(&self, file_name:&str) -> Result<bool, &str>
    {
        match fs::write(file_name, self.file_lines.as_str()){
            Ok(_) => return Ok(true),
            Err(_) => return Err("Unable to write file"),
        };
    }

}