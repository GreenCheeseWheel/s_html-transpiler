use std::{fs, path, ffi::OsStr};

#[derive(Debug)]
pub struct Reader
{
    pub file_path:String,
    file_lines: String,
}

impl Reader {
    pub fn instantiate(file_path:String) -> Result<Reader, &'static str>
    {
        let extension = path::Path::new(&file_path)
            .extension()
            .and_then(OsStr::to_str);        

        if extension.unwrap_or_default() != "srth"
        {
            return Err("Incorrect file extension provided");
        } 


        let reader_inst = Reader {
            file_path,
            file_lines: "".to_string()
        };

        Ok(reader_inst)
    }

    pub fn build(&mut self) -> Result<bool, &str>
    {
        let file_lines = if let Ok(data) = fs::read_to_string(&self.file_path) {
            data
        }
        else {
            return Err("File could not be read");
        };

        self.file_lines = file_lines;

        Ok(true)
    }

    pub fn get_lines(&self) -> &str
    {
        &self.file_lines.as_str()
    }

}